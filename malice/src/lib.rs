//! # malice
//! "milli ALICE" aka `malice` is a tiny framework defining some sensible defaults to analyze the ALICE open data.
//! # Features
//! `malice` supports twp IO backends. The first and recommended one is the pure Rust `root-io` crate. The second one is behind the `cpp` feature gate and depends on the c++ ROOT framework.
//! # Example
//! Here is a very simple example analysis using `malice` and other crates from the [alice-rs](https://github.com/cbourjau/alice-rs) repository.
//! It measures the pseudorapidity distribution of the reconstructed tracks.
//! For a more comprehensive, but still small, example (including concurrency) check out [simple-analysis](https://github.com/cbourjau/alice-rs/tree/master/simple-analysis).
//!
//! ``` rust,ignore
//! extern crate alice_open_data;
//! extern crate histogram;
//! extern crate malice;
//! extern crate root_io;
//!
//! use histogram::*;
//! use root_io::RootFile;
//!
//! use malice::{Event, DatasetIntoIter as DsIntoIter};
//! use malice::{default_track_filter, default_event_filter};
//!
//! fn main() {
//!     // Iterator over files of the Open Data set
//!     let files: Vec<_> = alice_open_data::all_files_10h()
//!         .expect("No data files found. Did you download with alice-open-data?")
//!         .into_iter()
//!         .collect();
//!
//!     // Create an iterator over `malice::event::Event`s
//!     let events = files
//!         .iter()
//!         .map(|path| RootFile::new_from_file(&path).expect("Failed to open file"))
//!         .map(|rf| rf.items()[0].as_tree().unwrap())
//!         .flat_map(|tree| match DsIntoIter::new(&tree) {
//!             Ok(s) => s,
//!             Err(err) => panic!("An error occured! Message: {}", err),
//!         });
//!
//!     // Fold the `malice::event::Events` with the analysis
//!     let _analysis_result: SimpleAnalysis = events
//!         // Apply a sensible default event filter
//!         .filter(default_event_filter)
//!         .fold(SimpleAnalysis::new(), |analysis, ev| { analysis.process_event(&ev) });
//!     // Do something with the result...
//! }
//!
//! pub struct SimpleAnalysis {
//!     // Histogram of the pseudorapidity (eta) distribution of valid tracks
//!     pub eta_distribution: Histogram<i32, [usize; 1]>,
//! }
//!
//! impl SimpleAnalysis {
//!     fn new() -> SimpleAnalysis {
//! 	// 50 bins from -0.9 to 0.9
//! 	let (neta, eta_min, eta_max) = (50, -0.9, 0.9);
//!         SimpleAnalysis {
//! 	    eta_distribution: HistogramBuilder::<[usize; 1]>::new()
//!                 .add_equal_width_axis(neta, eta_min, eta_max)
//!                 .build()
//!                 .expect("Error building histogram"),
//!         }
//!     }
//!
//!     // Update the histogram with the given event
//!     fn process_event(mut self, event: &Event) -> Self
//!     {
//!         // Fill only if we have a valid primary vertex
//!         if let Some(prime_vtx) = event.primary_vertex() {
//!             self.eta_distribution
//!                 .extend(
//!                     event.tracks()
//! 		    // Apply a sensible default "cut" on the valid tracks
//! 			.filter(|tr| default_track_filter(&tr, &prime_vtx))
//!                         .map(|tr| [tr.eta() as f64]));
//! 	};
//!         self
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
pub use crate::event::{Event, TracksIter, TriggerMask};
pub use crate::primary_vertex::PrimaryVertex;
pub use crate::track::{Flags, ItsClusters, Track};
pub use crate::utils::{default_event_filter, default_track_filter, is_hybrid_track};

#[cfg(test)]
mod tests {
    use alice_open_data;
    use async_std;
    use futures::{future, StreamExt};
    use root_io::RootFile;

    use super::{default_event_filter, default_track_filter, Event};

    #[async_std::test]
    async fn test_filters() {
        let f = alice_open_data::test_file().unwrap();
        let rf = RootFile::new_from_file(&f).await.unwrap();
        let t = rf.items()[0].as_tree().await.unwrap();
        let events = Event::stream_from_tree(&t).await.unwrap();
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
            .map(|path| RootFile::new_from_file(&path).expect("Failed to open file"))
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
            let tree = RootFile::new_from_file(&file)
                .expect("Failed to open file")
                .items()[0]
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
                let tree = RootFile::new_from_file(&file)
                    .expect("Failed to open file")
                    .items()[0]
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
                let tree = RootFile::new_from_file(&file)
                    .expect("Failed to open file")
                    .items()[0]
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
