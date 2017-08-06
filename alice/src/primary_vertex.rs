use alice_sys::ESD_t;

#[derive(Debug)]
pub struct PrimaryVertex {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub n_contrib: i32
}

impl PrimaryVertex {
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
