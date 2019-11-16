//! This module contains the core structs and parsers needed to read
//! the self-description of a root file. These parsers can be used to
//! build new parsers using the [root-ls](https://github.com/cbourjau/alice-rs) cli.

mod data_source;
mod file;
mod file_item;
pub mod parsers;
mod tkey;
mod tstreamer;
mod tstreamerinfo;
mod typeid;
pub mod types;

pub(crate) use self::data_source::{Source};
pub(crate) use self::parsers::*;
pub(crate) use self::tkey::*;
pub(crate) use self::tstreamer::{tstreamer, TStreamer};
pub(crate) use self::tstreamerinfo::{tstreamerinfo, TStreamerInfo};
pub(crate) use self::typeid::*;
pub(crate) use self::types::*;

pub use self::file::RootFile;
pub use self::file_item::FileItem;
