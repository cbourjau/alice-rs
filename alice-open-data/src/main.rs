extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate reqwest;
extern crate indicatif;
#[macro_use]
extern crate clap;

use std::env;
use std::fs::{File, DirBuilder};
use std::io::copy;
use std::io::{Read};
use std::path::PathBuf;

use indicatif::ProgressBar;
use clap::{Arg, App};

/// The json data of the datasets is wrapped in an object with key "Dataset"
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct SillyWrapper {
    dataset: Dataset
}

/// Summary information of the given dataset/run
#[derive(Serialize, Deserialize, Debug)]
struct Dataset {
    name: String,
    description: String,
    path: String,
    files: i32,
    file_details: Vec<FileDetails>,
}

/// Details about one specific file in a given run
#[derive(Serialize, Deserialize, Debug)]
struct FileDetails {
    file_path: String,
    file_size: i64,
    file_checksum: String,
    file_timestamp: String,
    original_filename: String,
}


/// Base path to the local ALICE open data directory
pub fn data_dir() -> PathBuf {
    let mut dir = env::home_dir().expect("Cannot find user's home directory");
    dir.push("lhc_open_data");
    dir
}

/// Hardcoded path to a specific file. Useful for testing.
/// That file should be the the first to be downloaded automatically.
pub fn test_file() -> PathBuf {
    let mut dir = data_dir();
    dir.push("alice/data/2010/LHC10h/000139038/ESDs/pass2/10000139038001.10/AliESDs.root");
    dir
}

/// Get the details for a specific dataset
fn get_dataset_info(run: u32) -> Result<Dataset, reqwest::Error> {
    // Runs are blocks of data taking
    // One URL per run
    let urls = [
        "http://opendata.cern.ch/record/1102/files/LHC10h_PbPb_ESD_139038.json",
        "http://opendata.cern.ch/record/1103/files/LHC10h_PbPb_ESD_139173.json",
        "http://opendata.cern.ch/record/1104/files/LHC10h_PbPb_ESD_139437.json",
        "http://opendata.cern.ch/record/1105/files/LHC10h_PbPb_ESD_139438.json",
        "http://opendata.cern.ch/record/1106/files/LHC10h_PbPb_ESD_139465.json",
    ];
    let url = urls.iter()
        .find(|url| url.contains(&run.to_string()))
        .expect("No data for given run number");
    let mut content = String::new();
    reqwest::get(*url)?
    .read_to_string(&mut content)
        .expect("Could not read to string");
    let wrapper: SillyWrapper = serde_json::from_str(&content).unwrap();
    Ok(wrapper.dataset)
}

fn main() {
    let matches = App::new("alice-open-data")
        .version("0.1")
        .about("Download specified amount of ALICE open Pb--Pb data to ~/lhc_open_data \
                Visit `http://opendata.cern.ch/search?cc=ALICE-Reconstructed-Data` for \
                more information."
               )
        .arg(Arg::with_name("amount")
             .help("Download amount specified in GB. Does not re-download existing files. \
                    1GB is enough for simple debugging. Use 50+GB to make meaningful \
                    plots.")
             .required(true)
             .index(1))
        .get_matches();
    let max_vol = value_t!(matches.value_of("amount"), i64).unwrap_or_else(|e| e.exit());
    // convert from GB to B
    let max_vol = max_vol * (1_000_000_000);
    let runs = [
        139_038,
        139_173,
        139_437,
        139_438,
        139_465,
    ];
    let files: Vec<FileDetails> = runs.iter()
        .flat_map(|r| get_dataset_info(*r).unwrap().file_details)
        .collect();
    let total_size = files.iter().fold(0, |acc, v| acc + v.file_size) as f64 / 1e9;
    println!("Total available data: {} files with total of {} GB", files.len(), total_size);

    let max_files: Vec<FileDetails> =
        files
        .into_iter()
        .scan(0, |acc, v| {
            if max_vol > *acc {
                *acc += v.file_size;
                Some(v)
            } else {
                None
            }})
        .collect();

    let pbar = ProgressBar::new(max_files.len() as u64);
    let max_files = pbar.wrap_iter(max_files.iter());

    for f in max_files {
        let url = "http://opendata.cern.ch".to_string() + &f.file_path;
        let mut dest = data_dir();
        let mut sub_dir = f.original_filename.to_owned();
        // Remove the leading "\" from the original path
        sub_dir.remove(0);
        dest.push(sub_dir);
        // Do not re-download if the file already exists
        if dest.exists() {
            continue;
        }
        // Make sure the dir exists
        if let Some(dir) = dest.parent() {
            DirBuilder::new().recursive(true).create(dir).unwrap();
        }
        let mut f = File::create(dest).unwrap();
        let mut resp = reqwest::get(&url).expect("Could not read file");
        copy(&mut resp, &mut f).unwrap();
    }
}


#[cfg(test)]
mod tests {
    use super::{SillyWrapper, File};
    use serde_json;

    #[test]
    fn test_all() {
        let data = r#"{
    "Dataset": {
    "name": "LHC10h_PbPb_ESD_139173",
    "description": "PbPb ESD data sample at 3500 GeV",
    "path": "/eos/opendata/alice/2010/LHC10h/000139173/ESD",
    "files": 2639,
    "file_details": [
    {
      "file_path": "/eos/opendata/alice/2010/LHC10h/000139173/ESD/0012/AliESDs.root",
      "file_size": 391427669,
      "file_checksum": "afb038c2279baa31fcca8b7c04e96109",
      "file_timestamp": "2013-01-17 15:01:44",
      "original_filename": "/alice/data/2010/LHC10h/000139173/ESDs/pass2/10000139173001.10/AliESDs.root"
     }]
     }}"#;
        serde_json::from_str::<SillyWrapper>(data).unwrap();
    }

    #[test]
    fn test_details() {
        let data = r#"{
      "file_path": "/eos/opendata/alice/2010/LHC10h/000139173/ESD/0012/AliESDs.root",
      "file_size": 391427669,
      "file_checksum": "afb038c2279baa31fcca8b7c04e96109",
      "file_timestamp": "2013-01-17 15:01:44",
      "original_filename": "/alice/data/2010/LHC10h/000139173/ESDs/pass2/10000139173001.10/AliESDs.root"
      }"#;
        let ds: File = serde_json::from_str(data).unwrap();
    }    

}
