extern crate alice_open_data;
extern crate root_io;
extern crate malice;
extern crate gnuplot;
extern crate histogram;

use malice::dataset_rust::DatasetIntoIter as DsIntoIter;
use malice::event::Event;
// use malice::track;
// use malice::analysis::cuts;
use root_io::RootFile;

mod distribution;

fn main() {
    let files: Vec<_> = alice_open_data::all_files_10h()
        .expect("No data files found. Did you download with alice-open-data?")
        .into_iter()
        // .take(50)
        .collect();
    if files.is_empty() {
        panic!("Somehow no files were found! Something is fishy!");
    }
    let events = files
        .into_iter()
        .map(|path| RootFile::new_from_file(&path).expect("Failed to open file"))
        .map(|rf| rf.items()[0].as_tree().unwrap())
        .flat_map(|tree| {
            match DsIntoIter::new(&tree) {
                Ok(s) => s,
                Err(err) => panic!("An error occured! Message: {}", err)
            }});
    let analysis_result = single_distribution_analysis(Box::new(events));
    analysis_result.visualize();
}

/// Filters and folds a stream of `events`; producing the result of the analysis
/// The idea of this function is that it may be applied to `M` streams in parallel.
/// For that, the returned object must implementent `alice::Merge`
fn single_distribution_analysis(events: Box<Iterator<Item=Event>>) -> distribution::Distribution {
    events
        // Event selection
        .filter(default_event_filter)
        // Analysis; Fold this chunk of events
        .fold(distribution::Distribution::new(), |analysis, ev| {
            analysis.process_event(&ev)
        })

}


/// A simple but reasonable default event selection
/// Returns true if the given event passes the recommended selection criterion
pub fn default_event_filter(event: &Event) -> bool {
    // Check if the event has a reconstructed primary vertex
    if let Some(ref pv) = event.primary_vertex() {
        // Primary vertex must be within +- 8cm
        // of the nominal interaction point along beam axis
        if pv.z.abs() > 8.0 {
            return false;
        }
    } else {
        return false;
    }
    true
}
