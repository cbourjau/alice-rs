/// Default event and track selection (cuts in particle physics lingo)
use rand::{thread_rng, Rng};

use ::event::Event;
use ::track;
use ::trigger_mask;
use ::track_traits::{Longitude, TransverseMomentum};


/// A simple but reasonable default event selection
/// Returns true if the given event passes the recommended selection criterion
pub fn default_event_filter(event: &Event) -> bool {
    // Check if the event has a reconstructed primary vertex
    if let Some(ref pv) = event.primary_vertex {
        // Primary vertex must be within +- 8cm
        // of the nominal interaction point along beam axis
        if pv.z.abs() > 8.0 {
            return false;
        }
    } else {
        return false;
    }
    // Require some activity in the central region
    if event.multiplicity <= 0 {
        return false;
    }
    // Only use events which fired the minimu bias trigger
    if !event.trigger_mask.contains(trigger_mask::MINIMUM_BIAS) {
        return false;
    }
    true
}

/// Apply default track selection cuts
/// The returned Event contains only those tracks that passed the cuts
/// The cuts are partly inspired by those define around
/// AliESDtrackCuts.cxx:1366
pub fn filter_tracks(mut ev: Event) -> Event {
    {
        let pv = ev.primary_vertex.as_ref().expect("No primary vertex for found!");
        // see AliESDtrackCuts.cxx:1366
        ev.tracks = ev.tracks
            .into_iter()
            .filter(|tr| tr.flags.contains(track::ITS_REFIT))
            .filter(|tr| tr.flags.contains(track::TPC_REFIT))
            .filter(|tr| tr.dca_to_point_xy(pv.x, pv.y) < 2.4)
            .filter(|tr| tr.dca_to_point_z(pv.z) < 3.2)
            .filter(|tr| tr.eta().abs() < 1.0)
            .filter(|tr| tr.pt() > 0.15)
            .filter(|tr| tr.quality_tpc.n_clusters > 70)
            .filter(|tr| tr.quality_tpc.chi2_per_cluster() <= 4.0)
            .filter(|tr| tr.quality_its.chi2_per_cluster() <= 36.0)
            .collect();
    }
    // Shuffle selected tracks to avoid correlations from datataking orderings
    // Trust me, this is needed!
    thread_rng().shuffle(ev.tracks.as_mut_slice());
    ev
}
