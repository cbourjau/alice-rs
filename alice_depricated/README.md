alice
=====

The high level wrapper around `alice-sys`. The idea is to wrap the clunky concept of a "ROOT" tree into an `Iterator` over `Event`s.
An analysis is then implemented in a "map-reduce" fashion over this iterator.

The clunky object exposed by `alice-sys` is called `ESD_t` in this crate. That object is essentially a large struct which is constantly overwritten whenever the next "event" is read from disk. The idea is therefore to:

1. Call `load_event` on the `ESD_t` object to have the latest event in memory
2. Create a thin wrapper around it called `ESD` (this might actually not be necessary?)
3. Create a new struct called `Event` based on the currently loaded data. `Event` is guaranteed to not change when the IO thread calls `load_event` the next time.
	
`Event` `impl`s a set of traits which may also be implemented in the future by event-like structs which, for example, were created from collisions in a different experiment. An analysis should idealy only interface with the data through the "event_traits" (and "track_traits").

An important part of this crate is the `Dataset`. A dataset is essentially an iterator over the entire sample of events. Internally, each `Dataset` is nothing but a `chan::Reciever`, which is fed with events from `N` parallel IO threads. A `Dataset` may be cloned, but each clone will iterate over a disjunct set of events. Thus, one can, for example, clone a `dataset_a` such that we have `dataset_a` and `dataset_b`. We can now run our analysis in parallel on both datasets, yielding two results `result_a` and `result_b`. The analysis may now implement the `Merge` trait which allows us to combine the two results into one final one. Note that this is distictly different from `rayon`'s ParallelIterator. An analysis might be long running and memory intensive. Thus it is important to only instatiate a fixed number of analyses. As far as I can tell, this is not exactly `rayon`'s use case. (`rayon`'s `join` is used internally, though).

This process is simplified for the user. The user only has to "install" a given analysis into `dataset_a` without any explicit cloning. Internally, the analysis will be split over `M` chunks of the dataset which are run in parallel. The return value of the `Dataset`'s `install` function is the merged result of the analysis.

See the `simple-analysis` crate for an example on how to use this crate.
