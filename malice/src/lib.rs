//! # malice
//! "milli ALICE" aka `malice` is a tiny framework defining some sensible defaults to analyze the ALICE open data.
//! # Features
//! `malice` supports two IO back-ends. The first and recommended one is the pure Rust `root-io` crate. The second one is behind the `cpp` feature gate and depends on the c++ ROOT framework.
//! # Example
//! Here is a very simple example "analysis" using `malice` which
//! counts the number of tracks in an event.  For a more
//! comprehensive, but still small, example check out
//! [simple-analysis](https://github.com/cbourjau/alice-rs/tree/master/examples/simple-analysis).
//!
//! ``` rust
//! use alice_open_data;
//! use malice::{default_event_filter, default_track_filter};
//! use malice::event_iterator_from_files;
//!
//! let file = alice_open_data::test_file()
//!     .expect("No data files found. Did you download with alice-open-data?");
//!
//! // Create an iterator over all the events in all the given files
//! let events = event_iterator_from_files(vec![file].into_iter());
//!
//! for event in events.filter(default_event_filter) {
//!      // Fill only if we have a valid primary vertex
//!      if let Some(prime_vtx) = event.primary_vertex() {
//!          let n_tracks = event
//!          .tracks()
//!          // Apply a sensible default "cut" on the valid tracks
//!          .filter(|tr| default_track_filter(&tr, &prime_vtx))
//!          .count();
//!          println!("This event had {} valid tracks", n_tracks);
//!     }
//! }
//! ```

#[macro_use]
extern crate bitflags;

#[cfg(feature = "cpp")]
pub mod dataset_cpp;
#[cfg(feature = "cpp")]
mod esd;
mod event;
mod primary_vertex;
mod track;
mod utils;

// re-exports
pub use crate::event::{event_stream_from_tree, Event, TriggerMask};
pub use crate::primary_vertex::PrimaryVertex;
pub use crate::track::{Flags, ItsClusters, Track};
pub use crate::utils::{default_event_filter, default_track_filter, is_hybrid_track};

use failure::Error;
use futures::prelude::*;
use futures::stream::StreamExt;

use root_io::{RootFile, Source};

use std::pin::Pin;

#[cfg(not(target_arch = "wasm32"))]
type EventStream = Pin<Box<dyn Stream<Item = Result<Event, Error>> + Send>>;
#[cfg(target_arch = "wasm32")]
type EventStream = Pin<Box<dyn Stream<Item = Result<Event, Error>>>>;

/// A helper function which turns a path to an ALICE ESD file into a
/// stream over the `Events` of that file.
pub async fn event_stream_from_esd_file<T>(p: T) -> EventStream
where
    T: Into<Source>,
{
    let tmp = {
        move || async {
            let rf = RootFile::new(p).await?;
            let tree = rf.items()[0].as_tree().await?;
            event_stream_from_tree(&tree).await
        }
    }();
    // Turn Result<Stream> into a Stream of Results
    match tmp.await {
	#[cfg(not(target_arch = "wasm32"))]
        Ok(s) => s.map(Ok).boxed(),
	#[cfg(target_arch = "wasm32")]
        Ok(s) => s.map(Ok).boxed_local(),	
        Err(err) => stream::iter(vec![Err(err)]).boxed(),
    }
}

/// Main entry point for analyses running over ALICE's open
/// data. Produces an iterator over events from an iterator over files
/// (either local or remote).
///
/// The necessary IO is done on a separate thread such that IO bound
/// tasks do not interfere with the CPU bound tasks of the analysis
/// itself. If an IO error is encountered the respective file will be
/// skipped.
///
/// This function is not available on the wasm32 target and must not
/// be called from an asynchronous context itself.
#[cfg(not(target_arch = "wasm32"))]
pub fn event_iterator_from_files<I, S>(sources: I) -> impl Iterator<Item = Event>
where
    I: IntoIterator<Item = S> + Send + 'static,
    S: Into<Source> + Send,
{
    use std::sync::mpsc::sync_channel;
    use std::thread::spawn;

    const BUFFERED_EVENTS: usize = 10;
    let (sender, receiver) = sync_channel(BUFFERED_EVENTS);
    spawn(|| {
        let mut rt = tokio::runtime::Runtime::new().expect("Failed to start IO runtime");
        rt.block_on(async move {
            stream::iter(sources)
                .then(event_stream_from_esd_file)
                .flatten()
                .try_for_each(|event| async {
                    // Errors if the receiving end has hung up
                    sender.send(event).map_err(Into::into)
                })
                .await
        })
        .expect("Failed to start IO processing.");
    });
    receiver.into_iter()
}

