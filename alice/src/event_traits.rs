/// Various traits which might be implemented by an "event"
/// Events might be stored in very different formats. The main
/// difference between such formats is usually that the very fine
/// grained details about some detectors have been left out to safe
/// disk space. Therefore, some kind of events might offer more
/// details than others. The only type of event published by ALICE
/// thus far are in the Event Summary Data (ESD). But others exists
/// and might be published in the future.
/// Furthermore, one might want to implmented different types in this
/// crate as well, Since different analyses will always only require a
/// subset of an ESD.
use primary_vertex;

pub trait Tracks<T> {
    /// An iterable over some sort of reconstructed track
    fn tracks(&self) -> &Vec<T>;
}

pub trait PrimaryVertex {
    /// The primary vertex of an event. This is the point where the
    /// collision is most likely to have occured based on the
    /// reconstructed tracks.
    /// Returns `None` if no primary vertex was reconstructed for this event.
    fn primary_vertex(&self) -> Option<&primary_vertex::PrimaryVertex>;
}

pub trait Multiplicity {
    /// The multiplicity of the event
    /// There are many different estimators for the multiplicity, so I
    /// think this part of the API will need to change very soon!
    fn multiplicity(&self) -> f64;
}
