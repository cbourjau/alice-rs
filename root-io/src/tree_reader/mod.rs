//! A convenience wrapper and needed parsers to work with ROOT's
//! `TTree`s. A Tree may be thought of as a table where each row
//! represents a particle collision. Each column may contain one or
//! several elements per collision. This module provides two Iterator
//! structs in order to iterate over these columns (`TBranches` in
//! ROOT lingo).
use nom::error::VerboseError;
use thiserror::Error;

use crate::core::DecompressionError;
use crate::tree_reader::ReadError::ParseError;

pub use self::tree::{Tree, ttree};

mod branch;
mod container;
mod leafs;
pub mod tree;

#[derive(Error, Debug)]
pub enum ReadError {
    #[error("Error reading data")]
    IoError(#[from] std::io::Error),
    #[error("Error fetching data from online source")]
    ReqwestError(#[from] reqwest::Error),
    #[error("Error decompressing data")]
    DecompressionError(#[from] DecompressionError),
    #[error("Error parsing data")]
    ParseError(VerboseError<Vec<u8>>),
}

impl From<VerboseError<Vec<u8>>> for ReadError {
    fn from(e: VerboseError<Vec<u8>>) -> ReadError {
        ParseError(e)
    }
}

#[derive(Error, Debug)]
pub enum WriteError {
    #[error(transparent)]
    ReadError(#[from] ReadError),
    #[error(transparent)]
    FmtError(#[from] std::fmt::Error)
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use tokio;

    use std::path::PathBuf;

    use crate::core::RootFile;

    #[tokio::test]
    async fn simple_tree() {
        let path = PathBuf::from("./src/test_data/simple.root");
        let f = RootFile::new(path.as_path())
            .await
            .expect("Failed to open file");
        f.items()[0].as_tree().await.unwrap();
    }
}
