extern crate ndarray as nd;
extern crate itertools;

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

    // pub fn fill<'a>(&mut self, values: &[f64; 3])
    //     where &'a[nd::Ix]: nd::NdIndex<D>
    // {
    //     if let Some(indices) = self.find_bin_indices(values) {
    //         let slice_idxs = indices.as_slice();
    //         self.counts[slice_idxs] += 1.0;
    //     }
    // }
    /// Find indices of bins along each axis
    fn find_bin_indices(&self, values: &[f64]) -> Option<Vec<usize>> {
        self.edges.iter().zip(values)
            .map(|(edges1d, value)| {
                edges1d
                    .windows(2)
                    .rposition(
                        |bin_edges| {
                            &bin_edges[0] <= value && value < &bin_edges[1]
                        }
                    )
            })
            .collect()
    }
}

impl<D> Histogram<D>
where
    D: nd::Dimension
{
    pub fn fill_1(&mut self, values: &[f64; 1])
        where
        [usize; 1]: nd::NdIndex<D>
    {
        if let Some(idxs) = self.find_bin_indices(values) {
            self.counts[[idxs[0]]] += 1.0;
        }
    }

    pub fn fill_2(&mut self, values: &[f64; 2])
        where
        [usize; 2]: nd::NdIndex<D>
    {
        if let Some(idxs) = self.find_bin_indices(values) {
            self.counts[[idxs[0], idxs[1]]] += 1.0;
        }
    }
    pub fn fill_3(&mut self, values: &[f64; 3])
        where
        [usize; 3]: nd::NdIndex<D>
    {
        if let Some(idxs) = self.find_bin_indices(values) {
            self.counts[[idxs[0], idxs[1], idxs[2]]] += 1.0;
        }
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
            self.fill_3(&value);
        }
    }
}

impl<D> Extend<[f64; 1]> for Histogram<D>
    where D: nd::Dimension, [usize; 1]: nd::NdIndex<D>
{
    fn extend<T: IntoIterator<Item=[f64; 1]>>(&mut self, values: T) {
        for value in values {
            self.fill_1(&value);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_histogram() {
        let mut h = Histogram::new((1, 1), &[0., 0.], &[1., 1.]);
        assert_eq!(h.counts, nd::arr2(&[[0.]]));
        h.fill_2(&[0.5, 0.5]);
        assert_eq!(h.counts, nd::arr2(&[[1.]]));

        let mut h = Histogram::new((1, 1, 1), &[0., 0., 0.], &[1., 1., 1.]);
        assert_eq!(h.counts, nd::arr3(&[[[0.]]]));
        h.fill_3(&[0.5, 0.5, 0.5]);
        assert_eq!(h.counts, nd::arr3(&[[[1.]]]));
    }
}
