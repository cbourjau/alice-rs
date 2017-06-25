use alice_sys as ffi;
use primary_vertex::PrimaryVertex;
use track::Track;


#[derive(Debug)]
pub struct Event {
    pub primary_vertex: Option<PrimaryVertex>,
    pub tracks: Vec<Track>,
    pub multiplicity: i32,
}

impl Event {
    pub fn new_from_esd(esd: *const ffi::CEsd) -> Event {
        Event {
            primary_vertex: PrimaryVertex::new(esd),
            tracks: Track::read_tracks_from_esd(esd),
            multiplicity: unsafe {ffi::get_multiplicity(esd)},
        }
    }
}
