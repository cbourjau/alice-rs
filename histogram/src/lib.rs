use std::cmp::Ordering;
use std::fs::File;
use std::io::prelude::*;

use bincode::serialize;
use failure::Error;
use ndarray as nd;
use wasm_bindgen::prelude::*;

// Re-export some ndarray things
pub use nd::Axis;
pub use nd::Dimension;
pub use nd::IxDyn;

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Histogram {
    edges: Vec<Vec<BinEdges>>,
    bins: nd::Array<f64, IxDyn>,
}

#[wasm_bindgen]
impl Histogram {
    /// Find the bin index containing `value` on `axis`
    /// Return None if the the value is not in range
    fn find_bin_index_axis(&self, axis: usize, value: f64) -> Option<usize> {
        let (edges1d, value) = (&self.edges[axis], value);
        edges1d.binary_search_by(|bin| bin.cmp_with(value)).ok()
    }

    /// Find indices of bins along each axis
    fn find_bin_indices(&self, values: &[f64]) -> Option<Vec<usize>> {
        (0..values.len())
            .map(|dim| self.find_bin_index_axis(dim, values[dim]))
            .collect()
    }

    pub fn fill(&mut self, values: &[f64]) {
        if let Some(bin) = self.bin_mut(values) {
            *bin += 1.0;
        }
    }

    /// The center position of each bin along axis
    pub fn centers(&self, axis: usize) -> Vec<f64> {
        self.edges[axis].iter().map(|bin| bin.center()).collect()
    }

    /// The width of each bin along `axis`
    pub fn widths(&self, axis: usize) -> Vec<f64> {
        self.edges[axis].iter().map(|bin| bin.width()).collect()
    }

    pub fn values(&self) -> Box<[f64]> {
        self.bins.clone().into_raw_vec().into_boxed_slice()
    }

    /// Sum all bins along `axis` returning a new histogram with
    /// reduced dimensionality.
    ///
    /// Panics if `axis` is out of bounds.
    pub fn sum_axis(&self, axis: u32) -> Histogram {
        let axis = axis as usize;
        let bins = self.bins.sum_axis(Axis(axis));
        let edges = self
            .edges
            .iter()
            .enumerate()
            .filter_map(|(n, ax_edges)| {
                if n == axis {
                    None
                } else {
                    Some(ax_edges.clone())
                }
            })
            .collect();
        Histogram { bins, edges }
    }

    /// Multiply the values inside this this histogram by a scalar
    /// value.
    pub fn mul(self, factor: f64) -> Histogram {
        Histogram {
            bins: self.bins * factor,
            ..self
        }
    }
}

// The following impl block is not wasm compatible
impl Histogram {
    /// Get a mutable reference to the bin including `values`. Panics
    /// if `values` dimensionality is incompatible with that of the
    /// histogram.
    pub fn bin_mut(&mut self, values: &[f64]) -> Option<&mut f64> {
        if values.len() != self.edges.len() {
            panic!("Expected values slice of len {}", self.edges.len());
        }
        self.find_bin_indices(values)
            .and_then(move |idx| self.bins.get_mut(idx.as_slice()))
    }

    /// Dump histogram (without edges) to a file of `name`.
    /// The binary layout is:
    /// `(array_version: u8, ndim: u64, shape: [ndim; u64], a_size: u64, a: [a_size; A])`
    pub fn dump_to_file(&self, name: &str) -> Result<(), Error> {
        let buf = serialize(&self.bins)?;
        let mut f = File::create(name)?;
        f.write_all(buf.as_slice())?;
        Ok(())
    }
}

#[wasm_bindgen]
#[derive(Default)]
pub struct HistogramBuilder {
    edges: Vec<Vec<f64>>,
}

#[wasm_bindgen]
impl HistogramBuilder {
    #[wasm_bindgen(constructor)]
    pub fn new() -> HistogramBuilder {
        HistogramBuilder { edges: Vec::new() }
    }

    /// Create a new n-dimensional histogram
    pub fn build(&self) -> Option<Histogram> {
        let edges: Vec<Vec<BinEdges>> = self
            .edges
            .iter()
            .map(|edges1d| edges_to_bins(edges1d))
            .collect();
        if edges.len() == 0 {
            return None;
        }
        let shape: Vec<_> = edges.iter().map(|edges| edges.len()).collect();

        let bins = nd::ArrayD::zeros(IxDyn(shape.as_ref()));
        Some(Histogram { bins, edges })
    }

    pub fn add_equal_width_axis(mut self, nbins: usize, min: f64, max: f64) -> HistogramBuilder {
        let width = (max - min) / nbins as f64;
        self.edges.push(
            (0..=nbins)
                .map(|i| min + width * i as f64)
                .collect::<Vec<f64>>(),
        );
        self
    }

    pub fn add_variable_width_axis(mut self, edges1d: &[f64]) -> HistogramBuilder {
        self.edges.push(edges1d.to_vec());
        self
    }
}

#[derive(Clone, Debug)]
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
        let h = HistogramBuilder::new()
            .add_equal_width_axis(1, 0., 1.)
            .add_equal_width_axis(1, 0., 1.)
            .build()
            .unwrap();
        assert_eq!(h.find_bin_indices(&[-1.0, -1.0]), None, "Wrong indices");
        assert_eq!(h.find_bin_indices(&[2.0, 2.0]), None, "Wrong indices");
        assert_eq!(
            h.find_bin_indices(&[0.5, 0.5]),
            Some(vec![0, 0]),
            "Wrong indices"
        );
    }

    #[test]
    fn init_histogram() {
        let h = HistogramBuilder::new()
            .add_equal_width_axis(1, 0., 1.)
            .add_equal_width_axis(1, 0., 1.)
            .build()
            .unwrap();
        assert_eq!(h.edges[0].len(), 1);
        assert_eq!(h.bins, nd::arr2(&[[0.]]).into_dyn());

        let h = HistogramBuilder::new()
            .add_equal_width_axis(1, 0., 1.)
            .add_equal_width_axis(1, 0., 1.)
            .add_equal_width_axis(1, 0., 1.)
            .build()
            .unwrap();

        assert_eq!(h.bins, nd::arr3(&[[[0.]]]).into_dyn());
    }

    #[test]
    fn faulty_init() {
        // No axis
        let opt = HistogramBuilder::new().build();
        assert!(opt.is_none());
    }

    #[test]
    fn filling() {
        let mut h = HistogramBuilder::new()
            .add_equal_width_axis(2, 0., 2.)
            .add_equal_width_axis(2, 0., 2.)
            .build()
            .unwrap();
        // underflow both bins
        h.fill(&[-5., -5.]);
        assert_eq!(h.bins, nd::arr2(&[[0., 0.], [0., 0.]]).into_dyn());
        // underflow one bin
        h.fill(&[-5., 1.]);
        assert_eq!(h.bins, nd::arr2(&[[0., 0.], [0., 0.]]).into_dyn());
        // underflow / overflow
        h.fill(&[-5., 5.]);
        assert_eq!(h.bins, nd::arr2(&[[0., 0.], [0., 0.]]).into_dyn());
        // both fit
        h.fill(&[0.5, 0.5]);
        assert_eq!(h.bins, nd::arr2(&[[1., 0.], [0., 0.]]).into_dyn());
    }

    #[test]
    fn edges_and_centers() {
        let h = HistogramBuilder::new()
            .add_equal_width_axis(2, -1., 1.)
            .add_equal_width_axis(2, -1., 1.)
            .build()
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
