use std::path::PathBuf;

use core::*;

#[test]
fn root_file_methods() {
    let paths = [
        "./src/test_data/simple.root",
        "./src/test_data/HZZ.root",
        "./src/test_data/HZZ-lz4.root",
        "./src/test_data/HZZ-lzma.root",
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
        "./src/test_data/sample-5.30.00-lzma.root",
        "./src/test_data/sample-5.30.00-uncompressed.root",
        "./src/test_data/sample-5.30.00-zlib.root",
        "./src/test_data/sample-6.08.04-lzma.root",
        "./src/test_data/sample-6.08.04-uncompressed.root",
        "./src/test_data/sample-6.08.04-zlib.root",
        "./src/test_data/sample-6.10.05-lz4.root",
        "./src/test_data/sample-6.10.05-lzma.root",
        "./src/test_data/sample-6.10.05-uncompressed.root",
        "./src/test_data/sample-6.10.05-zlib.root",
        // Contains TStreamerSTLstring
        "./src/test_data/small-evnt-tree-fullsplit.root",
        "./src/test_data/small-flat-tree.root",
        "./src/test_data/Zmumu.root",
        "./src/test_data/Zmumu-lz4.root",
        "./src/test_data/Zmumu-lzma.root",
        "./src/test_data/Zmumu-uncompressed.root",
        "./src/test_data/Zmumu-zlib.root",
        "./src/test_data/foriter.root",
        "./src/test_data/foriter2.root",
        "./src/test_data/mc10events.root",
        // Contains TStreamerSTLstring
        "./src/test_data/nesteddirs.root",
    ]
    .into_iter()
    .map(|p| PathBuf::from(p));
    for p in paths {
        println!("{:?}", p);
        let f = RootFile::new_from_file(&p).expect("Failed to open file");
        let mut s = String::new();
        f.streamer_info_as_yaml(&mut s).unwrap();
        f.streamer_info_as_rust(&mut s).unwrap();
        for item in f.items() {
            item.name();
            item.verbose_info();
        }
    }
}

#[test]
#[ignore]
fn root_file_methods_esd() {
    let paths = ["./src/test_data/AliESDs.root"]
        .into_iter()
        .map(|p| PathBuf::from(p));
    for p in paths {
        println!("{:?}", p);
        let f = RootFile::new_from_file(&p).expect("Failed to open file");
        let mut s = String::new();
        f.streamer_info_as_yaml(&mut s).unwrap();
        f.streamer_info_as_rust(&mut s).unwrap();
        for item in f.items() {
            item.name();
            item.verbose_info();
        }
    }
}
