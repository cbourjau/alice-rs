use nom::bytes::complete::take;
use nom::multi::length_count;
use nom::number::complete::{be_i16, be_u16, be_u32, be_u64};
use nom::sequence::tuple;
use nom::*;
use nom_supreme::ParserExt;

use crate::core::compression::{decompress, DecompressionError};
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

/// Header of a TKey
/// Usually, TKeys are followed up by their content, but there is one "index" in every
/// root file where only the TKey headers are stored for faster later `Seek`ing
pub fn tkey_header<'s, E>(input: Span<'s>) -> RResult<'s, TKeyHeader, E>
where
    E: RootError<Span<'s>>,
{
    let (i, hdr) = tuple((
        be_u32.context("total size"),
        be_u16.context("version"),
        be_u32.context("uncompressed length"),
        be_u32.context("datime"),
        be_i16.context("key length"),
        be_i16.context("cycle"),
    ))
    .flat_map(make_fn(
        |(total_size, version, uncomp_len, datime, key_len, cycle)| {
            tuple((
                seek_point(version).context("seek key"),
                seek_point(version).context("seek pdir"),
                string.context("class name"),
                string.context("object name"),
                string.context("object title"),
            ))
            .map(
                move |(seek_key, seek_pdir, class_name, obj_name, obj_title)| TKeyHeader {
                    total_size,
                    version,
                    uncomp_len,
                    datime,
                    key_len,
                    cycle,
                    seek_key,
                    seek_pdir,
                    class_name: class_name.to_string(),
                    obj_name: obj_name.to_string(),
                    obj_title: obj_title.to_string(),
                },
            )
        },
    ))
    .context("tkey header")
    .parse(input)?;

    Ok((i, hdr))
}

/// Parse a file-pointer based on the version of the file
fn seek_point<'s, E>(version: u16) -> impl RParser<'s, u64, E>
where
    E: RootError<Span<'s>>,
{
    move |i| {
        if version > 1000 {
            be_u64.parse(i)
        } else {
            be_u32.map(|v| v as u64).parse(i)
        }
    }
}

/// Parse a full TKey including its payload
pub fn tkey<'s, E>(input: Span<'s>) -> RResult<'s, TKey, E>
where
    E: RootError<Span<'s>>,
{
    let (i, hdr) = tkey_header.parse(input)?;
    let buflen = hdr.total_size - hdr.key_len as u32;
    let uncomp_len = hdr.uncomp_len;

    let mut opthdr = Some(hdr);

    take(buflen)
        .map_res::<_, _, DecompressionError>(|buf: Span| {
            let obj = if uncomp_len as usize > buf.len() {
                decompress(&buf)?
            } else {
                buf.to_vec()
            };
            Ok(TKey {
                hdr: opthdr.take().unwrap(),
                obj,
            })
        })
        .context("tkey")
        .parse(i)
}
// Note that tkey current
/// Special thing for the keylist in the file header
// Note that tkey_headers currently does not parse the entire input buffer
// See: read_cms_file_remote for an example
pub(crate) fn tkey_headers<'s, E>(input: Span<'s>) -> RResult<'s, Vec<TKeyHeader>, E>
where
    E: RootError<Span<'s>>,
{
    length_count(be_u32, tkey_header)
        .complete()
        .context("count-prefixed data")
        .parse(input)
}
