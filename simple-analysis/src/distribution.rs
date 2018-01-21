/// Measure a bunch of simple distribtions.
///  - Single particle distribution in eta and phi
///  - Distribution of events' primary vertices along the nominal
///    interaction point along beam axis

use std::f64::consts::PI;
use gnuplot::{Figure, AxesCommon};

use histogram::*;

use alice::analysis::traits::{Merge, Visualize};
use alice::track_traits::{Azimuth, Longitude};
use alice::event_traits::{Tracks, PrimaryVertex};

pub struct Distribution {
    pub single_particles: Histogram<f32, [usize; 3]>,
    pub z_vertex: Histogram<i32, [usize; 1]>,
}

impl Distribution {
    pub fn new() -> Distribution {
        // eta, phi, z
        let nphi = 120;
        let neta = 120;
        let (nzvtx, zmin, zmax) = (80, -8., 8.);
        Distribution {
            single_particles: HistogramBuilder::<[usize; 3]>::new()
                .add_equal_width_axis(neta, -0.8, 0.8)
                .add_equal_width_axis(nphi, 0., 2. * PI)
                .add_equal_width_axis(nzvtx, zmin, zmax)
                .build()
                .expect("Error building histogram"),
            z_vertex: HistogramBuilder::<[usize; 1]>::new()
                .add_equal_width_axis(nzvtx, zmin, zmax)
                .build()
                .expect("Error building histogram"),
        }
    }
}

impl Distribution {
    pub fn process_event<E, T>(mut self, event: &E) -> Self
        where E: Tracks<T> + PrimaryVertex,
              T: Azimuth + Longitude
    {
        // Fill only if we have a valid z-vtx position
        if let Some(prime_vertex) = event.primary_vertex() {
            self.single_particles
                .extend(
                    event.tracks().iter().map(|tr| [tr.eta(), tr.phi(), prime_vertex.z]));
            self.z_vertex.fill(&[prime_vertex.z]);
        };
        self
    }
}

impl Merge for Distribution {
    fn merge(&mut self, b: &Self) {
        // We simply add the content of the one histograms in this case.
        // For other analyses, this might be a more complicted operation
        self.single_particles.add(&b.single_particles);
        self.z_vertex.add(&b.z_vertex);
    }
}

impl Visualize for Distribution {
    fn visualize(&self) {
        let mut fg = Figure::new();

        fg.axes2d()
            .set_pos_grid(2, 2, 0)
            .set_title("η track distribution", &[])
            .set_x_label("η", &[])
            .set_y_label("# particles", &[])
            .lines(&self.single_particles.centers(0),
                   // Sum over phi and z
                   &self.single_particles
                       .counts
                       .sum_axis(Axis(1))
                       .sum_axis(Axis(1)),
                   &[]);

        fg.axes2d()
            .set_pos_grid(2, 2, 1)
            .set_title("φ track distribution", &[])
            .set_x_label("φ [rad]", &[])
            .set_y_label("# particles", &[])
            .lines(&self.single_particles.centers(1),
                   // Sum over eta and z
                   &self.single_particles
                       .counts
                       .sum_axis(Axis(2))
                       .sum_axis(Axis(0)),
                   &[]);

        fg.axes2d()
            .set_pos_grid(2, 2, 2)
            .set_title("Primary vertex position", &[])
            .set_x_label("z [cm]", &[])
            .set_y_label("# events", &[])
            .lines(&self.z_vertex.centers(0),
                   // Sum over eta and z
                   &self.z_vertex.counts,
                   &[]);
        fg.show();
    }
}
