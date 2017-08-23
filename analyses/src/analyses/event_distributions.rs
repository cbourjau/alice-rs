use gnuplot as gpl;
use gnuplot::AxesCommon;

use histogram::*;

use alice::event::Event;
use alice::track::Track;

use super::{ProcessEvent, Visualize};

pub struct EventDistributions {
    pub histogram: Histogram<[usize; 2]>,
    // Vector of all seen multiplicities to quickly find the percentile bins
    // (quick, dirty and not scalable!)
    multiplicities: Vec<i32>,
}

impl EventDistributions {
    pub fn new() -> EventDistributions {
        EventDistributions {
            // mult, z_vtx
            histogram: HistogramBuilder::<[usize; 2]>::new()
                .add_equal_width_axis(20, 0., 6e3)
                .add_equal_width_axis(8, -8., 8.)
                .build()
                .expect("Error building histogram"),
            multiplicities: Vec::<i32>::new(),
        }
    }
}

impl ProcessEvent for EventDistributions {
    fn process_event(&mut self, sel_event: &Event, _sel_tracks: &[&Track]) {
        if let Some(ref pv) = sel_event.primary_vertex {
            self.histogram.fill(&[sel_event.multiplicity as f64, pv.z]);
            self.multiplicities.push(sel_event.multiplicity)
        };
    }
}

impl Visualize for EventDistributions {
    fn visualize(&self) {
        let mut fg = gpl::Figure::new();

        fg.axes2d()
            .set_pos_grid(1, 2, 0)
            .set_title("Multiplicity distribution", &[])
            .set_x_label("SPD tracklets", &[])
            .set_x_ticks(Some((gpl::AutoOption::Auto::<f64>, 0)),
                         &[],
                         &[gpl::Rotate(45.), gpl::TextOffset(0., -2.5)])
            .set_y_log(Some(10.))
            .boxes_set_width(&self.histogram.centers(0),
                             // sum over z_vtx
                             &self.histogram.counts.sum(Axis(1)),
                             &self.histogram.widths(0),
                             &[]);

        fg.axes2d()
            .set_pos_grid(1, 2, 1)
            .set_title("zvtx distribution", &[])
            .set_x_label("zvtx", &[])
            .boxes_set_width(&self.histogram.centers(1),
                             // sum over mult
                             &self.histogram.counts.sum(Axis(0)),
                             &self.histogram.widths(1),
                             &[]);
        fg.show();

        // Caluclate percentile bins
        let nevents = self.multiplicities.len();
        let mut mults = self.multiplicities.as_slice().to_owned();
        mults.sort();
        println!("{:?}",
                 (1..10)
                     .map(|i| mults[nevents / 10 * i - 1])
                     .collect::<Vec<_>>());
    }
}
