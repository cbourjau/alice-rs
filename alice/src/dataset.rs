use std::path::PathBuf;
use futures::sync::mpsc::{channel, Receiver};
use futures::{Stream, Sink};
use rayon;


use event::Event;
use esd::ESD;

use std::sync::{Arc, Mutex};

pub struct Dataset {
    event_stream: Receiver<Event>,
    // path: PathBuf,
    // rx: Option<mpsc::Receiver<Event>>,
}

impl Dataset {
    pub fn new(paths: &[PathBuf]) -> Dataset {
        Dataset {event_stream: event_stream(2, paths)}
    }
}

fn event_stream(workers: usize, paths: &[PathBuf]) -> Receiver<Event> {
    let conf = rayon::Configuration::new().num_threads(workers);
    let pool = rayon::ThreadPool::new(conf).unwrap();
    let buf_size = 5;
    let (tx, rx) = channel::<Event>(buf_size);
    // FIXME: ROOT's global interpreter can't handle if if we open the
    // the first two files simultaniously...
    let esd_factory = Arc::new(Mutex::new(|p: PathBuf| {ESD::new(&p)}));
    for path in paths {
        let mut tx = tx.clone().wait();
        let path = path.clone();
        let fact = esd_factory.clone();
        pool.spawn(
            move || {
                let mut ievent = -1;
                let mut esd = {
                    let fact = fact.lock().unwrap();
                    fact(path)
                };
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
            }
        )
    }
    rx
}



impl Iterator for Dataset {
    type Item = Event;

    /// Load the next event from the file
    fn next(&mut self) -> Option<Event> {
        if let Some(ev) = self.event_stream.by_ref().wait().next() {
            return ev.ok();
        } else {
            return None;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{thread, time};
    extern crate alice_open_data;

    #[test]
    fn init_and_drop_dataset() {
        Dataset::new(&[alice_open_data::test_file().unwrap()]);
    }

    #[test]
    fn iterate_items() {
        let ds = Dataset::new(&[alice_open_data::test_file().unwrap()]);
        assert!(ds.count() > 0);
    }

    #[test]
    /// Provoke that we drop the dataset (and its reciever) before the
    /// Sender is finished reading an event from disk.  Make sure we
    /// get some sort of log message and not a panic
    fn quick_iterate_and_drop() {
        {
            let mut ds = Dataset::new(&[alice_open_data::test_file().unwrap()]);
            // Start of the IO thread by getting the first event
            let _ev = ds.next();
            // Drop the dataset here
        }
        // wait 1s for the next event to be read
        thread::sleep(time::Duration::from_secs(1));
    }

    #[test]
    fn fold_stream() {
        use futures::future::{ok, Future};
        let files: Vec<_> = alice_open_data::all_files_10h().unwrap()
            .into_iter()
            .take(5)
            .collect();
        let ds = event_stream(2, files.as_slice());
        // let blub = ds.map(|_| 1).collect();
        // println!("{:?}", blub.wait());
        let nevents = ds
            .filter(|ev| ev.primary_vertex.is_some())
            .fold(0, |acc, _| ok(acc + 1))
            .wait().unwrap();
        assert!(nevents > 0);
    }    
}