/// Create a stream of events found in the given files (local or
/// remote). You probably want to use `event_iterator_from_files`
/// instead unless you are a on the `wasm32` target.
pub fn event_stream_from_files<SI, S>(sources: SI) -> impl Stream<Item = Result<Event, Error>>
where
    SI: IntoIterator<Item = S>,
    S: Into<Source> + Send,
{
    futures::stream::iter(sources)
	.map(Into::into)
	.then(event_stream_from_esd_file).flatten()
}

#[cfg(test)]
mod tests {
    use alice_open_data;
    use async_std;
    use futures::{future, StreamExt};
    use root_io::RootFile;

    use super::{default_event_filter, default_track_filter, event_stream_from_tree};

    #[async_std::test]
    async fn test_filters() {
        let f = alice_open_data::test_file().unwrap();
        let rf = RootFile::new(f).await.unwrap();
        let t = rf.items()[0].as_tree().await.unwrap();
        let events = event_stream_from_tree(&t).await.unwrap();
        let mut cnt_evts = 0;
        let mut cnt_tracks = 0;
        let mut cnt_tracks_valid = 0;
        events
            .filter(|ev| future::ready(default_event_filter(ev)))
            .for_each(|ev| {
                cnt_evts += 1;
                cnt_tracks += ev.tracks().count();
                if let Some(pv) = ev.primary_vertex() {
                    cnt_tracks_valid +=
                        ev.tracks().filter(|t| default_track_filter(t, &pv)).count();
                }
                future::ready(())
            })
            .await;
        assert_eq!(cnt_evts, 2);
        assert_eq!(cnt_tracks, 11958);
        assert_eq!(cnt_tracks_valid, 2773);
    }

    #[test]
    #[cfg(feature = "cpp")]
    fn rust_cpp_identical_many_files() {
        use super::dataset_cpp::DatasetIntoIter as DsIntoIter_cpp;
        use super::dataset_rust::DatasetIntoIter as DsIntoIter_rust;

        let n_files = 500;
        let rust_iter = alice_open_data::all_files_10h()
            .unwrap()
            .into_iter()
            .take(n_files)
            .map(|path| RootFile::new(path).expect("Failed to open file"))
            .map(|rf| rf.items()[0].as_tree().unwrap())
            .flat_map(|tree| match DsIntoIter_rust::new(&tree) {
                Ok(s) => s,
                Err(err) => panic!("An error occured! Message: {}", err),
            });
        let cpp_iter = alice_open_data::all_files_10h()
            .unwrap()
            .into_iter()
            .take(n_files)
            .flat_map(|path| match DsIntoIter_cpp::new(&path) {
                Ok(s) => [path.to_owned()].to_vec().into_iter().cycle().zip(s),
                Err(err) => panic!("An error occured! Message: {}", err),
            });
        for (i, (rust_ev, (path, cpp_ev))) in rust_iter.zip(cpp_iter).enumerate() {
            // println!("{:?}", path);
            assert_eq!(rust_ev, cpp_ev, "Event {} differs in file {:?}", i, path);
        }
    }

