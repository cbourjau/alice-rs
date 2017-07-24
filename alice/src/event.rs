use alice_sys as ffi;
use primary_vertex::PrimaryVertex;
use track::Track;
use trigger::Trigger;
use vzero::V0;

#[derive(Debug)]
pub struct Event {
    raw_esd: *const ffi::ESD,
    pub primary_vertex: Option<PrimaryVertex>,
    pub tracks: Vec<Track>,
    pub multiplicity: i32,
    pub vzero: V0,
}

impl Event {
    pub fn new_from_esd(esd: &ffi::ESD) -> Event {
        Event {
            raw_esd: esd,
            primary_vertex: PrimaryVertex::new(esd),
            tracks: Track::read_tracks_from_esd(esd),
            multiplicity: esd.AliMultiplicity_fNtracks,
            vzero: V0::from_esd(esd),
        }
    }

    /// Indices of the fired trigger wrt the trigger-strings in
    /// `AliESDRun_fTriggerClasses`
    fn fired_trigger_indices(&self) -> Vec<usize> {
        let mut ret = Vec::new();
        let masks = [
            unsafe { (*self.raw_esd).AliESDHeader_fTriggerMask },
            // 'Next50' is not available in LHC10h data; only later
            // unsafe { (*self.raw_esd).AliESDHeader_fTriggerMaskNext50 },
        ];
        for (mask, offset) in masks.iter().zip([0, 50].iter()) {
            for i in 0..64 {
                if (mask & (1u64 << i)) > 0 {
                    // For some reasone, only the first 50 bits are used
                    // panic if this assumption is wrong
                    if i >= 50 {panic!("More than 50 bits used!")};
                    ret.push((i + offset) as usize)
                }
            }
        }
        ret
    }

    /// String representation of triggers fired for this event
    fn fired_trigger_strings(&self) -> Vec<&str> {
        use std::ffi::CStr;

        let mut trg_clss = unsafe {(*self.raw_esd).AliESDRun_fTriggerClasses};
        self.fired_trigger_indices().iter()
            .map(|i| unsafe {
                CStr::from_ptr(ffi::tobjarray_getname_at(&mut trg_clss, *i as i32))
                    .to_str()
                    .expect("Trigger string not valid utf-8")
            })
            .collect()
    }

    /// Trigger flags which fired for this event
    pub fn triggers(&self) -> Trigger {
        let run_number = unsafe {(*self.raw_esd).AliESDRun_fRunNumber} as u32;
        self.fired_trigger_strings().iter()
            .map(|s| Trigger::new_from_str(s, run_number))
            .collect()
    }
}
