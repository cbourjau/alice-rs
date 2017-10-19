use alice_sys as ffi;
use primary_vertex::PrimaryVertex;
use track::Track;
use trigger_mask::TriggerMask;
// use vzero::V0;

#[derive(Debug)]
pub struct Event {
    pub primary_vertex: Option<PrimaryVertex>,
    pub tracks: Vec<Track>,
    pub multiplicity: i32,
    // pub vzero: V0,
    pub trigger_mask: TriggerMask
}

impl Event {
    pub fn new_from_esd(esd: &ffi::ESD_t) -> Event {
        Event {
            // raw_esd: esd,
            primary_vertex: PrimaryVertex::new(esd),
            tracks: Track::read_tracks_from_esd(esd),
            multiplicity: esd.AliMultiplicity_fNtracks,
            // vzero: V0::from_esd(esd),
            trigger_mask: TriggerMask::new(esd)
        }
    }
}
