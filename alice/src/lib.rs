extern crate libc;
#[macro_use]
extern crate bitflags;
extern crate alice_sys;
extern crate log;
extern crate indicatif;
#[macro_use]
extern crate chan;
extern crate rayon;
#[macro_use]
extern crate lazy_static;

pub mod dataset;
pub mod event;
pub mod primary_vertex;
pub mod track;
pub mod trigger_mask;
pub mod vzero;
pub mod esd;


#[cfg(test)]
mod tests {
    use esd::ESD;
    use dataset::Dataset;
    use track;
    use trigger_mask::TriggerMask;
    extern crate alice_open_data;
    
    #[test]
    fn primary_vertices() {
        let ds = Dataset::new([alice_open_data::test_file().unwrap()], 2);
        let sum = ds
            .filter(|ev| {ev.primary_vertex.is_some()})
            .fold(0.0, |mut acc, ev| {acc += ev.primary_vertex.unwrap().x.abs();
                                      acc});
        assert!(sum > 0., "Primary vertices did not load!");
    }

    #[test]
    fn tracks() {
        let ds = Dataset::new([alice_open_data::test_file().unwrap()], 2);
        for ev in ds.filter(|ev| {ev.primary_vertex.is_some()}) {
            let pv = ev.primary_vertex.unwrap();
            let etas =
                ev.tracks
                .iter()
                .filter(|tr| {tr.flags.contains(track::ITS_REFIT)})
                .filter(|tr| {tr.dca_to_point_xy(pv.x, pv.y) < 2.4})
                .filter(|tr| {tr.dca_to_point_z(pv.z) < 3.2})
                .map(|tr| {tr.eta()});
            assert!(etas.count() > 0, "No tracks loaded?");
        }
    }

    #[test]
    fn triggers() {
        let ds = Dataset::new([alice_open_data::test_file().unwrap()], 2);
        // Combine many events to be sure that we have some triggers
        let many_trgs = ds
            .map(|ev| ev.trigger_mask)
            .collect::<TriggerMask>();
        assert!(!many_trgs.is_empty());
    }

    #[test]
    /// Poor mans check if we are leaking memory when destroying the esd reader object
    fn release_mem_esds() {
        for _ in 0..1000 {
            ESD::new(&alice_open_data::test_file().unwrap());
        }
    }

}
