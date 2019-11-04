use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::PathBuf;

use failure::Error;
use reqwest::{
    header::{RANGE, USER_AGENT},
    Client, Url,
};

#[cfg(target_arch = "wasm32")]
use std::sync::mpsc::{sync_channel, SyncSender};

use async_trait::async_trait;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::spawn_local;

#[async_trait(?Send)]
pub trait DataSource: std::fmt::Debug {
    async fn fetch(&self, start: u64, len: u64) -> Result<Vec<u8>, Error>;
}

#[derive(Debug, Clone)]
pub struct LocalDataSource(PathBuf);

impl LocalDataSource {
    pub fn new(path: PathBuf) -> Self {
        Self(path)
    }
}

#[async_trait(?Send)]
impl DataSource for LocalDataSource {
    async fn fetch(&self, start: u64, len: u64) -> Result<Vec<u8>, Error> {
        let mut f = File::open(&self.0)?;
        f.seek(SeekFrom::Start(start))?;
        let mut buf = vec![0; len as usize];
        f.read_exact(&mut buf)?;
        Ok(buf)
    }
}

#[derive(Debug, Clone)]
pub struct RemoteDataSource {
    client: Client,
    url: Url,
}

impl RemoteDataSource {
    pub fn new(url: &str) -> Result<Self, Error> {
        Ok(Self {
            client: Client::new(),
            url: url.parse()?,
        })
    }
}

#[async_trait(?Send)]
impl DataSource for RemoteDataSource {
    async fn fetch(&self, start: u64, len: u64) -> Result<Vec<u8>, Error> {
        let rsp = Client::new()
            .get(self.url.clone())
            .header(USER_AGENT, "alice-rs")
            .header(RANGE, format!("bytes={}-{}", start, start + len - 1))
            .send()
            .await?;
        let bytes = rsp.bytes().await?;
        Ok(bytes.as_ref().to_vec())
    }
}

#[cfg(target_arch = "wasm32")]
fn wait_it_out<F>(future: F) -> Result<Vec<u8>, Error>
where
    F: 'static + Future<Output = Result<Response, reqwest::Error>>,
{
    let (tx, rx) = sync_channel(1);

    // Create a future with output (); instead we send back the result via a channel
    // Would have probably been cleaner to use `.map`...
    async fn await_and_send_back<F>(fut: F, tx: SyncSender<Result<Vec<u8>, Error>>) -> ()
    where
        F: 'static + Future<Output = Result<Response, reqwest::Error>>,
    {
        let res = {
            match fut.await {
                Ok(rsp) => match rsp.bytes().await {
                    Ok(bytes) => Ok(bytes.as_ref().to_vec()),
                    Err(e) => Err(e.into()),
                },
                Err(e) => Err(e.into()),
            }
        };
        tx.send(res).expect("Failed to send back bytes");
    }
    spawn_local(await_and_send_back(future, tx));
    rx.recv()?
}
