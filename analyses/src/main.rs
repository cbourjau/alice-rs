#[macro_use]
extern crate ndarray;
extern crate histogram;
extern crate alice;
extern crate gnuplot;
extern crate glob;
extern crate rustfft;
extern crate num_traits as libnum;
extern crate rand;
extern crate indicatif;
extern crate alice_open_data;

use rand::{thread_rng, Rng};
use glob::glob;
// use indicatif::ProgressBar;

use alice::dataset::Dataset;
use alice::track::{self, Track};
use alice::trigger_mask;

mod analyses;

use analyses::{ProcessEvent, Visualize};

trait Analysis: ProcessEvent + Visualize {}
impl<T> Analysis for T where T: ProcessEvent + Visualize {}


fn main() {
    let mut search_dir = alice_open_data::data_dir().unwrap();
    search_dir.push("alice/data/2010/LHC10h/**/AliESDs.root");
    let files: Vec<_> = glob(search_dir.to_str().unwrap())
        .expect("Can't resolve glob")
        .map(|path| path.unwrap())
        .take(50)
        .collect();
    // let pbar = ProgressBar::new(files.len() as u64);
    // let files = pbar.wrap_iter(files.iter());
    // let datasets = files.map(|path| Dataset::new(path)).flat_map(|ev| ev);
    let dataset = Dataset::new(files.as_slice());

    let mut analyses: Vec<Box<Analysis>> = vec![
        // Box::new(analyses::PtMultiplicity::new()),
        Box::new(analyses::SingleParticleDistributions::new()),
        Box::new(analyses::ParticlePairDistributions::new()),
        Box::new(analyses::EventDistributions::new()),
        ];

    let sel_events = dataset
        .filter(|ev| {
                    ev.primary_vertex
                        .as_ref()
                        .map(|pv| pv.z.abs() < 8.)
                        .unwrap_or(false)
                })
        .filter(|ev| ev.multiplicity > 1)
        .filter(|ev| ev.trigger_mask.contains(trigger_mask::MINIMUM_BIAS));

    let mut rng = thread_rng();
    for ev in sel_events {
        let mut filtered_tracks: Vec<&Track> = {
            let pv = ev.primary_vertex.as_ref().unwrap();
            // see AliESDtrackCuts.cxx:1366
            ev.tracks
                .iter()
                .filter(|tr| tr.flags.contains(track::ITS_REFIT))
                .filter(|tr| tr.dca_to_point_xy(pv.x, pv.y) < 2.4)
                .filter(|tr| tr.dca_to_point_z(pv.z) < 3.2)
                .filter(|tr| tr.eta().abs() < 0.8)
                .filter(|tr| tr.quality_tpc.ncls > 70)
                .filter(|tr| tr.pt() > 0.15)
                .collect()
        };
        // Shuffle selected tracks to avoid correlations from previous orderings
        rng.shuffle(filtered_tracks.as_mut_slice());

        for analysis in &mut analyses {
            (*analysis).process_event(&ev, filtered_tracks.as_slice());
        }
    }
    for analysis in analyses {
        (*analysis).visualize();
    }
}
