use std::f64::consts::PI;
use std::f64::INFINITY;

use ndarray as nd;
use gnuplot as gpl;
use gnuplot::AxesCommon;
use gnuplot::PlotOption::*;

use histogram::*;

use alice::event::Event;

use super::utils::COLORS;

use super::{ProcessEvent, Visualize};
use super::ArrayBaseExt;


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
        let pt_edges = [0.25, 0.5, 0.75, 1.0, 1.5, 2.0, 2.5, 3.0, 4.0];
        let multiplicity_edges = [// 7., 24., 63., 140.,
                                  276.,
                                  510.,
                                  845.,
                                  1325.,
                                  2083.,
                                  INFINITY];
        ParticlePairDistributions {
            singles: HistogramBuilder::<[usize; 5]>::new()
                .add_equal_width_axis(nzvtx, zmin, zmax)
                .add_variable_width_axis(&multiplicity_edges)
                .add_equal_width_axis(neta, -0.8, 0.8)
                .add_equal_width_axis(nphi, 0., 2. * PI)
                .add_variable_width_axis(&pt_edges)
                .build()
                .expect("Error building histogram"),
            pairs: HistogramBuilder::<[usize; 8]>::new()
                .add_equal_width_axis(nzvtx, zmin, zmax)
                .add_variable_width_axis(&multiplicity_edges)
                .add_equal_width_axis(neta, -0.8, 0.8)
                .add_equal_width_axis(neta, -0.8, 0.8)
                .add_equal_width_axis(nphi, 0., 2. * PI)
                .add_equal_width_axis(nphi, 0., 2. * PI)
                .add_variable_width_axis(&pt_edges)
                .add_variable_width_axis(&pt_edges)
                .build()
                .expect("Error building histogram"),
            event_counter: HistogramBuilder::<[usize; 2]>::new()
                .add_equal_width_axis(nzvtx, zmin, zmax)
                .add_variable_width_axis(&multiplicity_edges)
                .build()
                .expect("Error building histogram"),
        }
    }

    pub fn finalize(&self) -> nd::Array<f64, nd::IxDyn> {
        let shape = self.singles.counts.shape();
        let (nzvtx, nmult, neta, nphi, npt) =
            (shape[0], shape[1], shape[2], shape[3], shape[4]);
        let ext_shape1 = [nzvtx, nmult, 1, neta, 1, nphi, 1, npt];
        let ext_shape2 = [nzvtx, nmult, neta, 1, nphi, 1, npt, 1];
        let new_shape = [nzvtx, nmult, neta, neta, nphi, nphi, npt, npt];
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

    /// Get the relative uncertainties on the dphi projection as shape
    /// (dphi, pt, pt, multiplicity) This assumes that the single
    /// particle distrubtions have negligable uncertainties.
    fn get_relative_uncert_dphi(&self) -> nd::Array<f64, nd::IxDyn> {
        let p_sum = self.pairs.counts
            .sum_axis(Axis(2))  // eta1
            .sum_axis(Axis(2))  // eta2
            .sum_axis(Axis(0));  // z_vtx position
        // Coordinate transform: (phi1, phi2) -> ((phi1 + phi2), (phi1 - phi2))
        let p_sum = roll_diagonal(&p_sum);
        // Sum over (phi1 + phi2)
        let p_sum = p_sum.sum_axis(Axis(0));
        // Absolute uncertainties assuming binomila distribution: sqrt(N)
        // Thus, relative: sqrt(N) / N = 1 / sqrt(N)
        p_sum.mapv(|n| 1.0 / n.powf(0.5))
    }

    /// Calculate the relative uncertainties for the Fourier
    /// coefficients. Each coefficient has the same uncertainty!
    /// Variance of Vn is sum of variances of the dimension decomposed; i.e. sum along dphi
    /// Input: pair_dist(dphi, pt, pt, multiplicity) used for decomposition
    /// Return shape: (pt, pt, multiplicity)
    fn get_absolute_uncert_vn(&self, pairs_dist: &nd::Array<f64, nd::IxDyn>)
                             -> nd::Array<f64, nd::IxDyn> {
        let abs_uncert = self.get_relative_uncert_dphi() * pairs_dist;// absolute sigmas
        abs_uncert
            .mapv(|sigma| sigma.powf(2.0)) // var = sigma^2
            .sum_axis(Axis(0))             // Sum over dphi axis
            .mapv(|sigma| sigma.powf(0.5)) // sigma = sqrt{ sum_i {sigma_i^2} }
        
    }
}

