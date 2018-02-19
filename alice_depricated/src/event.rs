use alice_sys as ffi;
use primary_vertex::PrimaryVertex;
use track::Track;
use trigger_mask::TriggerMask;
use event_traits;

/// `Event` struct bundles together various data which makes up one
/// "event"
#[derive(Debug)]
pub struct Event {
    pub primary_vertex: Option<PrimaryVertex>,
    pub tracks: Vec<Track>,
    pub multiplicity: i32,
    // pub vzero: V0,
    pub trigger_mask: TriggerMask
}

impl Event {
    /// Create an event from the data that is currently loaded in an ESD object
    /// Note that the ESD object's data will change after calling `load_event` on it
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

impl event_traits::Tracks<Track> for Event {
    /// Get the tracks (reconstructions of particles traversing the
    /// detector) of this event
    fn tracks(&self) -> &Vec<Track> {
        &self.tracks
    }
}

impl event_traits::PrimaryVertex for Event {
    /// Get the most likely position where the collision took place in
    /// the detector (aka, primary vertex).
    /// The primary vertex is estimated from the reconstructed
    /// tracks. It is the most likely common origin of the tracks. In
    /// some cases, there might be too few tracks in an event to
    /// reconstruct it. If the event has not primary vertex, this
    /// function returns `None`.
    fn primary_vertex(&self) -> Option<&PrimaryVertex> {
        self.primary_vertex.as_ref()
    }
}

impl event_traits::Multiplicity for Event {
    /// Return the number of reconstructed tracks. Not very sophisticated...
    fn multiplicity(&self) -> f64 {
        self.multiplicity as f64
    }
}
