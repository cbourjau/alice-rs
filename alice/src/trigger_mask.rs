use std::ffi::CStr;

use alice_sys as ffi;

bitflags! {
    pub struct TriggerMask: u64 {
        const MINIMUM_BIAS = 0b0000_0001;
        const HIGH_MULT =    0b0000_0010;
    }
}

impl TriggerMask {
    /// Trigger flags which fired for this event
    pub fn new(esd: &ffi::ESD_t) -> TriggerMask {
        let run_number = esd.AliESDRun_fRunNumber as u32;
        fired_trigger_strings(esd).iter()
            .map(|s| string_to_mask(s, run_number))
            .collect()        
    }
}

/// Indices of the fired trigger wrt the trigger-strings in
/// `AliESDRun_fTriggerClasses`
fn fired_trigger_indices(esd: &ffi::ESD_t) -> Vec<usize> {
    let mut ret = Vec::new();
    let masks = [
        esd.AliESDHeader_fTriggerMask,
        // 'Next50' is not available in LHC10h data; only later
        // unsafe { esd.AliESDHeader_fTriggerMaskNext50 },
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
fn fired_trigger_strings(esd: &ffi::ESD_t) -> Vec<&str> {
    let mut trg_clss = esd.AliESDRun_fTriggerClasses;
    fired_trigger_indices(esd).iter()
        .map(|i| unsafe {
            CStr::from_ptr(ffi::tobjarray_getname_at(&mut trg_clss, *i as i32))
                .to_str()
                .expect("Trigger string not valid utf-8")
        })
        .collect()
}

/// Get the trigger mask for a given trigger string in a given run
fn string_to_mask(s: &str, run_number: u32) -> TriggerMask {
    // LHC10h
    if 136_851 <= run_number && run_number <= 139_517 {
        match s {
            "CMBAC-B-NOPF-ALL"  |
            "CMBS2A-B-NOPF-ALL" |
            "CMBS2C-B-NOPF-ALL" |
            "CMBACS2-B-NOPF-ALL"|
            "CMBACS2-B-NOPF-ALLNOTRD" => MINIMUM_BIAS,
            "C0SMH-B-NOPF-ALL" |
            "C0SMH-B-NOPF-ALLNOTRD" => HIGH_MULT,
            _ => TriggerMask::empty(),
        }
    } else {
        TriggerMask::empty()
    }
}
