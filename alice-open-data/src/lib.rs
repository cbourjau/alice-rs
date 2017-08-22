extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate reqwest;

use std::env;
use std::io::{Read};
use std::path::PathBuf;

/// The json data of the datasets is wrapped in an object with key "Dataset"
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SillyWrapper {
    dataset: Dataset
}

/// Summary information of the given dataset/run
#[derive(Serialize, Deserialize, Debug)]
pub struct Dataset {
    name: String,
    description: String,
    path: String,
    files: i32,
    pub file_details: Vec<FileDetails>,
}

/// Details about one specific file in a given run
#[derive(Serialize, Deserialize, Debug)]
pub struct FileDetails {
    pub file_path: String,
    pub file_size: i64,
    file_checksum: String,
    file_timestamp: String,
    pub original_filename: String,
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
pub fn get_dataset_info(run: u32) -> Result<Dataset, reqwest::Error> {
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
