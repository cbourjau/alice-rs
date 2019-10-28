use core::*;
use nom::*;
use nom::number::complete::*;
use nom::combinator::map;

#[derive(Debug, Clone)]
pub struct TKeyHeader {
    pub(crate) total_size: u32,
    version: u16,
    pub(crate) uncomp_len: u32,
    datime: u32,
    pub(crate) key_len: i16,
    cycle: i16,
    pub(crate) seek_key: SeekPointer,
    seek_pdir: SeekPointer,
    pub(crate) class_name: String,
    pub(crate) obj_name: String,
    obj_title: String,
}

/// A `TKey` wraps a streamed oject. The object is decompress when
/// reading from disc if applicable.
#[derive(Debug)]
pub struct TKey {
    pub(crate) hdr: TKeyHeader,
    pub(crate) obj: Vec<u8>,
}

named!(
    #[doc=r#"Header of a TKey Usually, TKeys are followed up by their
content, but there is one "index" in ever root file where only the
TKey headers are stored for faster later `Seek`ing"#],
    pub tkey_header<&[u8], TKeyHeader>,
    do_parse!(total_size: be_u32 >>
              version: be_u16 >>
              uncomp_len: be_u32 >>
              datime: be_u32 >>
              key_len: be_i16 >>
              cycle: be_i16 >>
              seek_key: call!(seek_point, version) >>
              seek_pdir: call!(seek_point, version) >>
              class_name: string >>
              obj_name: string >>
              obj_title: string >>
              (TKeyHeader {
                  total_size: total_size,
                  version: version,
                  uncomp_len: uncomp_len,
                  datime: datime,
                  key_len: key_len,
                  cycle: cycle,
                  seek_key: seek_key,
                  seek_pdir: seek_pdir,
                  class_name: class_name,
                  obj_name: obj_name,
                  obj_title: obj_title,
              })
    )
);


/// Parse a file-pointer based on the version of the file
fn seek_point(input: &[u8], version: u16) -> nom::IResult<&[u8], u64> {
    if version > 1000 {
        be_u64(input)
    } else {
        map(be_u32, u64::from)(input)
    }
}

named!(
    #[doc="Parse a full TKey including its payload"],
    pub tkey<&[u8], TKey>,
    do_parse!(hdr: tkey_header >>
              obj: take!(hdr.total_size - hdr.key_len as u32) >>
              ({
                  let obj = if hdr.uncomp_len as usize > obj.len() {
                      (decompress(obj).unwrap().1)
                  } else {
                      obj.to_vec()
                  };
                  TKey {hdr, obj}
              })
    )
);

/// Special thing for the keylist in the file header
pub(crate) fn tkey_headers(input: &[u8]) -> IResult<&[u8], Vec<TKeyHeader>> {
    length_count!(input, be_i32, tkey_header)
}
