extern crate bincode;
extern crate failure;
extern crate itertools;
extern crate ndarray;
extern crate num_traits as libnum;
extern crate serde;

use bincode::serialize;
use failure::Error;
use ndarray as nd;
use serde::Serialize;

use std::cmp::Ordering;
use std::fs::File;
use std::io::prelude::*;
use std::marker::PhantomData;
use std::ops::{Add, AddAssign};

// Re-export some ndarray things
pub use nd::Axis;
pub use nd::Dimension;
pub use nd::IxDyn;

#[derive(Debug)]
pub struct Histogram<A, D> {
    edges: Vec<Vec<BinEdges>>,
    pub counts: nd::Array<A, IxDyn>,
    dim: PhantomData<D>,
}

pub trait Centers {
    fn centers(&self, axis: usize) -> Vec<f64>;
}

pub trait Widths {
    fn widths(&self, axis: usize) -> Vec<f64>;
}

impl<A, D> Centers for Histogram<A, D> {
    /// The center position of each bin along axis
    fn centers(&self, axis: usize) -> Vec<f64> {
        self.edges[axis].iter().map(|bin| bin.center()).collect()
    }
}

impl<A, D> Widths for Histogram<A, D> {
    /// The width of each bin along `axis`
    fn widths(&self, axis: usize) -> Vec<f64> {
        self.edges[axis].iter().map(|bin| bin.width()).collect()
    }
}

macro_rules! impl_histogram {
    ($N:expr, $($idx:expr)*) => {
        impl<A> Histogram<A, [usize; $N]>
        where
            A: Copy + libnum::Zero + Add + AddAssign + libnum::One + PartialOrd,
        {
            /// Find the bin index containing `value` on `axis`
            /// Return None if the the value is not in range
            pub fn find_bin_index_axis(&self, axis: usize, value: f64) -> Option<usize> {
                let (edges1d, value) = (&self.edges[axis], value);
                edges1d.binary_search_by(|bin| bin.cmp_with(value)).ok()
            }

            /// Find indices of bins along each axis
            fn find_bin_indices(&self, values: &[f64; $N]) -> Option<[usize; $N]> {
                let mut idxs = [0; $N];
                for dim in 0..$N {
                    match self.find_bin_index_axis(dim, values[dim]) {
                        Some(idx) => idxs[dim] = idx,
                        None => return None,
                    }
                }
                Some(idxs)
            }

            pub fn fill(&mut self, values: &[f64; $N]) {
                if let Some(idxs) = self.find_bin_indices(values) {
                    self.counts[idxs.as_ref()] += A::one();
                }
            }
            pub fn fill_by_index<I>(&mut self, indices: [usize; $N]) {
                self.counts[indices.as_ref()] += A::one();
            }
            pub fn fill_by_index_bulk<T>(&mut self, indices_slice: T)
            where
                T: IntoIterator<Item = [usize; $N]>,
            {
                for idxs in indices_slice {
                    self.counts[idxs.as_ref()] += A::one();
                }
            }
            pub fn fill_bulk<T>(&mut self, values: T, npairs: usize)
            where
                T: IntoIterator<Item = [f64; $N]>,
            {
                let mut indices = Vec::<[usize; $N]>::with_capacity(npairs);
                indices.extend(values.into_iter().filter_map(|v| self.find_bin_indices(&v)));
                for idxs in indices {
                    self.counts[idxs.as_ref()] += A::one();
                }
            }

            pub fn add(&mut self, other: &Histogram<A, [usize; $N]>) {
                // assert_eq!(self.edges.as_slice(), other.edges.as_slice());
                self.counts += &other.counts;
            }

            /// Dump histogram (without edges) to a file of `name`.
            /// The binary layout is:
            /// `(array_version: u8, ndim: u64, shape: [ndim; u64], a_size: u64, a: [a_size; A])`
            pub fn dump_to_file(&self, name: &str) -> Result<(), Error>
            where
                A: Serialize,
            {
                let buf = serialize(&self.counts)?;
                let mut f = File::create(name)?;
                f.write_all(buf.as_slice())?;
                Ok(())
            }
        }

        impl<A> Extend<[f64; $N]> for Histogram<A, [usize; $N]>
        where
            A: Copy + libnum::Num + AddAssign + PartialOrd,
        {
            fn extend<T>(&mut self, values: T)
            where
                T: IntoIterator<Item = [f64; $N]>,
            {
                let indices: Vec<_> = values
                    .into_iter()
                    .filter_map(|v| self.find_bin_indices(&v))
                    .collect();
                for idxs in indices {
                    self.counts[idxs.as_ref()] += A::one();
                }
            }
        }
    };
}

