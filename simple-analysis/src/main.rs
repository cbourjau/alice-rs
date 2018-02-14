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
    // Track selection
    // .map(filter_tracks)
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
    // Require some activity in the central region
    // if event.multiplicity <= 0 {
    //     return false;
    // }
    // Only use events which fired the minimu bias trigger
    // if !event.trigger_mask.contains(trigger_mask::MINIMUM_BIAS) {
    //     return false;
    // }
    true
}

// /// Apply default track selection cuts
// /// The returned Event contains only those tracks that passed the cuts
// /// The cuts are partly inspired by those define around
// /// AliESDtrackCuts.cxx:1366
// pub fn filter_tracks(mut ev: Event) -> Event {
//     {
//         let pv = ev.primary_vertex().expect("No primary vertex for found!");
//         // see AliESDtrackCuts.cxx:1366
//         // ev.tracks = ev.tracks()
//         // Part of the SPD were turned off due to cooling issues in
//         // LHC10h data-taking In order to keep a flat acceptance in
//         // phi and eta, we allow tracks without a hit in the SPD to
//         // _not_ have an ITS refit The expense of this looser cut is
//         // an increased number of secondary particles
//             // .filter(|tr| {
//             //     // SPD && ITS_REFIT
//             //     (tr.quality_its.clusters_on_layer.intersects(
//             //         track::SPD_INNER | track::SPD_OUTER)
//             //      & tr.flags.contains(track::ITS_REFIT)) ||
//             //     // !SPD && ITS_REFIT
//             //     (!tr.quality_its.clusters_on_layer.intersects(
//             //         track::SPD_INNER | track::SPD_OUTER)
//             //      & tr.flags.contains(track::ITS_REFIT)) ||
//             //     // !SPD && !ITS_REFIT
//             //     (!tr.quality_its.clusters_on_layer.intersects(
//             //         track::SPD_INNER | track::SPD_OUTER)
//             //      & !tr.flags.contains(track::ITS_REFIT))
//         // })
//             .filter(|tr| tr.flags.contains(track::Flags::ITS_REFIT))
//             .filter(|tr| tr.flags.contains(track::Flags::TPC_REFIT))
//             .filter(|tr| tr.dca_to_point_xy(pv.x, pv.y) < 2.4)
//             .filter(|tr| tr.dca_to_point_z(pv.z) < 3.2)
//             .filter(|tr| tr.eta().abs() < 0.9)

//             .filter(|tr| tr.pt() > 0.15)
//             // .filter(|tr| tr.quality_tpc.n_clusters > 70)
//             // .filter(|tr| tr.quality_tpc.chi2_per_cluster() <= 4.0)
//             // .filter(|tr| tr.quality_its.chi2_per_cluster() <= 36.0)
//             .collect();
//     }
//     // Shuffle selected tracks to avoid correlations from datataking orderings
//     // Trust me, this is needed!
//     // thread_rng().shuffle(ev.tracks.as_mut_slice());
//     ev
// }
