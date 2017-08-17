use std::f64::consts::PI;
use std::f64::INFINITY;

use rustfft::FFTplanner;
use rustfft::num_complex::Complex;
use rustfft::num_traits::Zero;

use ndarray as nd;
use gnuplot as gpl;
use gnuplot::AxesCommon;
use gnuplot::PlotOption::*;

use histogram::*;

use alice::event::Event;
use alice::track::Track;

use super::utils::COLORS;

use super::{ProcessEvent, Visualize};
use super::nanmean;


pub struct ParticlePairDistributions {
    singles: Histogram<[usize; 5]>,
    pub pairs: Histogram<[usize; 8]>,
    event_counter: Histogram<[usize; 2]>,
}

impl ParticlePairDistributions {
    pub fn new() -> ParticlePairDistributions {
        // eta, phi, z
        let nphi = 20;
        let neta = 16;
        let (nzvtx, zmin, zmax) = (8, -8., 8.);
        let pt_edges = [0.5, 1.5];
        let multiplicity_edges = [// 7., 24., 63., 140.,
            276., 510., 845., 1325., 2083., INFINITY];
        ParticlePairDistributions {
            singles: HistogramBuilder::<[usize; 5]>::new()
                .add_equal_width_axis(neta, -0.8, 0.8)
                .add_equal_width_axis(nphi, 0., 2. * PI)
                .add_variable_width_axis(&pt_edges)
                .add_equal_width_axis(nzvtx, zmin, zmax)
                .add_variable_width_axis(&multiplicity_edges)
                .build().expect("Error building histogram"),
            pairs: HistogramBuilder::<[usize; 8]>::new()
                .add_equal_width_axis(neta, -0.8, 0.8)
                .add_equal_width_axis(neta, -0.8, 0.8)
                .add_equal_width_axis(nphi, 0., 2. * PI)
                .add_equal_width_axis(nphi, 0., 2. * PI)
                .add_variable_width_axis(&pt_edges)
                .add_variable_width_axis(&pt_edges)
                .add_equal_width_axis(nzvtx, zmin, zmax)
                .add_variable_width_axis(&multiplicity_edges)
                .build().expect("Error building histogram"),
            event_counter: HistogramBuilder::<[usize; 2]>::new()
                .add_equal_width_axis(nzvtx, zmin, zmax)
                .add_variable_width_axis(&multiplicity_edges)
                .build().expect("Error building histogram"),
        }
    }

    pub fn finalize(&self) -> nd::Array<f64, nd::IxDyn> {
        let shape = self.singles.counts.shape();
        let (neta, nphi, npt, nzvtx, nmult) =
            (shape[0], shape[1], shape[2], shape[3], shape[4]);
        let ext_shape1 = [1, neta, 1, nphi, 1, npt, nzvtx, nmult];
        let ext_shape2 = [neta, 1, nphi, 1, npt, 1, nzvtx, nmult];
        let new_shape = [neta, neta, nphi, nphi, npt, npt, nzvtx, nmult];
        let phiphi = (&self.singles
                          .counts
                          .to_owned()
                          .into_shape(ext_shape1.as_ref())
                          .expect("Can't reshape")
                          .broadcast(new_shape.as_ref())
                          .expect("Can't broadcast")) *
                     (&self.singles
                          .counts
                          .to_owned()
                          .into_shape(ext_shape2.as_ref())
                          .expect("Can't reshape"));

        // * 2, since the folded single distributions are a "double count"
        &self.pairs.counts / &phiphi * &self.event_counter.counts * 2.0
    }

    /// Get the relative uncertainties on the dphi projection as shape (dphi, multiplicity)
    /// This assumes that the single particle distrubtions have negligable uncertainties.
    fn get_uncert_dphi(&self) -> nd::Array<f64, nd::IxDyn> {
        let p_sum = self.pairs.counts
            .sum(Axis(4))
            .sum(Axis(4))
            .sum(Axis(4))
            .sum(Axis(0))
            .sum(Axis(0));
        let p_sum = roll_diagonal(&p_sum);
        let p_sum = p_sum.sum(Axis(0));

        p_sum.mapv(|v| v.powf(0.5) / v) //  / ss_sum
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
                        .map(|tr| [tr.eta(), tr.phi(), tr.pt(), pv.z, multiplicity]));
            self.event_counter.fill(&[pv.z, multiplicity]);

            // Convert to indices before the nested loop; relies on
            // the fact that the hist is square in eta-eta, phi-phi,
            // and pt-pt plane!
            // Sure, I should compute the z_vtx-index and mult-index
            // outside of the loop, But then I need to treat their
            // Options as well

            // Sort tracks by pt
            let mut sel_tracks = sel_tracks.to_owned();
            sel_tracks
                .sort_by(|tr1, tr2| {
                    tr1.pt()
                        .partial_cmp(&tr2.pt())
                        .expect("Could not sort")
                });
            let trk_indices: Vec<Vec<usize>> =
                sel_tracks
                .iter()
                .filter_map(|tr|
                            [self.pairs.find_bin_index_axis(0, tr.eta()),
                             self.pairs.find_bin_index_axis(2, tr.phi()),
                             self.pairs.find_bin_index_axis(4, tr.pt()),
                             self.pairs.find_bin_index_axis(6, pv.z),
                             self.pairs.find_bin_index_axis(7, multiplicity),]
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
                             tr1[2], tr2[2],
                             tr1[3], tr1[4]]
                        })
                });
            for idxs in pair_idxs {
                self.pairs.fill_by_index::<[usize; 8]>(idxs);
            }
        };
    }
}