impl_histogram!(1, 0);
impl_histogram!(2, 0 1);
impl_histogram!(3, 0 1 2);
impl_histogram!(4, 0 1 2 3);
impl_histogram!(5, 0 1 2 3 4);
impl_histogram!(6, 0 1 2 3 4 5);
impl_histogram!(7, 0 1 2 3 4 5 6);
impl_histogram!(8, 0 1 2 3 4 5 6 7);

#[derive(Default)]
pub struct HistogramBuilder<D> {
    edges: Vec<Vec<f64>>,
    phantom: PhantomData<D>,
}

macro_rules! impl_histogram_builder {
    ($N:expr, $($idx:expr)*) => {
        impl HistogramBuilder<[usize; $N]> {
            pub fn new() -> HistogramBuilder<[usize; $N]> {
                HistogramBuilder {
                    edges: Vec::new(),
                    phantom: PhantomData,
                }
            }
            /// Create a new n-dimensional histogram
            pub fn build<A>(&self) -> Option<Histogram<A, [usize; $N]>>
            where
                A: Clone + libnum::Num,
            {
                let edges: Vec<Vec<BinEdges>> = self
                    .edges
                    .iter()
                    .map(|edges1d| edges_to_bins(edges1d))
                    .collect();
                if edges.len() != $N {
                    return None;
                }
                let mut shape = [0; $N];
                for dim in 0..$N {
                    shape[dim] = edges[dim].len();
                }

                let counts = nd::ArrayD::<A>::zeros(IxDyn(shape.as_ref()));
                Some(Histogram::<A, [usize; $N]> {
                    counts: counts,
                    edges: edges,
                    dim: PhantomData,
                })
            }
            pub fn add_equal_width_axis(
                &mut self,
                nbins: usize,
                min: f64,
                max: f64,
            ) -> &mut HistogramBuilder<[usize; $N]> {
                let width = (max - min) / nbins as f64;
                self.edges.push(
                    (0..=nbins)
                        .map(|i| min + width * i as f64)
                        .collect::<Vec<f64>>(),
                );
                self
            }
            pub fn add_variable_width_axis<'a>(
                &'a mut self,
                edges1d: &[f64],
            ) -> &'a mut HistogramBuilder<[usize; $N]> {
                self.edges.push(edges1d.to_vec());
                self
            }
        }
    };
}

impl_histogram_builder!(1, 0);
impl_histogram_builder!(2, 0 1);
impl_histogram_builder!(3, 0 1 2);
impl_histogram_builder!(4, 0 1 2 3);
impl_histogram_builder!(5, 0 1 2 3 4);
impl_histogram_builder!(6, 0 1 2 3 4 5);
impl_histogram_builder!(7, 0 1 2 3 4 5 6);
impl_histogram_builder!(8, 0 1 2 3 4 5 6 7);

#[derive(Debug)]
struct BinEdges {
    lower: f64,
    upper: f64,
}

impl BinEdges {
    pub fn width(&self) -> f64 {
        self.upper - self.lower
    }
    pub fn center(&self) -> f64 {
        self.lower + 0.5 * self.width()
    }
    /// Compute if a given `value` is below, within or above the given binary
    /// A bins interval is half open on [low, high)
    pub fn cmp_with(&self, value: f64) -> Ordering {
        if value < self.lower {
            Ordering::Greater
        } else if value < self.upper {
            Ordering::Equal
        } else {
            Ordering::Less
        }
    }
}

