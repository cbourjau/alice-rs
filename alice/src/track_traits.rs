/// Traits which may be implemented by tracks/tracklets/hits...


pub trait Azimuth {
    /// The direction of a track in the azimuthal direction `phi`
    /// This function wraps around the interval [0, 2pi)
    fn phi(&self) -> f64;
}

pub trait Longitude {
    /// Direction of a track in the spherical coordinate  `theta`
    fn theta(&self) -> f64;
    /// Direction of a track in pseudorapidity `eta`
    fn eta(&self) -> f64 {
        // -TMath::Log(TMath::Tan(0.5 * Theta()))
        -((0.5 * self.theta()).tan()).ln()
    }
}

pub trait TransverseMomentum {
    /// Transverse momentum of the given track
    fn pt(&self) -> f64;
}
