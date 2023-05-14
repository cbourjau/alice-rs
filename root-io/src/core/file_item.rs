use failure::Error;
use nom::multi::length_value;

use crate::core::{checked_byte_count, decompress, Context, Source, TKeyHeader};
use crate::tree_reader::{ttree, Tree};

/// Describes a single item within this file (e.g. a `Tree`)
#[derive(Debug)]
pub struct FileItem {
    source: Source,
    tkey_hdr: TKeyHeader,
}

impl FileItem {
    /// New file item from the information in a TKeyHeader and the associated file
    pub(crate) fn new(tkey_hdr: &TKeyHeader, source: Source) -> FileItem {
        FileItem {
            source,
            tkey_hdr: tkey_hdr.to_owned(),
        }
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

    async fn get_buffer(&self) -> Result<Vec<u8>, Error> {
        let start = self.tkey_hdr.seek_key + self.tkey_hdr.key_len as u64;
        let len = self.tkey_hdr.total_size - self.tkey_hdr.key_len as u32;
        let comp_buf = self.source.fetch(start, len as u64).await?;

        let buf = if self.tkey_hdr.total_size < self.tkey_hdr.uncomp_len {
            // Decompress the read buffer; buf is Vec<u8>
            let (_, buf) = decompress(comp_buf.as_slice()).unwrap();
            buf
        } else {
            comp_buf
        };
        Ok(buf)
    }

    pub(crate) async fn get_context<'s>(&self) -> Result<Context, Error> {
        let buffer = self.get_buffer().await?;
        let k_map_offset = 2;
        Ok(Context {
            source: self.source.clone(),
            offset: (self.tkey_hdr.key_len + k_map_offset) as u64,
            s: buffer,
        })
    }

    /// Parse this `FileItem` as a `Tree`
    pub async fn as_tree(&self) -> Result<Tree, Error> {
        let ctx = self.get_context().await?;
        let buf = ctx.s.as_slice();

        let res = length_value(checked_byte_count, |i| ttree(i, &ctx))(buf);
        match res {
            Ok((_, obj)) => Ok(obj),
            Err(nom::Err::Error(e)) | Err(nom::Err::Failure(e)) => {
                Err(format_err!("Supplied parser failed! {:?}", e))
            }
            _ => panic!(),
        }
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use crate::core::RootFile;
    use std::path::Path;

    #[tokio::test]
    async fn open_simple() {
        let path = Path::new("./src/test_data/simple.root");
        let f = RootFile::new(path).await.expect("Failed to open file");
        assert_eq!(f.items().len(), 1);
        assert_eq!(f.items()[0].tkey_hdr.obj_name, "tree");
        // Only streamers; not rules
        assert_eq!(f.streamer_infos().await.unwrap().len(), 18);
    }

    #[tokio::test]
    #[cfg(not(target_arch = "wasm32"))]
    async fn open_esd() {
        use alice_open_data;
        let path = alice_open_data::test_file().unwrap();

        let f = RootFile::new(path.as_path())
            .await
            .expect("Failed to open file");

        assert_eq!(f.items().len(), 2);
        assert_eq!(f.items()[0].tkey_hdr.obj_name, "esdTree");
        assert_eq!(f.items()[1].tkey_hdr.obj_name, "HLTesdTree");
        assert_eq!(f.streamer_infos().await.unwrap().len(), 87);
    }
}
