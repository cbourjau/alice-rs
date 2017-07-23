use std::f64::INFINITY;
use std::f64::consts::PI;

use histogram::{Histogram, Dim};

use alice::event::Event;
use alice::track::Track;

use process_event::ProcessEvent;

pub struct PtMultiplicity
{
    pub histogram: Histogram<Dim<[usize; 2]>>,
}

impl PtMultiplicity
{
    pub fn new() -> PtMultiplicity {
        // pt vs mult
        let mut h = Histogram::new((20, 2), &[0., 0.], &[4., 1.]);
        // Overwrite edges of mult dimension
        h.overwrite_edges(1, vec![0f64, 1000f64, INFINITY]);
        PtMultiplicity {
            histogram: h
        }
    }
}
    

impl ProcessEvent for PtMultiplicity {
    fn process_event(&mut self, sel_event: &Event, sel_tracks: &[&Track]) {
        let multiplicity = sel_event.multiplicity;
        self.histogram.extend(
            sel_tracks.iter().map(|tr| [tr.pt(), multiplicity as f64])
        );
    }
}

pub struct SingleParticleDistributions {
    pub histogram: Histogram<Dim<[usize; 3]>>
}

impl SingleParticleDistributions {
    pub fn new() -> SingleParticleDistributions {
        // eta, phi, z
        let nphi = 20;
        SingleParticleDistributions {
            histogram: Histogram::new((20, nphi, 8,),
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


pub struct EventDistributions {
    pub histogram: Histogram<Dim<[usize; 2]>>
}

impl EventDistributions {
    pub fn new() -> EventDistributions {
        EventDistributions {
            // mult, z_vtx
            histogram: Histogram::new((20, 8),
                                      &[0., -8.],
                                      &[10e3, 8.])
        }
    }
}

impl ProcessEvent for EventDistributions {
    fn process_event(&mut self, sel_event: &Event, _sel_tracks: &[&Track]) {
        if let Some(ref pv) = sel_event.primary_vertex {
            self.histogram.fill_2(&[sel_event.multiplicity as f64, pv.z])
        };
    }
}

pub struct ParticlePairDistributions {
    pub histogram: Histogram<Dim<[usize; 5]>>
}

impl ParticlePairDistributions {
    pub fn new() -> ParticlePairDistributions {
        // eta, phi, z
        let nphi = 20;
        ParticlePairDistributions {
            histogram: Histogram::new((20, 20, nphi, nphi, 8,),
                                      &[-1., -1., 0., 0., -8.],
                                      &[1., 1., 2. * PI, 2. * PI, 8.])
        }
    }
}

impl ProcessEvent for ParticlePairDistributions {
    fn process_event(&mut self, sel_event: &Event, sel_tracks: &[&Track]) {
        // Fill only if we have a valid z-vtx position
        if let Some (ref pv) = sel_event.primary_vertex {
            for i_t1 in 0..sel_tracks.len() {
                for i_t2 in 0..sel_tracks.len() {
                    if i_t1 >= i_t2 {
                        continue;
                    }
                    let _tmp = [sel_tracks[i_t1].eta(),
                                            sel_tracks[i_t2].eta(),
                                            sel_tracks[i_t1].phi(),
                                            sel_tracks[i_t2].phi(),
                                pv.z];
                    self.histogram.fill_5(&_tmp);
                }
            }
        };
    }
}
