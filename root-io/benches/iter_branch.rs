extern crate alice_open_data;
extern crate criterion;
extern crate nom;
extern crate root_io;

use nom::number::complete::{be_f32, be_i32, be_u32};

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use futures::StreamExt;
use tokio::runtime::Runtime;

use root_io::RootFile;

fn fixed_size_branch() {
    let path = alice_open_data::test_file().unwrap();

    let fut = async {
        let f = RootFile::new(path.as_path())
            .await
            .expect("Failed to open file");
        let t = f.items()[0].as_tree().await.unwrap();
        let iter = t
            .branch_by_name("PrimaryVertex.AliVertex.fNContributors")
            .unwrap()
            .as_fixed_size_iterator(|i| be_i32(i));
        iter.for_each(|el| async move {
            black_box(el);
        })
        .await
    };
    let rt = Runtime::new().unwrap();
    rt.block_on(fut);
}

fn var_size_branch() {
    let fut = async {
        let path = alice_open_data::test_file().unwrap();
        let f = RootFile::new(path.as_path())
            .await
            .expect("Failed to open file");
        let t = f.items()[0].as_tree().await.unwrap();

        let track_counter: Vec<_> = t
            .branch_by_name("Tracks")
            .unwrap()
            .as_fixed_size_iterator(|i| be_u32(i))
            .collect()
            .await;
        let iter = t
            .branch_by_name("Tracks.fX")
            .unwrap()
            .as_var_size_iterator(|i| be_f32(i), track_counter);
        iter.for_each(|el| async {
            black_box(el);
        })
        .await
    };
    let rt = Runtime::new().unwrap();
    rt.block_on(fut);
}

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fixed_size_branch", |b| b.iter(|| fixed_size_branch));
    c.bench_function("var_size_branch", |b| b.iter(|| var_size_branch));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
