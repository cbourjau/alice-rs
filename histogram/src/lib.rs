use std::f64::INFINITY;


#[derive(Debug)]
pub struct Histogram {
    pub edges: Vec<f64>,
    pub counts: Vec<f64>,
}

impl Histogram {
    pub fn new(nbins: usize, min: f64, max: f64) -> Histogram {
        let width = (max - min) / nbins as f64;
        let edges: Vec<f64> =
            vec![-INFINITY].into_iter()
            .chain((0..nbins+1).map(|idx| {min + width * idx as f64}))
            .chain(vec![INFINITY].into_iter())
            .collect();
        Histogram {
            edges: edges,
            counts: vec![0f64; nbins + 2],
        }
    }

    pub fn fill(&mut self, value: f64) {
        let idx = self.edges.iter().rposition(|e| {e <= &value}).expect("No bin found!");
        self.counts[idx] += 1.0;
    }

    pub fn centers(&self) -> Vec<f64>{
        let x: f64 = 0.0;
        x.is_nan();
        self.edges.iter()
            .skip(1)
            .zip(self.edges.iter().take(self.edges.len() - 1))
            .map(|(low, high)| {low + 0.5 * (high - low)})
            .collect()
    }
}

impl Extend<f64> for Histogram {
    fn extend<T: IntoIterator<Item=f64>>(&mut self, values: T) {
        for value in values {
            self.fill(value);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
