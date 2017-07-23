extern crate ndarray as nd;
extern crate itertools;

// Re-export some ndarray things
pub use nd::Dimension;
pub use nd::Dim;
pub use nd::Axis;

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
            (0..nbin+1)
                .map(|idx| {min + width * idx as f64})
                .collect::<Vec<f64>>()
        })
        .collect()
}

impl<D> Histogram<D>
where
    D: nd::Dimension
{
    /// Create a new n-dimensional histogram
    pub fn new<Sh>(nbins: Sh, mins: &[f64], maxs: &[f64]) -> Histogram<D>
        where Sh: nd::ShapeBuilder<Dim=D>
    {
        let counts: nd::Array<f64, D> = nd::Array::zeros(nbins);
        Histogram {
            edges: calculate_edges(counts.shape(), mins, maxs),
            counts: counts,
        }
    }

    /// Overwrite the bin edges along a given dimension
    /// Panics if length of new and old edges differ
    pub fn overwrite_edges(&mut self, dim: usize, edges: Vec<f64>) {
        if self.edges[dim].len() != edges.len() {
            panic!("Old and new numer of bin edges differ");
        }
        self.edges[dim] = edges;
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

macro_rules! impl_histogram {
    ($N:expr, $($idx:expr)*) => {
        impl Histogram<Dim<[usize; $N]>> {
            /// Find indices of bins along each axis
            fn find_bin_indices(&self, values: &[f64; $N]) -> Option<[usize; $N]> {
                let mut idxs = [0; $N];
                for dim in 0..$N {
                    let (edges1d, value) = (&self.edges[dim], values[dim]);
                    let idx = edges1d
                        .windows(2)
                        .position(
                            |bin_edges| &bin_edges[0] <= &value && &value < &bin_edges[1]);
                    if let Some(idx) = idx {
                        idxs[dim] = idx;
                    } else {
                        return None;
                    }
                }
                Some(idxs)
            }

            pub fn fill(&mut self, values: &[f64; $N])
            {
                if let Some(idxs) = self.find_bin_indices(values) {
                    self.counts[idxs] += 1.0;
                }
            }

        }

        impl Extend<[f64; $N]> for Histogram<Dim<[usize; $N]>> {
            fn extend<T: IntoIterator<Item=[f64; $N]>>(&mut self, values: T) {
                for value in values {
                    self.fill(&value);
                }
            }
        }
    }
}

impl_histogram!(1, 0);
impl_histogram!(2, 0 1);
impl_histogram!(3, 0 1 2);
impl_histogram!(4, 0 1 2 3);
impl_histogram!(5, 0 1 2 3 4);
impl_histogram!(6, 0 1 2 3 4 5);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calc_indices() {
        let h = Histogram::new((1, 1), &[0., 0.], &[1., 1.]);
        assert_eq!(h.find_bin_indices(&[-1.0, -1.0]), None, "Wrong indices");
        assert_eq!(h.find_bin_indices(&[2.0, 2.0]), None, "Wrong indices");
        assert_eq!(h.find_bin_indices(&[0.5, 0.5]), Some([0, 0]), "Wrong indices");
    }

    #[test]
    fn init_histogram() {
        let mut h = Histogram::new((1, 1), &[0., 0.], &[1., 1.]);
        assert_eq!(h.counts, nd::arr2(&[[0.]]));
        h.fill(&[0.5, 0.5]);
        assert_eq!(h.counts, nd::arr2(&[[1.]]));

        let mut h = Histogram::new((1, 1, 1), &[0., 0., 0.], &[1., 1., 1.]);
        assert_eq!(h.counts, nd::arr3(&[[[0.]]]));
        h.fill(&[0.5, 0.5, 0.5]);
        assert_eq!(h.counts, nd::arr3(&[[[1.]]]));
    }
}
