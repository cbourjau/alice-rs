//! Structs and iterators concerned with iterating over events stored in `root_io::Tree`s.

use failure::Error;
use nom;

use root_io::core::parsers::checked_byte_count;
use root_io::core::parsers::{tnamed, tobjarray_no_context};
use root_io::core::types::ClassInfo;
use root_io::tree_reader::{ColumnFixedIntoIter, ColumnVarIntoIter, Tree};

use event::Event;
use track::{Flags, ItsClusters, TrackParameters};

/// Struct of `Iterator`s over some of the columns (aka branches) of the `root_io::Tree`.
pub struct DatasetIntoIter {
    aliesdrun_frunnumber: ColumnFixedIntoIter<i32>,
    aliesdrun_ftriggerclasses: ColumnFixedIntoIter<Vec<String>>,
    aliesdheader_ftriggermask: ColumnFixedIntoIter<u64>,
    primaryvertex_alivertex_fposition: ColumnFixedIntoIter<[f32; 3]>,
    primaryvertex_alivertex_fncontributors: ColumnFixedIntoIter<i32>,
    tracks_fx: ColumnVarIntoIter<f32>,
    tracks_fp: ColumnVarIntoIter<[f32; 5]>,
    tracks_falpha: ColumnVarIntoIter<f32>,
    tracks_fflags: ColumnVarIntoIter<u64>,
    tracks_fitschi2: ColumnVarIntoIter<f32>,
    tracks_fitsncls: ColumnVarIntoIter<i8>,
    tracks_fitsclustermap: ColumnVarIntoIter<u8>,
    tracks_ftpcchi2: ColumnVarIntoIter<f32>,
    tracks_ftpcncls: ColumnVarIntoIter<u16>,
}

impl DatasetIntoIter {
    /// Create a new `DatasetIntoIter` from the given `root_io::Tree`. The `Tree` must be a so-called ALICE "ESD" tree.
    pub fn new(t: &Tree) -> Result<DatasetIntoIter, Error> {
        use nom::*;
        // The following connects the ESD's TTree branches to the respective iterator fields of `DatasetIntoIter`
        // You can use ROOT's TBrowser or this projects `root-ls` tool to explor the branches of an existing .root file.
        let track_counter: Vec<_> = ColumnFixedIntoIter::new(&t, "Tracks", be_u32)?.collect();
        Ok(DatasetIntoIter {
            aliesdrun_frunnumber: ColumnFixedIntoIter::new(&t, "AliESDRun.fRunNumber", be_i32)?,
            aliesdrun_ftriggerclasses: ColumnFixedIntoIter::new(
                &t,
                "AliESDRun.fTriggerClasses",
                parse_trigger_classes,
            )?,
            aliesdheader_ftriggermask: ColumnFixedIntoIter::new(
                &t,
                "AliESDHeader.fTriggerMask",
                be_u64,
            )?,
            primaryvertex_alivertex_fposition: ColumnFixedIntoIter::new(
                &t,
                "PrimaryVertex.AliVertex.fPosition[3]",
                |i| count_fixed!(i, f32, be_f32, 3),
            )?,
            primaryvertex_alivertex_fncontributors: ColumnFixedIntoIter::new(
                &t,
                "PrimaryVertex.AliVertex.fNContributors",
                be_i32,
            )?,
            tracks_fx: ColumnVarIntoIter::new(&t, "Tracks.fX", be_f32, &track_counter)?,
            tracks_fp: ColumnVarIntoIter::new(
                &t,
                "Tracks.fP[5]",
                |i| count_fixed!(i, f32, be_f32, 5),
                &track_counter,
            )?,
            tracks_falpha: ColumnVarIntoIter::new(&t, "Tracks.fAlpha", be_f32, &track_counter)?,
            tracks_fflags: ColumnVarIntoIter::new(&t, "Tracks.fFlags", be_u64, &track_counter)?,

            tracks_fitschi2: ColumnVarIntoIter::new(
                &t,
                "Tracks.fITSchi2",
                |i| parse_custom_mantissa(i, 8),
                &track_counter,
            )?,
            tracks_fitsncls: ColumnVarIntoIter::new(&t, "Tracks.fITSncls", be_i8, &track_counter)?,
            tracks_fitsclustermap: ColumnVarIntoIter::new(
                &t,
                "Tracks.fITSClusterMap",
                be_u8,
                &track_counter,
            )?,

            tracks_ftpcncls: ColumnVarIntoIter::new(&t, "Tracks.fTPCncls", be_u16, &track_counter)?,
            tracks_ftpcchi2: ColumnVarIntoIter::new(
                &t,
                "Tracks.fTPCchi2",
                |i| parse_custom_mantissa(i, 8),
                &track_counter,
            )?,
        })
    }
}

