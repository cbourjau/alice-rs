#[macro_use]
extern crate ndarray;
extern crate histogram;
extern crate alice;
extern crate gnuplot;
extern crate glob;
extern crate rustfft;

extern crate rand;
extern crate indicatif;

use rand::{thread_rng, Rng};
use glob::glob;
use indicatif::ProgressBar;

use alice::dataset::Dataset;
use alice::track::Track;
use alice::track;
use alice::trigger_mask;

mod analyses;

use histogram::*;

use analyses::{ProcessEvent, Visualize};

fn main() {
    let files: Vec<_> = glob("/home/christian/lhc_data/alice/data/2010/LHC10h/000139510/ESDs/pass2/*/AliESDs.root")
        .expect("Can't resolve glob")
        .map(|path| path.unwrap())
        .collect();
    let pbar = ProgressBar::new(files.len() as u64);
    let files = pbar.wrap_iter(files.iter());
    let datasets = files
        .map(|path| Dataset::new(path.to_str().unwrap()))
        .flat_map(|ev| ev);

    trait Analysis: ProcessEvent + Visualize {}
    impl<T> Analysis for T where T: ProcessEvent + Visualize {}

    let mut analyses: Vec<Box<Analysis>> = vec![
        Box::new(analyses::PtMultiplicity::new()),
        Box::new(analyses::SingleParticleDistributions::new()),
        Box::new(analyses::ParticlePairDistributions::new()),
        Box::new(analyses::EventDistributions::new()),
        ];

    let mut hist_ntracks_v0 = HistogramBuilder::<Ix2>::new()
                .add_equal_width_axis(10, 0., 2e3)
                .add_equal_width_axis(10, 0., 6e2)
                .build().expect("Error building histogram");

    let sel_events = datasets
        .filter(|ev| {ev.primary_vertex.as_ref()
                      .map(|pv| pv.z.abs() < 8.)
                      .unwrap_or(false)})
        .filter(|ev| ev.multiplicity > 500)
        .filter(|ev| ev.multiplicity < 2000)
        .filter(|ev| ev.trigger_mask.contains(trigger_mask::MINIMUM_BIAS));

    let mut rng = thread_rng();
    for ev in sel_events {
        let mut filtered_tracks: Vec<&Track> = {
            let pv = ev.primary_vertex.as_ref().unwrap();
            // see AliESDtrackCuts.cxx:1366
            ev.tracks.iter()
                .filter(|tr| tr.flags.contains(track::ITS_REFIT))
                .filter(|tr| tr.dca_to_point_xy(pv.x, pv.y) < 2.4)
                .filter(|tr| tr.dca_to_point_z(pv.z) < 3.2)
                .filter(|tr| tr.eta().abs() < 0.8)
                .filter(|tr| tr.quality_tpc.ncls > 70)
                .filter(|tr| tr.pt() > 0.15)
                .collect()
        };
        rng.shuffle(filtered_tracks.as_mut_slice());

        // Correlation between number of tracks and multiplicity
        let v0_mult = ev.vzero.multiplicity_v0a() + ev.vzero.multiplicity_v0c();
        hist_ntracks_v0.fill(&[v0_mult as f64, ev.multiplicity as f64]);
        for analysis in analyses.iter_mut() {
            (*analysis).process_event(&ev, filtered_tracks.as_slice());
        }
    }
    for analysis in analyses {
        (*analysis).visualize();
    }
}
