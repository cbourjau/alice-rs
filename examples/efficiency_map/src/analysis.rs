/// Measure a bunch of simple distribtions.
///  - Single particle distribution in eta and phi
///  - Distribution of events' primary vertices along the nominal
///    interaction point along beam axis

use malice::Event;
use malice::default_track_filter;
use failure::Error;
use std::f64::consts::PI;

use histogram::*;

pub struct Analysis {
    pub single_particles: Histogram<f32, [usize; 3]>,
}

impl Analysis {
    pub fn new() -> Analysis {
        // z, eta, phi
        let nphi = 72;
        let neta = 40;
        let (nzvtx, zmin, zmax) = (10, -10., 10.);
        Analysis {
            single_particles: HistogramBuilder::<[usize; 3]>::new()
                .add_equal_width_axis(nzvtx, zmin, zmax)
                .add_equal_width_axis(neta, -0.8, 0.8)
                .add_equal_width_axis(nphi, 0., 2. * PI)
                .build()
                .expect("Error building histogram"),
        }
    }

    pub fn process_event(mut self, event: &Event) -> Self
    {
        // Fill only if we have a valid primary vertex
        if let Some(prime_vtx) = event.primary_vertex() {
            if event.tracks()
                .filter(|tr| default_track_filter(&tr, &prime_vtx))
                // Only use events in the top 30% multiplicity class
                .count() < 751 {
                    return self;
                }
            self.single_particles
                .extend(
                    event.tracks()
                        .filter(|tr| default_track_filter(&tr, &prime_vtx))
                        .map(|tr| [prime_vtx.z as f64, tr.eta() as f64, tr.phi() as f64]));
        };
        self
    }

    pub fn write_to_disc(&self) -> Result<(), Error> {
        self.single_particles.dump_to_file("singles.hist")?;
        Ok(())
    }    
}
