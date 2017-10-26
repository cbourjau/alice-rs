#![feature(test)]
extern crate test;

extern crate analyses;
extern crate alice_open_data;
extern crate alice;
extern crate rand;
extern crate indicatif;
extern crate glob;

extern crate rayon;

mod selections;

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
        .collect();
    let dataset = Dataset::new(files, 2);
    let mut analyses = dataset.install(&pair_analysis);
    let (mut analysis, analyses) = analyses.split_first_mut().unwrap();
    for a in analyses.into_iter().skip(1) {
        analysis.merge(a);
    }
    analysis.visualize();
}

fn pair_analysis(events: DatasetProducer) -> analyses::ParticlePairDistributions
{
    events
        // Event selection
        .filter(selections::default_event_filter)
        // Track selection
        .map(selections::filter_tracks)
        // Analysis; Fold this chunk of events
        .fold(analyses::ParticlePairDistributions::new(), |analysis, ev| {
            analysis.process_event(&ev)
        })

}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use test::Bencher;

//     #[bench]
//     fn bench_pairs(b: &mut Bencher) {
//         b.iter(|| {
//             let files: Vec<_> = alice_open_data::all_files_10h()
//                 .expect("No data files found. Did you download with alice-open-data?")
//                 .into_iter()
//                 .take(2)
//                 .collect();
//             pair_analysis(files)
//         });
//     }
// }
