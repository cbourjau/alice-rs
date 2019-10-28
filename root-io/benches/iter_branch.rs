extern crate criterion;
extern crate root_io;
extern crate alice_open_data;
extern crate nom;

use nom::number::complete::{be_i32, be_u32, be_f32};

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use root_io::RootFile;

fn fixed_size_branch() {
    let path = alice_open_data::test_file().unwrap();

    let f = RootFile::new_from_file(&path).expect("Failed to open file");
    let t = f.items()[0].as_tree().unwrap();
    let iter = t
        .branch_by_name("PrimaryVertex.AliVertex.fNContributors").unwrap()
        .as_fixed_size_iterator(|i| be_i32(i)).unwrap();
    iter.for_each(|el| {black_box(el);});
}

fn var_size_branch() {    
    let path = alice_open_data::test_file().unwrap();
    let f = RootFile::new_from_file(&path).expect("Failed to open file");
    let t = f.items()[0].as_tree().unwrap();

    let track_counter: Vec<_> = t
        .branch_by_name("Tracks").unwrap()
        .as_fixed_size_iterator(|i| be_u32(i)).unwrap()
        .collect();
    let iter = t
        .branch_by_name("Tracks.fX").unwrap()
        .as_var_size_iterator(|i| be_f32(i), &track_counter).unwrap();
    iter.for_each(|el| {black_box(el);});
}

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fixed_size_branch", |b| b.iter(|| fixed_size_branch));
    c.bench_function("var_size_branch", |b| b.iter(|| var_size_branch));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