/// Turn a vector of edges to a vector of `BinEdges`
fn edges_to_bins(edges1d: &[f64]) -> Vec<BinEdges> {
    edges1d
        .windows(2)
        .map(|window| BinEdges {
            lower: window[0],
            upper: window[1],
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calc_indices() {
        let h = HistogramBuilder::<[usize; 2]>::new()
            .add_equal_width_axis(1, 0., 1.)
            .add_equal_width_axis(1, 0., 1.)
            .build::<f32>()
            .unwrap();
        assert_eq!(h.find_bin_indices(&[-1.0, -1.0]), None, "Wrong indices");
        assert_eq!(h.find_bin_indices(&[2.0, 2.0]), None, "Wrong indices");
        assert_eq!(
            h.find_bin_indices(&[0.5, 0.5]),
            Some([0, 0]),
            "Wrong indices"
        );
    }

    #[test]
    fn init_histogram() {
        let h = HistogramBuilder::<[usize; 2]>::new()
            .add_equal_width_axis(1, 0., 1.)
            .add_equal_width_axis(1, 0., 1.)
            .build::<f32>()
            .unwrap();
        assert_eq!(h.edges[0].len(), 1);
        assert_eq!(h.counts, nd::arr2(&[[0.]]).into_dyn());

        let h = HistogramBuilder::<[usize; 3]>::new()
            .add_equal_width_axis(1, 0., 1.)
            .add_equal_width_axis(1, 0., 1.)
            .add_equal_width_axis(1, 0., 1.)
            .build::<f32>()
            .unwrap();

        assert_eq!(h.counts, nd::arr3(&[[[0.]]]).into_dyn());
    }

    #[test]
    fn faulty_init() {
        // Only 1 dim added
        let opt = HistogramBuilder::<[usize; 2]>::new()
            .add_equal_width_axis(1, 0., 1.)
            .build::<f32>();
        assert!(opt.is_none());
        // added 3 dims
        let opt = HistogramBuilder::<[usize; 2]>::new()
            .add_equal_width_axis(1, 0., 1.)
            .add_equal_width_axis(1, 0., 1.)
            .add_equal_width_axis(1, 0., 1.)
            .build::<f32>();
        assert!(opt.is_none());
    }

    #[test]
    fn filling() {
        let mut h = HistogramBuilder::<[usize; 2]>::new()
            .add_equal_width_axis(2, 0., 2.)
            .add_equal_width_axis(2, 0., 2.)
            .build::<f32>()
            .unwrap();
        // underflow both bins
        h.fill(&[-5., -5.]);
        assert_eq!(h.counts, nd::arr2(&[[0., 0.], [0., 0.]]).into_dyn());
        // underflow one bin
        h.fill(&[-5., 1.]);
        assert_eq!(h.counts, nd::arr2(&[[0., 0.], [0., 0.]]).into_dyn());
        // underflow / overflow
        h.fill(&[-5., 5.]);
        assert_eq!(h.counts, nd::arr2(&[[0., 0.], [0., 0.]]).into_dyn());
        // both fit
        h.fill(&[0.5, 0.5]);
        assert_eq!(h.counts, nd::arr2(&[[1., 0.], [0., 0.]]).into_dyn());
    }

    #[test]
    fn edges_and_centers() {
        let h = HistogramBuilder::<[usize; 2]>::new()
            .add_equal_width_axis(2, -1., 1.)
            .add_equal_width_axis(2, -1., 1.)
            .build::<f32>()
            .unwrap();
        assert_eq!(h.edges[0][0].lower, -1.0);
        assert_eq!(h.edges[0][0].upper, 0.0);
        assert_eq!(h.edges[0][1].lower, 0.0);
        assert_eq!(h.edges[0][1].upper, 1.0);

        assert_eq!(h.centers(0), &[-0.5, 0.5]);
    }

    #[test]
    fn bin_edges() {
        let be = BinEdges {
            lower: 0.0,
            upper: 1.0,
        };
        // Read as "Bin is greater than value"!
        assert_eq!(be.cmp_with(2.0), Ordering::Less);
        assert_eq!(be.cmp_with(0.5), Ordering::Equal);
        assert_eq!(be.cmp_with(-1.0), Ordering::Greater);
    }
}
