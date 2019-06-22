use malice::default_event_filter;
use malice::DatasetIntoIter as DsIntoIter;
use root_io::RootFile;

mod distribution;
use distribution::SimpleAnalysis;

fn main() {
    let files = alice_open_data::all_files_10h()
        .expect("No data files found. Did you download with alice-open-data?");

    // Create an iterator over all the events in all the given files
    let events = files.into_iter().flat_map(|path| {
        let events_in_file = RootFile::new_from_file(&path)
            .and_then(|rf| rf.items()[0].as_tree())
            .and_then(|tree| DsIntoIter::new(&tree));
        // Check if an error occurred. If so, we skip this file
        if let Err(ref e) = events_in_file {
            println!("Error occured while processing {:?}: {:?}", path, e);
        }
        // A little magic to flatten a Result<impl Iterator>
        events_in_file.into_iter().flatten()
    });

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
