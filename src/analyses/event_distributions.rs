use gnuplot::{Figure, AxesCommon};

use histogram::*;

use alice::event::Event;
use alice::track::Track;

use super::{ProcessEvent, Visualize};

pub struct EventDistributions {
    pub histogram: Histogram<Ix2>
}

impl EventDistributions {
    pub fn new() -> EventDistributions {
        EventDistributions {
            // mult, z_vtx
            histogram: HistogramBuilder::<Ix2>::new()
                .add_equal_width_axis(20, 0., 3e3)
                .add_equal_width_axis(8, -8., 8.)
                .build().expect("Error building histogram")
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

impl Visualize for EventDistributions {
    fn visualize(&self) {
        let mut fg = Figure::new();

        fg.axes2d()
            .set_pos_grid(1, 2, 0)
            .set_title("Multiplicity distribution", &[])
            .set_x_label("SPD tracklets", &[])
            .boxes_set_width(&self.histogram.centers(0),
                             // sum over z_vtx
                             &self.histogram.counts.sum(Axis(1)),
                             &self.histogram.widths(0),
                             &[]);

        fg.axes2d()
            .set_pos_grid(1, 2, 1)
            .set_title("zvtx distribution", &[])
            .set_x_label("zvtx", &[])
            .boxes_set_width(&self.histogram.centers(0),
                             // sum over mult
                             &self.histogram.counts.sum(Axis(0)),
                             &self.histogram.widths(1),
                             &[]);
    }
}
