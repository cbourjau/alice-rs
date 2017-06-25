use std::ffi::CString;
use alice_sys as ffi;
use event::Event;

#[derive(Debug)]
pub struct Dataset {
    esd: *const ffi::CEsd,
    current_event: i64,
}

impl Dataset {
    pub fn new() -> Dataset {
        // let eos_url = "root://eospublic.cern.ch/";
        // let eos_path = "/eos/opendata/alice/2010/LHC10h/000139437/ESD/0153/AliESDs.root";
        // let path = CString::new(format!("{}{}", eos_url, eos_path)).unwrap();
        let local_path = CString::new("/home/christian/Downloads/AliESDs.root").unwrap();
        let ptr = unsafe {ffi::esd_new(local_path.as_ptr())};
        Dataset {esd: ptr,
                 current_event: -1}
    }
    fn load_next(&self, ievent: i64) -> Option<()> {
        // A return value <= 0 means failure; welcome to AliRoot
        match unsafe {ffi::esd_load_next(self.esd, ievent)} {
            a if a <= 0 => None,
            _ => Some(())
        }
    }
}

impl Drop for Dataset {
    fn drop(&mut self)
    {
        unsafe { ffi::esd_destroy(self.esd); }
    }
}

impl Iterator for Dataset {
    type Item = Event;

    /// Load the next event from the file
    fn next(&mut self) -> Option<Event> {
        self.current_event += 1;
        self.load_next(self.current_event)
            .map(|_| {Event::new_from_esd(self.esd)})
    }
}
