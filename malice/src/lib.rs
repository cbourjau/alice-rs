extern crate nom;
extern crate failure;
extern crate root_io;
#[cfg(feature = "cpp")]
extern crate alice_sys;
#[macro_use]
extern crate bitflags;

pub mod event;
pub mod dataset_rust;
#[cfg(feature = "cpp")]
mod esd;
#[cfg(feature = "cpp")]
pub mod dataset_cpp;
pub mod track;
pub mod merge;
pub mod primary_vertex;

#[cfg(test)]
mod tests {
    extern crate alice_open_data;
    use root_io::RootFile;

    #[test]
    #[cfg(feature = "cpp")]
    fn rust_cpp_identical_many_files() {
        use super::dataset_rust::DatasetIntoIter as DsIntoIter_rust;
        use super::dataset_cpp::DatasetIntoIter as DsIntoIter_cpp;

        let n_files = 500;
        let rust_iter = alice_open_data::all_files_10h().unwrap()
            .into_iter()
            .take(n_files)
            .map(|path| RootFile::new_from_file(&path).expect("Failed to open file"))
            .map(|rf| rf.items()[0].as_tree().unwrap())
            .flat_map(|tree| {
            match DsIntoIter_rust::new(&tree) {
                Ok(s) => s,
                Err(err) => panic!("An error occured! Message: {}", err)
            }});
        let cpp_iter = alice_open_data::all_files_10h().unwrap()
            .into_iter()
            .take(n_files)
            .flat_map(|path| {
                match DsIntoIter_cpp::new(&path) {
                    Ok(s) => [path.to_owned()].to_vec().into_iter().cycle().zip(s),
                    Err(err) => panic!("An error occured! Message: {}", err)
                }});
        for (i, (rust_ev, (path, cpp_ev))) in rust_iter.zip(cpp_iter).enumerate() {
            // println!("{:?}", path);
            assert_eq!(rust_ev, cpp_ev, "Event {} differs in file {:?}", i, path);
        }
    }

    #[test]
    #[cfg(feature = "cpp")]
    fn rust_cpp_identical_funky_file_1() {
        use super::dataset_rust::DatasetIntoIter as DsIntoIter_rust;
        use super::dataset_cpp::DatasetIntoIter as DsIntoIter_cpp;
        
        let file = alice_open_data::all_files_10h().unwrap()
            .into_iter()
            .find(|p| p.to_str().unwrap()
                  // This file contains a bunch of "empty" baskets; i.e. baskets which claim to have events but are just zeros...
                  .contains("10000139038001.770/AliESDs.root"))
            .expect("Funky file not found");
        let rust_iter = {
            let tree = RootFile::new_from_file(&file).expect("Failed to open file").items()[0].as_tree().unwrap();
            match DsIntoIter_rust::new(&tree) {
                Ok(s) => s,
                Err(err) => panic!("An error occured! Message: {}", err)
            }};
        let cpp_iter = match DsIntoIter_cpp::new(&file) {
                    Ok(s) => s,
                    Err(err) => panic!("An error occured! Message: {}", err)
        };
        for (rust_ev, cpp_ev) in rust_iter.zip(cpp_iter) {
            assert_eq!(rust_ev, cpp_ev);
        }
    }
    #[test]
    #[cfg(feature = "cpp")]
    fn rust_cpp_identical_funky_file_2() {
        use super::dataset_rust::DatasetIntoIter as DsIntoIter_rust;
        use super::dataset_cpp::DatasetIntoIter as DsIntoIter_cpp;
        let funkies = [
            // This files has baskets which, after parsing, have 0 bytes :P
            "10000139038002.40/AliESDs.root",
            // events with 0 tracks at end of basket
            "10000139038001.310/AliESDs.root",
        ];
        for funky in &funkies {
            let file = alice_open_data::all_files_10h().unwrap()
                .into_iter()
                .find(|p| p.to_str().unwrap().contains(funky))
                .expect("Funky file not found");
            let mut rust_iter = {
                let tree = RootFile::new_from_file(&file).expect("Failed to open file").items()[0].as_tree().unwrap();
                match DsIntoIter_rust::new(&tree) {
                    Ok(s) => s,
                    Err(err) => panic!("An error occured! Message: {}", err)
                }};
            let mut cpp_iter = match DsIntoIter_cpp::new(&file) {
                Ok(s) => s,
                Err(err) => panic!("An error occured! Message: {}", err)
            };
            assert_eq!(rust_iter.count(), cpp_iter.count());
        }
        for funky in &funkies {
            let file = alice_open_data::all_files_10h().unwrap()
                .into_iter()
                .find(|p| p.to_str().unwrap().contains(funky))
                .expect("Funky file not found");
            let mut rust_iter = {
                let tree = RootFile::new_from_file(&file).expect("Failed to open file").items()[0].as_tree().unwrap();
                match DsIntoIter_rust::new(&tree) {
                    Ok(s) => s,
                    Err(err) => panic!("An error occured! Message: {}", err)
                }};
            let mut cpp_iter = match DsIntoIter_cpp::new(&file) {
                Ok(s) => s,
                Err(err) => panic!("An error occured! Message: {}", err)
            };
            for (_i, (rust_ev, cpp_ev)) in rust_iter.zip(cpp_iter).enumerate() {
                assert_eq!(rust_ev, cpp_ev);
            }
        }
        // let cpp_iter = match DsIntoIter_cpp::new(&file) {
        //     Ok(s) => s,
        //     Err(err) => panic!("An error occured! Message: {}", err)
        // };
        // assert_eq!(rust_iter.count(), cpp_iter.count());
        // for (i, (rust_ev, cpp_ev)) in rust_iter.zip(cpp_iter).enumerate() {
        //     println!("{}", i);
        //     assert_eq!(rust_ev, cpp_ev);
        // }
    }

    #[test]
    fn bench_rust() {
        let n_files = 50;
        use super::dataset_rust::DatasetIntoIter;
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

    #[test]
    #[cfg(feature = "cpp")]
    fn bench_cpp() {
        let n_files = 50;
        use super::dataset_cpp::DatasetIntoIter;
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
    
}

