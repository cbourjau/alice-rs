use std::rc::Rc;

use failure::Error;
use nom::*;

use core::{checked_byte_count, decompress};
use core::{Context, DataSource, TKeyHeader};
use tree_reader::{ttree, Tree};

/// Describes a single item within this file (e.g. a `Tree`)
#[derive(Debug)]
pub struct FileItem {
    source: Rc<dyn DataSource>,
    tkey_hdr: TKeyHeader,
}

impl FileItem {
    /// New file item from the information in a TKeyHeader and the associated file
    pub(crate) fn new<S: DataSource + 'static>(tkey_hdr: &TKeyHeader, source: Rc<S>) -> FileItem {
        FileItem {
            source: source,
            tkey_hdr: tkey_hdr.to_owned(),
        }
    }

    /// Parse this item as a `Tree`. Instead one could also call
    /// `parse_with` with the `ttree` parser
    pub fn as_tree(&self) -> Result<Tree, Error> {
        self.parse_with(ttree)
    }

    /// Information about this file item in Human readable form
    pub fn verbose_info(&self) -> String {
        format!("{:#?}", self.tkey_hdr)
    }
    pub fn name(&self) -> String {
        format!(
            "`{}` of type `{}`",
            self.tkey_hdr.obj_name, self.tkey_hdr.class_name
        )
    }

    /// Read (and possibly decompress) data from disk and parse it as
    /// the appropriate type using the TStreamerInfo types.
    /// The return type of the parser function must not contain a
    /// reference to the parsed buffer
    pub(crate) fn parse_with<O, F>(&self, parser: F) -> Result<O, Error>
    where
        F: for<'s> Fn(&'s [u8], &'s Context<'s>) -> IResult<&'s [u8], O>,
    {
        let start = self.tkey_hdr.seek_key + self.tkey_hdr.key_len as u64;
        let len = self.tkey_hdr.total_size - self.tkey_hdr.key_len as u32;
        let comp_buf = self.source.fetch(start, len as u64)?;

        let buf = {
            if self.tkey_hdr.total_size < self.tkey_hdr.uncomp_len {
                // Decompress the read buffer; buf is Vec<u8>
                let (_, buf) = decompress(comp_buf.as_slice()).unwrap();
                buf
            } else {
                comp_buf
            }
        };
        let s = buf.as_slice();
        let k_map_offset = 2;
        let context = Context {
            source: self.source.clone(),
            offset: (self.tkey_hdr.key_len + k_map_offset) as u64,
            s,
        };
        // wrap parser in a byte count
        let res = length_value!(s, checked_byte_count, call!(&parser, &context));
        match res {
            Ok((_, obj)) => Ok(obj),
            _ => Err(format_err!("Supplied parser failed!")),
        }
    }
}

#[cfg(test)]
mod tests {
    use core::RootFile;
    use std::path::PathBuf;

    #[test]
    fn open_simple() {
        let path = PathBuf::from("./src/test_data/simple.root");
        let f = RootFile::new_from_file(&path).expect("Failed to open file");
        assert_eq!(f.items().len(), 1);
        assert_eq!(f.items()[0].tkey_hdr.obj_name, "tree");
        // Only streamers; not rules
        assert_eq!(f.streamers().unwrap().len(), 18);
    }

    // Skip this test on MacOs since the downloaded file is not working on Travis
    #[test]
    #[cfg(not(target_os="macos"))]
    fn open_esd() {
        use alice_open_data;
        let path = alice_open_data::test_file().unwrap();

        let f = RootFile::new_from_file(&path).expect("Failed to open file");

        assert_eq!(f.items().len(), 2);
        assert_eq!(f.items()[0].tkey_hdr.obj_name, "esdTree");
        assert_eq!(f.items()[1].tkey_hdr.obj_name, "HLTesdTree");
        assert_eq!(f.streamers().unwrap().len(), 87);
    }
}
