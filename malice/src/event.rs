//! Structs and `bitflags` related to a given event

use std::slice::Iter;

use failure::Error;
use futures::prelude::*;
use nom;
use nom::combinator::map;
use nom::number::complete::*;
use nom::sequence::tuple;

use root_io::core::parsers::{parse_custom_mantissa, parse_tobjarray_of_tnameds};
use root_io::stream_zip;
use root_io::tree_reader::Tree;

use crate::primary_vertex::PrimaryVertex;
use crate::track::{Flags, ItsClusters, Track, TrackParameters};

bitflags! {
    /// Triggers are low level qualifier of an event. One event may "fire" several triggers.
    pub struct TriggerMask: u64 {
        /// Exact definition may vary from run-to-run. Should be used as the default trigger
        const MINIMUM_BIAS = 0b0000_0001;
        /// Exact definition vary from run-to-run. Marks an event with very high activity
        const HIGH_MULT =    0b0000_0010;
    }
}

/// A model for a subset of an event as stored in the published data
#[derive(Debug, PartialEq)]
pub struct Event {
    primaryvertex_alivertex_fposition: (f32, f32, f32),
    pub(crate) primaryvertex_alivertex_fncontributors: i32,
    pub(crate) aliesdrun_frunnumber: i32,
    pub(crate) aliesdrun_ftriggerclasses: Vec<String>,
    pub(crate) aliesdheader_ftriggermask: u64,
    pub(crate) tracks_fx: Vec<f32>,
    pub(crate) tracks_fp: Vec<TrackParameters>,
    pub(crate) tracks_falpha: Vec<f32>,
    pub(crate) tracks_fflags: Vec<Flags>,
    pub(crate) tracks_fitschi2: Vec<f32>,
    pub(crate) tracks_fitsncls: Vec<i8>,
    pub(crate) tracks_fitsclustermap: Vec<ItsClusters>,
    pub(crate) tracks_ftpcchi2: Vec<f32>,
    pub(crate) tracks_ftpcncls: Vec<u16>,
}

/// Iterator over [`Track`](struct.Track.html)s
pub struct TracksIter<'e> {
    pub(crate) x: Iter<'e, f32>,
    pub(crate) p: Iter<'e, TrackParameters>, // fn(&[f32; 5]) -> TrackParameters>,
    pub(crate) alpha: Iter<'e, f32>,
    pub(crate) flags: Iter<'e, Flags>,
    pub(crate) its_chi2: Iter<'e, f32>,
    pub(crate) its_ncls: Iter<'e, i8>,
    pub(crate) its_clustermap: Iter<'e, ItsClusters>,

    pub(crate) tpc_chi2: Iter<'e, f32>,
    pub(crate) tpc_ncls: Iter<'e, u16>,
}

impl Event {
    /// Iterator over **all** `Track`s in this event
    pub fn tracks(&self) -> TracksIter {
        TracksIter {
            x: self.tracks_fx.iter(),
            p: self.tracks_fp.iter(),
            alpha: self.tracks_falpha.iter(),
            flags: self.tracks_fflags.iter(),
            its_chi2: self.tracks_fitschi2.iter(),
            its_ncls: self.tracks_fitsncls.iter(),
            its_clustermap: self.tracks_fitsclustermap.iter(),

            tpc_chi2: self.tracks_ftpcchi2.iter(),
            tpc_ncls: self.tracks_ftpcncls.iter(),
        }
    }
    /// The primary vertex of this event, if it exists. Else `None`
    pub fn primary_vertex(&self) -> Option<PrimaryVertex> {
        // 0 contributors means that there is no primar vertex
        if self.primaryvertex_alivertex_fncontributors > 0 {
            Some(PrimaryVertex {
                x: self.primaryvertex_alivertex_fposition.0,
                y: self.primaryvertex_alivertex_fposition.1,
                z: self.primaryvertex_alivertex_fposition.2,
                n_contrib: self.primaryvertex_alivertex_fncontributors,
            })
        } else {
            None
        }
    }

    /// Return the number of reconstructed tracks. Not very
    /// sophisticated, and probably not what what you want! Should
    /// rather be the number of **valid** tracks. FIXME.
    pub fn multiplicity(&self) -> f32 {
        self.tracks_fx.len() as f32
    }

    /// The `TriggerMask` of this event. Use this to select minimum bias events, for example
    pub fn trigger_mask(&self) -> TriggerMask {
        // The infromation which triggers fired is stored in a bitmask
        // Then we use the bit mask to find the string describing the
        // fired trigger Then, we convert the fired trigger to a
        // Trigger mask and lastly, we collect all fired triggers into
        // one mask
        (0..50) // Only 50 bits were used in the mask - YOLO!
            .map(|i| (self.aliesdheader_ftriggermask & (1 << i)) != 0)
            .zip(self.aliesdrun_ftriggerclasses.iter())
            .filter_map(|(fired, trigger_name)| if fired { Some(trigger_name) } else { None })
            .map(|name| string_to_mask(name, self.aliesdrun_frunnumber))
            .collect()
    }

