# root-io

`root-io` provides basic support for reading data stored in binary `.root` files commonly used in particle physics experiments. This crates provides:

  - Core types and parsers to read the layout description of custom classes contained in a given file
  - Tools to generate `yaml` describing the streamed objects (aka. `TStreamerInfo`)
  - Tools to generate (buggy) `Rust` code as a starting point for a new parser
  - Set of types and parsers needed to read so-called `TTree`s
  
The majority of the exposed API serves the latter point; striving to enable an easy iteration over data stored in `TTree`s. In particular, `root-io` supports reading `TBranches` (i.e. akin to "columns" of a database) with a variable number of elements in each entry (i.e. `TBranches` of `TClonesArray`).

The `root-ls` crate utilizes this crate to in a CLI to inspect a given root file and to deploy the code-gen tools.
  
  
