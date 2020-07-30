use std::f32::consts::PI;
use track_dca_sys::AliExternalTrackParam;

use wasm_bindgen::prelude::*;

bitflags! {
    /// Clusters in the ITS associated with the curren track
    /// See AliESDTrack::HasPointOnITSLayer
    #[wasm_bindgen]
    pub struct ItsClusters: u8 {
        const SPD_INNER = 1;
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
    #[wasm_bindgen]
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
        const HMPID_OUT = 0x0001_0000;
        const HMPID_PID = 0x0002_0000;
        const EMCAL_MATCH = 0x0004_0000;
        const TRD_BACKUP = 0x0008_0000;
        const TOF_MISMATCH = 0x0010_0000;
        const PHOS_MATCH = 0x0020_0000;
        const ITS_UPG = 0x0040_0000;
        const SKIP_FRIEND = 0x0080_0000;
        const GLOBAL_MERGE = 0x0100_0000;
        const MULT_IN_V0 = 0x0200_0000;
        const MULT_SEC = 0x0400_0000;
        const EMBEDDED = 0x0800_0000;
        const ITS_PURE_SA = 0x1000_0000;
        const TRDS_TOP = 0x2000_0000;
        const ESD_PID = 0x4000_0000;
        const TIME = 0x8000_0000;
    }
}

/// Probabilities of this track being of various particle types. These
/// numbers stem from the "combined detector response"
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PidProbabilities {
    pub electron: f32,
    pub muon: f32,
    pub pion: f32,
    pub kaon: f32,
    pub proton: f32,
}

/// A `Track` is a reconstruction of the trajectory of a particle traversing the detector.
#[wasm_bindgen]
#[derive(Debug)]
pub struct Track {
    pub x: f32,
    pub(crate) parameters: TrackParameters,
    pub(crate) alpha: f32,
    pub flags: Flags,
    pub its_chi2: f32,
    pub its_ncls: i8,
    pub its_clustermap: ItsClusters,
    pub(crate) tpc_chi2: f32,
    pub tpc_ncls: u16,
    pub pid_probabilities: PidProbabilities,
}

/// An obscure set of parameters which makes sense for the actual
/// reconstruction of the tracks, but is a pain for subsequent
/// analysis
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct TrackParameters {
    // Tracks.fP[0]
    loc_y: f32,
    // Tracks.fP[1]
    loc_z: f32,
    // Tracks.fP[2]
    loc_sin: f32,
    // Tracks.fP[3]
    tang: f32,
    // Tracks.fP[4]
    one_over_pt: f32,
}

impl TrackParameters {
    /// In AliESD files, these parameters are saved in "Tracks.fP[5]"
    pub(crate) fn new(paras: &(f32, f32, f32, f32, f32)) -> TrackParameters {
        TrackParameters {
            /// local Y-coordinate of a track (cm)
            loc_y: paras.0,
            /// local Z-coordinate of a track (cm)
            loc_z: paras.1,
            /// local sine of the track momentum azimuthal angle
            loc_sin: paras.2,
            /// tangent of the track momentum dip angle
            tang: paras.3,
            /// 1/pT where the sign denotes the charge (1/(GeV/c))
            one_over_pt: paras.4,
        }
    }
}

#[wasm_bindgen]
impl Track {
    /// Longitudinal (not boosted) angle of the `Track`
    fn theta(&self) -> f32 {
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

    pub fn charge_sign(&self) -> i8 {
        if self.parameters.one_over_pt > 0.0 {
            1
        } else {
            -1
        }
    }

    /// Three-momentum (px, py, pz). Results for straight tracks are meaningless.
    fn pxpypz(&self) -> (f32, f32, f32) {
        let pt = self.pt();
        let cs = self.alpha.cos();
        let sn = self.alpha.sin();
        let r = ((1.0 - self.parameters.loc_sin) * (1.0 + self.parameters.loc_sin)).sqrt();
        (
            pt * (r * cs - self.parameters.loc_sin * sn),
            pt * (self.parameters.loc_sin * cs + r * sn),
            pt * self.parameters.tang,
        )
    }

    pub fn px(&self) -> f32 {
        self.pxpypz().0
    }
    pub fn py(&self) -> f32 {
        self.pxpypz().1
    }
    pub fn pz(&self) -> f32 {
        self.pxpypz().2
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

    /// Chi<sup>2</sup> normalized to the number of clusters. This is a measure
    /// of how well the reconstruction fit the observed clusters
    pub fn tpc_chi2_per_cluster(&self) -> f32 {
        self.tpc_chi2 / f32::from(self.tpc_ncls)
    }

    /// Chi<sup>2</sup> normalized to the number of clusters. This is a measure
    /// of how well the reconstruction fit the observed clusters
    pub fn its_chi2_per_cluster(&self) -> f32 {
        self.its_chi2 / f32::from(self.its_ncls)
    }

    pub fn dca_to_other_track(&self, other: &Track, mag_field: f64) -> f64 {
	AliExternalTrackParam::from(self).get_dca(&other.into(), mag_field).0
    }
}

impl From<&Track> for AliExternalTrackParam {
    fn from(track: &Track) -> AliExternalTrackParam{
	unsafe {
	    AliExternalTrackParam::new(
		track.x as f64,
		track.alpha as f64,
		[
		    track.parameters.loc_y as f64,
		    track.parameters.loc_z as f64,
		    track.parameters.loc_sin as f64,
		    track.parameters.tang as f64,
		    track.parameters.one_over_pt as f64
		].as_ptr()
	    )
	}
    }
}