/// Iterator over models from the schema
impl Iterator for DatasetIntoIter {
    type Item = Event;

    fn next(&mut self) -> Option<Self::Item> {
        // Essentially, we call `.next()` on each of the the member-iterators.
        Some(Event {
            primaryvertex_alivertex_fposition: self.primaryvertex_alivertex_fposition.next()?,
            primaryvertex_alivertex_fncontributors: self
                .primaryvertex_alivertex_fncontributors
                .next()?,
            aliesdrun_frunnumber: self.aliesdrun_frunnumber.next()?,
            aliesdrun_ftriggerclasses: self.aliesdrun_ftriggerclasses.next()?,
            aliesdheader_ftriggermask: self.aliesdheader_ftriggermask.next()?,
            tracks_fx: self.tracks_fx.next()?,
            tracks_fp: self
                .tracks_fp
                .next()?
                .iter()
                .map(|a| TrackParameters::new(&a))
                .collect(),
            tracks_falpha: self.tracks_falpha.next()?,
            tracks_fflags: self
                .tracks_fflags
                .next()?
                .into_iter()
                .map(|v| Flags::from_bits(v).unwrap())
                .collect(),
            tracks_fitschi2: self.tracks_fitschi2.next()?,
            tracks_fitsncls: self.tracks_fitsncls.next()?,
            tracks_fitsclustermap: self
                .tracks_fitsclustermap
                .next()?
                .into_iter()
                .map(|v| ItsClusters::from_bits(v).unwrap())
                .collect(),

            tracks_ftpcchi2: self.tracks_ftpcchi2.next()?,
            tracks_ftpcncls: self.tracks_ftpcncls.next()?,
        })
    }
}

/// ESD trigger classes are strings describing a particular
/// Trigger. Each event (but in reality every run) might have a
/// different "menu" of available triggers. The trigger menu is saved
/// as an `TObjArray` of `TNamed` objects for each event. This breaks
/// it down to a simple vector
fn parse_trigger_classes(input: &[u8]) -> nom::IResult<&[u8], Vec<String>> {
    let vals = length_value!(input, checked_byte_count, tobjarray_no_context);
    vals.map(|arr| {
        arr.iter()
            .map(|&(ref ci, ref el)| match *ci {
                ClassInfo::References(0) => "".to_string(),
                _ => match tnamed(el.as_slice()).map(|tn| tn.name) {
                    nom::IResult::Done(_, n) => n,
                    _ => panic!(),
                },
            })
            .collect::<Vec<String>>()
    })
}

/// Some Double_32 values are saved with a custom mantissa... The
/// number of bytes can be found in the comment string of the filed
/// (check the YAML code for ALIESD)
/// This function reconstructs a float from the exponent and mantissa
/// TODO: Use ByteOrder crate to be cross-platform!
fn parse_custom_mantissa(input: &[u8], nbits: usize) -> nom::IResult<&[u8], f32> {
    use nom::*; // cannot use module path in macro
    pair!(input, be_u8, be_u16).map(|(exp, man)| {
        // let nbits = 8;
        let mut s = u32::from(exp);
        // Move the exponent into the last 23 bits
        s <<= 23;
        s |= (u32::from(man) & ((1 << (nbits + 1)) - 1)) << (23 - nbits);
        f32::from_bits(s)
    })
}

#[cfg(test)]
mod tests {
    extern crate alice_open_data;
    use super::*;
    use root_io::RootFile;
    #[test]
    fn read_rust() {
        let max_chi2 = alice_open_data::all_files_10h()
            .unwrap()
            .into_iter()
            .take(100)
            .map(|path| RootFile::new_from_file(&path).expect("Failed to open file"))
            .map(|rf| rf.items()[0].as_tree().unwrap())
            .flat_map(|tree| match DatasetIntoIter::new(&tree) {
                Ok(s) => s,
                Err(err) => panic!("An error occured! Message: {}", err),
            })
            .flat_map(|m| m.tracks_fitschi2.into_iter())
            .fold(0.0, |max, chi2| if chi2 > max { chi2 } else { max });
        println!("Rust max(chi2): {}", max_chi2);
    }
}
