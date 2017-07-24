use alice_sys::ESD;
use std::f64::consts::PI;

bitflags! {
    /// Trackflags based on AliVTrack
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
struct TrackParameters {
    loc_y: f64,
    loc_z: f64,
    loc_sin: f64,
    tang: f64,
    one_over_pt: f64,
}

impl TrackParameters {
    fn new(paras: &[f64; 5usize]) -> TrackParameters {
        TrackParameters {
            loc_y: paras[0],
            loc_z: paras[1],
            loc_sin: paras[2],
            tang: paras[3],
            one_over_pt: paras[4],
        }
    }
}

/// Things related to the quality of TPC tracks (incomplete)
#[derive(Debug)]
pub struct QualityTPC {
    d: f64,
    z: f64,
    cdd: f64,
    cdz: f64,
    czz: f64,
    cchi2: f64,
    chi2: f64,
    chi2_iter1: f64,
    signal: f64,
    signal_s: f64,
    points: [f64; 4],
    pub ncls: u16,
    ncls_f: u16,
    signal_n: u16,
    ncls_iter1: u16,
    ncls_f_iter1: u16,
}

impl QualityTPC {
    pub fn new_from_esd(esd: &ESD, idx: usize) -> QualityTPC {
        QualityTPC {
            d: esd.Tracks_fdTPC[idx],
            z: esd.Tracks_fzTPC[idx],
            cdd: esd.Tracks_fCddTPC[idx],
            cdz: esd.Tracks_fCdzTPC[idx],
            czz: esd.Tracks_fCzzTPC[idx],
            cchi2: esd.Tracks_fCchi2TPC[idx],
            chi2: esd.Tracks_fTPCchi2[idx],
            chi2_iter1: esd.Tracks_fTPCchi2Iter1[idx],
            signal: esd.Tracks_fTPCsignal[idx],
            signal_s: esd.Tracks_fTPCsignalS[idx],
            points: esd.Tracks_fTPCPoints[idx],
            ncls: esd.Tracks_fTPCncls[idx],
            ncls_f: esd.Tracks_fTPCnclsF[idx],
            signal_n: esd.Tracks_fTPCsignalN[idx],
            ncls_iter1: esd.Tracks_fTPCnclsIter1[idx],
            ncls_f_iter1: esd.Tracks_fTPCnclsFIter1[idx],
        }
    }
}

#[derive(Debug)]
pub struct Track {
    // So called external track parameters
    parameters: TrackParameters,
    x: f64,
    alpha: f64,
    // Flags set for this track; wrapped with bitflag class for safety
    pub flags: Flags,
    pub quality_tpc: QualityTPC,
}

impl Track {
    /// Create a humanly useful track from the "external" track parameters
    /// This is copied from AliExternalTrackParam.cxx
    /// Returns None if the track had 1/pt <= 0
    pub fn read_tracks_from_esd(esd: &ESD) -> Vec<Track> {
        let mut tracks = Vec::<Track>::new();
        let n_tracks = esd.Tracks_ as usize;
        for i in 0..n_tracks {
            tracks.push(
                Track {
                    parameters: TrackParameters::new(&esd.Tracks_fP[i]),
                    x: esd.Tracks_fX[i],
                    alpha: esd.Tracks_fAlpha[i],
                    flags: Flags::from_bits(esd.Tracks_fFlags[i])
                        .expect("Unknown flag observed!"),
                    quality_tpc: QualityTPC::new_from_esd(esd, i),
                }
            )
        }
        tracks
    }

    pub fn phi(&self) -> f64 {
        let mut phi = self.parameters.loc_sin.asin() + self.alpha;
        if phi < 0. {
            phi += 2. * PI;
        } else if phi >= 2. * PI {
            phi -= 2. * PI;
        }
        return phi;
    }
    pub fn theta(&self) -> f64 {
        // 0.5*TMath::Pi() - TMath::ATan(fP[3]);
        0.5 * PI - self.parameters.tang.atan()
    }
    pub fn eta(&self) -> f64 {
        // -TMath::Log(TMath::Tan(0.5 * Theta()))
        -((0.5 * self.theta()).tan()).ln()
    }
    pub fn pt(&self) -> f64 {
        1.0 / self.parameters.one_over_pt.abs()
    }
    /// Estimate the distance of closest approach of this track to a given point
    /// neglecting the track curvature. This returns the closest approach in the xy plane
    pub fn dca_to_point_xy(&self, x: f64, y: f64) -> f64 {
        // Double_t sn=TMath::Sin(alpha), cs=TMath::Cos(alpha);
        // x= GetX(), y=GetParameter()[0], snp=GetParameter()[2];
        let xv = x * self.alpha.cos() + y * self.alpha.sin();
        let yv = -x * self.alpha.sin() + y * self.alpha.cos();
        let diff_x = self.x - xv;
        let diff_y = self.parameters.loc_y - yv;
        let loc_sin = self.parameters.loc_sin;
        let ret = (diff_x * loc_sin
                   - diff_y * ((1. - loc_sin) * (1. + loc_sin)).sqrt()).abs();
        ret
    }
    // Distance of closes approch of this track in z
    pub fn dca_to_point_z(&self, z: f64) -> f64 {
        self.parameters.loc_z - z
    }
}
