use nom::{
    bytes::complete::take, combinator::map, multi::length_count, number::complete::*, IResult,
};

use crate::core::*;

#[derive(Debug, Clone)]
#[allow(dead_code)]
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

/// Header of a TKey Usually, TKeys are followed up by their
/// content, but there is one "index" in ever root file where only the
/// TKey headers are stored for faster later `Seek`ing
pub fn tkey_header(input: &[u8]) -> nom::IResult<&[u8], TKeyHeader> {
    let (input, total_size) = be_u32(input)?;
    let (input, version) = be_u16(input)?;
    let (input, uncomp_len) = be_u32(input)?;
    let (input, datime) = be_u32(input)?;
    let (input, key_len) = be_i16(input)?;
    let (input, cycle) = be_i16(input)?;
    let (input, seek_key) = seek_point(input, version)?;
    let (input, seek_pdir) = seek_point(input, version)?;
    let (input, class_name) = string(input)?;
    let (input, obj_name) = string(input)?;
    let (input, obj_title) = string(input)?;
    Ok((
        input,
        TKeyHeader {
            total_size,
            version,
            uncomp_len,
            datime,
            key_len,
            cycle,
            seek_key,
            seek_pdir,
            class_name,
            obj_name,
            obj_title,
        },
    ))
}

/// Parse a file-pointer based on the version of the file
fn seek_point(input: &[u8], version: u16) -> nom::IResult<&[u8], u64> {
    if version > 1000 {
        be_u64(input)
    } else {
        map(be_u32, u64::from)(input)
    }
}

/// Parse a full TKey including its payload
pub fn tkey(input: &[u8]) -> nom::IResult<&[u8], TKey> {
    let (input, hdr) = tkey_header(input)?;
    let (input, obj) = take(hdr.total_size - hdr.key_len as u32)(input)?;
    let obj = if hdr.uncomp_len as usize > obj.len() {
        decompress(obj).unwrap().1
    } else {
        obj.to_vec()
    };
    Ok((input, TKey { hdr, obj }))
}

/// Special thing for the keylist in the file header
pub(crate) fn tkey_headers(input: &[u8]) -> IResult<&[u8], Vec<TKeyHeader>> {
    length_count(be_u32, tkey_header)(input)
}
