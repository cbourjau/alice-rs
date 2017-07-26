use std::f64::consts::PI;
use ndarray as nd;
use gnuplot::{Figure, AxesCommon, Auto, Fix, ContourStyle};

use histogram::{Histogram, Dim};

use alice::event::Event;
use alice::track::Track;

use super::{ProcessEvent, Visualize};
use super::nanmean;


pub struct ParticlePairDistributions {
    singles: Histogram<Dim<[usize; 3]>>,
    pub pairs: Histogram<Dim<[usize; 5]>>,
    event_counter: Histogram<Dim<[usize; 1]>>,
}

impl ParticlePairDistributions {
    pub fn new() -> ParticlePairDistributions {
        // eta, phi, z
        let nphi = 20;
        let neta = 16;
        let (nzvtx, zmin, zmax) = (8, -8., 8.);
        ParticlePairDistributions {
            singles: Histogram::new((neta, nphi, nzvtx),
                                    &[-0.8, 0., zmin],
                                    &[0.8, 2. * PI, zmax]),
            pairs: Histogram::new((neta, neta, nphi, nphi, nzvtx),
                                  &[-0.8, -0.8, 0., 0., zmin],
                                  &[0.8, 0.8, 2. * PI, 2. * PI, zmax]),
            event_counter: Histogram::new((nzvtx,), &[zmin], &[zmax]),
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
            for i_t1 in 0..sel_tracks.len() {
                for i_t2 in 0..sel_tracks.len() {
                    if i_t1 >= i_t2 {
                        continue;
                    }
                    let (lead, sublead) =
                        match sel_tracks[i_t1].pt() >= sel_tracks[i_t2].pt() {
                            true => (sel_tracks[i_t1], sel_tracks[i_t2]),
                            false => (sel_tracks[i_t2], sel_tracks[i_t1]),
                        };
                    self.pairs.fill(&[lead.eta(), sublead.eta(),
                                      lead.phi(), sublead.phi(),
                                      pv.z]);
                }
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
