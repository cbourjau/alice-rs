extern crate alice_open_data;
extern crate failure;
extern crate gnuplot;
extern crate histogram;
extern crate malice;
extern crate root_io;

use std::sync::mpsc;
use std::thread::spawn;

use malice::default_event_filter;
use malice::DatasetIntoIter as DsIntoIter;
use root_io::RootFile;

mod distribution;
use distribution::SimpleAnalysis;

fn main() {
    let files: Vec<_> = alice_open_data::all_files_10h()
        .expect("No data files found. Did you download with alice-open-data?")
        .into_iter()
        // .take(50)
        .collect();
    if files.is_empty() {
        panic!("Somehow no files were found! Something is fishy!");
    }
    // Create a channel to transfer items from the worker thread.
    let (tx, rx) = mpsc::sync_channel(10);
    // Move this iterator to a new worker thread and run it there.
    spawn(move || {
        files
            .into_iter()
            .map(|path| RootFile::new_from_file(&path).expect("Failed to open file"))
            .map(|rf| rf.items()[0].as_tree().unwrap())
            .flat_map(|tree| match DsIntoIter::new(&tree) {
                Ok(s) => s,
                Err(err) => panic!("An error occured! Message: {}", err),
            })
            .for_each(|ev| {
                tx.send(ev).unwrap();
            })
    });

    let analysis: SimpleAnalysis = rx
        .into_iter()
        .filter(default_event_filter)
        .fold(SimpleAnalysis::new(), |analysis, ev| {
            analysis.process_event(&ev)
        });
    analysis.write_to_disc().unwrap();
    analysis.compute_centrality_edges();
    analysis.visualize();
}
