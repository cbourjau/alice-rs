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
}

impl Iterator for Dataset {
    type Item = Event;

    /// Load the next event from the file
    fn next(&mut self) -> Option<Event> {
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
                            if let Err(err) = tx.send(ev) {
                                // The reciever has hung up
                                warn!("{}, stopping IO", err);
                                break;
                            }
                        },
                        // We are out of events in this file
                        None => break
                    };
                }
            });
        }
        self.rx.as_ref().unwrap().recv().ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{thread, time};

    #[test]
    fn init_and_drop_dataset() {
        Dataset::new("/home/christian/Downloads/AliESDs.root");
    }

    #[test]
    fn iterate_items() {
        let ds = Dataset::new("/home/christian/Downloads/AliESDs.root");
        assert!(ds.count() > 0);
    }

    #[test]
    /// Provoke that we drop the dataset (and its reciever) before the
    /// Sender is finished reading an event from disk.  Make sure we
    /// get some sort of log message and not a panic
    fn quick_iterate_and_drop() {
        {
            let mut ds = Dataset::new("/home/christian/lhc_data/alice/data/2010/LHC10h/000139510/ESDs/pass2/10000139510001.10/AliESDs.root");
            // Start of the IO thread by getting the first event
            let _ev = ds.next();
            // Drop the dataset here
        }
        // wait 1s for the next event to be read
        thread::sleep(time::Duration::from_secs(1));
    }
}