    #[test]
    #[cfg(feature = "cpp")]
    fn rust_cpp_identical_funky_file_1() {
        use super::dataset_cpp::DatasetIntoIter as DsIntoIter_cpp;
        use super::dataset_rust::DatasetIntoIter as DsIntoIter_rust;

        let file = alice_open_data::all_files_10h()
            .unwrap()
            .into_iter()
            .find(|p| {
                p.to_str()
                    .unwrap()
                    // This file contains a bunch of "empty" baskets; i.e. baskets which claim to have events but are just zeros...
                    .contains("10000139038001.770/AliESDs.root")
            })
            .expect("Funky file not found");
        let rust_iter = {
            let tree = RootFile::new(file).expect("Failed to open file").items()[0]
                .as_tree()
                .unwrap();
            match DsIntoIter_rust::new(&tree) {
                Ok(s) => s,
                Err(err) => panic!("An error occured! Message: {}", err),
            }
        };
        let cpp_iter = match DsIntoIter_cpp::new(&file) {
            Ok(s) => s,
            Err(err) => panic!("An error occured! Message: {}", err),
        };
        for (rust_ev, cpp_ev) in rust_iter.zip(cpp_iter) {
            assert_eq!(rust_ev, cpp_ev);
        }
    }
    #[test]
    #[cfg(feature = "cpp")]
    fn rust_cpp_identical_funky_file_2() {
        use super::dataset_cpp::DatasetIntoIter as DsIntoIter_cpp;
        use super::dataset_rust::DatasetIntoIter as DsIntoIter_rust;
        let funkies = [
            // This files has baskets which, after parsing, have 0 bytes :P
            "10000139038002.40/AliESDs.root",
            // events with 0 tracks at end of basket
            "10000139038001.310/AliESDs.root",
        ];
        for funky in &funkies {
            let file = alice_open_data::all_files_10h()
                .unwrap()
                .into_iter()
                .find(|p| p.to_str().unwrap().contains(funky))
                .expect("Funky file not found");
            let mut rust_iter = {
                let tree = RootFile::new(file).expect("Failed to open file").items()[0]
                    .as_tree()
                    .unwrap();
                match DsIntoIter_rust::new(&tree) {
                    Ok(s) => s,
                    Err(err) => panic!("An error occured! Message: {}", err),
                }
            };
            let mut cpp_iter = match DsIntoIter_cpp::new(&file) {
                Ok(s) => s,
                Err(err) => panic!("An error occured! Message: {}", err),
            };
            assert_eq!(rust_iter.count(), cpp_iter.count());
        }
        for funky in &funkies {
            let file = alice_open_data::all_files_10h()
                .unwrap()
                .into_iter()
                .find(|p| p.to_str().unwrap().contains(funky))
                .expect("Funky file not found");
            let mut rust_iter = {
                let tree = RootFile::new(file).expect("Failed to open file").items()[0]
                    .as_tree()
                    .unwrap();
                match DsIntoIter_rust::new(&tree) {
                    Ok(s) => s,
                    Err(err) => panic!("An error occured! Message: {}", err),
                }
            };
            let mut cpp_iter = match DsIntoIter_cpp::new(&file) {
                Ok(s) => s,
                Err(err) => panic!("An error occured! Message: {}", err),
            };
            for (_i, (rust_ev, cpp_ev)) in rust_iter.zip(cpp_iter).enumerate() {
                assert_eq!(rust_ev, cpp_ev);
            }
        }
        // let cpp_iter = match DsIntoIter_cpp::new(&file) {
        //     Ok(s) => s,
        //     Err(err) => panic!("An error occured! Message: {}", err)
        // };
        // assert_eq!(rust_iter.count(), cpp_iter.count());
        // for (i, (rust_ev, cpp_ev)) in rust_iter.zip(cpp_iter).enumerate() {
        //     println!("{}", i);
        //     assert_eq!(rust_ev, cpp_ev);
        // }
    }

    #[test]
    #[cfg(feature = "cpp")]
    fn bench_cpp() {
        let n_files = 50;
        use super::dataset_cpp::DatasetIntoIter;
        let _max_chi2 = alice_open_data::all_files_10h()
            .unwrap()
            .into_iter()
            .take(n_files)
            .flat_map(|path| match DatasetIntoIter::new(&path) {
                Ok(s) => s,
                Err(err) => panic!("An error occured! Message: {}", err),
            })
            .flat_map(|event| event.tracks().map(|tr| tr.its_chi2).collect::<Vec<_>>())
            .fold(0.0, |max, chi2| if chi2 > max { chi2 } else { max });
    }
}
