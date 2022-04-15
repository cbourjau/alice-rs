//! Benchmarks which were used to compare an earlier version with the
//! official c++ ROOT version. Note that these benchmarks are not
//! ported to the async code, yet!

#[macro_use]
extern crate criterion;
extern crate malice;
extern crate root_io;

use criterion::{Bencher, Criterion, Fun};
use futures::stream::{self, StreamExt};

extern crate alice_open_data;
use malice::event_stream_from_esd_file;

async fn read_rust(n_files: usize) {
    let files = alice_open_data::all_files_10h().unwrap().into_iter();
    let _max_chi2 = stream::iter(files)
        .take(n_files)
        .then(|f| event_stream_from_esd_file(f))
        .map(|res_event_stream| res_event_stream.map(|evt| evt.unwrap()))
        .flatten()
        .map(|event| {
            event
                .tracks()
                .map(|tr| tr.its_chi2)
                .fold(0.0, |max, chi2| if chi2 > max { chi2 } else { max })
        })
        .fold(0.0, |max, chi2| async move {
            if chi2 > max {
                chi2
            } else {
                max
            }
        });
}

#[cfg(feature = "cpp")]
fn read_cpp(n_files: usize) {
    use malice::dataset_cpp::DatasetIntoIter;
    let _max_chi2 = alice_open_data::all_files_10h()
        .unwrap()
        .into_iter()
        .take(n_files)
        .flat_map(|path| match DatasetIntoIter::new(&path) {
            Ok(s) => s,
            Err(err) => panic!("An error occured! Message: {}", err),
        })
        .flat_map(|event| event.tracks().map(|tr| tr.itschi2).collect::<Vec<_>>())
        .fold(0.0, |max, chi2| if chi2 > max { chi2 } else { max });
}

fn bench_rust(b: &mut Bencher, n_files: &usize) {
    b.iter(|| read_rust(*n_files));
}
#[cfg(feature = "cpp")]
fn bench_cpp(b: &mut Bencher, n_files: &usize) {
    b.iter(|| read_cpp(*n_files));
}

fn criterion_benchmark(c: &mut Criterion) {
    let funs = vec![
        Fun::new("Rust", bench_rust),
        #[cfg(feature = "cpp")]
        Fun::new("cpp", bench_cpp),
    ];
    let n_files = 1usize;
    c.bench_functions("Rust", funs, n_files);
}

criterion_group! {
    name = benches;
    config = Criterion::default()
    .sample_size(5)
        .warm_up_time(::std::time::Duration::from_secs(10))
        .measurement_time(::std::time::Duration::from_secs(200))
        .with_plots();
    targets = criterion_benchmark
}

criterion_main!(benches);
