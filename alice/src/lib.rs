extern crate libc;
#[macro_use]
extern crate bitflags;
extern crate histogram;

pub mod dataset;
pub mod event;
pub mod primary_vertex;
pub mod track;


#[cfg(test)]
mod tests {
    use dataset::Dataset;
    use track;
    use histogram::Histogram;

    #[test]
    fn primary_vertices() {
        let ds = Dataset::new();
        let sum = ds
            .filter(|ev| {ev.primary_vertex.is_some()})
            .fold(0.0, |mut acc, ev| {acc += ev.primary_vertex.unwrap().x;
                                      acc});
        println!("{}", sum);
    }

    #[test]
    fn tracks() {
        let ds = Dataset::new();
        let mut hist = Histogram::new(20, -2., 2.);
        for ev in ds.filter(|ev| {ev.primary_vertex.is_some()}) {
            let pv = ev.primary_vertex.unwrap();
            let etas =
                ev.tracks
                .iter()
                .filter(|tr| {tr.flags.contains(track::ITS_REFIT)})
                .filter(|tr| {tr.dca_to_point_xy(pv.x, pv.y) < 2.4})
                .filter(|tr| {tr.dca_to_point_z(pv.z) < 3.2})
                // .inspect(|tr| {println!("tr: {:?}", tr)})
                .map(|tr| {tr.eta()});
            hist.extend(etas);
        }
        println!("{:?}", hist);
    }
}
