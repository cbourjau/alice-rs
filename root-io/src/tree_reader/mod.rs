//! A convenience wrapper and needed parsers to work with ROOT's
//! `TTree`s. A Tree may be thought of as a table where each row
//! represents a particle collision. Each column may contain one or
//! several elements per collision. This module provides two Iterator
//! structs in order to iterate over these columns (`TBranches` in
//! ROOT lingo).

mod branch;
mod container;
mod leafs;
mod tree;

pub use self::tree::{ttree, Tree};

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use tokio;

    use super::ttree;
    use crate::core::RootFile;

    #[tokio::test]
    async fn simple_tree() {
        let path = PathBuf::from("./src/test_data/simple.root");
        let f = RootFile::new_from_file(&path)
            .await
            .expect("Failed to open file");
        f.items()[0].parse_with(ttree).await.unwrap();
    }

    #[test]
    fn api_test() {
        // Just a fake! Nothing is happening here
        quote! {
            let ntracks = tree.fbranches.get("Tracks")?.datacolumn_builder()
                .element_parser(be_u32)
                .iter()
                .collect();
            tree.fbranches.get("Tracks.fP")?.datacolumn_builder()
                .element_parser(count!(be_f32, 5))
                .elements_per_entry_counter(ntracks)
                .iter();
        };
        // Alternative:
        quote! {
            let f = RootFile::new_from_file(&path)?;
            let tree = f.items[0].as_tree()?;
            let tracks_table = tree["Tracks"].build_table()?;
            // Have the results in a one dim array
            let p: ndarray::Arraybase<f32, Ix2> = tracks_table["Tracks.fP"]
                // Iterator over events
                .iter()
                // parse the byte slice; perhaps one can re-export some nom parsers?
                // Otherwise, transmute or from_raw_slice?, or be_f32 & .chunk()?
                .map(|ev_slice: &[u8]| many0!(ev_slice, be_f32).unwrap().1)
                // If needed, create a 2D read-only view from the 1D slice
                .map(|v| ndarray::from_shape((v.len() / 5, 5), v)?);
                .next();

            let p = ndarray::from_shape((p.len() / 5, 5), p.as_slice())?;
        };
        // Macro magic
        quote! {
            let tracks_table = build_table!(tree, "Tracks");
            let p: Vec<[5; f32]> = tracks_table.fP.itre().next();
        };

        quote! {
            // rootrs print-model file.root --tree=esdTree --branches=Tracks.fAlpha,PrimaryVertex.fCov[3]
            use esd::MyModel; // Auto generated model for user selected branches
            let f = RootFile::new_from_file(&path)?;
            let tree = f.items[0].as_tree()?;
            let model = MyModel::new(tree);
            model.iter()
                .map(|ev| {
                    ev.tracks
                        .iter()
                        .map(|tr| tr.alpha)
                });
        };
    }
}
