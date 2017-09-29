extern crate analyses;
extern crate alice_open_data;
extern crate alice;
extern crate rand;
extern crate indicatif;
extern crate glob;

use rand::{thread_rng, Rng};
// use indicatif::ProgressBar;

use alice::dataset::Dataset;
use alice::track;
use alice::trigger_mask;
use alice::event::Event;

use analyses::{ProcessEvent, Visualize};


fn main() {
    let files: Vec<_> = alice_open_data::all_files_10h()
        .expect("No data files found. Did you download with alice-open-data?")
        .into_iter()
        .take(10)
        .collect();
    // let pbar = ProgressBar::new(files.len() as u64);
    // let files = pbar.wrap_iter(files.iter());
    // let datasets = files.map(|path| Dataset::new(path)).flat_map(|ev| ev);
    let dataset = Dataset::new(files, 2);

    let analysis = dataset
        // Event selection
        .filter(|ev| {
                    ev.primary_vertex.as_ref()
                        .map(|pv| pv.z.abs() < 8.)
                        .unwrap_or(false)
                })
        .filter(|ev| ev.multiplicity > 1)
        .filter(|ev| ev.trigger_mask.contains(trigger_mask::MINIMUM_BIAS))
        // Track selection
        .map(|ev| filter_tracks(ev))
    // Analysis
        .fold(analyses::ParticlePairDistributions::new(), |analysis, ev| {
            analysis.process_event(&ev)
        });
    analysis.visualize();
}

// fn filter_event(ev: &Event) -> Option<&Event> {
//     let pred =
//         // 
//         ev.primary_vertex.as_ref()
//         .map(|pv| pv.z.abs() < 8.)
//         .unwrap_or(false)
//         && 
// }

/// Filter out invalid tracks
fn filter_tracks(mut ev: Event) -> Event {
    {
        let pv = ev.primary_vertex.as_ref().unwrap();
        // see AliESDtrackCuts.cxx:1366
        ev.tracks = ev.tracks
            .into_iter()
            .filter(|tr| tr.flags.contains(track::ITS_REFIT))
            .filter(|tr| tr.dca_to_point_xy(pv.x, pv.y) < 2.4)
            .filter(|tr| tr.dca_to_point_z(pv.z) < 3.2)
            .filter(|tr| tr.eta().abs() < 0.8)
            .filter(|tr| tr.quality_tpc.ncls > 70)
            .filter(|tr| tr.pt() > 0.15)
            .collect();
    }
    // Shuffle selected tracks to avoid correlations from datataking orderings
    thread_rng().shuffle(ev.tracks.as_mut_slice());
    ev
}
