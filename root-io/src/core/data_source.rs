use std::path::PathBuf;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::future::Future;

use failure::Error;
use reqwest::{Client, Response, Url, header::{RANGE, USER_AGENT}};

#[cfg(not(target_arch = "wasm32"))]
use tokio::runtime::Runtime;

pub trait DataSource: std::fmt::Debug {
    fn fetch(&self, start: u64, len: u64) -> Result<Vec<u8>, Error>;
}

#[derive(Debug, Clone)]
pub struct LocalDataSource(PathBuf);

impl LocalDataSource {
    pub fn new(path: PathBuf) -> Self {
        Self(path)
    }
}

impl DataSource for LocalDataSource {
    fn fetch(&self, start: u64, len: u64) -> Result<Vec<u8>, Error> {
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
    url: Url
}

impl RemoteDataSource {
    pub fn new(url: &str) -> Result<Self, Error> {
        Ok(Self {
            client: Client::new(),
            url: url.parse()?
        })
    }
}

impl DataSource for RemoteDataSource {
    fn fetch(&self, start: u64, len: u64) -> Result<Vec<u8>, Error> {
        let fut = Client::new()
            .get(self.url.clone())
            .header(USER_AGENT, "alice-rs")
            .header(RANGE, format!("bytes={}-{}", start, start + len - 1))
            .send();
        wait_it_out(fut)
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn wait_it_out(future: impl Future<Output=Result<Response, reqwest::Error>>) -> Result<Vec<u8>, Error> {
    let rt = Runtime::new()?;
    let rsp = rt.block_on(future)?;
    let bytes = rt.block_on(rsp.bytes())?;
    Ok(bytes.as_ref().to_vec())
}

#[cfg(target_arch = "wasm32")]
fn wait_it_out(future: impl Future<Output=Result<Response, reqwest::Error>>) -> Result<Vec<u8>, Error> {
    unimplemented!()
}
