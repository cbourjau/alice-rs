use std::f64::consts::PI;
use gnuplot::{Figure, AxesCommon};

use histogram::*;

use alice::event::Event;
use alice::track::Track;

use super::{ProcessEvent, Visualize};

pub struct SingleParticleDistributions {
    pub histogram: Histogram<[usize; 3]>
}

impl SingleParticleDistributions {
    pub fn new() -> SingleParticleDistributions {
        // eta, phi, z
        let nphi = 20;
        let neta = 16;
        let (nzvtx, zmin, zmax) = (8, -8., 8.);
        SingleParticleDistributions {
            histogram: HistogramBuilder::<[usize; 3]>::new()
                .add_equal_width_axis(neta, -0.8, 0.8)
                .add_equal_width_axis(nphi, 0., 2. * PI)
                .add_equal_width_axis(nzvtx, zmin, zmax)
                .build().expect("Error building histogram"),
        }
    }
}

impl ProcessEvent for SingleParticleDistributions {
    fn process_event(&mut self, sel_event: &Event, sel_tracks: &[&Track]) {
        // Fill only if we have a valid z-vtx position
        sel_event.primary_vertex.as_ref().map(|pv| {
            self.histogram.extend(
                sel_tracks.iter().map(|tr| [tr.eta(), tr.phi(), pv.z])
            );
        });
    }
}

impl Visualize for SingleParticleDistributions {
    fn visualize(&self) {
        let mut fg = Figure::new();

        fg.axes2d()
            .set_pos_grid(1, 2, 0)
            .set_title("η track distribution", &[])
            .set_x_label("η", &[])
            .lines(&self.histogram.centers(0),
                   // Sum over phi and z
                   &self.histogram.counts.sum(Axis(1)).sum(Axis(1)),
                   &[]);

        fg.axes2d()
            .set_pos_grid(1, 2, 1)
            .set_title("phi track distribution", &[])
            .set_x_label("phi", &[])
            .lines(&self.histogram.centers(1),
                   // Sum over eta and z
                   &self.histogram.counts.sum(Axis(2)).sum(Axis(0)),
                   &[]);
        fg.show();
    }
}
