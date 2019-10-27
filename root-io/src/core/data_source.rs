use std::path::PathBuf;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

use failure::Error;

use alice_open_data::client;
use reqwest::{Client, Url, header::{RANGE, USER_AGENT}};

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
            client: client()?,
            url: url.parse()?
        })
    }
}

impl DataSource for RemoteDataSource {
    fn fetch(&self, start: u64, len: u64) -> Result<Vec<u8>, Error> {
        let mut rsp = client()?
            .get(self.url.clone())
            .header(USER_AGENT, "alice-rs")
            .header(RANGE, format!("bytes={}-{}", start, start + len - 1))
            .send()?;
        let mut buf = vec![0; len as usize];
        rsp.read_exact(&mut buf)?;
        Ok(buf)
    }
}
