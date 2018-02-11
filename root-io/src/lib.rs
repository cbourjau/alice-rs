//! # Root-io
//! This crate provides a way to retrieve data saved in the
//! [ROOT](https://root.cern.ch/) binary format commonly used for
//! particle physics experiments. The library strives provides the
//! means to inspect the contents of arbitrary ROOT files. It provies
//! basic tools to generate parsers and structs of the data found in a
//! particular file. This code may provide a reasonable starting point
//! if one wants to parse a particular type of c++-class from a ROOT
//! file. This library includes many of the structs and parsers needed
//! for this boostrapping procedure. Furthermore, `root-io` provides a
//! simple mean to read data stored in so-called `TTrees`.  The goal
//! of this library is primarily to make the data
//! [published](http://opendata.cern.ch/) by the ALICE collaboration
//! accessible in pure Rust. An example of its usage for that purpose
//! is demonstrated as an [example
//! analysis](https://github.com/cbourjau/alice-rs/tree/master/simple-analysis).
//!
//! *The API surface is almost certainly to small to build uppon this
//! library. Many more types and parsers are currently private to this
//! crate*


#![recursion_limit="256"]
#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate nom;
#[macro_use]
extern crate quote;
#[macro_use]
extern crate failure;
extern crate flate2;
extern crate xz2;
extern crate crossbeam_channel;

// pub mod core_types;
pub mod core;
mod code_gen;
pub mod tree_reader;
mod tests;

pub use core::{RootFile, FileItem};

/// Offset when using Context; should be in `Context`, maybe?
const MAP_OFFSET: u64 = 2;

