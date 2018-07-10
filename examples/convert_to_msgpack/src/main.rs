//! Simple (and dirty) way to dump a subset of the data from the .root files to json files

extern crate rmp_serde as rmps;
extern crate serde;
#[macro_use]
extern crate serde_derive;

extern crate alice_open_data;
extern crate malice;
extern crate root_io;

use std::fs::File;
use std::io::Write;

use serde::Serialize;
use rmps::Serializer;

use root_io::RootFile;
use malice::{DatasetIntoIter as DsIntoIter, Event};
use malice::{default_event_filter, default_track_filter};

#[derive(Debug, PartialEq, Serialize)]
struct MiniEvent {
    multiplicity: u32,
    zvtx: f32,
    etas: Vec<f32>,
    phis: Vec<f32>,
}

impl<'a> From<&'a Event> for MiniEvent {
    fn from(event: &Event) -> Self {
        // Fill only if we have a valid primary vertex This fails if
        // there are events without a primary vertex, but such events
        // are not valid anyways!
        let prime_vtx = event
            .primary_vertex()
            .expect("Event has not primary vertex!");

        let tracks: Vec<_> = event.tracks()
	// Apply a sensible default "cut" on the valid tracks
	    .filter(|tr| default_track_filter(&tr, &prime_vtx))
            .collect();
        let etas: Vec<_> = tracks.iter().map(|tr| tr.eta()).collect();
        let phis: Vec<_> = tracks.iter().map(|tr| tr.phi()).collect();

        Self {
            multiplicity: tracks.len() as u32,
            zvtx: prime_vtx.z,
            etas: etas,
            phis: phis,
        }
    }
}

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
    println!("Wrote {} events to events.json", event_counter);
}
