# simple-analysis
This crate demonstrates how all the other parts of this repository work together.

## How to run it
First, download a few files using [`alice-download`](https://github.com/cbourjau/alice-rs/tree/master/alice-download).
Then compile and run the analysis in **release** mode from the `simple-analysis` folder
```shell
alice-download 5  # Downloads 5GB to ~/lhc_open_data
cargo run --release
```
Note that this analysis attempts to produce some figures using Gnuplot at the end. So make sure you have gnuplot installed as well.

# What is happening?

The `main.rs` shows how the IO part is spawned in its own thread. That thread sends `malice::Event`as messages. The reciever is converted into an iterator over `Event`s.
The analysis itself is implemented as a `fold` over this iterator.
Note that this set up can easily be adapted to have `M` IO threads and `N` consuming analysis threads.
The `crossbeam_channel` crate is a good fit for such a `mpmc` approach. 

The analysis itself should probably only consume events fitting some selection criteria.
A reasonable event selection is provided by `malice::default_events_filter`.

Within the analysis, one probably wants to filter the reconstructed tracks as well. Again, `malice` provides a reasonable default `malice::default_track_filter`.

This example analysis also visualizes the results using the `gnuplot-rs` crate.
The below figures are the result of this analysis.
The top two figures show the distribution of particles in the longitudinal (`eta`) and azimuthal (`phi`) dimension.
The bottom figure shows the distribution of where exactly the collisions took place within the detector. Namely, The collisions may be slightly displaced from the center of the detector along the beam axis.

![result-plot](./result.png)
