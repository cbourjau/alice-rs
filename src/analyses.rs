use std::f64::INFINITY;
use std::f64::consts::PI;
use ndarray as nd;

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
                                      &[3e3, 8.])
        }
    }
}

impl ProcessEvent for EventDistributions {
    fn process_event(&mut self, sel_event: &Event, _sel_tracks: &[&Track]) {
        if let Some(ref pv) = sel_event.primary_vertex {
            self.histogram.fill(&[sel_event.multiplicity as f64, pv.z])
        };
    }
}

pub struct ParticlePairDistributions {
    singles: Histogram<Dim<[usize; 3]>>,
    pub pairs: Histogram<Dim<[usize; 5]>>,
    event_counter: Histogram<Dim<[usize; 1]>>,
}

impl ParticlePairDistributions {
    pub fn new() -> ParticlePairDistributions {
        // eta, phi, z
        let nphi = 36;
        let neta = 30;
        let (nzvtx, zmin, zmax) = (8, -8., 8.);
        ParticlePairDistributions {
            singles: Histogram::new((neta, nphi, nzvtx),
                                    &[-1., 0., zmin],
                                    &[1., 2. * PI, zmax]),
            pairs: Histogram::new((neta, neta, nphi, nphi, nzvtx),
                                  &[-1., -1., 0., 0., zmin],
                                  &[1., 1., 2. * PI, 2. * PI, zmax]),
            event_counter: Histogram::new((nzvtx, ), &[zmin], &[zmax]),
        }
    }

    pub fn finalize(&self) -> nd::Array<f64, nd::Dim<[usize; 5]>> {
        let nphi = 36;
        let neta = 30;
        let nzvtx = 8;
        let phiphi = (&self.singles.counts.to_owned()
                      .into_shape((1, neta, 1, nphi, nzvtx))
                      .expect("Can't reshape")
                      .broadcast((neta, neta, nphi, nphi, nzvtx))
                      .expect("Can't broadcast"))
            * (&self.singles.counts.to_owned()
               .into_shape((neta, 1, nphi, 1, nzvtx))
               .expect("Can't reshape"));

        &self.pairs.counts / &phiphi * &self.event_counter.counts
    }
}

impl ProcessEvent for ParticlePairDistributions {
    fn process_event(&mut self, sel_event: &Event, sel_tracks: &[&Track]) {
        // Fill only if we have a valid z-vtx position
        if let Some (ref pv) = sel_event.primary_vertex {
            self.singles.extend(
                sel_tracks.iter().map(|tr| [tr.eta(), tr.phi(), pv.z])
            );
            self.event_counter.fill(&[pv.z]);
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
                    self.pairs.fill(&_tmp);
                }
            }
        };
    }
}
