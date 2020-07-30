use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
#[cfg(not(target_arch = "wasm32"))]
use std::path::Path;
use std::path::PathBuf;

use failure::Error;
use reqwest::{
    header::{RANGE, USER_AGENT},
    Client, Url,
};

/// The source from where the Root file is read. Construct it using
/// `.into()` on a `Url` or `Path`. The latter is not availible for
/// the `wasm32` target.
#[derive(Debug, Clone)]
pub struct Source(SourceInner);

// This inner enum hides the differentiation between the local and
// remote files from the public API
#[derive(Debug, Clone)]
enum SourceInner {
    /// A local source, i.e. a file on disc.
    Local(PathBuf),
    Remote {
        client: Client,
        url: Url,
    },
}

impl Source {
    pub fn new<T: Into<Self>>(thing: T) -> Self {
        thing.into()
    }

    pub async fn fetch(&self, start: u64, len: u64) -> Result<Vec<u8>, Error> {
        match &self.0 {
            SourceInner::Local(path) => {
                let mut f = File::open(&path)?;
                f.seek(SeekFrom::Start(start))?;
                let mut buf = vec![0; len as usize];
                f.read_exact(&mut buf)?;
                Ok(buf)
            }
            SourceInner::Remote { client, url } => {
                let rsp = client
                    .get(url.clone())
                    .header(USER_AGENT, "alice-rs")
                    .header(RANGE, format!("bytes={}-{}", start, start + len - 1))
                    .send()
                    .await?;
                let bytes = rsp.bytes().await?;
                Ok(bytes.as_ref().to_vec())
            }
        }
    }
}

impl From<Url> for Source {
    fn from(url: Url) -> Self {
        Self(SourceInner::Remote {
            client: Client::new(),
            url,
        })
    }
}

// Disallow the construction of a local source object on wasm since
// wasm does not have a (proper) file system.
#[cfg(not(target_arch = "wasm32"))]
impl From<&Path> for Source {
    fn from(path: &Path) -> Self {
        path.to_path_buf().into()
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl From<PathBuf> for Source {
    fn from(path_buf: PathBuf) -> Self {
        Self(SourceInner::Local(path_buf))
    }
}
