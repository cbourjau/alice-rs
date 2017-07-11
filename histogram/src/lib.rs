extern crate ndarray as nd;
extern crate itertools;

use std::f64::INFINITY;
use nd::Dimension;

use itertools::{multizip};

#[derive(Debug)]
pub struct Histogram<D>
where D: Dimension {
    pub edges: Vec<Vec<f64>>,
    pub counts: nd::Array<f64, D>,
}

/// Calculate the edges along each dimension base on nbins, min, and max
fn calculate_edges(nbins: &[usize], mins: &[f64], maxs: &[f64]) -> Vec<Vec<f64>> {
    let widths = multizip((nbins, mins, maxs))
        .map(|(&nbin, &min, &max)| {(max - min) / nbin as f64});
    multizip((nbins, widths, mins))
        .map(|(nbin, width, min)| {
            vec![-INFINITY].into_iter()
                .chain((0..nbin+1).map(|idx| {min + width * idx as f64}))
                .chain(vec![INFINITY].into_iter())
                .collect::<Vec<f64>>()
        })
        .collect()
}

impl<D> Histogram<D>
where
    D: nd::Dimension
{
    /// Create a new 3-dimensional histogram
    pub fn new(nbins: &[usize; 3], mins: &[f64; 3], maxs: &[f64; 3]) -> Histogram<nd::Ix3> {
        Histogram {
            edges: calculate_edges(nbins, mins, maxs),
            counts: nd::Array::zeros((nbins[0], nbins[1], nbins[2])),
        }
    }

    pub fn fill(&mut self, values: &[f64; 3])
        where [usize; 3]: nd::NdIndex<D>
    {
        let indices = self.find_bin_indices(values);
        self.counts[[indices[0], indices[1], indices[2]]] += 1.0;
    }
    
    /// Find indices of bins along each axis
    fn find_bin_indices(&self, values: &[f64]) -> Vec<usize> {
        self.edges.iter().zip(values)
            .map(|(edges1d, value)| {
                edges1d.iter().rposition(|e| {e <= value}).expect("No bin found!")
            })
            .collect()
    }
}
pub trait Centers {
    fn centers (&self, axis: usize) -> Vec<f64>;
}

pub trait Widths {
    fn widths(&self, axis:usize) -> Vec<f64>;
}

impl<D> Centers for Histogram<D>
    where D: nd::Dimension
{
    /// The center position of each bin along axis
    fn centers(&self, axis: usize) -> Vec<f64>{
        self.edges[axis].iter()
            .skip(1)
            .zip(self.edges[axis].iter().take(self.edges[axis].len() - 1))
            .map(|(low, high)| {low + 0.5 * (high - low)})
            .collect()
    }
}

impl<D> Widths for Histogram<D>
    where D: nd::Dimension
{
    /// The width of each bin along `axis`
    fn widths(&self, axis:usize) -> Vec<f64>{
        self.edges[axis].iter()
            .skip(1)
            .zip(self.edges[axis].iter().take(self.edges[axis].len() - 1))
            .map(|(low, high)| {high - low})
            .collect()
    }
}

impl<D> Extend<[f64; 3]> for Histogram<D>
    where D: nd::Dimension, [usize; 3]: nd::NdIndex<D>
{
    fn extend<T: IntoIterator<Item=[f64; 3]>>(&mut self, values: T) {
        for value in values {
            self.fill(&value);
        }
    }
}
