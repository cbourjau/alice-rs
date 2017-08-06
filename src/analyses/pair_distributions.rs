use std::f64::consts::PI;
use std::f64::INFINITY;

use rustfft::FFTplanner;
use rustfft::num_complex::Complex;
use rustfft::num_traits::Zero;

use ndarray as nd;
use gnuplot::{Figure, AxesCommon, Auto, Fix, ContourStyle};

use histogram::*;

use alice::event::Event;
use alice::track::Track;

use super::{ProcessEvent, Visualize};
use super::nanmean;


pub struct ParticlePairDistributions {
    singles: Histogram<Ix4>,
    pub pairs: Histogram<Ix6>,
    event_counter: Histogram<Ix2>,
}

impl ParticlePairDistributions {
    pub fn new() -> ParticlePairDistributions {
        // eta, phi, z
        let nphi = 36;
        let neta = 16;
        let (nzvtx, zmin, zmax) = (8, -8., 8.);
        let multiplicity_edges = vec![500., 1000., 2000., INFINITY];
        ParticlePairDistributions {
            singles: HistogramBuilder::<Ix4>::new()
                .add_equal_width_axis(neta, -0.8, 0.8)
                .add_equal_width_axis(nphi, 0., 2. * PI)
                .add_equal_width_axis(nzvtx, zmin, zmax)
                .add_variable_width_axis(&multiplicity_edges)
                .build().expect("Error building histogram"),
            pairs: HistogramBuilder::<Ix6>::new()
                .add_equal_width_axis(neta, -0.8, 0.8)
                .add_equal_width_axis(neta, -0.8, 0.8)
                .add_equal_width_axis(nphi, 0., 2. * PI)
                .add_equal_width_axis(nphi, 0., 2. * PI)
                .add_equal_width_axis(nzvtx, zmin, zmax)
                .add_variable_width_axis(&multiplicity_edges)
                .build().expect("Error building histogram"),
            event_counter: HistogramBuilder::<Ix2>::new()
                .add_equal_width_axis(nzvtx, zmin, zmax)
                .add_variable_width_axis(&multiplicity_edges)
                .build().expect("Error building histogram"),
        }
    }

    pub fn finalize(&self) -> nd::Array<f64, nd::Dim<[usize; 6]>> {
        let shape = self.singles.counts.shape();
        let (neta, nphi, nzvtx, nmult) = (shape[0], shape[1], shape[2], shape[3]);
        let phiphi = (&self.singles
                          .counts
                          .to_owned()
                          .into_shape((1, neta, 1, nphi, nzvtx, nmult))
                          .expect("Can't reshape")
                          .broadcast((neta, neta, nphi, nphi, nzvtx, nmult))
                          .expect("Can't broadcast")) *
                     (&self.singles
                          .counts
                          .to_owned()
                          .into_shape((neta, 1, nphi, 1, nzvtx, nmult))
                          .expect("Can't reshape"));

        // * 2, since the folded single distributions are a "double count"
        &self.pairs.counts / &phiphi * &self.event_counter.counts * 2.0
    }

    /// Get the uncertainties on the dphi projection as shape (dphi, multiplicity)
    fn get_uncert_dphi(&self) -> nd::Array<f64, nd::Dim<[usize; 2]>> {
        let shape = self.singles.counts.shape();
        let (neta, nphi, nzvtx, nmult) = (shape[0], shape[1], shape[2], shape[3]);
        let ss = (&self.singles
                  .counts
                  .to_owned()
                  .into_shape((1, neta, 1, nphi, nzvtx, nmult))
                  .expect("Can't reshape")
                  .broadcast((neta, neta, nphi, nphi, nzvtx, nmult))
                  .expect("Can't broadcast")) *
            (&self.singles
             .counts
             .to_owned()
             .into_shape((neta, 1, nphi, 1, nzvtx, nmult))
             .expect("Can't reshape"));
        let ss_sum = ss.sum(Axis(4)).sum(Axis(0)).sum(Axis(0));
        let ss_sum = roll_diagonal(&ss_sum);
        // / 2, since the folded single distributions are a "double count"
        let ss_sum = ss_sum.sum(Axis(0)) / 2.0;

        let p_sum = self.pairs.counts.sum(Axis(4)).sum(Axis(0)).sum(Axis(0));
        let p_sum = roll_diagonal(&p_sum);
        let p_sum = p_sum.sum(Axis(0));

        p_sum.mapv(|v| v.powf(0.5)) / ss_sum
    }
}

