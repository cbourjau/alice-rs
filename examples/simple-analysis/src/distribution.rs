/// Measure a bunch of simple distribtions.
///  - Single particle distribution in eta and phi
///  - Distribution of events' primary vertices along the nominal
///    interaction point along beam axis

use std::f64::consts::PI;
use gnuplot::{Figure, AxesCommon};
use malice::Event;
use malice::default_track_filter;
use failure::Error;

use histogram::*;

pub struct SimpleAnalysis {
    pub single_particles: Histogram<f32, [usize; 3]>,
    pub z_vertex: Histogram<i32, [usize; 1]>,
    pub multiplicity: Histogram<f32, [usize; 1]>,
}

impl SimpleAnalysis {
    pub fn new() -> SimpleAnalysis {
        // eta, phi, z
        let nphi = 120;
        let neta = 120;
        let nmult = 3000;
        let (nzvtx, zmin, zmax) = (100, -10., 10.);
        SimpleAnalysis {
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
            multiplicity: HistogramBuilder::<[usize; 1]>::new()
                .add_equal_width_axis(nmult, 0.0, nmult as f64)
                .build()
                .expect("Error building histogram"),            
        }
    }
}


impl SimpleAnalysis {
    pub fn process_event(mut self, event: &Event) -> Self
    {
        // Fill only if we have a valid primary vertex
        if let Some(prime_vtx) = event.primary_vertex() {
            self.single_particles
                .extend(
                    event.tracks()
                        .filter(|tr| default_track_filter(&tr, &prime_vtx))
                        .map(|tr| [tr.eta() as f64, tr.phi() as f64, prime_vtx.z as f64]));
            self.z_vertex.fill(&[prime_vtx.z as f64]);
            self.multiplicity.fill(&[event
                                     .tracks()
                                     .filter(|tr| default_track_filter(&tr, &prime_vtx))
                                     .count() as f64]);
        };
        self
    }
    pub fn write_to_disc(&self) -> Result<(), Error> {
        self.single_particles.dump_to_file("hybrid")?;
        self.z_vertex.dump_to_file("z_pos")?;
        Ok(())
    }    
}

impl SimpleAnalysis {
    pub fn visualize(&self) {
        let mut fg = Figure::new();
        let eta_bin_width = self.single_particles.widths(0)[0] as f32;
        fg.axes2d()
            .set_pos_grid(2, 2, 0)
            .set_title("η track distribution", &[])
            .set_x_label("η", &[])
            .set_y_label("<dN_{ch} / dη >_{event}", &[])
            .boxes(&self.single_particles.centers(0),
                   // Sum over phi and z
                   (&self.single_particles
                    .counts
                    .sum_axis(Axis(1))
                    .sum_axis(Axis(1))
                    / self.z_vertex.counts.scalar_sum() as f32
                    / eta_bin_width
                   ).view(), 
                   &[]);

        let phi_bin_width = self.single_particles.widths(2)[0] as f32;
        fg.axes2d()
            .set_pos_grid(2, 2, 1)
            .set_title("φ track distribution", &[])
            .set_x_label("φ [rad]", &[])
            .set_y_label("<dN_{ch} / dφ >_{event}", &[])
            .boxes(&self.single_particles.centers(1),
                   // Sum over eta and z
                   (&self.single_particles
                    .counts
                    .sum_axis(Axis(2))
                    .sum_axis(Axis(0))
                    / self.z_vertex.counts.scalar_sum() as f32
                    / phi_bin_width
                   ).view(), 
                   &[]);

        fg.axes2d()
            .set_pos_grid(2, 2, 2)
            .set_title("Primary vertex position", &[])
            .set_x_label("z [cm]", &[])
            .set_y_label("# events", &[])
            .boxes(&self.z_vertex.centers(0),
                   &self.z_vertex.counts,
                   &[]);

        fg.axes2d()
            .set_pos_grid(2, 2, 3)
            .set_title("N_{ch} distribution", &[])
            .set_x_label("N_{ch}", &[])
            .set_y_label("# events", &[])
            // .set_x_log(Some(10.0))
            .set_y_log(Some(10.0))
            .boxes(&self.multiplicity.centers(0),
                   &self.multiplicity.counts,
                   &[]);
        fg.show();
        let tot = self.multiplicity.counts.scalar_sum();
        let cum: Vec<_> = self.multiplicity.counts
            .iter()
            .scan(0.0, |state, el| {
                *state += el;
                Some((1.0 - *state / tot) * 100.0)
            })
            .enumerate()
            .collect();
        for el in &cum {
            println!("{:?}", el);
        }
    }
}
