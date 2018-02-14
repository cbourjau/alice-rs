/// The most likely position in the detector where the current event
/// took place. The primary vertex is the most likely common origin of
/// all the reconstructed tracks.
#[derive(Debug)]
pub struct PrimaryVertex {
    /// `x` coordinates in the detector's reference frame
    pub x: f32,
    /// `y` coordinates in the detector's reference frame
    pub y: f32,
    /// `z` coordinates in the detector's reference frame; z is along the beam axis
    pub z: f32,
    /// Number of tracks contributed to this vertex
    pub n_contrib: i32
}