    pub async fn stream_from_tree(t: &Tree) -> Result<impl Stream<Item = Self>, Error> {
        let track_counter: Vec<_> = t
            .branch_by_name("Tracks")?
            .as_fixed_size_iterator(|i| be_u32(i))
            .collect::<Vec<_>>()
            .await;
        let s = stream_zip!(
            t.branch_by_name("AliESDRun.fRunNumber")?
                .as_fixed_size_iterator(|i| be_i32(i)),
            t.branch_by_name("AliESDRun.fTriggerClasses")?
                .as_fixed_size_iterator(parse_tobjarray_of_tnameds),
            t.branch_by_name("AliESDHeader.fTriggerMask")?
                .as_fixed_size_iterator(|i| be_u64(i)),
            t.branch_by_name("PrimaryVertex.AliVertex.fPosition[3]")?
                .as_fixed_size_iterator(|i| tuple((be_f32, be_f32, be_f32))(i)),
            t.branch_by_name("PrimaryVertex.AliVertex.fNContributors")?
                .as_fixed_size_iterator(|i| be_i32(i)),
            t.branch_by_name("Tracks.fX")?
                .as_var_size_iterator(|i| be_f32(i), &track_counter),
            t.branch_by_name("Tracks.fP[5]")?.as_var_size_iterator(
                |i| map(tuple((be_f32, be_f32, be_f32, be_f32, be_f32)), |p| {
                    TrackParameters::new(&p)
                })(i),
                &track_counter
            ),
            t.branch_by_name("Tracks.fAlpha")?
                .as_var_size_iterator(|i| be_f32(i), &track_counter),
            t.branch_by_name("Tracks.fFlags")?.as_var_size_iterator(
                |i| map(be_u64, |uint| Flags::from_bits(uint).unwrap())(i),
                &track_counter
            ),
            t.branch_by_name("Tracks.fITSchi2")?
                .as_var_size_iterator(|i| parse_custom_mantissa(i, 8), &track_counter),
            t.branch_by_name("Tracks.fITSncls")?
                .as_var_size_iterator(|i| be_i8(i), &track_counter),
            t.branch_by_name("Tracks.fITSClusterMap")?
                .as_var_size_iterator(
                    |i| map(be_u8, |uint| ItsClusters::from_bits(uint).unwrap())(i),
                    &track_counter
                ),
            t.branch_by_name("Tracks.fTPCncls")?
                .as_var_size_iterator(|i| be_u16(i), &track_counter),
            t.branch_by_name("Tracks.fTPCchi2")?
                .as_var_size_iterator(|i| parse_custom_mantissa(i, 8), &track_counter),
        )
        .map(
            |(
                aliesdrun_frunnumber,
                aliesdrun_ftriggerclasses,
                aliesdheader_ftriggermask,
                primaryvertex_alivertex_fposition,
                primaryvertex_alivertex_fncontributors,
                tracks_fx,
                tracks_fp,
                tracks_falpha,
                tracks_fflags,
                tracks_fitschi2,
                tracks_fitsncls,
                tracks_fitsclustermap,
                tracks_ftpcncls,
                tracks_ftpcchi2,
            )| {
                Self {
                    aliesdrun_frunnumber,
                    aliesdrun_ftriggerclasses,
                    aliesdheader_ftriggermask,
                    primaryvertex_alivertex_fposition,
                    primaryvertex_alivertex_fncontributors,
                    tracks_fx,
                    tracks_fp,
                    tracks_falpha,
                    tracks_fflags,
                    tracks_fitschi2,
                    tracks_fitsncls,
                    tracks_fitsclustermap,
                    tracks_ftpcchi2,
                    tracks_ftpcncls,
                }
            },
        );
        Ok(s)
    }
}

impl<'e> Iterator for TracksIter<'e> {
    type Item = Track;

    fn next(&mut self) -> Option<Track> {
        Some(Track {
            x: *self.x.next()?,
            parameters: *self.p.next()?,
            alpha: *self.alpha.next()?,
            flags: *self.flags.next()?,
            its_chi2: *self.its_chi2.next()?,
            its_ncls: *self.its_ncls.next()?,
            its_clustermap: *self.its_clustermap.next()?,

            tpc_chi2: *self.tpc_chi2.next()?,
            tpc_ncls: *self.tpc_ncls.next()?,
        })
    }
}

/// Convert a given trigger description to a `TriggerMask`. This
/// mapping may depend on the run number
fn string_to_mask(s: &str, run_number: i32) -> TriggerMask {
    // LHC10h
    if 136_851 <= run_number && run_number <= 139_517 {
        match s {
            "CMBAC-B-NOPF-ALL"
            | "CMBS2A-B-NOPF-ALL"
            | "CMBS2C-B-NOPF-ALL"
            | "CMBACS2-B-NOPF-ALL"
            | "CMBACS2-B-NOPF-ALLNOTRD" => TriggerMask::MINIMUM_BIAS,
            "C0SMH-B-NOPF-ALL" | "C0SMH-B-NOPF-ALLNOTRD" => TriggerMask::HIGH_MULT,
            _ => TriggerMask::empty(),
        }
    } else {
        TriggerMask::empty()
    }
}
