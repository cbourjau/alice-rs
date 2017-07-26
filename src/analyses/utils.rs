use ndarray as nd;
use ndarray::{Axis};

pub fn nanmean<D>(a: &nd::Array<f64, D>, axis: usize) -> nd::Array<f64, D::Smaller>
    where D: nd::RemoveAxis
{
    let mut mask: nd::Array<f64, nd::Dim<nd::IxDynImpl>> = nd::Array::zeros(a.shape());
    let mut a_fixed = a.clone();
    for (v, m) in a_fixed.iter_mut().zip(mask.iter_mut()) {
        if !v.is_nan() {
            *m = 1f64;
        } else { // Set to 0 if nan
            *v = 0f64;
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
        let a = nd::arr1(&[0.0, NAN]);
        assert_eq!(nanmean(&a, 0), nd::arr0::<f64>(0.0));

        let a = nd::arr1(&[2.0, NAN, 0.0]);
        assert_eq!(nanmean(&a, 0), nd::arr0::<f64>(1.0));

        let a = nd::arr1(&[NAN]);
        assert!(nanmean(&a, 0)[[]].is_nan());

        let a = nd::arr2(&[[5.0, NAN], [5.0, NAN]]);
        assert_eq!(nanmean(&a, 1), nd::arr1::<f64>(&[5.0, 5.0]));
        // This test is not working since we nan != nan
        // let a = nd::arr2(&[[5.0, NAN], [5.0, NAN]]);
        // assert_eq!(nanmean(a, 0), nd::arr1::<f64>(&[5.0, NAN]));
    }
}
