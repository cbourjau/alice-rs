use failure::Error;
use nom::combinator::rest;
use nom::number::complete::*;
use nom::*;

use crate::core::*;

#[derive(Debug, Clone)]
pub(crate) enum Container {
    /// Decompressed content of a `TBasket`
    InMemory(Vec<u8>),
    /// Filename, start byte, and len of a `TBasket` on disk
    OnDisk(Source, u64, u64),
}

impl Container {
    /// Return the number of entries and the data; reading it from disk if necessary
    pub(crate) async fn raw_data(self) -> Result<(u32, Vec<u8>), Error> {
        let buf = match self {
            Container::InMemory(buf) => buf,
            Container::OnDisk(source, seek, len) => source.fetch(seek, len).await?,
        };
        match tbasket2vec(buf.as_slice()) {
            Ok((_, v)) => Ok(v),
            _ => Err(format_err!("tbasket2vec parser failed")),
        }
    }
    // /// For debugging: Try to find the file of this container. Out of luck if the container was inlined
    // pub(crate) fn file(&self) -> Option<PathBuf> {
    //     match *self {
    //         // No file name available
    //         Container::InMemory(_) => None,
    //         Container::OnDisk(ref p, _, _) => Some(p.to_owned())
    //     }
    // }
}

/// Return a tuple indicating the number of elements in this basket
/// and the content as a Vec<u8>
fn tbasket2vec(input: &[u8]) -> IResult<&[u8], (u32, Vec<u8>)>
{
    do_parse!(input,
              hdr: tkey_header >>
              _ver: be_u16 >>
              _buf_size: be_u32 >>
              _entry_size: be_u32 >>
	      n_entry_buf: be_u32 >>
	      last: be_u32 >>
	      _flag: be_i8 >>
              buf: rest >>
              ({
                  let buf = if hdr.uncomp_len as usize > buf.len() {
                      decompress(buf).unwrap().1
                  } else {
                      buf.to_vec()
                  };
                  // Not the whole buffer is filled, no, no, no, that
                  // would be to easy! Its only filled up to `last`,
                  // whereby we have to take the key_len into account...
                  let useful_bytes = (last - hdr.key_len as u32) as usize;
                  (n_entry_buf, buf.as_slice()[..useful_bytes].to_vec())
              }))
}

#[cfg(test)]
mod tests {
    use crate::core::tkey_header;
    use nom::*;
    use std::fs::File;
    use std::io::{BufReader, Read, Seek, SeekFrom};

    use super::tbasket2vec;

    #[test]
    fn basket_simple() {
        let path = "./src/test_data/simple.root";
        let f = File::open(&path).unwrap();
        let mut reader = BufReader::new(f);
        // Go to first basket
        reader.seek(SeekFrom::Start(218)).unwrap();
        // size from fbasketbytes
        let mut buf = vec![0; 86];
        // let mut buf = vec![0; 386];
        reader.read_exact(&mut buf).unwrap();

        println!("{}", buf.to_hex(16));
        println!("{:?}", tkey_header(&buf));
        // println!("{:#?}", tbasket(&buf, be_u32));
        println!("{:#?}", tbasket2vec(&buf));
    }

    // /// Test the first basket of the "Tracks.fP[5]" branch
    // #[test]
    // fn basket_esd() {
    //     // This test is broken since the numbers were hardcoded for a specific file
    //     use alice_open_data;
    //     let path = alice_open_data::test_file().unwrap();

    //     let f = File::open(&path).unwrap();
    //     let mut reader = BufReader::new(f);
    //     // Go to first basket
    //     reader.seek(SeekFrom::Start(77881)).unwrap();
    //     // size from fbasketbytes
    //     let mut buf = vec![0; 87125];
    //     reader.read_exact(&mut buf).unwrap();

    //     println!("{:?}", tkey_header(&buf).unwrap().1);
    //     // println!("{:#?}", tbasket(&buf, |i| count!(i, be_f32, 15)).unwrap().1);
    //     println!("{:#?}", tbasket2vec(&buf));
    // }
}
