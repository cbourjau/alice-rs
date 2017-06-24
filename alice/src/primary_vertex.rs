use alice_sys::{primary_vertex_get_pos, CEsd};

#[derive(Debug)]
pub struct PrimaryVertex {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub n_contrib: i32
}

impl PrimaryVertex {
    pub fn new(esd: *const CEsd) -> Option<PrimaryVertex>{
        unsafe {
            let a = primary_vertex_get_pos(esd);
            // 0 contributors means that there is no primar vertex
            if a.n_contrib > 0 {
                Some(PrimaryVertex {x: a.x, y: a.y, z: a.z, n_contrib: a.n_contrib})
            } else {
                None
            }
        }
    }
}
