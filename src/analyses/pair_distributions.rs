use std::f64::consts::PI;

use ndarray as nd;
use gnuplot::{Figure, AxesCommon, Auto, Fix, ContourStyle};

use histogram::*;

use alice::event::Event;
use alice::track::Track;

use super::{ProcessEvent, Visualize};
use super::nanmean;


pub struct ParticlePairDistributions {
    singles: Histogram<Ix3>,
    pub pairs: Histogram<Ix5>,
    event_counter: Histogram<Ix1>,
}

impl ParticlePairDistributions {
    pub fn new() -> ParticlePairDistributions {
        // eta, phi, z
        let nphi = 20;
        let neta = 16;
        let (nzvtx, zmin, zmax) = (8, -8., 8.);
        ParticlePairDistributions {
            singles: HistogramBuilder::<Ix3>::new()
                .add_equal_width_axis(neta, -0.8, 0.8)
                .add_equal_width_axis(nphi, 0., 2. * PI)
                .add_equal_width_axis(nzvtx, zmin, zmax)
                .build().expect("Error building histogram"),
            pairs: HistogramBuilder::<Ix5>::new()
                .add_equal_width_axis(neta, -0.8, 0.8)
                .add_equal_width_axis(neta, -0.8, 0.8)
                .add_equal_width_axis(nphi, 0., 2. * PI)
                .add_equal_width_axis(nphi, 0., 2. * PI)
                .add_equal_width_axis(nzvtx, zmin, zmax)
                .build().expect("Error building histogram"),
            event_counter: HistogramBuilder::<Ix1>::new()
                .add_equal_width_axis(nzvtx, zmin, zmax)
                .build().expect("Error building histogram"),
        }
    }

    pub fn finalize(&self) -> nd::Array<f64, nd::Dim<[usize; 5]>> {
        let nphi = 20;
        let neta = 16;
        let nzvtx = 8;
        let phiphi = (&self.singles
                          .counts
                          .to_owned()
                          .into_shape((1, neta, 1, nphi, nzvtx))
                          .expect("Can't reshape")
                          .broadcast((neta, neta, nphi, nphi, nzvtx))
                          .expect("Can't broadcast")) *
                     (&self.singles
                          .counts
                          .to_owned()
                          .into_shape((neta, 1, nphi, 1, nzvtx))
                          .expect("Can't reshape"));

        // * 2, since the folded single distributions are a "double count"
        &self.pairs.counts / &phiphi * &self.event_counter.counts * 2.0
    }
}

impl ProcessEvent for ParticlePairDistributions {
    fn process_event(&mut self, sel_event: &Event, sel_tracks: &[&Track]) {
        // Fill only if we have a valid z-vtx position
        if let Some(ref pv) = sel_event.primary_vertex {
            self.singles
                .extend(sel_tracks.iter().map(|tr| [tr.eta(), tr.phi(), pv.z]));
            self.event_counter.fill(&[pv.z]);

            // Convert to indices before the nested loop; relies on
            // the fact that the hist is square in eta-eta and phi-phi
            // plane!
            let trk_indices: Vec<Vec<usize>> =
                sel_tracks
                .iter()
                .filter_map(|tr|
                            [self.pairs.find_bin_index_axis(0, tr.eta()),
                             self.pairs.find_bin_index_axis(2, tr.phi()),
                             self.pairs.find_bin_index_axis(4, pv.z),]
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
                             tr1[2]]
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

        fg.axes3d()
            .set_pos_grid(1, 2, 1)
            .set_title("phi phi", &[])
            .set_x_label("phi1", &[])
            .set_y_label("phi2", &[])
        // __average__ over z, eta1, eta2 (should be all at once, actually)!
            .surface(&nanmean(&nanmean(&nanmean(&corr2, 4), 0), 0),
                   corr2.shape()[2],
                   corr2.shape()[3],
                     Some((0., 0., 2.*PI, 2.*PI)), &[])
            .show_contours(true, false, ContourStyle::Spline(2,2), Auto, Auto)
            .set_x_range(Auto, Fix(2.*PI))
            .set_y_range(Auto, Fix(2.*PI));
        fg.show();
    }
}
