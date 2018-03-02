# simple-analysis
This crate demonstrates how all the other parts of this repository work together.
The `main.rs` shows how the IO part is spawned in its own thread. That thread sends `malice::Event`as messages. The reciever is converted into an iterator over `Event`s.
The analysis itself is implemented as a `fold` over this iterator.
Note that this set up can easily be adapted to have `M` IO threads and `N` consuming analysis threads.
The `crossbeam_channel` crate is a good fit for such a `mpmc` approach. 

The analysis itself should probably only consume events fitting some selection criteria.
A reasonable event selection is provided by `malice::default_events_filter`.

Within the analysis, one probably wants to filter the reconstructed tracks as well. Again, `malice` provides a reasonable default `malice::default_track_filter`.

The processing of one stream is seen in the function `single_distribution_analysis` in `main.rs`.
The very first step of every analysis is the validation of the event. Oftentimes, one is only interested in event which fulfill very particular criteria. In this case, we just use a set of standard criteria (`cuts` in particle physics lingo) provided by the `alice` crate.

This example analysis also visualized the results using `gnuplot-rs`.
The below figures are the result of this analysis.
The top two figures show the distribution of particles in the longitudinal (`eta`) and azimuthal (`phi`) dimension.
The bottom figure shows the distribution of where exactly the collisions took place within the detector. Namely, The collisions may be slightly displaced from the center of the detector along the beam axis.

![result-plot](./result.png)
