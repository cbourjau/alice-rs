extern crate alice_sys;
extern crate alice_open_data;

use std::ffi::{CString, CStr};
use alice_sys::*;

#[test]
fn init_esd_object() {
    let local_path = alice_open_data::test_file().unwrap();
    let local_path = CString::new(local_path.to_str().unwrap()).unwrap();
    let esd = unsafe { esd_new(local_path.as_ptr()) };
    let mut sum = 0;
    for i in 0..10 {
        unsafe { esd_load_next(esd, i); }
        sum += unsafe { (*esd).Tracks_ };
    }
    assert!(sum >= 0, "No tracks loaded?!");

    // Check trigger strings
    // Iterate through triggers
    unsafe { esd_load_next(esd, 0); }
    println!("Run number:  {}", unsafe { (*esd).AliESDRun_fRunNumber });
    let mut trgs = unsafe { (*esd).AliESDRun_fTriggerClasses };
    let nentries = unsafe {tobjarray_getentriesfast(&mut trgs)};
    for i in 0..nentries {
        let cstr = unsafe{ CStr::from_ptr(tobjarray_getname_at(&mut trgs, i)) };
        println!("trg: '{:?}'", cstr);
    }
    // For fired triggers per event see
    // AliESDEvent::GetFiredTriggerClasses and
    // AliESDEvent::IsTriggerClassFired
    for i in 0..10 {
        println!("Loading trigger classes from many events");
        unsafe { esd_load_next(esd, i); }
        let mut trgs = unsafe { (*esd).AliESDRun_fTriggerClasses };
        let _nentries = unsafe {tobjarray_getentriesfast(&mut trgs)};
    }

    for i in 0..30 {
        println!("Fired triggers in event {}", i);
        unsafe { esd_load_next(esd, i); }

        let tbits = fired_trigger_bits(esd);
        println!("vec: {:?}", tbits);

        let mut trg_clss = unsafe { (*esd).AliESDRun_fTriggerClasses };
        for i in tbits {
            let cstr = unsafe{ CStr::from_ptr(tobjarray_getname_at(&mut trg_clss, i as i32)) };
            println!("fired trg: {:?}", cstr);
        }
    }
}

fn fired_trigger_bits(esd: *mut ESD_t)  -> Vec<usize> {
    let mut ret = Vec::new();
    let masks = [
        unsafe { (*esd).AliESDHeader_fTriggerMask },
        // unsafe { (*esd).AliESDHeader_fTriggerMaskNext50 },
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
