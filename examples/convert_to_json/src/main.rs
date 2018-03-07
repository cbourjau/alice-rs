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
        .map(|path| RootFile::new_from_file(&path).expect("Failed to open file"))
        .map(|rf| rf.items()[0].as_tree().unwrap())
        .flat_map(|tree| match DsIntoIter::new(&tree) {
            Ok(s) => s,
            Err(err) => panic!("An error occured! Message: {}", err),
        });

    let mut f = File::create("events.json").expect("Could not create file!");
    f.write_all(b"[\n").unwrap();
    let mut event_counter = 0;
    let _analysis_result = events
        // Apply a sensible default event filter
        .filter(default_event_filter)
        .flat_map(|ev| to_json(&ev))
        .take(10_000)
        .for_each(|json| {
            serde_json::to_writer(&mut f, &json).unwrap();
            f.write_all(b",\n").unwrap();
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