impl ProcessEvent for ParticlePairDistributions {
    fn process_event(&mut self, sel_event: &Event, sel_tracks: &[&Track]) {
        // Fill only if we have a valid z-vtx position
        let multiplicity = sel_event.multiplicity as f64;
        if let Some(ref pv) = sel_event.primary_vertex {
            self.singles
                .extend(sel_tracks
                        .iter()
                        .map(|tr| [tr.eta(), tr.phi(), pv.z, multiplicity]));
            self.event_counter.fill(&[pv.z, multiplicity]);

            // Convert to indices before the nested loop; relies on
            // the fact that the hist is square in eta-eta and phi-phi
            // plane!
            // Sure, I should compute the z_vtx-index and mult-index outside of the loop,
            // But then I need to treat their Options as well
            let trk_indices: Vec<Vec<usize>> =
                sel_tracks
                .iter()
                .filter_map(|tr|
                            [self.pairs.find_bin_index_axis(0, tr.eta()),
                             self.pairs.find_bin_index_axis(2, tr.phi()),
                             self.pairs.find_bin_index_axis(4, pv.z),
                             self.pairs.find_bin_index_axis(5, multiplicity),]
                            .into_iter()
                            .cloned()
                            .collect::<Option<Vec<usize>>>()
                ).collect();
            let trk_indices = trk_indices.as_slice();
            let pair_idxs = trk_indices
                .iter()
                .enumerate()
                .flat_map(move |(i1, tr1)| {
                    trk_indices
                        .iter()
                        .enumerate()
                        .take_while(move |&(i2, _)| i1 > i2)
                        .map(move |(_, tr2)| {
                            [tr1[0], tr2[0],
                             tr1[1], tr2[1],
                             tr1[2], tr1[3]]
                        })
                });
            for idxs in pair_idxs.into_iter() {
                self.pairs.fill_by_index(&idxs);
            }
        };
    }
}

impl Visualize for ParticlePairDistributions {
    fn visualize(&self) {
        let corr2 = self.finalize();

        let mut fg = Figure::new();

        fg.axes3d()
            .set_pos_grid(1, 2, 0)
            .set_title("eta eta", &[])
            .set_x_label("eta1", &[])
            .set_y_label("eta2", &[])
        // __average__ over z, phi1, phi2 (should be all at once, actually)!
            .surface(&nanmean(&nanmean(&nanmean(&corr2, 4), 2), 2),
                     corr2.shape()[0],
                     corr2.shape()[1],
                     Some((-0.8, -0.8, 0.8, 0.8)), &[])
            .show_contours(true, false, ContourStyle::Spline(2,2), Auto, Auto);

        // __average__ over z, eta1, eta2 (should be all at once, actually)!
        let phi_phi = get_phi_phi(&corr2);
        fg.axes3d()
            .set_pos_grid(1, 2, 1)
            .set_title("phi phi", &[])
            .set_x_label("phi1", &[])
            .set_y_label("phi2", &[])
            .surface(&phi_phi,
                     corr2.shape()[2],
                     corr2.shape()[3],
                     Some((0., 0., 2.*PI, 2.*PI)), &[])
            .show_contours(true, false, ContourStyle::Spline(2,2), Auto, Auto)
            .set_x_range(Auto, Fix(2.*PI))
            .set_y_range(Auto, Fix(2.*PI));
        fg.show();

        let mut fg = Figure::new();
        // transform coordinates (rotate 45 degrees)
        let phi_delta_phi_tilde = roll_diagonal(&phi_phi);
        fg.axes3d()
            .set_pos_grid(1, 2, 0)
            .set_title("Dphi Tphi", &[])
            .set_x_label("Dphi", &[])
            .set_y_label("Tphi", &[])
            .surface(&phi_delta_phi_tilde,
                   corr2.shape()[2],
                   corr2.shape()[3],
                     Some((0., 0., 2.*PI, 2.*PI)), &[])
            .show_contours(true, false, ContourStyle::Spline(2,2), Auto, Auto)
            .set_x_range(Auto, Fix(2.*PI))
            .set_y_range(Auto, Fix(2.*PI));

        let phi_uncert = self.get_uncert_dphi();
        fg.axes2d()
            .set_pos_grid(1, 2, 1)
            .set_title("Dphi Projection", &[])
            .set_x_label("Dphi", &[])
            .y_error_lines(&self.pairs.centers(2),
                           // average over phi_tilde
                           &nanmean(&phi_delta_phi_tilde, 0),
                           &phi_uncert,
                           &[]);
        fg.show();
        let output = fourier_decompose(&nanmean(&phi_delta_phi_tilde, 0));
        println!("amp: {:?}", output.iter().map(|v| v.to_polar().0).collect::<Vec<_>>());
        println!("phase: {:?}", output.iter().map(|v| v.to_polar().1).collect::<Vec<_>>());
    }
}

