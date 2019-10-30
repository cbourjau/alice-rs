#[cfg(not(target_arch = "wasm32"))]
use std::{
    fs::{DirBuilder, File},
    io::Write,
    path::PathBuf,
};

use failure::{Error, format_err};
use reqwest::{Client, Url};

fn root_url() -> Url {
    if !cfg!(target_arch = "wasm32") {
        "http://opendata.cern.ch/"
    } else {
        "http://cirrocumuli.com/"
    }.parse().unwrap()
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
    let bytes: Vec<_> = resp.bytes().await?.into_iter().collect();
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
    // Do to CORS we have to get change the urls based on the target for now
    
    let uri = "http://opendata.cern.ch/".parse::<Url>()?.join(
        if !cfg!(target_arch = "wasm32") {
            match run {
                139_038 => "record/1102/files/ALICE_LHC10h_PbPb_ESD_139038_file_index.txt",
                139_173 => "record/1103/files/ALICE_LHC10h_PbPb_ESD_139173_file_index.txt",
                139_437 => "record/1104/files/ALICE_LHC10h_PbPb_ESD_139437_file_index.txt",
                139_438 => "record/1105/files/ALICE_LHC10h_PbPb_ESD_139438_file_index.txt",
                139_465 => "record/1106/files/ALICE_LHC10h_PbPb_ESD_139465_file_index.txt",
                _ => return Err(format_err!("Invalid run number")),
            }
        } else {
            "http://cirrocumuli.com/ALICE_LHC10h_PbPb_ESD_139038_file_index.txt"
        }
    )?;

    let req = Client::new().get(uri);
    let resp = req.send().await?;
    if resp.status().is_success() {
        let content = resp.text().await?;
        content
            .lines()
            .map(|l| root_url()
                 .join(&l[26..])
                 .map_err(Into::into)
            )
            .collect()
    } else {
        Err(format_err!("Could not download list of files"))
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests_x84 {
    use std::{env, fs};
    use super::*;
    use tokio;

    #[tokio::test]
    async fn download_partial() {
        use reqwest::header::RANGE;
        let client = Client::builder()
            .build()
            .unwrap();
        let url = root_url().join("/eos/opendata/alice/2010/LHC10h/000139038/ESD/0001/AliESDs.root").unwrap();
        let (start, len) = (13993603, 68936);
        let rsp = client
            .get(url)
            .header("User-Agent", "alice-rs")
            .header(RANGE, &format!("bytes={}-{}", start, start + len -1))
            .send().await.unwrap();
        dbg!(&rsp);

        let partial = rsp.bytes().await.unwrap();
        assert_eq!(partial.len(), len);
        #[cfg(not(target_arch = "wasm32"))]
        {
            let from_disc = std::fs::read(test_file().unwrap()).unwrap();
            assert!(
                partial.iter()
                    .skip(start)
                    .zip(from_disc.iter())
                    .all(|(el1, el2)| el1 == el2)
            );
        }
    }

    #[tokio::test]
    async fn test_get_file_lists() {
        let runs = [139_038, 139_173, 139_437, 139_438, 139_465];
        for run in runs.iter() {
            println!("Testing run {}", run);
            super::get_file_list(*run).await.unwrap();
        }
    }

    #[tokio::test]
    async fn test_download_file() {
        let uris = &super::get_file_list(139038).await.unwrap();
        let resp = Client::new().get(uris[0].clone()).send().await.unwrap();
        println!("{:?}", resp);
        println!("{:?}", uris[0].path());
    }
    #[tokio::test]
    async fn test_download_file_high_level() {
        let uri = super::get_file_list(139038).await.unwrap()[0].clone();
        {
            // Remobe old stuff:
            let mut dir = env::temp_dir();
            dir.push("eos");
            if dir.exists() {
                fs::remove_dir_all(dir).unwrap();
            }
        }
        let base_dir = env::temp_dir();
        // Download if file does not exist
        assert_eq!(
            super::download(base_dir.clone(), uri.clone()).await.unwrap(),
            14283265
        );
        // Don't download twice
        assert_eq!(super::download(base_dir.clone(), uri.clone()).await.unwrap(), 0);
    }
}
