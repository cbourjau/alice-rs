use std::ffi::CString;
use alice_sys as ffi;
use event::Event;

pub struct Dataset {
    esd: *mut ffi::ESD,
    current_event: i64,
}

impl Dataset {
    pub fn new(path: &str) -> Dataset {
        // let eos_url = "root://eospublic.cern.ch/";
        // let eos_path = "/eos/opendata/alice/2010/LHC10h/000139437/ESD/0153/AliESDs.root";
        // let path = CString::new(format!("{}{}", eos_url, eos_path)).unwrap();
        let local_path = CString::new(path).unwrap();
        let esd = unsafe {ffi::esd_new(local_path.as_ptr())};
        let ds = Dataset {esd: esd,
                          current_event: -1};
        ds
    }
    fn load_next(&mut self) -> Option<Event> {
        self.current_event += 1;
        let ievent = self.current_event;
        // A return value <= 0 means failure; welcome to AliRoot
        // let state_ptr: *mut c_void = &mut self.esd as *mut _ as *mut c_void;
        let state_ptr = unsafe {self.esd.as_ref().unwrap()};
        match unsafe {ffi::esd_load_next(self.esd, ievent)} {
            a if a <= 0 => None,
            _ => Some(Event::new_from_esd(state_ptr))
        }
    }
}

impl Drop for Dataset {
    fn drop(&mut self)
    {
        // let state_ptr: *mut c_void = &mut self.esd as *mut _ as *mut c_void;
        unsafe { ffi::esd_destroy(self.esd); }
    }
}

impl Iterator for Dataset {
    type Item = Event;

    /// Load the next event from the file
    fn next(&mut self) -> Option<Event> {
        self.load_next()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_and_drop_dataset() {
        Dataset::new("/home/christian/Downloads/AliESDs.root");
    }

    #[test]
    fn iterate_items() {
        let ds = Dataset::new("/home/christian/Downloads/AliESDs.root");
        assert!(ds.count() > 0);
    }
}
