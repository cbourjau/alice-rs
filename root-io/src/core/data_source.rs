use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
#[cfg(not(target_arch = "wasm32"))]
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;

use failure::Error;
use reqwest::{
    header::{RANGE, USER_AGENT},
    Client, Url,
};

use async_trait::async_trait;

/// The source from where the Root file is read. Construct it using
/// `.into()` on a `Url` or `Path`. The latter is not availible for
/// the `wasm32` target.
#[derive(Debug, Clone)]
pub struct Source {
    inner: Arc<dyn DataSource + Send + Sync>,
}

impl Source {
    pub fn new<T: Into::<Self>>(thing: T) -> Self {
	thing.into()
    }
    
    pub async fn fetch(&self, start: u64, len: u64) -> Result<Vec<u8>, Error> {
	self.inner.fetch(start, len).await
    }
}

#[async_trait(?Send)]
trait DataSource: std::fmt::Debug {
    async fn fetch(&self, start: u64, len: u64) -> Result<Vec<u8>, Error>;
}

/// A local source, i.e. a file on disc.
#[derive(Debug, Clone)]
struct LocalDataSource(PathBuf);

/// A remote source, i.e a file which is fetched with a http request
#[derive(Debug)]
struct RemoteDataSource {
    client: Client,
    url: Url,
}

impl From<Url> for Source {
    fn from(url: Url) -> Self {
	Self {
	    inner: Arc::new(
		RemoteDataSource {
		    client: Client::new(),
		    url,
	    })
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl From<&Path> for Source {
    fn from(path: &Path) -> Self {
	Self {
	    inner: Arc::new(
		LocalDataSource(path.to_path_buf())
	    )
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl From<PathBuf> for Source {
    fn from(path_buf: PathBuf) -> Self {
	Self {
	    inner: Arc::new(
		LocalDataSource(path_buf)
	    )
        }
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
