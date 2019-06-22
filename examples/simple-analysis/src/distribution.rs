//! Measure a bunch of simple distribtions.
//!  - Single particle distribution in eta and phi
//!  - Distribution of events' primary vertices along the nominal
//!    interaction point along beam axis
use std::f64::consts::PI;

use failure::Error;
use gnuplot::{AutoOption, AxesCommon, Figure, PlotOption, Tick};

use histogram::*;
use malice::default_track_filter;
use malice::Event;

pub struct SimpleAnalysis {
    pub single_particles: Histogram<f32, [usize; 3]>,
    pub z_vertex: Histogram<i32, [usize; 1]>,
    pub multiplicity: Histogram<f32, [usize; 1]>,
}

impl SimpleAnalysis {
    pub fn new() -> SimpleAnalysis {
        // eta, phi, z
        let nphi = 120 / 2;
        let neta = 120 / 2;
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
    pub fn process_event(&mut self, event: &Event) {
        // Fill only if we have a valid primary vertex
        if let Some(prime_vtx) = event.primary_vertex() {
            self.single_particles.extend(
                event
                    .tracks()
                    .filter(|tr| default_track_filter(&tr, &prime_vtx))
                    .map(|tr| {
                        [
                            f64::from(tr.eta()),
                            f64::from(tr.phi()),
                            f64::from(prime_vtx.z),
                        ]
                    }),
            );
            self.z_vertex.fill(&[f64::from(prime_vtx.z)]);
            self.multiplicity.fill(&[event
                .tracks()
                .filter(|tr| default_track_filter(&tr, &prime_vtx))
                .count() as f64]);
        };
    }

    /// Example of how one may write the results to disc
    pub fn write_to_disc(&self) -> Result<(), Error> {
        self.single_particles.dump_to_file("hybrid")?;
        self.z_vertex.dump_to_file("z_pos")?;
        Ok(())
    }
}

impl SimpleAnalysis {
    /// Visualized the data using gnuplot-rs
    pub fn visualize(&self) {
        let mut fg = Figure::new();
        let eta_bin_width = self.single_particles.widths(0)[0] as f32;
        let plot_options = [PlotOption::Color("#d95f02"), PlotOption::FillAlpha(0.8)];
        fg.axes2d()
            .set_pos_grid(2, 2, 0)
            .set_title("η track distribution", &[])
            .set_x_label("η", &[])
            .set_y_label("⟨dN_{ch} / dη ⟩_{event}", &[])
            .boxes(
                &self.single_particles.centers(0),
                // Sum over phi and z
                (&self
                    .single_particles
                    .counts
                    .sum_axis(Axis(1))
                    .sum_axis(Axis(1))
                    / self.z_vertex.counts.scalar_sum() as f32
                    / eta_bin_width)
                    .view(),
                &plot_options,
            );

        let phi_bin_width = self.single_particles.widths(1)[0] as f32;
        let x_ticks = vec![
            Tick::Major(0.0, AutoOption::Fix("0".to_owned())),
            Tick::Major(0.5 * PI, AutoOption::Fix("0.5 π".to_owned())),
            Tick::Major(PI, AutoOption::Fix("π".to_owned())),
            Tick::Major(1.5 * PI, AutoOption::Fix("1.5π".to_owned())),
            Tick::Major(2.0 * PI, AutoOption::Fix("2π".to_owned())),
        ];

        fg.axes2d()
            .set_pos_grid(2, 2, 1)
            .set_title("φ track distribution", &[])
            .set_x_label("φ [rad]", &[])
            .set_y_label("⟨dN_{ch} / dφ ⟩_{event}", &[])
            .set_x_range(AutoOption::Fix(0.0), AutoOption::Fix(2.0 * PI))
            .set_x_ticks_custom(x_ticks, &[], &[])
            .boxes(
                &self.single_particles.centers(1),
                // Sum over eta and z
                (&self
                    .single_particles
                    .counts
                    .sum_axis(Axis(2))
                    .sum_axis(Axis(0))
                    / self.z_vertex.counts.scalar_sum() as f32
                    / phi_bin_width)
                    .view(),
                &plot_options,
            );

        fg.axes2d()
            .set_pos_grid(2, 2, 2)
            .set_title("Primary vertex position", &[])
            .set_x_label("z [cm]", &[])
            .set_y_label("# events", &[])
            .boxes(
                &self.z_vertex.centers(0),
                &self.z_vertex.counts,
                &plot_options,
            );

        fg.axes2d()
            .set_pos_grid(2, 2, 3)
            .set_title("N_{ch} distribution", &[])
            .set_x_label("N_{ch}", &[])
            .set_y_label("# events", &[])
            // .set_x_log(Some(10.0))
            .set_y_log(Some(10.0))
            .boxes(
                &self.multiplicity.centers(0),
                &self.multiplicity.counts,
                &plot_options,
            );
        fg.show();
    }

    /// Compute the centrality edges based on the N_ch/Event distribution
    pub fn compute_centrality_edges(&self) {
        let tot = self.multiplicity.counts.scalar_sum();
        let cum: Vec<_> = self
            .multiplicity
            .counts
            .iter()
            .scan(0.0, |state, el| {
                *state += el;
                Some(*state)
            })
            // convert to %; 100% is first bin
            .map(|v| (1.0 - v / tot) * 100.0)
            // Bin width is one track, so we just enumerate to have the number of tracks
            .enumerate()
            .collect::<Vec<_>>();
        let percent_edges = (1..11)
            .rev()
            .filter_map(|v| {
                let want_this_percent = (v * 10) as f32;
                cum.iter()
                    .find(|bin_percent| bin_percent.1 <= want_this_percent)
            })
            .collect::<Vec<_>>();
        println!("Number of valid tracks | less than %");
        for cent_edge in percent_edges {
            println!("{:4} | {:3}%", cent_edge.0, cent_edge.1);
        }
    }
}
