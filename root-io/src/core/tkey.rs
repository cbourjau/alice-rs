use std::io::{SeekFrom};
use nom::*;
use ::core::*;

#[derive(Debug, Clone)]
pub struct TKeyHeader {
    pub(crate) total_size: u32,
    version: u16,
    pub(crate) uncomp_len: u32,
    datime: u32,
    pub(crate) key_len: i16,
    cycle: i16,
    pub(crate) seek_key: SeekFrom,
    seek_pdir: SeekFrom,
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

/// Header of a TKey
/// Usually, TKeys are followed up by their content, but there is one
/// "index" in ever root file where only the TKey headers are stored
/// for faster later `Seek`ing
named!(
    pub tkey_header<&[u8], TKeyHeader>,
    do_parse!(total_size: be_u32 >>
              version: be_u16 >>
              uncomp_len: be_u32 >>
              datime: be_u32 >>
              key_len: be_i16 >>
              cycle: be_i16 >>
              seek_key: apply!(seek_point, version) >>
              seek_pdir: apply!(seek_point, version) >>
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

named_args!(
    seek_point(version: u16)<SeekFrom>,
    map!(
        alt_complete!(
            cond_reduce!(version > 1000, be_u64) |
            be_u32 => {|v| u64::from(v)}),
        SeekFrom::Start
    )
);


/// Parse a full TKey including its payload
named!(
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
