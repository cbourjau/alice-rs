# Convert subset of data to msgpack

This example demonstrates how to convert a small subset of the Open Data to the [`msgpack`](https://msgpack.org/) format.
This might be useful as a quick and dirty way to analyze some data in Python (e.g. with [`msgpack-python`](https://github.com/msgpack/msgpack-python) or some other language.
The file `./analyze.py` demonstrates how a subsequent analysis in Python may look like.

To create the binary data file you to need:

1. [Install Rust](https://rustup.rs/)
2. Download some files using [`alice-download`](../../alice-download/README.md) from this repository
3. Run this example with `cargo run --release` (in this folder)
	
