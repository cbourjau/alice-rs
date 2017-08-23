use libnum;
use ndarray as nd;
use ndarray::{Axis, Array, ArrayBase, ArrayView, Data, Dimension, IntoDimension};

pub const COLORS: [&str; 9] = ["#4D4D4D","#5DA5DA","#FAA43A","#60BD68",
                               "#F17CB0","#B2912F","#B276B2","#DECF3F",
                               "#F15854"];

pub fn nanmean<A, D, S>(a: &nd::ArrayBase<S, D>, axis: Axis) -> nd::Array<A, D::Smaller>
    where A: nd::LinalgScalar + libnum::Float,
          S: nd::Data<Elem = A> + nd::DataMut + nd::DataClone,
          D: nd::RemoveAxis,
          <D as nd::Dimension>::Smaller: nd::RemoveAxis
{
    // Create a mask of the same shape as `a` and set it all to zero
    let mut mask = a.clone().mapv(|_| A::zero());
    let mut a_fixed = a.clone();
    for (v, m) in a_fixed.iter_mut().zip(mask.iter_mut()) {
        if !v.is_nan() {
            *m = A::one();
        } else {
            // Set to 0 if nan
            *v = A::zero();
        }
    }
    let sum = a_fixed.sum(axis);
    sum / &mask.sum(axis)
}


pub fn keep_axes<A, D, NewDim>(a: &ArrayView<A, D>, keep: NewDim) -> Array<A, nd::IxDyn>
    where A: 'static + Clone + libnum::Float,
          NewDim: IntoDimension,
          D: Dimension
{
    let mut a = a.to_owned();
    let keep = keep.into_dimension();
    let keep = keep.as_array_view();
    for (i, keep_me) in keep.iter().enumerate() {
        roll_axis(&mut a, Axis(i), Axis(*keep_me));
    }
    // Find number of elements to remove
    let n_removed = a.shape()
        .iter()
        .skip(keep.len())
        .fold(1, |acc, &val| val * acc);
    // Figure out shape where all the bins to merge are in the last dim
    let mut tmp_shape = a.shape()
        .iter()
        .take(keep.len())
        .cloned()
        .collect::<Vec<usize>>();
    tmp_shape.push(n_removed);
    // reshape a such that all the dimensions to be merged are on the last axis
    let a = a.into_shape(tmp_shape.as_slice())
        .expect("Invalid reshaping");
    nanmean(&a, nd::Axis(keep.len()))
}

pub fn roll_axis<A, S, D>(mut a: &mut ArrayBase<S, D>, to: Axis, from: Axis)
    where S: Data<Elem = A>,
          D: Dimension
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

    #[test]
    fn keep_axes_test() {
        let a = nd::arr2(&[[5.0, 0.0]]);
        assert_eq!(keep_axes(&a.view(), [0]).shape(), &[1]);
        // shape [1, 3, 2]
        let a = nd::arr3(&[[[5.0, 0.0], [5.0, 0.0], [5.0, 0.0]]]);
        assert_eq!(keep_axes(&a.view(), [0, 2]).shape(), &[1, 2]);
        assert_eq!(keep_axes(&a.view(), [1, 2]).shape(), &[3, 2]);
    }
}
