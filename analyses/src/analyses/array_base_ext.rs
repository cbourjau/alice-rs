use ndarray::{Axis, ArrayBase, Array, Data, DataMut, Dimension, RemoveAxis};
use rustfft::{self, FFTplanner};
use rustfft::num_complex::Complex;
use rustfft::num_traits::Zero;

use libnum;

use std::fmt::Debug;

pub trait ArrayBaseExt<A, S, D>
    // where A: Clone + rustfft::FFTnum,
    //       S: Data<Elem=A>,
    //       D: Dimension
{
    fn decompose (&self, axis: Axis) -> Array<Complex<A>, D>
        where A: Clone + rustfft::FFTnum + Debug;
    fn nanmean(&self, axis: Axis) -> Array<A, D::Smaller>
    where A: libnum::Float,
          S: DataMut<Elem=A>,
          D: RemoveAxis + Dimension;
}

impl<A, S, D> ArrayBaseExt<A, S, D> for ArrayBase<S, D>
    where S: Data<Elem=A>,
          D: Dimension
{
    fn decompose(&self, axis: Axis) ->  Array<Complex<A>, D>
        where A: Clone + rustfft::FFTnum + Debug
    {
        // Prepare the output array
        let mut out = self.clone().mapv(|_| Complex::<A>::zero());

        // Prepare 1D output array for fft
        let out_len = self.shape()[axis.index()];
        let mut output: Vec<Complex<A>> = vec![Zero::zero(); out_len];
        let mut planner = FFTplanner::new(false);
        let fft = planner.plan_fft(out_len);
        
        for (mut out_lane, in_lane) in out.lanes_mut(axis).into_iter()
            .zip(self.lanes(axis)) {
                let mut input: Vec<Complex<A>>= in_lane.to_vec().iter()
                    .map(|v| Complex::new(*v, A::zero()))
                    .collect();
                fft.process(&mut input, &mut output);
                for (i, el) in out_lane.iter_mut().enumerate() {
                    *el = output[i];
                }
            }
        out
    }

    fn nanmean(&self, axis: Axis) -> Array<A, D::Smaller>
        where A: libnum::Float,
              D: RemoveAxis
    {
        // Create a mask of the same shape as `a` and set it all to zero
        let mask = self.mapv(|val| match val.is_nan() {
            true => A::zero(),
            false => A::one(),
        });
        let fixed_invalid = self.mapv(|val| match val.is_nan() {
            true => A::zero(),
            false => val,
        });
        let sum = fixed_invalid.sum_axis(axis);
        sum / &mask.sum_axis(axis)
    }
}


#[cfg(test)]
mod tests {
    use ndarray as nd;
    use super::*;
    use std::f64::NAN;
    #[test]
    fn nanmean_tests() {
        let a = nd::arr2(&[[0.0, NAN]]);
        assert_eq!(a.nanmean(Axis(1)), nd::arr1::<f64>(&[0.0]));

        let a = nd::arr2(&[[2.0, NAN, 0.0]]);
        assert_eq!(a.nanmean(Axis(1)), nd::arr1::<f64>(&[1.0]));

        let a = nd::arr2(&[[NAN]]);
        assert!(a.nanmean(Axis(1))[[0]].is_nan());

        let a = nd::arr2(&[[5.0, NAN], [5.0, NAN]]);
        assert_eq!(a.nanmean(Axis(1)), nd::arr1::<f64>(&[5.0, 5.0]));
    }
    #[test]
    fn decompose_tests() {
        let a = nd::Array3::<f64>::zeros((2,3,4));
        let a = a.decompose(nd::Axis(1));
        assert_eq!(a.shape(), [2, 3, 4]);
    }
}
