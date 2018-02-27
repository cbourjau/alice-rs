use event::{Event, TriggerMask};
use track::{Track, Flags, ItsClusters};
use primary_vertex::PrimaryVertex;

/// A simple but reasonable default event selection
/// Returns true if the given event passes the recommended selection criterion
pub fn default_event_filter(event: &Event) -> bool {
    // Check if the event has a reconstructed primary vertex
    let good_vertex = match event.primary_vertex() {
        // Primary vertex must be within +- 10cm
        // of the nominal interaction point along beam axis
        Some(pv) => pv.z.abs() < 10.0,
        None => false,
    };
    good_vertex
    // Require some activity in the central region
        & (event.multiplicity() > 0.0)
    // Only use events which fired the minimu bias trigger
        & event.trigger_mask().contains(TriggerMask::MINIMUM_BIAS)
}

/// Applies a reasonable set of default track cuts returning `true` if
/// the `track` is valid
pub fn default_track_filter(tr: &Track, prime_vtx: &PrimaryVertex) -> bool {
    tr.flags.contains(Flags::ITS_REFIT)
        && tr.flags.contains(Flags::TPC_REFIT)
        // Distance of closest approach of this track to the primary
        // vertex in transverse plane [cm]
        && tr.dca_to_point_xy(prime_vtx.x, prime_vtx.y) < 2.4
        // Distance of closest approach of this track to the primary
        // vertex along beam axis [cm]
        && tr.dca_to_point_z(prime_vtx.z) < 3.2
        // Restrict tracks to good TPC coverage
        && tr.eta().abs() < 0.9
        // Minimal pT cut off
        && tr.pt() > 0.15
        // Minimal number of clusters in the TPC
        && tr.tpc_ncls > 70
        // Goodness of fit of this track wrt. the observed clusters; TPC
        && tr.tpc_chi2_per_cluster() <= 4.0
        // Goodness of fit of this track wrt. the observed clusters; ITS
        && tr.its_chi2_per_cluster() <= 36.0
}

/// So called hybrid tracks are sometimes used in order to achieve a
/// more uniform distribution of tracks in eta and phi. This function
/// cannot be used with the `default_track_filter` and might need more
/// debugging. Use with care.
pub fn is_hybrid_track(tr: &Track) -> bool {
    // SPD && ITS_REFIT
    tr.its_clustermap.intersects(ItsClusters::SPD_INNER | ItsClusters::SPD_OUTER)
        & tr.flags.contains(Flags::ITS_REFIT) ||
    // !SPD && ITS_REFIT
        !tr.its_clustermap.intersects(ItsClusters::SPD_INNER | ItsClusters::SPD_OUTER)
        & tr.flags.contains(Flags::ITS_REFIT) ||
    // !SPD && !ITS_REFIT
        !tr.its_clustermap.intersects(ItsClusters::SPD_INNER | ItsClusters::SPD_OUTER)
        & !tr.flags.contains(Flags::ITS_REFIT)
}
