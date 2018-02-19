use alice_sys::ESD_t;

/// The most likely position in the detector where the current event
/// took place. The primary vertex is the most likely common origin of
/// all the reconstructed tracks.
#[derive(Debug)]
pub struct PrimaryVertex {
    /// `x` coordinates in the detector's reference frame
    pub x: f64,
    /// `y` coordinates in the detector's reference frame
    pub y: f64,
    /// `z` coordinates in the detector's reference frame; z is along the beam axis
    pub z: f64,
    /// Number of tracks contributed to this vertex
    pub n_contrib: i32
}

impl PrimaryVertex {
    /// Instantiate from the currently loaded data in the given `esd` object.
    /// Note that the data in `esd` will change when `next_event` is called on it.
    pub fn new(esd: &ESD_t) -> Option<PrimaryVertex>{
        // 0 contributors means that there is no primar vertex
        if esd.PrimaryVertex_AliVertex_fNContributors > 0 {
            Some(PrimaryVertex {x: esd.PrimaryVertex_AliVertex_fPosition[0],
                                y: esd.PrimaryVertex_AliVertex_fPosition[1],
                                z: esd.PrimaryVertex_AliVertex_fPosition[2],
                                n_contrib: esd.PrimaryVertex_AliVertex_fNContributors})
        } else {
            None
        }
    }
}
