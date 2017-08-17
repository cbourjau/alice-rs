use libnum;
use ndarray as nd;
use ndarray::{Axis, Array, ArrayBase, ArrayView, Data, Dimension, IntoDimension};

pub const COLORS: [&str; 9] = ["#4D4D4D","#5DA5DA","#FAA43A","#60BD68",
                               "#F17CB0","#B2912F","#B276B2","#DECF3F",
                               "#F15854"];

pub fn nanmean<A, D, S>(a: &nd::ArrayBase<S, D>, axis: Axis) -> nd::Array<A, D::Smaller>
    where A: nd::LinalgScalar + libnum::Float,
          S: nd::Data<Elem=A> + nd::DataMut + nd::DataClone,
          D: nd::RemoveAxis,
          <D as nd::Dimension>::Smaller: nd::RemoveAxis,

{
    // Create a mask of the same shape as `a` and set it all to zero
    let mut mask = a.clone().mapv(|_| A::zero());
    let mut a_fixed = a.clone();
    for (v, m) in a_fixed.iter_mut().zip(mask.iter_mut()) {
        if !v.is_nan() {
            *m = A::one();
        } else { // Set to 0 if nan
            *v = A::zero();
        }
    }
    let sum = a_fixed.sum(axis);
    sum / &mask.sum(axis)
}


pub fn roll_axis<A, S, D>(mut a: &mut ArrayBase<S, D>, to: Axis, from: Axis)
    where S: Data<Elem=A>,
          D: Dimension,
{
    let i = to.index();
    let mut j = from.index();
    if j > i {
        while i != j {
            a.swap_axes(i, j);
            j -= 1;
        }
    } else {
        while i != j {
            a.swap_axes(i, j);
            j += 1;
        }
    }
    let sum = a_fixed.sum(Axis(axis));
    sum / &mask.sum(Axis(axis))
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::NAN;
    #[test]
    fn nanmean_tests() {
        let a = nd::arr2(&[[0.0, NAN]]);
        assert_eq!(nanmean(&a, Axis(1)), nd::arr1::<f64>(&[0.0]));

        let a = nd::arr2(&[[2.0, NAN, 0.0]]);
        assert_eq!(nanmean(&a, Axis(1)), nd::arr1::<f64>(&[1.0]));

        let a = nd::arr2(&[[NAN]]);
        assert!(nanmean(&a, Axis(1))[[0]].is_nan());

        let a = nd::arr2(&[[5.0, NAN], [5.0, NAN]]);
        assert_eq!(nanmean(&a, Axis(1)), nd::arr1::<f64>(&[5.0, 5.0]));
        // This test is not working since we nan != nan
        // let a = nd::arr2(&[[5.0, NAN], [5.0, NAN]]);
        // assert_eq!(nanmean(a, 0), nd::arr1::<f64>(&[5.0, NAN]));
    }
}
