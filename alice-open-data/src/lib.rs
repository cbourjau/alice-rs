extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate reqwest;

use std::env;
use std::io::{self, Read};
use std::path::PathBuf;
use std::fs::{File, DirBuilder};
use std::io::copy;


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
pub struct FileDetails {
    file_path: String,
    pub file_size: i64,
    file_checksum: String,
    file_timestamp: String,
    original_filename: String,
}

impl FileDetails {
    /// The url pointing to the current file
    pub fn url(&self) -> String {
        let url = "http://opendata.cern.ch".to_string();
        url + &self.file_path
    }

    /// Download the current file to the local collection
    pub fn download(&self) -> Result<(), Error> {
        let mut dest = data_dir()?;
        let mut sub_dir = self.original_filename.to_owned();
        // Remove the leading "\" from the original path
        sub_dir.remove(0);
        dest.push(sub_dir);
        // Do not re-download if the file already exists
        if dest.exists() {
            return Ok(());
        }
        // Make sure the dir exists
        if let Some(dir) = dest.parent() {
            DirBuilder::new().recursive(true).create(dir)?;
        }
        let mut f = File::create(dest)?;
        let mut resp = reqwest::get(&self.url())?;
        copy(&mut resp, &mut f)?;
        Ok(())
    }
}

/// Base path to the local ALICE open data directory
pub fn data_dir() -> Result<PathBuf, Error> {
    let mut dir = env::home_dir().ok_or(Error::NoHomeDir)?;
    dir.push("lhc_open_data");
    Ok(dir)
}

/// Hardcoded path to a specific file. Useful for testing.
/// That file should be the the first to be downloaded automatically.
pub fn test_file() -> Result<PathBuf, Error> {
    let mut dir = data_dir()?;
    dir.push("alice/data/2010/LHC10h/000139038/ESDs/pass2/10000139038001.10/AliESDs.root");
    Ok(dir)
}

/// Get the details for a specific dataset
pub fn file_details_of_run(run: u32) -> Result<Vec<FileDetails>, Error> {
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
    let wrapper: SillyWrapper = serde_json::from_str(&content)?;
    Ok(wrapper.dataset.file_details)
}

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    HttpError(reqwest::Error),
    ParseJson(serde_json::Error),
    NoHomeDir,
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Error {
        Error::HttpError(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error::ParseJson(err)
    }
}

#[cfg(test)]
mod tests {
    use super::{SillyWrapper, FileDetails};
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
        let _ds: FileDetails = serde_json::from_str(data).unwrap();
    }    

}
