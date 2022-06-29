//! This module contains the core structs and parsers needed to read
//! the self-description of a root file. These parsers can be used to
//! build new parsers using the [root-ls](https://github.com/cbourjau/alice-rs) cli.
use crate::core::ReadError::ParseError;
use thiserror::Error;

pub use self::compression::DecompressionError;
pub(crate) use self::compression::*;
pub use self::data_source::Source;
pub use self::file::RootFile;
pub use self::file_item::FileItem;
pub(crate) use self::parsers::*;
pub(crate) use self::tkey::*;
pub(crate) use self::tstreamer::{tstreamer, TStreamer};
pub(crate) use self::tstreamerinfo::{tstreamerinfo, TStreamerInfo};
pub(crate) use self::typeid::*;
pub(crate) use self::types::*;

mod compression;
mod data_source;
mod file;
mod file_item;
pub mod parsers;
mod tkey;
mod tstreamer;
mod tstreamerinfo;
mod typeid;
pub mod types;

#[derive(Error, Debug)]
pub enum SemanticError {
    #[error("Unsupported version {1} for {0:?} ({2})")]
    VersionNotSupported(Component, u32, &'static str),
}

#[derive(Debug)]
pub enum Component {
    TStreamerElement,
}

#[derive(Error, Debug)]
pub enum ReadError {
    #[error("Error reading data")]
    IoError(#[from] std::io::Error),
    #[error("Error fetching data from online source")]
    ReqwestError(#[from] reqwest::Error),
    #[error("Error decompressing data")]
    DecompressionError(#[from] DecompressionError),
    #[error("Error parsing data")]
    ParseError(VerboseErrorInfo),
}

pub trait UnwrapPrint<T> {
    fn unwrap_print(self) -> T;
}

impl<T> UnwrapPrint<T> for Result<T, ReadError> {
    fn unwrap_print(self) -> T {
        match self {
            Ok(v) => v,
            Err(ParseError(e)) => {
                panic!("Tried to unwrap a parse error:\n{}", e);
            }
            Err(e) => {
                panic!("Tried to unwrap a read error:\n{}", e)
            }
        }
    }
}

impl From<VerboseErrorInfo> for ReadError {
    fn from(e: VerboseErrorInfo) -> ReadError {
        ParseError(e)
    }
}

#[derive(Error, Debug)]
pub enum WriteError {
    #[error(transparent)]
    ReadError(#[from] ReadError),
    #[error(transparent)]
    FmtError(#[from] std::fmt::Error),
}
