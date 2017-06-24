use ffi;
use std::f64::consts::PI;

bitflags! {
    /// Trackflags based on AliVTrack
    pub struct Flags: u32 {
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
        const HMPID_OUT = 0x10000;
        const HMPID_PID = 0x20000;
        const EMCAL_MATCH = 0x40000;
        const TRD_BACKUP = 0x80000;
        const TOF_MISMATCH = 0x100000;
        const PHOS_MATCH = 0x200000;
        const ITS_UPG = 0x400000;
        const SKIP_FRIEND = 0x800000;
        const GLOBAL_MERGE = 0x1000000;
        const MULT_IN_V0 = 0x2000000;
        const MULT_SEC = 0x4000000;
        const EMBEDDED = 0x8000000;
        const ITS_PURE_SA = 0x10000000; 
        const TRDS_TOP = 0x20000000; 
        const ESD_PID = 0x40000000; 
        const TIME = 0x80000000;
    }
}

#[derive(Debug)]
pub struct Track {
    // The raw ESD track data
    esd_track: ffi::track_t,
    // Flags set for this track; wrapped with bitflag class for safety
    pub flags: Flags,
}

impl Track {
    /// Create a humanly useful track from the "external" track parameters
    /// This is copied from AliExternalTrackParam.cxx
    /// Returns None if the track had 1/pt <= 0
    pub fn read_tracks_from_esd(esd: *const ffi::CEsd) -> Vec<Track> {
        let n_tracks = unsafe {ffi::get_n_tracks(esd)};
        let mut esd_tracks = vec![<ffi::track_t>::new(); n_tracks];
        unsafe {ffi::get_ext_tracks_parameters(esd, esd_tracks.as_mut_ptr(), n_tracks)}
        esd_tracks.into_iter()
            .map(|t| {
                Track {
                    esd_track: t,
                    flags: Flags::from_bits(t.flags).expect("Unknown flag observed!"),
                }  
            })
            .collect()
    }
    pub fn phi(&self) -> f64 {
        let mut phi = self.esd_track.ext_track_paras.loc_sin.asin() + self.esd_track.alpha;
        if phi < 0. {
            phi += 2. * PI;
        } else if phi >= 2. * PI {
            phi -= 2. * PI;
        }
        return phi;
    }
    pub fn theta(&self) -> f64 {
        // 0.5*TMath::Pi() - TMath::ATan(fP[3]);
        0.5 * PI - self.esd_track.ext_track_paras.tang.atan()
    }
    pub fn eta(&self) -> f64 {
        // -TMath::Log(TMath::Tan(0.5 * Theta()))
        -((0.5 * self.theta()).tan()).ln()
    }
    pub fn pt(&self) -> f64 {
        1.0 / self.esd_track.ext_track_paras.one_over_pt.abs()
    }
    /// Estimate the distance of closest approach of this track to a given point
    /// neglecting the track curvature. This returns the closest approach in the xy plane
    pub fn dca_to_point_xy(&self, x: f64, y: f64) -> f64 {
        // Double_t sn=TMath::Sin(alpha), cs=TMath::Cos(alpha);
        // x= GetX(), y=GetParameter()[0], snp=GetParameter()[2];
        let xv = x * self.esd_track.alpha.cos() + y * self.esd_track.alpha.sin();
        let yv = -x * self.esd_track.alpha.sin() + y * self.esd_track.alpha.cos();
        let diff_x = self.esd_track.x - xv;
        let diff_y = self.esd_track.ext_track_paras.loc_y - yv;
        let loc_sin = self.esd_track.ext_track_paras.loc_sin;
        let ret = (diff_x * loc_sin
                   - diff_y * ((1. - loc_sin) * (1. + loc_sin)).sqrt()).abs();
        ret
    }
    // Distance of closes approch of this track in z
    pub fn dca_to_point_z(&self, z: f64) -> f64 {
        self.esd_track.ext_track_paras.loc_z - z
    }
}

impl ffi::track_t {
    pub fn new() -> ffi::track_t {
        ffi::track_t {ext_track_paras: ffi::ext_track_parameters_t {loc_y: 0.,
                                                                    loc_z: 0.,
                                                                    loc_sin: 0.,
                                                                    tang: 0.,
                                                                    one_over_pt: 0.},
                      alpha: 0.,
                      flags: 0,
                      x: 0.,
        }
    }
}
