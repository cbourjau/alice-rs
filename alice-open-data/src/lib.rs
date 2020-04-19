#[cfg(not(target_arch = "wasm32"))]
use std::{
    fs::{DirBuilder, File},
    io::Write,
    path::PathBuf,
};

use failure::{format_err, Error};
use reqwest::{Client, Url};

#[cfg(test)]
mod tests;

fn root_url() -> Url {
    if cfg!(target_arch = "wasm32") {
	// Proxy with CORS properly set
	"http://127.0.0.1:3030/opendata/".parse().unwrap()
    } else {
	"http://opendata-dev.web.cern.ch".parse().unwrap()
    }
}

/// Download the given file to the local collection
#[cfg(not(target_arch = "wasm32"))]
pub async fn download(base_dir: PathBuf, url: Url) -> Result<usize, Error> {
    let mut dest = base_dir;
    let mut sub_dir = url.path().to_owned();
    // Remove the leading "\" from the original path
    sub_dir.remove(0);
    dest.push(sub_dir);
    // Do not re-download if the file already exists
    if dest.exists() {
        return Ok(0);
    }
    // Make sure the dir exists
    if let Some(dir) = dest.parent() {
        DirBuilder::new().recursive(true).create(dir)?;
    }
    let resp = Client::new().get(url).send().await?;
    let bytes: Vec<_> = resp
        .error_for_status()?
        .bytes()
        .await?
        .into_iter()
        .collect();
    let mut f = File::create(dest)?;
    Ok(f.write(&bytes)?)
}

/// Base path to the local ALICE open data directory
#[cfg(not(target_arch = "wasm32"))]
pub fn data_dir() -> Result<PathBuf, Error> {
    let mut dir = dirs::home_dir().ok_or_else(|| format_err!("No home directory"))?;
    dir.push("lhc_open_data");
    Ok(dir)
}

/// Hardcoded path to a specific file. Useful for testing.
/// That file should be the the first to be downloaded automatically.
#[cfg(not(target_arch = "wasm32"))]
pub fn test_file() -> Result<PathBuf, Error> {
    let mut dir = data_dir()?;
    dir.push("eos/opendata/alice/2010/LHC10h/000139038/ESD/0001/AliESDs.root");
    Ok(dir)
}

/// Path to all files of `LHC10h`
#[cfg(not(target_arch = "wasm32"))]
pub fn all_files_10h() -> Result<Vec<PathBuf>, Error> {
    let mut search_dir = data_dir()?;
    search_dir.push("**/AliESDs.root");
    let files: Vec<_> = glob::glob(search_dir.to_str().unwrap())
        .expect("Can't resolve glob")
        .map(|path| path.unwrap())
        .collect();
    Ok(files)
}

pub async fn get_file_list(run: u32) -> Result<Vec<Url>, Error> {
    // Due to CORS we have to change the urls based on the target for now
    let uri =
        root_url()
            .join(
                match run {
                    139_038 => "record/1102/files/ALICE_LHC10h_PbPb_ESD_139038_file_index.txt",
                    139_173 => "record/1103/files/ALICE_LHC10h_PbPb_ESD_139173_file_index.txt",
                    139_437 => "record/1104/files/ALICE_LHC10h_PbPb_ESD_139437_file_index.txt",
                    139_438 => "record/1105/files/ALICE_LHC10h_PbPb_ESD_139438_file_index.txt",
                    139_465 => "record/1106/files/ALICE_LHC10h_PbPb_ESD_139465_file_index.txt",
                    _ => return Err(format_err!("Invalid run number")),
                }
            )?;

    let req = Client::new().get(uri);
    let resp = req.send().await?;
    if resp.status().is_success() {
        let content = resp.text().await?;
        content
            .lines()
            .map(|l| root_url().join(&l[26..]).map_err(Into::into))
            .collect()
    } else {
        Err(format_err!("Could not download list of files"))
    }
}
