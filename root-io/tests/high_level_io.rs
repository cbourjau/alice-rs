#![cfg(all(test, not(target_arch = "wasm32")))]

use root_io::*;
use std::path::PathBuf;

const TEST_FILES: &[&str] = &[
    "./src/test_data/simple.root",
    "./src/test_data/HZZ.root",
    "./src/test_data/HZZ-lz4.root",
    // "./src/test_data/HZZ-lzma.root",
    "./src/test_data/sample-5.23.02-uncompressed.root",
    "./src/test_data/sample-5.23.02-zlib.root",
    "./src/test_data/sample-5.24.00-zlib.root",
    "./src/test_data/sample-5.23.02-uncompressed.root",
    "./src/test_data/sample-5.23.02-zlib.root",
    "./src/test_data/sample-5.24.00-uncompressed.root",
    "./src/test_data/sample-5.24.00-zlib.root",
    "./src/test_data/sample-5.25.02-uncompressed.root",
    "./src/test_data/sample-5.25.02-zlib.root",
    "./src/test_data/sample-5.26.00-uncompressed.root",
    "./src/test_data/sample-5.26.00-zlib.root",
    "./src/test_data/sample-5.27.02-uncompressed.root",
    "./src/test_data/sample-5.27.02-zlib.root",
    "./src/test_data/sample-5.28.00-uncompressed.root",
    "./src/test_data/sample-5.28.00-zlib.root",
    "./src/test_data/sample-5.29.02-uncompressed.root",
    "./src/test_data/sample-5.29.02-zlib.root",
    // "./src/test_data/sample-5.30.00-lzma.root",
    "./src/test_data/sample-5.30.00-uncompressed.root",
    "./src/test_data/sample-5.30.00-zlib.root",
    // "./src/test_data/sample-6.08.04-lzma.root",
    "./src/test_data/sample-6.08.04-uncompressed.root",
    "./src/test_data/sample-6.08.04-zlib.root",
    "./src/test_data/sample-6.10.05-lz4.root",
    // "./src/test_data/sample-6.10.05-lzma.root",
    "./src/test_data/sample-6.10.05-uncompressed.root",
    "./src/test_data/sample-6.10.05-zlib.root",
    "./src/test_data/small-flat-tree.root",
    "./src/test_data/Zmumu.root",
    "./src/test_data/Zmumu-lz4.root",
    // "./src/test_data/Zmumu-lzma.root",
    "./src/test_data/Zmumu-uncompressed.root",
    "./src/test_data/Zmumu-zlib.root",
    "./src/test_data/foriter.root",
    "./src/test_data/foriter2.root",
    "./src/test_data/mc10events.root",
    // Contains TStreamerSTLstring
    "./src/test_data/nesteddirs.root",
    "./src/test_data/small-evnt-tree-fullsplit.root",
];

fn local_paths() -> Vec<PathBuf> {
    TEST_FILES.iter().map(PathBuf::from).collect()
}

#[cfg(not(target_arch = "wasm32"))]
mod local {
    use super::*;

    #[tokio::test]
    async fn root_file_methods() {
        let paths = local_paths();
        for p in paths {
            println!("{:?}", p);
            let f = RootFile::new(p.as_path())
                .await
                .expect("Failed to open file");
            let mut s = String::new();
            f.streamer_info_as_yaml(&mut s).await.unwrap();
            f.streamer_info_as_rust(&mut s).await.unwrap();
            for item in f.items() {
                item.name();
                if item.verbose_info().contains("TTree") {
                    item.as_tree().await.unwrap();
                }
            }
        }
    }

    #[tokio::test]
    async fn root_file_methods_esd() {
        use alice_open_data;
        let paths = [alice_open_data::test_file().unwrap()];
        for p in &paths {
            println!("{:?}", p);
            let f = RootFile::new(p.as_path())
                .await
                .expect("Failed to open file");
            let mut s = String::new();
            f.streamer_info_as_yaml(&mut s).await.unwrap();
            f.streamer_info_as_rust(&mut s).await.unwrap();
            for item in f.items() {
                item.name();
                item.verbose_info();
            }
        }
    }
}
