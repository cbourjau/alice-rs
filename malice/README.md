# malice
[![Crates.io Version](https://img.shields.io/crates/v/malice.svg)](https://crates.io/crates/malice)
https://docs.rs/malice/

"milli ALICE" aka `malice` is a tiny framework defining some sensible defaults to analyze the ALICE open data.

# Example

Here is a very simple example analysis using `malice` and other crates from this repository.
It measures the pseudorapidity distribution of the reconstructed tracks.
For a more comprehensive, but still small, example (including concurrency) check out [simple-analysis](https://github.com/cbourjau/alice-rs/tree/master/simple-analysis).


``` rust
extern crate alice_open_data;
extern crate histogram;
extern crate malice;
extern crate root_io;

use histogram::*;
use root_io::RootFile;

use malice::{Event, DatasetIntoIter as DsIntoIter};
use malice::{default_track_filter, default_event_filter};

fn main() {
    // Iterator over files of the Open Data set
    let files: Vec<_> = alice_open_data::all_files_10h()
        .expect("No data files found. Did you download with alice-open-data?")
        .into_iter()
        .collect();

    // Create an iterator over `malice::event::Event`s
    let events = files
        .iter()
        .map(|path| RootFile::new_from_file(&path).expect("Failed to open file"))
        .map(|rf| rf.items()[0].as_tree().unwrap())
        .flat_map(|tree| match DsIntoIter::new(&tree) {
            Ok(s) => s,
            Err(err) => panic!("An error occured! Message: {}", err),
        });

    // Fold the `malice::event::Events` with the analysis	
    let _analysis_result: SimpleAnalysis = events
        // Apply a sensible default event filter
        .filter(default_event_filter)
        .fold(SimpleAnalysis::new(), |analysis, ev| { analysis.process_event(&ev) });
    // Do something with the result...
}

pub struct SimpleAnalysis {
    // Histogram of the pseudorapidity (eta) distribution of valid tracks
    pub eta_distribution: Histogram<i32, [usize; 1]>,
}

impl SimpleAnalysis {
    fn new() -> SimpleAnalysis {
	// 50 bins from -0.9 to 0.9
	let (neta, eta_min, eta_max) = (50, -0.9, 0.9);
        SimpleAnalysis {
	    eta_distribution: HistogramBuilder::<[usize; 1]>::new()
                .add_equal_width_axis(neta, eta_min, eta_max)
                .build()
                .expect("Error building histogram"),
        }
    }

    // Update the histogram with the given event
    fn process_event(mut self, event: &Event) -> Self
    {
        // Fill only if we have a valid primary vertex
        if let Some(prime_vtx) = event.primary_vertex() {
            self.eta_distribution
                .extend(
                    event.tracks()
		    // Apply a sensible default "cut" on the valid tracks
			.filter(|tr| default_track_filter(&tr, &prime_vtx))
                        .map(|tr| [tr.eta() as f64]));
	};
        self
    }
}
```
