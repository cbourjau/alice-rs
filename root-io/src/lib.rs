//! # Root-io
//! This crate provides a way to retrieve data saved in the
//! [ROOT](https://root.cern.ch/) binary format commonly used in
//! particle physics experiments. This library provides the basic
//! means to inspect and process the contents of arbitrary ROOT
//! files. `Root-io` provides a simple mean to read
//! data stored in so-called `TTrees`.  The goal of this library is
//! primarily to make the data [published](http://opendata.cern.ch/)
//! by the ALICE collaboration accessible in pure Rust. An example of
//! its usage for that purpose is demonstrated as an [example
//! analysis](https://github.com/cbourjau/alice-rs/tree/master/examples/simple-analysis).
//!
//! The API surface is deliberately small to make the processing of said
//! files as easy as possible. If you are looking for a particular
//! parser chances have it that it exists but it is not marked as `pub`.
#![feature(negative_impls)]

#![allow(clippy::cognitive_complexity)]
#![recursion_limit = "256"]
extern crate alice_open_data;
#[macro_use]
extern crate bitflags;
extern crate flate2;
extern crate lzma_rs;
extern crate nom;
extern crate nom_locate;
#[macro_use]
extern crate quote;
extern crate reqwest;

pub use crate::core::{FileItem, RootFile, Source};

// pub mod core_types;
mod code_gen;
pub mod core;
pub mod test_utils;
mod tests;
pub mod tree_reader;

// Contains the stream_zip macro
pub mod utils;

/// Offset when using Context; should be in `Context`, maybe?
const MAP_OFFSET: u64 = 2;
