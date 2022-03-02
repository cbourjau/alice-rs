//! A convenience wrapper and needed parsers to work with ROOT's
//! `TTree`s. A Tree may be thought of as a table where each row
//! represents a particle collision. Each column may contain one or
//! several elements per collision. This module provides two Iterator
//! structs in order to iterate over these columns (`TBranches` in
//! ROOT lingo).
pub use self::tree::{Tree, ttree};

mod branch;
mod container;
mod leafs;
pub mod tree;

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use tokio;

    use std::path::PathBuf;

    use crate::core::RootFile;
    use crate::core::UnwrapPrint;


    #[tokio::test]
    async fn simple_tree() {
        let path = PathBuf::from("./src/test_data/simple.root");
        let f = RootFile::new(path.as_path())
            .await
            .unwrap_print();
        f.items()[0].as_tree().await.unwrap_print();
    }
}