impl ProcessEvent for ParticlePairDistributions {
    fn process_event(mut self, event: &Event) -> Self {
        // Fill only if we have a valid z-vtx position
        let multiplicity = event.multiplicity as f64;
        if let Some(ref pv) = event.primary_vertex {
            if let Some(z_idx) = self.pairs.find_bin_index_axis(0, pv.z) {
                if let Some(mult_idx) = self.pairs.find_bin_index_axis(1, multiplicity) {
                    self.singles
                        .extend(event.tracks
                                .iter()
                                .map(|tr| [pv.z, multiplicity, tr.eta(), tr.phi(), tr.pt()  ]));
                    self.event_counter.fill(&[pv.z, multiplicity]);

                    // Convert to indices before the nested loop; relies on
                    // the fact that the hist is square in eta-eta, phi-phi,
                    // and pt-pt plane!

                    // Sort tracks by pt
                    let mut tracks = event.tracks.clone();
                    tracks.sort_by(|tr1, tr2| tr1.pt().partial_cmp(&tr2.pt()).unwrap());
                    let trk_indices: Vec<Vec<usize>> = tracks
                        .iter()
                        .filter_map(|tr| {
                            [self.pairs.find_bin_index_axis(2, tr.eta()),
                             self.pairs.find_bin_index_axis(4, tr.phi()),
                             self.pairs.find_bin_index_axis(6, tr.pt()) ]
                                .into_iter()
                                .cloned()
                                .collect::<Option<Vec<usize>>>()
                        })
                        .collect();
                    let trk_indices = trk_indices.as_slice();
                    let pair_idxs = trk_indices.iter().enumerate().flat_map(move |(i1, tr1)| {
                        trk_indices
                            .iter()
                            .enumerate()
                            .take_while(move |&(i2, _)| i1 > i2)
                            .map(move |(_, tr2)| {
                                [z_idx, mult_idx, tr1[0], tr2[0], tr1[1], tr2[1], tr1[2], tr2[2]]
                            })
                    });
                    for idxs in pair_idxs {
                        self.pairs.fill_by_index::<[usize; 8]>(idxs);
                    }
                };
            };
        };
        self
    }
}

