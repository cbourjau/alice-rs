use std::path::PathBuf;
use std::convert::{AsRef};
use std::thread;
use std::time::Instant;

use chan::{self, Receiver};
use rayon::{self, join};
use indicatif::{ProgressBar, ProgressStyle};

use alice_sys::setup_root;

use event::Event;
use esd::ESD;
use analysis::traits::Merge;
use event_traits::{Tracks};


/// A dataset is a collection of events. Events may be accessed
/// throught the `Iterator` trait
#[derive(Clone)]
pub struct Dataset {
    receiver: Receiver<Event>,
}

impl Dataset {
    /// Create a new dataset from the files in `paths`. The dataset
    /// will use `n_workers` threads to read and unzip the data
    pub fn new<T>(paths: T, n_workers: usize) -> Dataset
        where T: AsRef<[PathBuf]>
    {
        // Set up ROOT; The ROOT interpreter has to be fired up single threaded.
        // This also makes ROOT thread aware and sets the log level
        unsafe {setup_root()};
        Dataset {receiver: setup_io_threads(paths, n_workers)}
    }
}

impl Iterator for Dataset {
    type Item = Event;

    /// Produce the next event from this dataset
    fn next(&mut self) -> Option<Event> {
        self.receiver.recv()
    }
}

impl<'f> Dataset {
    /// Excute `f` on this dataset. The analysis will be run in 4
    /// parallel threads The name of this function is somewhat related
    /// to rayon's `install`, but not really.
    pub fn install<F, T>(self, f: &'f F) -> T
        where F: Fn(Box<Iterator<Item=Event>>) -> T + Sync,
              T: Send + Merge
    {
        // use 4 threads;
        // FIXME: This should not be hard coded!
        let ((mut t1, t2), (t3, t4)) =
            join(|| {join(|| {f(Box::new(self.clone()))}, || f(Box::new(self.clone())))},
                       || {join(|| {f(Box::new(self.clone()))}, || f(Box::new(self.clone())))});
        // merge the output of the parallel threads into one
        for a in &[t2, t3, t4] {
            t1.merge(a);
        }
        t1
    }
}

/// Function to be executed in its own thread taking care of the progress bar
fn progress_bar(rx: &Receiver<i64>, nfiles: u64) {
    let pbar = ProgressBar::new(nfiles);
    pbar.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
    );
    let mut n_files_done = 0;
    let mut n_evts_done = 0;
    let started = Instant::now();
    loop {
        // Select will block until recv() succeeds
        chan_select! {
            rx.recv() -> v => match v {
                None => {},
                Some(n_evts) => {
                    n_files_done += 1;
                    n_evts_done += n_evts;
                    pbar.inc(1);
                    // This might end up being called within 0s!
                    if started.elapsed().as_secs() > 0 {
                        let rate = n_evts_done / started.elapsed().as_secs() as i64;
                        pbar.set_message(&format!("Evts/s: {}", rate));
                    }
                    if n_files_done >= nfiles {
                        break
                    }
                }
            }
        }
    }
    pbar.finish();
}

/// Spin up `workers` parallel IO threads. The threads will send the
/// read events to the returned receiver
fn setup_io_threads<T>(paths: T, workers: usize) -> Receiver<Event>
    where T: AsRef<[PathBuf]>
{
    let conf = rayon::Configuration::new().num_threads(workers);
    let pool = rayon::ThreadPool::new(conf).unwrap();
    let buf_size = 100;
    let (tx, rx) = chan::sync::<Event>(buf_size);
    // ProgressBar lives in its own thread; and increments when getting a message
    // That message also contains the number of events processed from the file
    let (tx_progress, rx_progress) = chan::async::<i64>();
    let nfiles = paths.as_ref().len() as u64;
    thread::spawn(move || progress_bar(&rx_progress, nfiles));

    for path in paths.as_ref() {
        let tx = tx.clone();
        let tx_progress = tx_progress.clone();
        let path = path.clone();
        // One thread per file. The file is only opened in the thread;
        // Rayon takes care of not running all threads at once.
        pool.spawn(
            move || {
                let mut ievent = -1;
                let mut esd = {
                    ESD::new(&path)
                };
                loop {
                    ievent += 1;
                    match esd.load_event(ievent) {
                        Some(_) => {
                            let esd_raw = unsafe { &mut *esd.raw };
                            let ev = Event::new_from_esd(esd_raw);
                            // keep an eye on the number of tracks in
                            // an event. There are some hardcoded
                            // limits on the c++ side. Those limits
                            // should be sufficient though, but needs
                            // more tests.
                            if ev.tracks().len() > 4_0000 {
                                println!("ntracks: {:?}", ev.tracks().len());
                            }
                            tx.send(ev);
                        },
                        // We are out of events in this file
                        // Increment progress bar and get out of here
                        None => {
                            tx_progress.send(ievent);
                            break
                        }
                    };
                }
            }
        )
    }
    rx
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::{thread, time};
    extern crate alice_open_data;

    #[test]
    fn init_and_drop_dataset() {
        Dataset::new([alice_open_data::test_file().unwrap()], 1);
    }

    #[test]
    fn iterate_items() {
        let ds = Dataset::new([alice_open_data::test_file().unwrap()], 1);
        assert!(ds.count() > 0);
    }

    #[test]
    /// Provoke that we drop the dataset (and its reciever) before the
    /// Sender is finished reading an event from disk.  Make sure we
    /// get some sort of log message and not a panic
    fn quick_iterate_and_drop() {
        {
            let mut ds = Dataset::new([alice_open_data::test_file().unwrap()], 1);
            // Start of the IO thread by getting the first event
            let _ev = ds.next();
            // Drop the dataset here
        }
        // wait 1s for the next event to be read
        thread::sleep(time::Duration::from_secs(1));
    }

    #[test]
    fn fold_stream() {
        let files: Vec<_> = alice_open_data::all_files_10h().unwrap()
            .into_iter()
            .take(5)
            .collect();
        let ds = Dataset::new(files, 2);
        // let blub = ds.map(|_| 1).collect();
        // println!("{:?}", blub.wait());
        let nevents = ds
            .filter(|ev| ev.primary_vertex.is_some())
            .fold(0, |acc, _| acc + 1);
        assert!(nevents > 0);
    }    
}
