extern crate alice_open_data;
extern crate alice;
extern crate gnuplot;
extern crate histogram;

use alice::dataset::Dataset;
use alice::event::Event;
use alice::analysis::cuts;
use alice::analysis::traits::{Visualize};

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
    let io_threads = 2;
    let dataset = Box::new(Dataset::new(files, io_threads));
    let analysis_result = dataset.install(&single_distribution_analysis);
    analysis_result.visualize();
}

/// Filters and folds a stream of `events`; producing the result of the analysis
/// The idea of this function is that it may be applied to `M` streams in parallel.
/// For that, the returned object must implementent `alice::Merge`
fn single_distribution_analysis(events: Box<Iterator<Item=Event>>) -> distribution::Distribution {
    events
    // Event selection
        .filter(cuts::default_event_filter)
    // Track selection
        .map(cuts::filter_tracks)
    // Analysis; Fold this chunk of events
        .fold(distribution::Distribution::new(), |analysis, ev| {
            analysis.process_event(&ev)
        })

}
