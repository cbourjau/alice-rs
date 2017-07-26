use std::f64::INFINITY;
use gnuplot::{Figure, AxesCommon};

use histogram::{Histogram, Dim, Centers, Axis};

use alice::event::Event;
use alice::track::Track;

use super::{ProcessEvent, Visualize};

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

impl Visualize for PtMultiplicity {
    fn visualize(&self) {
        let raa =
            &self.histogram.counts.subview(Axis(1), 0)
            / &self.histogram.counts.subview(Axis(1), 1);

        let mut fg = Figure::new();
        fg.axes2d()
            .set_pos_grid(1, 1, 0)
            .set_title("RAA", &[])
            .set_x_label("p_{T} [GeV]", &[])
            .lines(&self.histogram.centers(0),
                   &raa,
                   &[]);
        fg.show();
    }
}
