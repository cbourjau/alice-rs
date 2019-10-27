use std::path::PathBuf;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

use failure::Error;

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
