/// Merge two analyses
pub trait Merge<RHS = Self> {
    type Output;
    fn merge(mut self, rhs: &RHS) -> Self::Output;
}
