//! Simple (and dirty) way to dump a subset of the data from the .root files to json files

extern crate serde;
#[macro_use]
extern crate serde_json;
extern crate alice_open_data;
extern crate malice;
extern crate root_io;

use std::fs::File;
use std::io::Write;

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
        .filter_map(|path| {
            // Open the file
            RootFile::new_from_file(&path)
                // Get the relevant tree (there are two per file)
                .and_then(|rf| rf.items()[0].as_tree())
                // Convert into an Iterator over the data set
                .and_then(|tree| DsIntoIter::new(&tree))
                // Print an error if something went wrong
                .map_err(|err| {println!("Error for file {:?}; skipping it", path ); err})
                .ok()
        })
        // Create an owned iterator over events
        .flat_map(|dataset| dataset.into_iter());

    // Create the json file
    let mut f = File::create("events.json").expect("Could not create file!");
    // Poor man's way: Write the opening bracket
    f.write_all(b"[\n").unwrap();
    let mut event_counter = 0;
    let _analysis_result = events
        // Apply a sensible default event filter
        .filter(default_event_filter)
        .take(10_000)  // Stop after 10k valid events
        .flat_map(|ev| to_json(&ev))
        .for_each(|json| {
            // Put comma and linebreak in fron of previous entry
            if event_counter > 0 {
                f.write_all(b",\n").unwrap();
            }
            serde_json::to_writer(&mut f, &json).unwrap();
            event_counter += 1;
        });
    f.write_all(b"]").unwrap();
    println!("Wrote {} events to events.json", event_counter);
}

/// Update the histogram with the given event
fn to_json(event: &Event) -> Option<serde_json::Value>
{
    // Fill only if we have a valid primary vertex
    event.primary_vertex().map(|prime_vtx| {
        let tracks: Vec<_> = event.tracks()
	// Apply a sensible default "cut" on the valid tracks
	    .filter(|tr| default_track_filter(&tr, &prime_vtx))
            .collect();
        let etas: Vec<_> = tracks.iter().map(|tr| tr.eta()).collect();
        let phis: Vec<_> = tracks.iter().map(|tr| tr.phi()).collect();
        json!(
            {
                "multiplicity": tracks.len(),
                "zvtx": prime_vtx.z,
                "etas": etas,
                "phis": phis
            }
        )
    })
}
