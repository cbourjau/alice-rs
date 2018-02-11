#[macro_use]
extern crate criterion;
extern crate root_io;
extern crate alice_bench;

use criterion::{Bencher, Criterion, Fun};


extern crate alice_open_data;
use root_io::RootFile;

fn read_rust(n_files: usize) {
    use alice_bench::dataset_rust::DatasetIntoIter;
    let _max_chi2 = alice_open_data::all_files_10h().unwrap()
        .into_iter()
        .take(n_files)
        .map(|path| RootFile::new_from_file(&path).expect("Failed to open file"))
        .map(|rf| rf.items()[0].as_tree().unwrap())
        .flat_map(|tree| {
            match DatasetIntoIter::new(&tree) {
                Ok(s) => s,
                Err(err) => panic!("An error occured! Message: {}", err)
            }})
        .flat_map(|event| event.tracks().map(|tr| tr.itschi2).collect::<Vec<_>>())
        .fold(0.0, |max, chi2| if chi2 > max {chi2} else {max});
}

fn read_cpp(n_files: usize) {
    use alice_bench::dataset_cpp::DatasetIntoIter;
    let _max_chi2 = alice_open_data::all_files_10h().unwrap()
        .into_iter()
        .take(n_files)
        .flat_map(|path| {
            match DatasetIntoIter::new(&path) {
                Ok(s) => s,
                Err(err) => panic!("An error occured! Message: {}", err)
            }})
        .flat_map(|event|event.tracks().map(|tr| tr.itschi2).collect::<Vec<_>>())
        .fold(0.0, |max, chi2| if chi2 > max {chi2} else {max});
}

fn bench_rust(b: &mut Bencher, n_files: &usize) {
    b.iter(|| read_rust(*n_files));
}

fn bench_cpp(b: &mut Bencher, n_files: &usize) {
    b.iter(|| read_cpp(*n_files));
}

fn criterion_benchmark(c: &mut Criterion) {
    let funs = vec![
        Fun::new("Rust", bench_rust),
        // Fun::new("cpp", bench_cpp),
    ];
    let n_files = 10;
    c
        .sample_size(10)
        .warm_up_time(::std::time::Duration::from_secs(10))
        .measurement_time(::std::time::Duration::from_secs(200))
        .with_plots()
        .bench_functions("Rust", funs, &n_files);
}


criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
