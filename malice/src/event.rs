use std::slice::Iter;
use track::{Track, TrackParameters, Flags, ItsClusters};

/// A model for the / a subset of the ESD data
#[derive(Debug, PartialEq)]
pub struct Event {
    pub primaryvertex_alivertex_fposition: [f32; 3],
    pub primaryvertex_alivertex_fncontributors: i32,
    pub aliesdrun_frunnumber: i32,
    pub aliesdrun_ftriggerclasses: Vec<String>,
    pub aliesdheader_ftriggermask: u64, 
    pub(crate) tracks_fx: Vec<f32>,
    pub(crate) tracks_fp: Vec<TrackParameters>,
    pub(crate) tracks_falpha: Vec<f32>,
    pub(crate) tracks_fflags: Vec<Flags>,
    pub(crate) tracks_fitschi2: Vec<f32>,
    pub(crate) tracks_fitsncls: Vec<i8>,
    pub(crate) tracks_fitsclustermap: Vec<ItsClusters>,
}

/// Iterator over the tracks of an event
pub struct TracksIter<'e> {
    pub(crate) x: Iter<'e, f32>,
    pub(crate) p: Iter<'e, TrackParameters>,  // fn(&[f32; 5]) -> TrackParameters>,
    pub(crate) alpha: Iter<'e, f32>,
    pub(crate) flags: Iter<'e, Flags>,
    pub(crate) itschi2: Iter<'e, f32>,
    pub(crate) itsncls: Iter<'e, i8>,
    pub(crate) itsclustermap: Iter<'e, ItsClusters>,
}

impl Event {
    pub fn tracks(&self) -> TracksIter {
        TracksIter {
            x: self.tracks_fx.iter(),
            p: self.tracks_fp.iter(),
            alpha: self.tracks_falpha.iter(),
            flags: self.tracks_fflags.iter(),
            itschi2: self.tracks_fitschi2.iter(),
            itsncls: self.tracks_fitsncls.iter(),
            itsclustermap: self.tracks_fitsclustermap.iter(),
        }
    }
}

impl<'e> Iterator for TracksIter<'e> {
    type Item = Track;

    fn next(&mut self) ->Option<Track> {
        Some(Track {
            x: *self.x.next()?,
            parameters: *self.p.next()?,
            alpha: *self.alpha.next()?,
            flags: *self.flags.next()?,
            itschi2: *self.itschi2.next()?,
            itsncls: *self.itsncls.next()?,
            itsclustermap: *self.itsclustermap.next()?,
        })
    }
}
