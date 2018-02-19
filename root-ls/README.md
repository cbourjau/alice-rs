# root-ls

[![Crates.io Version](https://img.shields.io/crates/v/root-ls.svg)](https://crates.io/crates/root-ls)


A command line tool to inspect the types of objects contained in a `.root` file similar to ROOT's `TFile::ShowStreamerInfo()` function. However, `root-ls` is also able to produce (proably buggy) Rust code as a starting point to write a custom parser for the content of a file. If you are in that sort of business, you should take a look at the [`root-io`](https://crates.io/crates/root-io) crate.

## Installation
1. Get Rust via [rustup](https://rustup.rs/)
2. Install `root-ls` 

``` bash
cargo install root-ls
```

## Usage
- Dump the layout of the streamed objects as yaml
``` bash
root-ls ./simple.root to-yaml
```

- Create rust structs and parsers for the objects in this file; formatting the code with rustfmt
``` bash
root-ls ./simple.root to-rust --rustfmt

```

- Print a short summary of all the items in this file
``` bash
root-ls ./simple.root to-rust inspect
```

- Dump all the info there is on one particular item. Not pretty, but most precise (especially with optional `-v`)
``` bash
root-ls ./simple.root to-rust inspect --item-pos=0 -v
```
