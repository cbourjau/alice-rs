use alice_sys::ESD_t;
use std::f64::consts::PI;

use track_traits::{Azimuth, Longitude, TransverseMomentum};

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

/// An obscure set of parameters which makes sense for the actual
/// reconstruction of the tracks, but is a pain for subsequent
/// analysis
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
        let paras = paras.clone();
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
/// Note: If you want to enable more of thoes make sure to read the
/// activate the appropriate branch in the TTree!
#[derive(Debug)]
pub struct QualityTPC {
    // d: f64,
    // z: f64,
    // cdd: f64,
    // cdz: f64,
    // czz: f64,
    // cchi2: f64,
    chi2: f64,
    // chi2_iter1: f64,
    // signal: f64,
    // signal_s: f64,
    // points: [f64; 4],
    pub n_clusters: u16,
    // ncls_f: u16,
    // signal_n: u16,
    // ncls_iter1: u16,
    // ncls_f_iter1: u16,
}

impl QualityTPC {
    pub fn new_from_esd(esd: &ESD_t, idx: usize) -> QualityTPC {
        QualityTPC {
            // d: esd.Tracks_fdTPC[idx],
            // z: esd.Tracks_fzTPC[idx],
            // cdd: esd.Tracks_fCddTPC[idx],
            // cdz: esd.Tracks_fCdzTPC[idx],
            // czz: esd.Tracks_fCzzTPC[idx],
            // cchi2: esd.Tracks_fCchi2TPC[idx],
            chi2: esd.Tracks_fTPCchi2[idx],
            // chi2_iter1: esd.Tracks_fTPCchi2Iter1[idx],
            // signal: esd.Tracks_fTPCsignal[idx],
            // signal_s: esd.Tracks_fTPCsignalS[idx],
            // points: esd.Tracks_fTPCPoints[idx],
            n_clusters: esd.Tracks_fTPCncls[idx],
            // ncls_f: esd.Tracks_fTPCnclsF[idx],
            // signal_n: esd.Tracks_fTPCsignalN[idx],
            // ncls_iter1: esd.Tracks_fTPCnclsIter1[idx],
            // ncls_f_iter1: esd.Tracks_fTPCnclsFIter1[idx],
        }
    }
    pub fn chi2_per_cluster(&self) -> f64 {
        self.chi2 / self.n_clusters as f64
    }
}

/// A non-exhaustive list of quality attributs from the Inner Tracking System (ITS)
#[derive(Debug)]
pub struct QualityITS {
    /// Quality of fit to the used clusters (?)
    chi2: f64,
    /// Total number of clusters used for this track (?)
    n_clusters: i8,
    /// Layers of the ITS which had clusters
    pub clusters_on_layer: ItsClusters,
}

impl QualityITS {
    pub fn new_from_esd(esd: &ESD_t, idx: usize) -> QualityITS {
        QualityITS {
            chi2: esd.Tracks_fITSchi2[idx],
            n_clusters: esd.Tracks_fITSncls[idx],
            clusters_on_layer: ItsClusters::from_bits(esd.Tracks_fITSClusterMap[idx] as u8)
                .expect(&format!("Got unexpectd ITS cluster map {:?}",
                                 esd.Tracks_fITSClusterMap[idx])),
        }
    }
    pub fn chi2_per_cluster(&self) -> f64 {
        self.chi2 / self.n_clusters as f64
    }    
}

/// Bundling of data commonly associated with a given track
#[derive(Debug)]
pub struct Track {
    // So called external track parameters
    parameters: TrackParameters,
    x: f64,
    alpha: f64,
    // Flags set for this track; wrapped with bitflag class for safety
    pub flags: Flags,
    pub quality_tpc: QualityTPC,
    pub quality_its: QualityITS,
}

impl Track {
    /// Create a humanly useful track from the "external" track parameters
    /// This is copied from AliExternalTrackParam.cxx
    /// Returns None if the track had 1/pt <= 0
    pub fn read_tracks_from_esd(esd: &ESD_t) -> Vec<Track> {
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
                    quality_its: QualityITS::new_from_esd(esd, i),
                }
            )
        }
        tracks
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
        (diff_x * loc_sin - diff_y * ((1. - loc_sin) * (1. + loc_sin)).sqrt()).abs()
    }
    /// Distance of closes approch of this track in z
    pub fn dca_to_point_z(&self, z: f64) -> f64 {
        self.parameters.loc_z - z
    }
}

impl Azimuth for Track {
    fn phi(&self) -> f64 {
        let mut phi = self.parameters.loc_sin.asin() + self.alpha;
        if phi < 0. {
            phi += 2. * PI;
        } else if phi >= 2. * PI {
            phi -= 2. * PI;
        }
        phi
    }
}

impl Longitude for Track {
    fn theta(&self) -> f64 {
        // 0.5*TMath::Pi() - TMath::ATan(fP[3]);
        0.5 * PI - self.parameters.tang.atan()
    }
}

impl TransverseMomentum for Track {
    fn pt(&self) -> f64 {
        1.0 / self.parameters.one_over_pt.abs()
    }
}