impl Visualize for ParticlePairDistributions {
    fn visualize(&self) {
        let mut fg = gpl::Figure::new();
        // enable LaTeX
        fg.set_terminal("wxt enhanced", "");

        let corr2 = self.finalize();
        // __average__ over z, eta1, eta2 (should be all at once, actually)!
        let phi_phi = get_phi_phi(&corr2);
        // transform coordinates (rotate 45 degrees)
        let phi_delta_phi_tilde = roll_diagonal(&phi_phi);
        let dphi = phi_delta_phi_tilde.nanmean(Axis(0));

        let dphi_uncert = self.get_relative_uncert_dphi();
        {
            let mut dphi_plot = fg.axes2d()
                .set_pos_grid(2, 2, 0)
                .set_title(r"Projection onto Δφ", &[])
                .set_x_label(r"Δφ", &[]);
            for (idx, (dphi, dphi_uncert)) in
                dphi
                .subview(Axis(1), 0)  // pt1
                .subview(Axis(1), 0)  // pt2
                .axis_iter(Axis(1))   // mult
                .zip(dphi_uncert
                     .subview(Axis(1), 0)  // pt1
                     .subview(Axis(1), 0)  // pt2
                     .axis_iter(Axis(1)))  // mult
                .enumerate() {
                let color = Color(COLORS[idx]);
                dphi_plot.y_error_lines(&self.pairs.centers(2),
                                        // average over phi_tilde
                                        &dphi,
                                        &(&dphi * &dphi_uncert),
                                        &[color]);
            }
        }
        // shape: n, pt, pt, mult
        let vndelta = dphi
            .decompose(Axis(0))
            .mapv(|v| v.to_polar().0); // Only keep the amplitude

        let rel_vn_uncert = {
            let abs_vn_uncert = self.get_absolute_uncert_vn(&vndelta);
            &abs_vn_uncert
                .broadcast(vndelta.shape())
                .expect("Could not broadcast uncertainties!")
                / &vndelta
        };

        // We normalize by the isotropic mode 0
        let vndelta = {
            let (v0, vns) = vndelta.view().split_at(Axis(0), 1);
            &vns / &v0
        };
        // Add relative uncertainties to reflect the normalization;
        // convert to absolute uncertainties
        let abs_vn_uncert = {
            let (v0_uncert, vns_uncert) = rel_vn_uncert.view().split_at(Axis(0), 1);
            (&vns_uncert + &v0_uncert) * &vndelta
        };

        // Plot Vn as a function of n
        {
            let mut vn_plot =
                fg.axes2d()
                    .set_pos_grid(2, 2, 1)
                    .set_title("Fourier modes", &[])
                    .set_x_label("Mode n", &[])
                    .set_y_label("V_{n}", &[])
                    .set_grid_options(true, &[LineStyle(gpl::DotDotDash), Color("black")]);
            for (idx, (vn, uncert)) in vndelta
                .subview(Axis(1), 0)   // pt1
                .subview(Axis(1), 0)   // pt2
                .axis_iter(Axis(1))   // mulitiplicity
                .zip(abs_vn_uncert
                     .subview(Axis(1), 0)  // pt1
                     .subview(Axis(1), 0)  // pt2
                     .axis_iter(Axis(1)))  // multiplicity
                .into_iter()
                .enumerate() {
                let color = gpl::PlotOption::Color(COLORS[idx]);
                vn_plot.y_error_lines((1..5),
                                      &vn.slice(s![..5]),
                                      &uncert.slice(s![..5]),
                                      &[color,
                                        gpl::PlotOption::PointSymbol('S'),
                                        gpl::PlotOption::LineWidth(0.0)]);
            }
        }

        // Plot Vn as function of pT^t for fixed mult
        {
            let mut vn_plot =
                fg.axes2d()
                .set_pos_grid(2, 2, 2)
                .set_title("pt n=2", &[])
                .set_x_label("pT", &[])
                .set_y_label("V_{2}", &[])
                .set_grid_options(true, &[LineStyle(gpl::DotDotDash), Color("black")]);

            let high_mult_bin = vndelta.shape()[3] - 1;   // mult (last bin)
            let pt_t_bin = 3;  // [1.5, 2[
            let mode_n_bin = 2 - 1; // starts at 1; 1 = 2nd mode
            for (idx, (vn, uncert)) in vndelta
                // Select n=2 (bin 1)
                .subview(Axis(3), high_mult_bin)
                .subview(Axis(0), mode_n_bin) // n
                .subview(Axis(0), pt_t_bin)   // pT^t
                .lanes(Axis(0))               // pT^a
                .into_iter()
                .zip(abs_vn_uncert
                     .subview(Axis(3), high_mult_bin)  // mult
                     .subview(Axis(0), mode_n_bin)  // n
                     .subview(Axis(0), pt_t_bin)    // pT^t
                     .lanes(Axis(0)))               // pT^a
                .into_iter()
                .enumerate() {
                let color = gpl::PlotOption::Color(COLORS[idx]);
                vn_plot.y_error_lines(self.pairs.centers(4),
                                      &vn,
                                      &uncert,
                                      &[color, gpl::PlotOption::PointSymbol('S')]);
            }
        }
        {
            let mut vn_plot = fg.axes2d().set_pos_grid(2, 2, 3);
            vn_plot.label("asdf",
                          gpl::Coordinate::Graph(0.5),
                          gpl::Coordinate::Graph(0.5),
                          &[gpl::LabelOption::TextColor("black"),
                            gpl::LabelOption::MarkerSymbol('S')]);

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
/// (phi, phi, pt, pt, mult)
fn get_phi_phi(a: &nd::ArrayD<f64>) -> nd::ArrayD<f64> {
    a.nanmean(Axis(6)).nanmean(Axis(0)).nanmean(Axis(0))
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
        let mut a = nd::arr1(&[1., 2., 3.]);
        roll_by_one(&mut a.view_mut());
        assert_eq!(a, nd::arr1(&[2., 3., 1.]))
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
        let res = nd::arr2(&[[0., 1., 2.], [4., 5., 3.], [8., 6., 7.]])
            .into_shape((3, 3, 1))
            .unwrap();
        assert_eq!(rolled, res);
    }
}
