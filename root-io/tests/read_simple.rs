#![cfg(test)]

use failure::Error;
use nom::number::complete::*;
use futures::{Stream, StreamExt};

use root_io::{
    core::parsers::string,
    tree_reader::Tree,
    RootFile,
};

/// A model for the (or a subset) of the data.
/// This is the object which contains the data of one "event"
#[derive(Debug)]
struct Model {
    one: i32,
    two: f32,
    three: String,
}

impl Model {
    fn stream_from_tree(t: Tree) -> Result<impl Stream<Item=Self>, Error> {
        Ok(t.branch_by_name("one")?.as_fixed_size_iterator(|i| be_i32(i))
            .zip(t.branch_by_name("two")?.as_fixed_size_iterator(|i| be_f32(i)))
            .zip(t.branch_by_name("three")?.as_fixed_size_iterator(string))
            .map(|((one, two), three)| Self {one, two, three}))
    }
}

async fn read_simple(f: RootFile) {
    let t = f.items()[0].as_tree().await.unwrap();
    let s = Model::stream_from_tree(t).unwrap();
    s.for_each(|m| async move {
        println!("{:?}", m);
    }).await
}

#[cfg(not(target_arch="wasm32"))]
mod x64 {
    use super::*;
    use std::path::PathBuf;
    use tokio;

    #[tokio::test]
    async fn read_simple_local() {
        let path = PathBuf::from("./src/test_data/simple.root");
        let f = RootFile::new_from_file(&path).await.expect("Failed to open file");
        read_simple(f).await;
    }
}

#[cfg(target_arch="wasm32")]
mod wasm {
    use super::*;

    use wasm_bindgen_test::*;
    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn read_simple_remote() {
        let url = "http://cirrocumuli.com/test_data/simple.root";
        let f = RootFile::new_from_url(url).expect("Failed to open remote file");
        read_simple(f);
    }
}

