use std::slice::Iter;
use track::{Track, TrackParameters, Flags, ItsClusters};
use primary_vertex::PrimaryVertex;

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
    pub(crate) tracks_ftpcchi2: Vec<f32>,
    pub(crate) tracks_ftpcncls: Vec<u16>,
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

    pub(crate) tpc_chi2: Iter<'e, f32>,
    pub(crate) tpc_ncls: Iter<'e, u16>,
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

            tpc_chi2: self.tracks_ftpcchi2.iter(),
            tpc_ncls: self.tracks_ftpcncls.iter(),
        }
    }
    pub fn primary_vertex(&self) -> Option<PrimaryVertex>{
        // 0 contributors means that there is no primar vertex
        if self.primaryvertex_alivertex_fncontributors > 0 {
            Some(PrimaryVertex {x: self.primaryvertex_alivertex_fposition[0],
                                y: self.primaryvertex_alivertex_fposition[1],
                                z: self.primaryvertex_alivertex_fposition[2],
                                n_contrib: self.primaryvertex_alivertex_fncontributors})
        } else {
            None
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

            tpc_chi2: *self.tpc_chi2.next()?,
            tpc_ncls: *self.tpc_ncls.next()?,
        })
    }
}
