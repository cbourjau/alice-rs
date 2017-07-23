extern crate histogram;
extern crate alice;
extern crate gnuplot;
extern crate glob;

use gnuplot::{Figure, Caption, Color, AxesCommon};
use glob::glob;

use alice::dataset::Dataset;
use alice::track::Track;
use alice::track;
use alice::trigger;

mod analyses;
mod process_event;


use histogram::*;

use process_event::ProcessEvent;

fn main() {
    // let path = "~/lhc_data/alice/data/2010/LHC10h/000139510/ESDs/pass2/10000139510001.170/AliESDs.root";
    let files = glob("/home/christian/lhc_data/alice/data/2010/LHC10h/000139510/ESDs/pass2/*/AliESDs.root")
        .expect("Can't resolve glob");

    let datasets = files
        .inspect(|f| println!("files {:?}", f))
        .map(|path| Dataset::new(path.unwrap().to_str().unwrap()))
        .flat_map(|ev| ev);

    let mut pt_mult = analyses::PtMultiplicity::new();
    let mut single_dists = analyses::SingleParticleDistributions::new();
    let mut pair_dists = analyses::ParticlePairDistributions::new();
    let mut event_dists = analyses::EventDistributions::new();

    let mut hist_ntracks_v0 = Histogram::new((10, 10), &[0.0, 0.0], &[1e3, 1e3]);

    let sel_events = datasets
        .filter(|ev| {ev.primary_vertex.as_ref()
                      .map(|pv| pv.z.abs() < 10.)
                      .unwrap_or(false)})
        .filter(|ev| ev.triggers().contains(trigger::MINIMUM_BIAS));

    for ev in sel_events {
        let filtered_tracks: Vec<&Track> = {
            let pv = ev.primary_vertex.as_ref().unwrap();
            ev.tracks.iter()
                .filter(|tr| {tr.flags.contains(track::ITS_REFIT)})
                .filter(|tr| {tr.dca_to_point_xy(pv.x, pv.y) < 2.4})
                .filter(|tr| {tr.dca_to_point_z(pv.z) < 3.2})
                .filter(|tr| tr.eta().abs() < 0.8)
                .map(|tr| tr)
                .collect()
        };

        // Correlation between number of tracks and multiplicity
        hist_ntracks_v0.fill_2(&[filtered_tracks.len() as f64, ev.multiplicity as f64]);

        pt_mult.process_event(&ev, filtered_tracks.as_slice());
        single_dists.process_event(&ev, filtered_tracks.as_slice());
        event_dists.process_event(&ev, filtered_tracks.as_slice());
        pair_dists.process_event(&ev, filtered_tracks.as_slice());
    }

    let raa =
        &pt_mult.histogram.counts.subview(Axis(1), 0)
        / &pt_mult.histogram.counts.subview(Axis(1), 1);

    let mut fg = Figure::new();
    // hack canvas size
    // fg.set_terminal("pbm size 900, 600", "");

    fg.axes2d()
        .set_pos_grid(2, 3, 0)
        .set_title("η track distribution", &[])
        .set_x_label("η", &[])
        .lines(&single_dists.histogram.centers(0),
               // Sum over phi and z
               &single_dists.histogram.counts.sum(Axis(1)).sum(Axis(1)),
               &[Caption("A line"), Color("black")]);
    fg.axes2d()
        .set_pos_grid(2, 3, 1)
        .set_title("p_{T} distribution", &[])
        .set_x_label("p_{T} [GeV]", &[])
        .lines(&pt_mult.histogram.centers(0), &raa, &[Caption("A line"), Color("red")]);

    fg.axes2d()
        .set_pos_grid(2, 3, 2)
        .set_title("Multiplicity distribution", &[])
        .set_x_label("SPD tracklets", &[])
        .boxes_set_width(&event_dists.histogram.centers(0),
                         // sum over z_vtx
                         &event_dists.histogram.counts.sum(Axis(1)),
                         &event_dists.histogram.widths(0),
                         &[Caption("A line"), Color("red")]);

    fg.axes2d()
        .set_pos_grid(2, 2, 2)
        .set_title("eta eta", &[])
        .set_x_label("eta", &[])
        // Sum phi1, phi2, z
        .image(&pair_dists.histogram.counts.sum(Axis(2)).sum(Axis(2)).sum(Axis(2)),
               pair_dists.histogram.counts.shape()[0],
               pair_dists.histogram.counts.shape()[1],
               None, &[]);

    // let ratio = &hist_phi_phi.counts / &phiphi;
    let nphi = 20;
    let neta = 20;
    let phiphi = (&single_dists.histogram.counts.to_owned()
                  .into_shape((1, neta, 1, nphi, 8))
                  .expect("Can't reshape")
                  .broadcast((neta, neta, nphi, nphi, 8))
                  .expect("Can't broadcast"))
        * (&single_dists.histogram.counts.to_owned()
           .into_shape((neta, 1, nphi, 1, 8))
           .expect("Can't reshape"));

    let ratio = pair_dists.histogram.counts / phiphi;
    let ratio = ratio.mapv(|v| if v.is_finite() {v} else {0.});
    fg.axes2d()
        .set_pos_grid(2, 2, 3)
        .set_title("phi phi", &[])
        .set_x_label("phi1", &[])
        .set_y_label("phi2", &[])
        // Sum eta1, eta2, z
        .image(&ratio.sum(Axis(0)).sum(Axis(0)).sum(Axis(2)),
               ratio.shape()[2],
               ratio.shape()[3],
               None, &[]);
    fg.show();

    let mut fg = Figure::new();
    fg.axes2d()
        .set_pos_grid(1, 1, 0)
        .set_title("multcor", &[])
        // .set_x_label("ntracks", &[])
        // .set_y_label("ntracks accepted", &[])
        .image(&hist_ntracks_v0.counts,
               hist_ntracks_v0.counts.shape()[0], hist_ntracks_v0.counts.shape()[0],
               Some((0., 0., 1e3, 1e3)), &[]);
    fg.show();
}
