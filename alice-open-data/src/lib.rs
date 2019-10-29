use std::fs::{DirBuilder, File};
use std::io::Write;
use std::path::PathBuf;

#[cfg(target_arch = "wasm32")]
use futures::FutureExt;
use failure::{Error, format_err};
use reqwest::{Client, Response, Url};

/// Download the given file to the local collection
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
    let resp = download_with_https(url).await?;
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
    let uri = "http://opendata.cern.ch/record/".to_owned()
        + match run {
            139_038 => "1102/files/ALICE_LHC10h_PbPb_ESD_139038_file_index.txt",
            139_173 => "1103/files/ALICE_LHC10h_PbPb_ESD_139173_file_index.txt",
            139_437 => "1104/files/ALICE_LHC10h_PbPb_ESD_139437_file_index.txt",
            139_438 => "1105/files/ALICE_LHC10h_PbPb_ESD_139438_file_index.txt",
            139_465 => "1106/files/ALICE_LHC10h_PbPb_ESD_139465_file_index.txt",
            _ => return Err(format_err!("Invalid run number")),
        };
    let req = Client::new()
        .get(uri.as_str());
    let resp = req.send().await?;
    if resp.status().is_success() {
        let content = resp.text().await?;
        Ok(content
            .lines()
            .map(|l| format!("http://opendata.cern.ch/{}", &l[26..]))
            .map(|l| l.parse::<Url>().expect("Invalid file URI"))
            .collect())
    } else {
        Err(format_err!("Could not download list of files"))
    }
}

async fn download_with_https(uri: Url) -> Result<Response, Error> {
    return Ok(Client::new().get(uri).send().await?);
}

// #[cfg(target_arch = "wasm32")]
// fn download_with_https(uri: Url) -> Result<reqwest::Response, Error> {
//     use std::sync::mpsc::channel;
//     let (rx, tx) = channel();
//     let future = Client::new()
//         .get(uri)
//         .send()
//         .map(move |res| rx.send(res.unwrap()).unwrap());
//     wasm_bindgen_futures::spawn_local(future);
//     Ok(tx.recv()?)
// }

#[cfg(test)]
mod tests {
    use std::{env, fs};
    use super::*;
    use tokio;

    #[tokio::test]
    async fn download_partial() {
        use reqwest::header::RANGE;
        let client = Client::builder()
            .build()
            .unwrap();
        let rsp = client
        // .get("https://eospublichttp.cern.ch/eos/opendata/alice/2010/LHC10h/000139038/ESD/0001/AliESDs.root")
            .get("http://opendata.cern.ch//eos/opendata/alice/2010/LHC10h/000139038/ESD/0001/AliESDs.root")
            .header("User-Agent", "alice-rs")
            .header(RANGE, "bytes=0-1023")
            .send().await.unwrap();
        dbg!(&rsp);

        let partial = rsp.bytes().await.unwrap();
        assert_eq!(partial.len(), 1024);
        #[cfg(not(target_arch = "wasm32"))]
        {
            let from_disc = std::fs::read(test_file().unwrap()).unwrap();
            assert!(
                partial.iter()
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
        let resp = super::download_with_https(uris[0].clone()).await.unwrap();
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
