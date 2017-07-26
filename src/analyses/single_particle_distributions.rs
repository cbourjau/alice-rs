use std::f64::consts::PI;
use gnuplot::{Figure, AxesCommon};

use histogram::{Histogram, Dim, Centers, Axis};

use alice::event::Event;
use alice::track::Track;

use super::{ProcessEvent, Visualize};

pub struct SingleParticleDistributions {
    pub histogram: Histogram<Dim<[usize; 3]>>
}

impl SingleParticleDistributions {
    pub fn new() -> SingleParticleDistributions {
        // eta, phi, z
        let (neta, nphi, nz) = (20, 20, 8);
        
        SingleParticleDistributions {
            histogram: Histogram::new((neta, nphi, nz),
                                      &[-1., 0., -8.],
                                      &[1., 2. * PI, 8.])
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
