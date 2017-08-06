use std::thread;
use std::sync::mpsc;

use event::Event;
use esd::ESD;


pub struct Dataset {
    path: String,
    rx: Option<mpsc::Receiver<Event>>,
}

impl Dataset {
    pub fn new(path: &str) -> Dataset {
        // let eos_url = "root://eospublic.cern.ch/";
        // let eos_path = "/eos/opendata/alice/2010/LHC10h/000139437/ESD/0153/AliESDs.root";
        Dataset {path: path.to_owned(),
                 rx: None,}
    }
    fn load_next(&mut self) -> Option<Event> {
        if self.rx.is_none() {
            // buffer up to 5 events
            let (tx, rx) = mpsc::sync_channel(5);
            self.rx = Some(rx);
            let mut esd = ESD::new(&self.path);
            // Start a new thread which does the io for the current file.
            // Loaded events are sent to the reciever
            thread::spawn(move || {
                let mut ievent = -1;
                loop {
                    ievent += 1;
                    match esd.load_event(ievent) {
                        Some(_) => {
                            let esd_raw = unsafe { &mut *esd.raw };
                            let ev = Event::new_from_esd(esd_raw);
                            tx.send(ev).expect("Could not send event :(")
                        },
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
