
use event::Event;

#[derive(Debug)]
pub struct Dataset {
    esd: *const ffi::CEsd,
    current_event: i64,
}

impl Dataset {
    pub fn new() -> Dataset {
        let ptr = unsafe {ffi::esd_new()};
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
