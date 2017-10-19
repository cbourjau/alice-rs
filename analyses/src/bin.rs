#![feature(test)]
extern crate test;

extern crate analyses;
extern crate alice_open_data;
extern crate alice;
extern crate rand;
extern crate indicatif;
extern crate glob;

extern crate rayon;

use rand::{thread_rng, Rng};

use alice::dataset::{Dataset, DatasetProducer};
use alice::track;
use alice::trigger_mask;
use alice::event::Event;

use analyses::{
    ProcessEvent,
    Visualize,
    Merge
};

fn main() {
    let files: Vec<_> = alice_open_data::all_files_10h()
        .expect("No data files found. Did you download with alice-open-data?")
        .into_iter()
        .take(20)
        .collect();
    let dataset = Dataset::new(files, 4);
    let mut analyses = dataset.install(&pair_analysis);
    let (mut analysis, analyses) = analyses.split_first_mut().unwrap();
    for a in analyses.into_iter().skip(1) {
        analysis.merge(&a);
    }
    analysis.visualize();
}

fn pair_analysis(events: DatasetProducer) -> analyses::ParticlePairDistributions
{
    events
        // Event selection
        .filter(|ref ev| {
            ev.primary_vertex.as_ref()
                .map(|pv| pv.z.abs() < 8.)
                .unwrap_or(false)
        })
        .filter(|ref ev| ev.multiplicity > 1)
        .filter(|ref ev| ev.trigger_mask.contains(trigger_mask::MINIMUM_BIAS))
        // Track selection
        .map(|ev| filter_tracks(ev))
        // Analysis; Fold this chunk of events
        .fold(analyses::ParticlePairDistributions::new(), |analysis, ref ev| {
            analysis.process_event(ev)
        })

}

/// Filter out invalid tracks
fn filter_tracks(mut ev: Event) -> Event {
    {
        let pv = ev.primary_vertex.as_ref().unwrap();
        // see AliESDtrackCuts.cxx:1366
        ev.tracks = ev.tracks
            .into_iter()
            .filter(|ref tr| tr.flags.contains(track::ITS_REFIT))
            .filter(|ref tr| tr.dca_to_point_xy(pv.x, pv.y) < 2.4)
            .filter(|ref tr| tr.dca_to_point_z(pv.z) < 3.2)
            .filter(|ref tr| tr.eta().abs() < 0.8)
            .filter(|ref tr| tr.quality_tpc.ncls > 70)
            .filter(|ref tr| tr.pt() > 0.15)
            .collect();
    }
    // Shuffle selected tracks to avoid correlations from datataking orderings
    thread_rng().shuffle(ev.tracks.as_mut_slice());
    ev
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_pairs(b: &mut Bencher) {
        b.iter(|| {
            let files: Vec<_> = alice_open_data::all_files_10h()
                .expect("No data files found. Did you download with alice-open-data?")
                .into_iter()
                .take(2)
                .collect();
            pair_analysis(files)
        });
    }
}
