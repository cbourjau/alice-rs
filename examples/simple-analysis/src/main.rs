use malice::default_event_filter;
use malice::event_iterator_from_files;

mod distribution;
use distribution::SimpleAnalysis;

fn main() {
    let files = alice_open_data::all_files_10h()
        .expect("No data files found. Did you download with alice-open-data?");

    // Create an iterator over all the events in all the given files
    let events = event_iterator_from_files(files.into_iter());

    let mut analysis = SimpleAnalysis::new();
    for event in events.filter(default_event_filter) {
        analysis.process_event(&event);
    }
    // Optionally write results to disc
    analysis.write_to_disc().unwrap();
    // Optionally compute the centrality bin edges and print them in the terminal
    analysis.compute_centrality_edges();
    // Visualize the results of this analysis using gnuplot
    analysis.visualize();
}
