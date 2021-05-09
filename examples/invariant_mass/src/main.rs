use malice::{default_track_filter, default_event_filter, Event, Track};
use malice::event_iterator_from_files;
use histogram::{Histogram, HistogramBuilder};
use gnuplot::Figure;

use std::error::Error;

/// Create histogram for the invariant mass spectrum
fn invariant_mass_spectrum_hist() -> Result<Histogram, Box<dyn Error>> {
    Ok(HistogramBuilder::new()
        .add_equal_width_axis(300, 0.1, 3.6)
       .build()?)
}

struct FourMomentum {
    energy: f32,
    px: f32,
    py: f32,
    pz: f32,
}

impl FourMomentum {
    fn from_track(track: &Track, rest_mass: f32) -> Self {
	let px = track.px();
	let py = track.py();
	let pz = track.pz();
        let energy = (px * px + py * py + pz * pz + rest_mass * rest_mass).sqrt();
        Self { energy, px, py, pz }
    }

    fn wrong_invariant_mass(&self, other: &Self) -> f32 {
        (
            // some mass term here
            (self.energy + other.energy).powi(2)
                - (self.px + other.px).powi(2)
                - (self.py + other.py).powi(2)
                - (self.pz + other.pz).powi(2)
        ).sqrt()
    }
}

fn process_event(mut hist: Histogram, dca_hist: &mut Histogram, event: Event) -> Histogram {
    let prime_vtx = event.primary_vertex().unwrap();
    if prime_vtx.z.abs() > 5.0 {
        return hist;
    }
    let pions: Vec<_> = event.tracks()
        .filter(|tr| default_track_filter(tr, &prime_vtx))
        .collect();
    let pion_mass = 0.1395;  // GeV
    let proton_mass = 0.9383; // GeV
    let proton_pos: Vec<_> = pions
        .iter()
        .filter(|tr| tr.charge_sign() > 0)
        .filter(|tr| tr.pid_probabilities.proton > 0.5)
        .collect();
    let pions_neg: Vec<_> = pions
        .iter()
        .filter(|tr| tr.charge_sign() < 0)
        .filter(|tr| tr.pid_probabilities.pion > 0.8)
        .collect();

    let b = 5.00668049; // kG
    for tr_pos in proton_pos.iter() {
        for tr_neg in pions_neg.iter() {
            let dca = tr_pos.dca_to_other_track(tr_neg, b) as f64;
            dca_hist.fill(&[dca]);
            if dca > 0.2 {
                continue;
            }
            let inv_mass = FourMomentum::from_track(tr_pos, proton_mass)
                .wrong_invariant_mass(&FourMomentum::from_track(tr_neg, pion_mass));
            hist.fill(&[inv_mass as f64]);
            
        }
    }
    hist
}


fn main() {
    let files = alice_open_data::all_files_10h()
        .expect("No data files found. Did you download with alice-open-data?");

    // Create an iterator over all the events in all the given files
    let events = event_iterator_from_files(files.into_iter());
    let mut hist = invariant_mass_spectrum_hist().unwrap();
    let mut dca_hist = HistogramBuilder::new()
        .add_equal_width_axis(100, 0.0, 5.0)
        .build()
        .unwrap();
    
    for event in events.filter(default_event_filter) {
        hist = process_event(hist, &mut dca_hist, event);
    }

    use textplots::{Chart, Plot, Shape};
    let thing: Vec<_> = hist.clone().centers(0).iter()
        .zip(hist.values().iter())
        .map(|(cent, v)| (*cent as f32, *v as f32))
        .collect();
    Chart::new(180, 60, 0.0, 5.0)
        .lineplot(Shape::Bars(thing.as_slice()))
        .display();

    let mut fg = Figure::new();
    fg.axes2d()
        .boxes(&hist.clone().centers(0), hist.values().into_iter(), &[]);
    fg.show().unwrap();
    // let mut fg = Figure::new();
    // fg.axes2d()
    //     .boxes(&dca_hist.clone().centers(0), dca_hist.values().into_iter(), &[]);
    // fg.show().unwrap();    
}
