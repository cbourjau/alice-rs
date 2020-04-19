#![cfg(test)]
//! Integration tests of this library. It seems like
//! wasm-bindgen-tests does not properly pick up async integration
//! tests in a /tests dir. Hence this hack of having this directory
//! inside the /src folder.

use reqwest::Client;

use crate::*;

async fn download_partial() {
    use reqwest::header::RANGE;
    let client = Client::builder().build().unwrap();
    let url = get_file_list(139_038)
	.await
	.unwrap()[0]
	.clone();
    let (start, len) = (13993603, 68936);
    let rsp = dbg!(client
        .get(url)
        .header("User-Agent", "alice-rs")
        .header(RANGE, &format!("bytes={}-{}", start, start + len - 1)))
        .send()
        .await
        .unwrap();
    dbg!(&rsp);
    let partial = rsp.error_for_status().unwrap().bytes().await.unwrap();
    assert_eq!(partial.len(), len);
    #[cfg(not(target_arch = "wasm32"))]
    {
        let from_disc = std::fs::read(test_file().unwrap()).unwrap();
        assert!(partial
                .iter()
                .skip(start)
                .zip(from_disc.iter())
                .all(|(el1, el2)| el1 == el2));
    }
}

async fn test_get_file_lists() {
    let runs = [139_038, 139_173, 139_437, 139_438, 139_465];
    for run in runs.iter() {
        println!("Testing run {}", run);
        super::get_file_list(*run).await.unwrap();
    }
}

async fn test_download_file() {
    let uris = &super::get_file_list(139038).await.unwrap();
    Client::new().get(uris[0].clone()).send().await.unwrap();
}


#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests_x86 {
    use tokio;
    use std::{env, fs};

    #[tokio::test]
    async fn download_partial() {
	super::download_partial().await;
    }

    #[tokio::test]
    async fn test_get_file_lists() {
	super::test_get_file_lists().await;
    }

    #[tokio::test]
    async fn test_download_file() {
	super::test_download_file().await;
    }

    #[tokio::test]
    async fn test_download_file_high_level() {
        let uri = crate::get_file_list(139038).await.unwrap()[0].clone();
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
            super::download(base_dir.clone(), uri.clone())
                .await
                .unwrap(),
            14283265
        );
        // Don't download twice
        assert_eq!(
            super::download(base_dir.clone(), uri.clone())
                .await
                .unwrap(),
            0
        );
    }    
}


#[cfg(target_arch = "wasm32")]
mod test_wasm {
    wasm_bindgen_test_configure!(run_in_browser);

    use wasm_bindgen_test::{wasm_bindgen_test_configure, wasm_bindgen_test};

    #[wasm_bindgen_test]
    async fn download_partial() {
	super::download_partial().await;
    }

    #[wasm_bindgen_test]
    async fn test_get_file_lists() {
	super::test_get_file_lists().await;
    }

    #[wasm_bindgen_test]
    async fn test_download_file() {
	super::test_download_file().await;
    }    

}