impl Visualize for ParticlePairDistributions {
    fn visualize(&self) {
        let corr2 = self.finalize();
        // let mut fg = gpl::Figure::new();

        // fg.axes3d()
        //     .set_pos_grid(1, 2, 0)
        //     .set_title("eta eta", &[])
        //     .set_x_label("eta1", &[])
        //     .set_y_label("eta2", &[])
        // // __average__ over z, phi1, phi2 (should be all at once, actually)!
        //     .surface(&nanmean(&nanmean(&nanmean(&corr2, 4), 2), 2),
        //              corr2.shape()[0],
        //              corr2.shape()[1],
        //              Some((-0.8, -0.8, 0.8, 0.8)), &[])
        //     .show_contours(true, false, ContourStyle::Spline(2,2), Auto, Auto);

        // __average__ over z, eta1, eta2 (should be all at once, actually)!
        let phi_phi = get_phi_phi(&corr2);
        // fg.axes3d()
        //     .set_pos_grid(1, 2, 1)
        //     .set_title("phi phi", &[])
        //     .set_x_label("phi1", &[])
        //     .set_y_label("phi2", &[])
        //     .surface(&phi_phi,
        //              corr2.shape()[2],
        //              corr2.shape()[3],
        //              Some((0., 0., 2.*PI, 2.*PI)), &[])
        //     .show_contours(true, false, ContourStyle::Spline(2,2), Auto, Auto)
        //     .set_x_range(Auto, Fix(2.*PI))
        //     .set_y_range(Auto, Fix(2.*PI));
        // fg.show();

        let mut fg = gpl::Figure::new();
        // enable LaTex
        fg.set_terminal("wxt enhanced", "");
        // transform coordinates (rotate 45 degrees)
        let phi_delta_phi_tilde = roll_diagonal(&phi_phi);
        println!("shape: {:?}", phi_delta_phi_tilde.shape());

        // fg.axes3d()
        //     .set_pos_grid(1, 2, 0)
        //     .set_title("Dphi Tphi", &[])
        //     .set_x_label("Dphi", &[])
        //     .set_y_label("Tphi", &[])
        //     .surface(&phi_delta_phi_tilde,
        //            corr2.shape()[2],
        //            corr2.shape()[3],
        //              Some((0., 0., 2.*PI, 2.*PI)), &[])
        //     .show_contours(true, false, ContourStyle::Spline(2,2), Auto, Auto)
        //     .set_x_range(Auto, Fix(2.*PI))
        //     .set_y_range(Auto, Fix(2.*PI));
        
        let dphi_mult = nanmean(&phi_delta_phi_tilde, Axis(0));
        let dphi_mult_uncert = self.get_uncert_dphi();
        println!("shape: {:?}", dphi_mult.shape());
        println!("shape uncert: {:?}", dphi_mult_uncert.shape());
        {
            let mut dphi_plot = fg.axes2d()
                .set_pos_grid(1, 2, 0)
                .set_title(r"Projection onto Δφ", &[])
                .set_x_label(r"Δφ", &[]);
            for (idx, (dphi, dphi_uncert)) in dphi_mult
                .axis_iter(Axis(1))
                .zip(dphi_mult_uncert.axis_iter(Axis(1)))
                .enumerate()
            {
                let color = Color(COLORS[idx]);
                dphi_plot.y_error_lines(&self.pairs.centers(2),
                                        // average over phi_tilde
                                        &dphi,
                                        &(&dphi * &dphi_uncert),
                                        &[color]);
            }
        }

        // reduce to dphi vs. mult
        let dphi = &nanmean(&phi_delta_phi_tilde, Axis(0));
        let (nphi, nmult) = (dphi.shape()[0], dphi.shape()[1]);
        let vndelta = nd::Array1::<Complex<f64>>::from_iter(
            dphi.lanes(nd::Axis(0))
                .into_iter()
                .flat_map(|lane| fourier_decompose(&lane)))
            .into_shape([nmult, nphi])
            .expect(&format!("Could not reshape {} into ({}, {})",
                             dphi.len(), nmult, nphi));
        {
            let vndelta = vndelta.t().mapv(|v| v.to_polar().0.sqrt());
            let (v0, vns) = vndelta
                .view()
                .split_at(Axis(0), 1);
            let vndelta = &vns / &v0;
            let mut vn_plot = fg.axes2d()
                .set_pos_grid(1, 2, 1)
                .set_title("Fourier modes", &[])
                .set_x_label("Mode n", &[])
                .set_y_label("v_{n}", &[])
                .set_grid_options(true,
                                  &[LineStyle(gpl::DotDotDash), Color("black")]);
            for (idx, lane) in vndelta.lanes(nd::Axis(0)).into_iter().enumerate() {
                let color = gpl::PlotOption::Color(COLORS[idx]);
                vn_plot.points((1..5),
                               &lane.slice(s![..5]),
                               &[color, gpl::PlotOption::PointSymbol('S')]);
            }
        }
        fg.show();
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

/// Average the given array of shape (eta, eta, phi, phi, pt, pt, z, mult) to
/// (phi, phi, mult)
fn get_phi_phi(a: &nd::ArrayD<f64>) -> nd::ArrayD<f64> {
    nanmean(
        &nanmean(
            &nanmean(
                &nanmean(
                    &nanmean(a, Axis(4))
                        , Axis(4))
                    , Axis(4))
                , Axis(0))
            , Axis(0)
    )
}

/// Fourier decompose along the first axis
fn fourier_decompose(a: &nd::ArrayView1<f64>) -> Vec<Complex<f64>> {
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
