use futures::{Stream, StreamExt};
use nom::number::complete::*;
use nom::Parser;

use root_io::{
    core::parsers::string, stream_zip, tree_reader::tree::MissingBranch, tree_reader::Tree,
    RootFile,
};
use std::pin::Pin;

/// A model for the (or a subset) of the data.
/// This is the object which contains the data of one "event"
#[derive(Debug)]
#[allow(dead_code)]
struct Model {
    one: i32,
    two: f32,
    three: String,
}

impl Model {
    fn stream_from_tree(t: Tree) -> Result<Pin<Box<dyn Stream<Item = Self>>>, MissingBranch> {
        Ok(stream_zip!(
            t.branch_by_name("one")?
                .as_fixed_size_iterator(|i| be_i32(i)),
            t.branch_by_name("two")?
                .as_fixed_size_iterator(|i| be_f32(i)),
            t.branch_by_name("three")?
                .as_fixed_size_iterator(|i| string.map(|s| s.to_string()).parse(i))
        )
        .map(|(one, two, three)| Self { one, two, three })
        .boxed_local())
    }
}

async fn read_simple(f: RootFile) {
    let t = f.items()[0].as_tree().await.unwrap();
    let s = Model::stream_from_tree(t).unwrap();
    s.for_each(|m| async move {
        println!("{:?}", m);
    })
    .await
}

#[cfg(not(target_arch = "wasm32"))]
mod x64 {
    use super::*;
    use std::path::Path;

    #[tokio::test]
    async fn read_simple_local() {
        let path = Path::new("./src/test_data/simple.root");
        let f = RootFile::new(path).await.expect("Failed to open file");
        read_simple(f).await;
    }
}

#[cfg(all(test, target_arch = "wasm32"))]
mod wasm {
    use reqwest::Url;
    use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};

    use super::*;

    wasm_bindgen_test_configure!(run_in_browser);
    #[wasm_bindgen_test]
    async fn read_simple_remote() {
        let url = Url::parse("http://127.0.0.1:3030/github/cbourjau/alice-rs/master/root-io/src/test_data/simple.root").unwrap();
        let f = RootFile::new(url)
            .await
            .expect("Failed to open remote file");
        read_simple(f).await;
    }
}
