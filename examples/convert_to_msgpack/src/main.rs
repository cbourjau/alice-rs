//! Simple (and dirty) way to dump a subset of the data from the .root files to json files
use std::fs::File;
use std::io::Write;

use rmp_serde::Serializer;
use serde::Serialize;
use serde_derive::Serialize;

use malice::event_iterator_from_files;
use malice::{default_event_filter, default_track_filter, Event};

/// Struct holding all the information we want to dump to a new json
/// file.
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

        let tracks: Vec<_> = event
            .tracks()
            // Apply a sensible default "cut" on the valid tracks
            .filter(|tr| default_track_filter(&tr, &prime_vtx))
            .collect();
        let etas: Vec<_> = tracks.iter().map(|tr| tr.eta()).collect();
        let phis: Vec<_> = tracks.iter().map(|tr| tr.phi()).collect();

        Self {
            multiplicity: tracks.len() as u32,
            zvtx: prime_vtx.z,
            etas,
            phis,
        }
    }
}

fn main() {
    // Iterator over files of the Open Data set
    let files: Vec<_> = alice_open_data::all_files_10h()
        .expect("No data files found. Did you download with alice-open-data?");

    // Create an iterator over `malice::event::Event`s
    let events = event_iterator_from_files(files.into_iter());

    // Setup the output file
    let mut f = File::create("events.bin").expect("Could not create file!");
    let mut event_counter = 0;
    // Iterate through all the _valid_ events; at most 10k
    for event in events.filter(default_event_filter).take(10_000) {
        let event = MiniEvent::from(&event);
        let mut buf = Vec::new();
        event
            .serialize(&mut Serializer::new_named(&mut buf))
            .unwrap();
        // Write this event to the output file
        f.write_all(&buf).unwrap();
        event_counter += 1;
    }
    println!("Wrote {} events to events.json", event_counter);
}
