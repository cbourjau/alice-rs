/// A bunch of traits which can or must be implemented by analyzes running over datasets

/// Merge two results of the same analysis This trait must be
/// implemented for all analyses which should be used with datasets.
/// Otherwise, its not clear how the parallelized analysis results
/// should be merged into one
pub trait Merge<RHS = Self> {
    /// Combine the `rhs` analysis output with this one
    fn merge(&mut self, rhs: &RHS);
}

/// Create a visualization of the results of this analysis.
/// Not required but feels nice to do it in Rust :)
pub trait Visualize {
    /// Create a visual representation of the results of this analysis
    fn visualize(&self);
}
