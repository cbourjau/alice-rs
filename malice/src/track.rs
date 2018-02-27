use std::f32::consts::PI;

bitflags! {
    /// Clusters in the ITS associated with the curren track
    /// See AliESDTrack::HasPointOnITSLayer
    pub struct ItsClusters: u8 {
        const SPD_INNER = 1 << 0;
        const SPD_OUTER = 1 << 1;
        const SDD_INNER = 1 << 2;
        const SDD_OUTER = 1 << 3;
        const SSD_INNER = 1 << 4;
        const SSD_OUTER = 1 << 5;
    }
}

bitflags! {
    /// Various attributes of tracks.
    /// Flags are based on those found in AliRoot's AliVTrack.[h,cxx]
    pub struct Flags: u64 {
        const ITS_IN = 0x1;
        const ITS_OUT = 0x2;
        const ITS_REFIT = 0x4;
        const ITS_PID = 0x8;
        const TPC_IN = 0x10;
        const TPC_OUT = 0x20;
        const TPC_REFIT = 0x40;
        const TPC_PID = 0x80;
        const TRD_IN = 0x100;
        const TRD_OUT = 0x200;
        const TRD_REFIT = 0x400;
        const TRD_PID = 0x800;
        const TOF_IN = 0x1000;
        const TOF_OUT = 0x2000;
        const TOF_REFIT = 0x4000;
        const TOF_PID = 0x8000;
        const HMPID_OUT = 0x10_000;
        const HMPID_PID = 0x20_000;
        const EMCAL_MATCH = 0x40_000;
        const TRD_BACKUP = 0x80_000;
        const TOF_MISMATCH = 0x100_000;
        const PHOS_MATCH = 0x200_000;
        const ITS_UPG = 0x400_000;
        const SKIP_FRIEND = 0x800_000;
        const GLOBAL_MERGE = 0x1_000_000;
        const MULT_IN_V0 = 0x2_000_000;
        const MULT_SEC = 0x4_000_000;
        const EMBEDDED = 0x8_000_000;
        const ITS_PURE_SA = 0x10_000_000; 
        const TRDS_TOP = 0x20_000_000; 
        const ESD_PID = 0x40_000_000; 
        const TIME = 0x80_000_000;
    }
}

pub struct Track {
    pub(crate) x: f32,
    pub(crate) parameters: TrackParameters,
    pub(crate) alpha: f32,
    pub flags: Flags,
    pub(crate) its_chi2: f32,
    pub its_ncls: i8,
    pub its_clustermap: ItsClusters,
    pub(crate) tpc_chi2: f32,
    pub tpc_ncls: u16,
    
}

/// An obscure set of parameters which makes sense for the actual
/// reconstruction of the tracks, but is a pain for subsequent
/// analysis
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct TrackParameters {
    loc_y: f32,
    loc_z: f32,
    loc_sin: f32,
    tang: f32,
    one_over_pt: f32,
}

impl TrackParameters {
    /// In AliESD files, these parameters are saved in "Tracks.fP[5]"
    pub fn new(paras: &[f32; 5usize]) -> TrackParameters {
        TrackParameters {
            loc_y: paras[0],
            loc_z: paras[1],
            loc_sin: paras[2],
            tang: paras[3],
            one_over_pt: paras[4],
        }
    }
}

impl Track {
    /// Longitudinal (not boosted) angle of the `Track`
    pub fn theta(&self) -> f32 {
        0.5 * PI - self.parameters.tang.atan()
    }

    /// Direction of a track in pseudorapidity `eta`
    pub fn eta(&self) -> f32 {
        -((0.5 * self.theta()).tan()).ln()
    }    

    /// Azimuthal direction of the `Track`
    pub fn phi(&self) -> f32 {
        let mut phi = self.parameters.loc_sin.asin() + self.alpha;
        if phi < 0. {
            phi += 2. * PI;
        } else if phi >= 2. * PI {
            phi -= 2. * PI;
        }
        phi
    }

    /// Transverse momentum of the `Track`
    pub fn pt(&self) -> f32 {
        1.0 / self.parameters.one_over_pt.abs()
    }
    
    /// Estimate the distance of closest approach of this track to a given point
    /// neglecting the track curvature. This returns the closest approach in the xy plane
    pub fn dca_to_point_xy(&self, x: f32, y: f32) -> f32 {
        let xv = x * self.alpha.cos() + y * self.alpha.sin();
        let yv = -x * self.alpha.sin() + y * self.alpha.cos();
        let diff_x = self.x - xv;
        let diff_y = self.parameters.loc_y - yv;
        let loc_sin = self.parameters.loc_sin;
        (diff_x * loc_sin - diff_y * ((1. - loc_sin) * (1. + loc_sin)).sqrt()).abs()
    }

    /// Distance of closes approch of this track in z
    pub fn dca_to_point_z(&self, z: f32) -> f32 {
        self.parameters.loc_z - z
    }

    /// Chi^2 normalized to the number of clusters. This is a measure
    /// of how well the reconstruction fit the observed clusters
    pub fn tpc_chi2_per_cluster(&self) -> f32 {
        self.tpc_chi2 / self.tpc_ncls as f32
    }

    /// Chi^2 normalized to the number of clusters. This is a measure
    /// of how well the reconstruction fit the observed clusters
    pub fn its_chi2_per_cluster(&self) -> f32 {
        self.its_chi2 / self.its_ncls as f32
    }    
}
