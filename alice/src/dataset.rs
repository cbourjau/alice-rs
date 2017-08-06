use std::thread;
use std::sync::mpsc;

use std::ffi::CString;
use alice_sys as ffi;
use event::Event;

struct MyBox(*mut ffi::ESD);

unsafe impl Send for MyBox {}
unsafe impl Sync for MyBox {}

impl Drop for MyBox {
    fn drop(&mut self) {
        unsafe { ffi::esd_destroy(self.0); }
    }
}


pub struct Dataset {
    path: CString,
    rx: Option<mpsc::Receiver<Event>>,
}

impl Dataset {
    pub fn new(path: &str) -> Dataset {
        // let eos_url = "root://eospublic.cern.ch/";
        // let eos_path = "/eos/opendata/alice/2010/LHC10h/000139437/ESD/0153/AliESDs.root";
        let local_path = CString::new(path).unwrap();
        Dataset {path: local_path,
                 rx: None,}
    }
    fn load_next(&mut self) -> Option<Event> {
        // A return value <= 0 means failure; welcome to AliRoot
        // let state_ptr: *mut c_void = &mut self.esd as *mut _ as *mut c_void;
        if self.rx.is_none() {
            // buffer up to 5 events
            let (tx, rx) = mpsc::sync_channel(5);
            self.rx = Some(rx);
            // let esd = self.esd;//.clone();
            let esd = unsafe {ffi::esd_new(self.path.as_ptr())};
            let esd = MyBox(esd);
            // Start a new thread which does the io for the current file.
            // Loaded events are sent to the reciever
            thread::spawn(move || {
                // let esd = esd.lock().expect("Could not get lock of Mutex! Poisoned?");
                let mut ievent = -1;
                loop {
                    ievent += 1;
                    let ev = match unsafe {ffi::esd_load_next(esd.0, ievent)} {
                        a if a <= 0 => None,
                        _ => unsafe { Some(Event::new_from_esd(&*esd.0)) }
                    };
                    match ev {
                        Some(ev) => tx.send(ev).expect("Could not send event :("),
                        None => break
                    };
                }
            });
        }
        self.rx.as_ref().unwrap().recv().ok()
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