fn roll_diagonal<D>(a: &nd::Array<f64, D>) -> nd::Array<f64, D>
    where D: nd::Dimension + nd::RemoveAxis
{
    let mut b = a.clone();
    for (i, mut row) in b.outer_iter_mut().enumerate() {
        for _ in 0..i {
            roll_by_one(&mut row);
        }
    }
    b
}

fn roll_by_one<'a, D>(a: &mut nd::ArrayViewMut<'a, f64, D>)
    where D: nd::Dimension
{
    use std::mem;
    for mut lane in a.lanes_mut(nd::Axis(0)) {
        let len = lane.len();
        let mut last = lane[len - 1];
        for elt in lane.slice_mut(s![..-1;-1]) {
            last = mem::replace(elt, last);
        }
        lane[len - 1] = last;
    }
}

/// Average the given array of shape (eta, eta, phi, phi, z, mult) to
/// (phi, phi, mult)
fn get_phi_phi(a: &nd::Array6<f64>) -> nd::Array3<f64> {
    nanmean(&nanmean(&nanmean(a, 4), 0), 0)
}

/// Fourier decompose a 1D distribution
fn fourier_decompose(a: &nd::Array1<f64>) -> Vec<Complex<f64>> {
    let mut input: Vec<Complex<f64>> = a
        .to_vec()
        .iter()
        .map(|v| Complex::new(*v, 0.0))
        .collect();
    let mut output: Vec<Complex<f64>> = vec![Zero::zero(); a.len()];
    let mut planner = FFTplanner::new(false);
    let fft = planner.plan_fft(a.len());
    fft.process(&mut input, &mut output);
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn fourier() {
    //     let n = 5.;
    //     let a = nd::Array1::<f64>::range(0., 2. * PI, 2.0 * PI / n)
    //         .mapv(|v| v.cos());

    //     let output = fourier_decompose(&a);
    //     println!("amp: {:?}", output.iter().map(|v| v.to_polar().0).collect::<Vec<_>>());
    //     println!("phase: {:?}", output.iter().map(|v| v.to_polar().1).collect::<Vec<_>>());
    // }

    #[test]
    fn test_roll_by_one() {
        let mut a = nd::arr1(&[1.,2.,3.]);
        roll_by_one(&mut a.view_mut());
        assert_eq!(a, nd::arr1(&[2.,3.,1.]))
    }

    #[test]
    fn roll_2d_by_one() {
        let mut a = nd::Array2::<f64>::zeros((3, 3)); // nd::Array::linspace(0., 9., 9).into_shape((3, 3));
        for (i, el) in a.iter_mut().enumerate() {
            *el = i as f64;
        }
        for (i, mut row) in a.outer_iter_mut().enumerate() {
            for _ in 0..i {
                roll_by_one(&mut row);
            }
        }
        let res: nd::Array2<f64> = nd::arr2(&[[0., 1., 2.],
                                              [4., 5., 3.],
                                              [8., 6., 7.]]);
        assert_eq!(a, res);
    }

    #[test]
    fn test_roll_diagonal() {
        let mut a = nd::Array2::<f64>::zeros((3, 3));
        for (i, el) in a.iter_mut().enumerate() {
            *el = i as f64;
        }
        let a = a.into_shape((3, 3, 1)).unwrap();
        let rolled = roll_diagonal(&a);
        let res = nd::arr2(&[[0., 1., 2.],
                              [4., 5., 3.],
                              [8., 6., 7.]]).into_shape((3, 3, 1)).unwrap();
        assert_eq!(rolled, res);
    }
}
