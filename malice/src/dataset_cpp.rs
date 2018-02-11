use std::path::PathBuf;

use failure::Error;

use esd::ESD;
use event::Event;
use track::{TrackParameters, ItsClusters, Flags};


pub struct DatasetIntoIter {
    event_number: i64,
    esd_object: ESD,
}

impl DatasetIntoIter {
    pub fn new(p: &PathBuf) -> Result<DatasetIntoIter, Error> {
        // let track_counter: Vec<_> = ColumnFixedIntoIter::new(&t, "Tracks", be_u32)?.collect();
        Ok(DatasetIntoIter {
            event_number: -1,
            esd_object: ESD::new(&p),
        })
    }
}

/// Iterator over models from the schema
impl Iterator for DatasetIntoIter {
    type Item = Event;

    fn next(&mut self) -> Option<Self::Item> {
        self.event_number += 1;
        self.esd_object.load_event(self.event_number)?;
        let esd = unsafe { &mut *self.esd_object.raw };
        let n_tracks = esd.Tracks_ as usize;
        let pv_pos = {
            let pv = esd.PrimaryVertex_AliVertex_fPosition;
            [pv[0] as f32, pv[1] as f32, pv[2] as f32]
        };
        use alice_sys as ffi;
        use std::ffi::CStr;
        Some(Event {
            aliesdrun_frunnumber: esd.AliESDRun_fRunNumber,
            aliesdrun_ftriggerclasses: unsafe {
                let nentries = ffi::tobjarray_getentriesfast(&mut esd.AliESDRun_fTriggerClasses);
                (0..nentries)
                    .map(|i| CStr::from_ptr(ffi::tobjarray_getname_at(&mut esd.AliESDRun_fTriggerClasses, i))
                         .to_str().unwrap().to_string()
                    )
                    .collect()
                    // vec!["".to_string()]
            },
            aliesdheader_ftriggermask: esd.AliESDHeader_fTriggerMask,
            primaryvertex_alivertex_fposition: pv_pos,
            primaryvertex_alivertex_fncontributors: esd.PrimaryVertex_AliVertex_fNContributors,
            tracks_fx: esd.Tracks_fX[..n_tracks].into_iter().map(|v| *v as f32).collect(),
            tracks_fp: esd.Tracks_fP[..n_tracks].into_iter()
                .map(|a| TrackParameters::new(&[a[0] as f32, a[1] as f32, a[2] as f32,
                                                a[3] as f32, a[4] as f32]))
                .collect(),
            tracks_falpha: esd.Tracks_fAlpha[..n_tracks].iter().map(|v| *v as f32).collect(),
            tracks_fflags: esd.Tracks_fFlags[..n_tracks].into_iter()
                .map(|v| Flags::from_bits(*v).unwrap())
                .collect(),
            tracks_fitschi2: esd.Tracks_fITSchi2[..n_tracks].into_iter().map(|v| *v as f32).collect(),
            tracks_fitsncls: esd.Tracks_fITSncls[..n_tracks].to_vec(),
            tracks_fitsclustermap: esd.Tracks_fITSClusterMap[..n_tracks].into_iter()
                .map(|v| ItsClusters::from_bits(*v).unwrap())
                .collect()
        })
    }
}
    

#[cfg(test)]
mod tests {
    extern crate alice_open_data;

    use super::*;

    #[test]
    fn read_cpp() {
        let max_chi2 = alice_open_data::all_files_10h().unwrap()
            .into_iter()
            .take(100)
            .flat_map(|path| {
                match DatasetIntoIter::new(&path) {
                    Ok(s) => s,
                    Err(err) => panic!("An error occured! Message: {}", err)
                }})
            .flat_map(|m| m.tracks_fitschi2.into_iter())
            .fold(0.0, |max, chi2| if chi2 > max {chi2} else {max});
        println!("CPP max(chi2): {}", max_chi2);
    }
}
