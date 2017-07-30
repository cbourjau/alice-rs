extern crate histogram;
extern crate alice;
extern crate gnuplot;
extern crate glob;
extern crate ndarray;

use glob::glob;

use alice::dataset::Dataset;
use alice::track::Track;
use alice::track;
use alice::trigger;

mod analyses;

use histogram::*;

use analyses::{ProcessEvent, Visualize};

fn main() {
    let files = glob("/home/christian/lhc_data/alice/data/2010/LHC10h/000139510/ESDs/pass2/*/AliESDs.root")
        .expect("Can't resolve glob");

    let datasets = files
        .inspect(|f| println!("files {:?}", f))
        .map(|path| Dataset::new(path.unwrap().to_str().unwrap()))
        .flat_map(|ev| ev);

    let mut pt_mult = analyses::PtMultiplicity::new();
    let mut single_dists = analyses::SingleParticleDistributions::new();
    let mut pair_dists = analyses::ParticlePairDistributions::new();
    let mut event_dists = analyses::EventDistributions::new();

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
        .filter(|ev| ev.triggers().contains(trigger::MINIMUM_BIAS));

    for ev in sel_events {
        let filtered_tracks: Vec<&Track> = {
            let pv = ev.primary_vertex.as_ref().unwrap();
            // see AliESDtrackCuts.cxx:1366
            ev.tracks.iter()
                .filter(|tr| {tr.flags.contains(track::ITS_REFIT)})
                .filter(|tr| {tr.dca_to_point_xy(pv.x, pv.y) < 2.4})
                .filter(|tr| {tr.dca_to_point_z(pv.z) < 3.2})
                .filter(|tr| tr.eta().abs() < 0.8)
                .filter(|tr| tr.quality_tpc.ncls > 70)
                .filter(|tr| tr.pt() > 0.15)
                .map(|tr| tr)
                .collect()
        };

        // Correlation between number of tracks and multiplicity
        let v0_mult = ev.vzero.multiplicity_v0a() + ev.vzero.multiplicity_v0c();
        hist_ntracks_v0.fill(&[v0_mult as f64, ev.multiplicity as f64]);

        pt_mult.process_event(&ev, filtered_tracks.as_slice());
        single_dists.process_event(&ev, filtered_tracks.as_slice());
        event_dists.process_event(&ev, filtered_tracks.as_slice());
        pair_dists.process_event(&ev, filtered_tracks.as_slice());
    }

    single_dists.visualize();
    pt_mult.visualize();
    event_dists.visualize();
    pair_dists.visualize();
}
