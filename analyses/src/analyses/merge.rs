/// Merge two analyses
pub trait Merge<RHS = Self> {
    fn merge(&mut self, rhs: &RHS);
}
