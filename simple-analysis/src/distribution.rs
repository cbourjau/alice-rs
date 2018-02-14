/// Measure a bunch of simple distribtions.
///  - Single particle distribution in eta and phi
///  - Distribution of events' primary vertices along the nominal
///    interaction point along beam axis

use std::f64::consts::PI;
use gnuplot::{Figure, AxesCommon};
use malice::event::Event;
use malice::track;

use histogram::*;

// use malice::{Merge};
// use alice::track_traits::{Azimuth, Longitude};
// use alice::event_traits::{Tracks, PrimaryVertex};

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
    pub fn process_event(mut self, event: &Event) -> Self
    {
        // Fill only if we have a valid primary vertex
        if let Some(prime_vtx) = event.primary_vertex() {
            self.single_particles
                .extend(
                    event.tracks()
                    // .filter(|tr| {
                    //     // SPD && ITS_REFIT
                    //     (tr.quality_its.clusters_on_layer.intersects(
                    //         track::SPD_INNER | track::SPD_OUTER)
                    //      & tr.flags.contains(track::ITS_REFIT)) ||
                    //     // !SPD && ITS_REFIT
                    //     (!tr.quality_its.clusters_on_layer.intersects(
                    //         track::SPD_INNER | track::SPD_OUTER)
                    //      & tr.flags.contains(track::ITS_REFIT)) ||
                    //     // !SPD && !ITS_REFIT
                    //     (!tr.quality_its.clusters_on_layer.intersects(
                    //         track::SPD_INNER | track::SPD_OUTER)
                    //      & !tr.flags.contains(track::ITS_REFIT))
                    // })
                        .filter(|tr| tr.flags.contains(track::Flags::ITS_REFIT))
                        .filter(|tr| tr.flags.contains(track::Flags::TPC_REFIT))
                        .filter(|tr| tr.dca_to_point_xy(prime_vtx.x, prime_vtx.y) < 2.4)
                        .filter(|tr| tr.dca_to_point_z(prime_vtx.z) < 3.2)
                        .filter(|tr| tr.eta().abs() < 0.9)

                        .filter(|tr| tr.pt() > 0.15)
                        .filter(|tr| tr.tpc_ncls > 70)
                    // .filter(|tr| tr.quality_tpc.chi2_per_cluster() <= 4.0)
                    // .filter(|tr| tr.quality_its.chi2_per_cluster() <= 36.0)                        
                        .map(|tr| [tr.eta() as f64, tr.phi() as f64, prime_vtx.z as f64]));
            self.z_vertex.fill(&[prime_vtx.z as f64]);
        };
        self
    }
}

// impl Merge for Distribution {
//     fn merge(&mut self, b: &Self) {
//         // We simply add the content of the one histograms in this case.
//         // For other analyses, this might be a more complicted operation
//         self.single_particles.add(&b.single_particles);
//         self.z_vertex.add(&b.z_vertex);
//     }
// }

impl Distribution {
    pub fn visualize(&self) {
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
