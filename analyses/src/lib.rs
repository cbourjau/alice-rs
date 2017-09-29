#[macro_use]
extern crate ndarray;
extern crate histogram;
extern crate alice;
extern crate gnuplot;
extern crate glob;
extern crate rustfft;
extern crate num_traits as libnum;
extern crate rand;
extern crate indicatif;
extern crate alice_open_data;

pub mod analyses;
pub use analyses::{ProcessEvent, Visualize, ParticlePairDistributions};

