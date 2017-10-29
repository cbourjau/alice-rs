/// A few utilities which might be useful when writing analyses
use std::io::prelude::*;
use std::fs::File;
use ndarray as nd;
use serde::Serialize;
use bincode::{serialize, Infinite};


/// A nice color set for Gnuplot
pub const COLORS: [&str; 9] = ["#4D4D4D","#5DA5DA","#FAA43A","#60BD68",
                               "#F17CB0","#B2912F","#B276B2","#DECF3F",
                               "#F15854"];

/// Dump an ndarray to the given file.
/// The binary layout is:
/// `(array_version: u8, ndim: u64, shape: [ndim; u64], a_size: u64, a: [a_size; A])`
pub fn dump_to_file<A, D>(a: &nd::Array<A, D>, name: &str)
    where A: Serialize,
          D: nd::Dimension + Serialize
{
    let buf = serialize(&a, Infinite).unwrap();
    let mut f = File::create(name).expect("Could not create file");
    f.write_all(buf.as_slice()).expect("Could not write to file buffer");
}


// pub fn keep_axes<A, D, NewDim>(a: &ArrayView<A, D>, keep: NewDim) -> Array<A, nd::IxDyn>
//     where A: 'static + Clone + libnum::Float,
//           NewDim: IntoDimension,
//           D: Dimension
// {
//     let mut a = a.to_owned();
//     let keep = keep.into_dimension();
//     let keep = keep.as_array_view();
//     for (i, keep_me) in keep.iter().enumerate() {
//         roll_axis(&mut a, Axis(i), Axis(*keep_me));
//     }
//     // Find number of elements to remove
//     let n_removed = a.shape()
//         .iter()
//         .skip(keep.len())
//         .fold(1, |acc, &val| val * acc);
//     // Figure out shape where all the bins to merge are in the last dim
//     let mut tmp_shape = a.shape()
//         .iter()
//         .take(keep.len())
//         .cloned()
//         .collect::<Vec<usize>>();
//     tmp_shape.push(n_removed);
//     // reshape a such that all the dimensions to be merged are on the last axis
//     let a = a.into_shape(tmp_shape.as_slice())
//         .expect("Invalid reshaping");
//     a.nanmean(nd::Axis(keep.len()))
// }

// pub fn roll_axis<A, S, D>(mut a: &mut ArrayBase<S, D>, to: Axis, from: Axis)
//     where S: Data<Elem = A>,
//           D: Dimension
// {
//     let i = to.index();
//     let mut j = from.index();
//     if j > i {
//         while i != j {
//             a.swap_axes(i, j);
//             j -= 1;
//         }
//     } else {
//         while i != j {
//             a.swap_axes(i, j);
//             j += 1;
//         }
//     }
// }


// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn keep_axes_test() {
//         let a = nd::arr2(&[[5.0, 0.0]]);
//         assert_eq!(keep_axes(&a.view(), [0]).shape(), &[1]);
//         // shape [1, 3, 2]
//         let a = nd::arr3(&[[[5.0, 0.0], [5.0, 0.0], [5.0, 0.0]]]);
//         assert_eq!(keep_axes(&a.view(), [0, 2]).shape(), &[1, 2]);
//         assert_eq!(keep_axes(&a.view(), [1, 2]).shape(), &[3, 2]);
//     }
// }