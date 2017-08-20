ALICE-rs
========

This repository demonstrates how the public data released by the the CERN based ALICE collaboration can be analysed with the Rust programming language and minimal external dependencies.

*** This is not an official ALICE or CERN project ***

This collection of crates provides and demonstrates:

* Reading of so-called Event-summary-data (ESD) from ROOT's binary file format
  This is achieved through automatically generated bindings from the ROOT (MakeClass) as well as from the Rust side (bindgen). See `alice-sys`.
* A safe wrapper around a limited subset of the automatically generated bindings.
  These bindings enable a convinient asynchronous IO and re-organize the data with a rustic interface. Alternative backends like databases would be integrated at this this level.
* High performace n-dimensional histograms for streaming data
  Maintains a binned count of the data which can be successivly filled.
* Example analysis reproducng published results
  Using all of the above crates, an example analysis is implemented showcasting a clean and highly efficient API.

Dependencies
------------
There are no dependencies on any ALICE specific software. However, data was released in the ROOT framework's binary data format. Therefore, a working ROOT 5 or 6 installation is required for the IO process.

Furthermore, various crates from the rust ecosystem have been leveraged. Most notably gcc-rs, bindgen, and ndarray.


CERN, LHC, and ALICE
--------------------

ALICE is the dedicated Heavy Ion experiment at the Large Hadron Collider (LHC) in Switzerland. Just like most other CERN based experiments, its goal is to better understand the fundamental particles and forces of the universe. In particular, ALICE is concerned with the so called strong force which is the dominant actor for processes within an atomic nuclei. Many of the properties of this force manifest them self at extreme pressures and temperatures as they were found micro seconds after the Big Bang. By colliding heavy nuclei such as lead ions at nearly the speed of light, we are able to recreate such extreme conditions for a very brief moment at the LHC within the ALICE detector. By carefully studying the particles produced at such collisions we can deduce the properties of the strong force which will help us to better understand nuclear reactions, neutron stars, the first moments of the universe and much more.
