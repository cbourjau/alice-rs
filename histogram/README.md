# Histogram

This crate provides a very minimalistic histogram. The idea is to have a persistent data structure which is continiously filled with a stream of data. It is a standard tool in statistics and thus in particle physics where most phenomena are described through probability distributions.

Currently, this crate is really not particularly sophisticated. I just needed something to do the counting. It would be nice if the histograms of this crate could be more closely related to their underlying `ndarray`s. Such that once can directly perform calculations on them, such as adding two histograms or summing all bins along an axis such that the dimensionality of the resulting histogram is reduced by one.
