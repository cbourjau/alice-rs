use std::convert::TryFrom;
use std::fmt::Debug;
use std::io::Read;
/// Parsers of the ROOT core types. Note that objects in ROOT files
/// are often, but not always, preceeded by their size. The parsers in
/// this module do therefore not included this leading size
/// information. Usually, the user will want to do that with something
/// along the lines of `length_value!(checked_byte_count, tobject)`
/// themselves.
use std::str;

use failure::Error;
use flate2::bufread::ZlibDecoder;
use lz4_compress::decompress as lz4_decompress;
use lzma_rs::xz_decompress;
use nom::{
    self,
    bytes::complete::{take, take_until},
    combinator::{all_consuming, cond, eof, map, map_res, rest, verify},
    error::ParseError,
    multi::{count, length_data, length_value},
    number::complete::{be_i32, be_u16, be_u32, be_u64, be_u8},
    sequence::{pair, tuple},
    IResult,
};

use crate::core::*;

#[allow(clippy::trivially_copy_pass_by_ref)]
fn is_byte_count(v: &u32) -> bool {
    Flags::from_bits_truncate(*v).intersects(Flags::BYTE_COUNT_MASK)
}

/// Return the size in bytes of the following object in the input. The
/// count is the remainder of this object minus the size of the count.
pub fn checked_byte_count<'s, E>(input: &'s [u8]) -> nom::IResult<&[u8], u32, E>
where
    E: ParseError<&'s [u8]> + Debug,
{
    verify(
        map(verify(be_u32, is_byte_count), |v| {
            v & !Flags::BYTE_COUNT_MASK.bits()
        }),
        |v| *v != 0,
    )(input)
}

/// Read ROOT's version of short and long strings (preceeded by u8). Does not read null terminated!
pub fn string(input: &[u8]) -> nom::IResult<&[u8], String> {
    let (input, len) = match be_u8(input)? {
        (input, 255) => be_u32(input)?,
        (input, val) => (input, val as u32),
    };
    let (input, s) = map_res(take(len), str::from_utf8)(input)?;
    Ok((input, s.to_string()))
}

/// Parser for the most basic of ROOT types
pub fn tobject(input: &[u8]) -> nom::IResult<&[u8], TObject> {
    let (input, ver) = be_u16(input)?; // version_consume_extra_virtual >>
    let (input, id) = be_u32(input)?;
    let (input, bits) = map(be_u32, |v| {
        // TObjects read from disc must have the ON_HEAP flag
        TObjectFlags::from_bits_truncate(v | TObjectFlags::IS_ON_HEAP.bits())
    })(input)?;
    let (input, _ref) = cond(bits.intersects(TObjectFlags::IS_REFERENCED), be_u16)(input)?;
    Ok((input, TObject { ver, id, bits }))
}

/// Parse a `TList`
pub fn tlist<'s>(i: &'s [u8], ctx: &'s Context) -> IResult<&'s [u8], Vec<Raw<'s>>> {
    let (i, _ver) = verify(be_u16, |&v| v == 5)(i)?;
    let (i, (_tobj, _name, len)) = tuple((tobject, string, be_i32))(i)?;
    let (i, objs) = count(
        |i| {
            let wrapped_raw = |i| raw(i, ctx);
            let (i, obj) = length_value(checked_byte_count, wrapped_raw)(i)?;
            let (i, _) = length_data(be_u8)(i)?;
            Ok((i, obj))
        },
        len as usize,
    )(i)?;
    let (i, _) = rest(i)?;
    Ok((i, objs))
}

/// Parser for `TNamed` objects
pub fn tnamed(input: &[u8]) -> nom::IResult<&[u8], TNamed> {
    let (input, _ver) = be_u16(input)?;
    let (input, _tobj) = tobject(input)?;
    let (input, name) = string(input)?;
    let (input, title) = string(input)?;
    Ok((input, TNamed { name, title }))
}

/// Parse a `TObjArray`
pub fn tobjarray<'s, F, O>(
    parser: F,
    i: &'s [u8],
    context: &'s Context,
) -> nom::IResult<&'s [u8], Vec<O>>
where
    F: Fn(&Raw<'s>, &'s Context) -> nom::IResult<&'s [u8], O>,
{
    let (i, _ver) = be_u16(i)?;
    let (i, _tobj) = tobject(i)?;
    let (i, _name) = c_string(i)?;
    let (i, size) = be_i32(i)?;
    let (i, _low) = be_i32(i)?;
    let (i, objs) = count(
        map_res(
            |i| raw(i, context),
            |r| {
                let res = parser(&r, context).map(|(_i, res)| res);
                if res.is_err() {
                    res.as_ref().unwrap();
                }
                res
            },
        ),
        size as usize,
    )(i)?;
    Ok((i, objs))
}

/// Parse a `TObjArray` which does not have references pointing outside of the input buffer
pub fn tobjarray_no_context(input: &[u8]) -> nom::IResult<&[u8], Vec<(ClassInfo, &[u8])>> {
    let (input, _ver) = be_u16(input)?;
    let (input, _tobj) = tobject(input)?;
    let (input, _name) = c_string(input)?;
    let (input, size) = be_i32(input)?;
    let (input, _low) = be_i32(input)?;
    let (input, objs) = count(raw_no_context, size as usize)(input)?;
    let objs = objs.into_iter().map(|(ci, s)| (ci, s)).collect();
    Ok((input, objs))
}

/// Parser for `TObjString`
pub fn tobjstring(input: &[u8]) -> nom::IResult<&[u8], String> {
    let (input, _ver) = be_u16(input)?;
    let (input, _tobj) = tobject(input)?;
    let (input, name) = string(input)?;
    let (input, _eof) = eof(input)?;
    Ok((input, name))
}

/// Parse a so-called `TArray`. Note that ROOT's `TArray`s are actually not fixed size.
/// Example usage for TArrayI: `tarray(nom::complete::be_i32, input_slice)`
pub fn tarray<'s, E, F, O>(parser: F, i: &'s [u8]) -> nom::IResult<&'s [u8], Vec<O>, E>
where
    F: Fn(&'s [u8]) -> nom::IResult<&'s [u8], O, E>,
    E: ParseError<&'s [u8]> + Debug,
{
    let (i, counts) = be_i32(i)?;
    count(parser, counts as usize)(i)
}

fn decode_reader<'s>(bytes: &'s [u8], magic: &[u8]) -> nom::IResult<&'s [u8], Vec<u8>> {
    match magic {
        b"ZL" => map_res(rest, |bytes| {
            let mut ret = vec![];
            let mut decoder = ZlibDecoder::new(bytes);
            decoder.read_to_end(&mut ret)?;
            Ok::<_, Error>(ret)
        })(bytes),
        b"XZ" => map_res(rest, |bytes| {
            let mut ret = vec![];
            let mut reader = std::io::BufReader::new(bytes);
            xz_decompress(&mut reader, &mut ret).unwrap();
            Ok::<_, Error>(ret)
        })(bytes),
        b"L4" => {
            let (bytes, _checksum) = be_u64(bytes)?;
            map_res(rest, lz4_decompress)(bytes)
        }
        _ => panic!(), // m => return Err(format_err!("Unsupported compression format `{}`", m)),
    }
}

/// Decompress the given buffer. Figures out the compression algorithm from the preceeding \"magic\" bytes
pub fn decompress(input: &[u8]) -> nom::IResult<&[u8], Vec<u8>> {
    let (input, magic) = take(2usize)(input)?;
    let (input, _header) = take(7usize)(input)?;
    decode_reader(input, magic)
}

/// Parse a null terminated string
pub fn c_string<'s>(i: &'s [u8]) -> nom::IResult<&[u8], &str> {
    let (i, s) = map_res(take_until(b"\x00".as_ref()), str::from_utf8)(i)?;
    // consume the null tag
    let (i, _) = take(1usize)(i)?;
    Ok((i, s))
}

/// Figure out the class we are looking at. The data might not be
/// saved locally but rather in a reference to some other place in the
/// buffer.This is modeled after ROOT's `TBufferFile::ReadObjectAny` and
/// `TBufferFile::ReadClass`
pub fn classinfo<'s>(i: &'s [u8]) -> nom::IResult<&[u8], ClassInfo> {
    let (i, tag) = {
        let (i, bcnt) = be_u32(i)?;
        if !is_byte_count(&bcnt) || bcnt == Flags::NEW_CLASSTAG.bits() {
            (i, bcnt)
        } else {
            be_u32(i)?
        }
    };
    let (i, cl) = match tag as u32 {
        0xFFFF_FFFF => {
            let (i, cl) = map(c_string, ClassInfo::New)(i)?;
            (i, cl)
        }
        tag => {
            if Flags::from_bits_truncate(tag).contains(Flags::CLASS_MASK) {
                (i, ClassInfo::Exists(tag & !Flags::CLASS_MASK.bits()))
            } else {
                (i, ClassInfo::References(tag))
            }
        }
    };
    Ok((i, cl))
}

/// Figure out the class we are looking at. This parser immediately
/// resolves possible references returning the name of the object in
/// this buffer and the associated data. This function needs a
/// `Context`, though, which may not be available. If so, have a look
/// at the `classinfo` parser.
pub fn class_name_and_buffer<'s>(
    i: &'s [u8],
    context: &'s Context,
) -> nom::IResult<&'s [u8], (&'s str, &'s [u8])> {
    let ctx_offset = u32::try_from(context.offset)
        .expect("Encountered pointer larger than 32 bits. Please file a bug.");
    let (i, ci) = classinfo(i)?;
    Ok(match ci {
        ClassInfo::New(s) => {
            let (i, buf) = length_value(checked_byte_count, rest)(i)?;
            (i, (s, buf))
        }
        ClassInfo::Exists(tag) => {
            let name = {
                let abs_offset = tag & !Flags::CLASS_MASK.bits();
                let s = &context.s[((abs_offset - ctx_offset) as usize)..];
                let (_, (name, _)) = class_name_and_buffer(s, context)?;
                name
            };
            let (i, buf) = length_value(checked_byte_count, rest)(i)?;
            (i, (name, buf))
        }
        ClassInfo::References(tag) => {
            let (name, buf) = {
                let abs_offset = tag;
                // Sometimes, the reference points to `0`; so we return an empty slice
                if abs_offset == 0 {
                    ("", &context.s[..0])
                } else {
                    let s = &context.s[((abs_offset - ctx_offset) as usize)..];
                    let (_, (name, buf)) = class_name_and_buffer(s, context)?;
                    (name, buf)
                }
            };
            (i, (name, buf))
        }
    })
}

/// Parse a `Raw` chunk from the given input buffer. This is usefull when one does not know the exact type at the time of parsing
pub fn raw<'s>(input: &'s [u8], context: &'s Context) -> nom::IResult<&'s [u8], Raw<'s>> {
    let (input, (classinfo, obj)) = class_name_and_buffer(input, context)?;
    // obj: length_value!(checked_byte_count, call!(nom::rest)) >>
    Ok((input, Raw { classinfo, obj }))
}

/// Same as `raw` but doesn't require a `Context` as input. Panics if
/// a `Context` is required to parse the underlying buffer (i.e., the
/// given buffer contains a reference to some other part of the file.
pub fn raw_no_context(input: &[u8]) -> nom::IResult<&[u8], (ClassInfo, &[u8])> {
    use super::ClassInfo::*;
    let (input, ci) = classinfo(input)?;
    let (input, obj) = match ci {
        // point to beginning of slice
        References(0) => (input, &input[..0]),
        New(_) | Exists(_) => length_value(checked_byte_count, rest)(input)?,
        // If its a reference to any other thing but 0 it needs a context
        _ => panic!("Object needs context!"),
    };
    Ok((input, (ci, obj)))
}

/// ESD trigger classes are strings describing a particular
/// Trigger. Each event (but in reality every run) might have a
/// different "menu" of available triggers. The trigger menu is saved
/// as an `TObjArray` of `TNamed` objects for each event. This breaks
/// it down to a simple vector
pub fn parse_tobjarray_of_tnameds<'s>(input: &'s [u8]) -> nom::IResult<&[u8], Vec<String>> {
    // each element of the tobjarray has a Vec<u8>
    let (input, vals) = length_value(checked_byte_count, tobjarray_no_context)(input)?;
    let strings = vals
        .into_iter()
        .map(|(ci, el)| {
            if let ClassInfo::References(0) = ci {
                Ok("".to_string())
            } else {
                tnamed(el).map(|(_input, tn)| tn.name)
            }
        })
        .collect::<Result<Vec<String>, _>>();
    strings.map(|ss| (input, ss))
}

/// Some Double_* values are saved with a custom mantissa... The
/// number of bytes can be found in the comment string of the
/// generated YAML code (for ALICE ESD files at least).  This function
/// reconstructs a float from the exponent and mantissa
pub fn parse_custom_mantissa<'s>(input: &'s [u8], nbits: usize) -> nom::IResult<&[u8], f32> {
    // TODO: Use ByteOrder crate to be cross-platform?
    pair(be_u8, be_u16)(input).map(|(input, (exp, man))| {
        let mut s = u32::from(exp);
        // Move the exponent into the last 23 bits
        s <<= 23;
        s |= (u32::from(man) & ((1 << (nbits + 1)) - 1)) << (23 - nbits);
        (input, f32::from_bits(s))
    })
}

/// Parse a sized object and check that it used all its bytes.
pub fn parse_sized_object<'s, F, O>(parser: F) -> impl Fn(&'s [u8]) -> nom::IResult<&'s [u8], O>
where
    F: Fn(&'s [u8]) -> nom::IResult<&'s [u8], O>,
{
    move |i| length_value(checked_byte_count, all_consuming(&parser))(i)
}

#[cfg(test)]
mod classinfo_test {
    use super::classinfo;

    /// There is an issue where the following is parsed differently on
    /// nightly ( rustc 1.25.0-nightly (79a521bb9 2018-01-15)), than
    /// on stable, if verbose-errors are enabled for nom in the
    /// cargo.toml
    #[test]
    fn classinfo_not_complete_read() {
        let i = vec![
            128, 0, 0, 150, 64, 0, 1, 92, 0, 3, 0, 1, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0,
            0, 0, 64, 0, 0, 103, 128, 0, 0, 193, 64, 0, 0, 95, 0, 3, 64, 0, 0, 85, 0, 4, 64, 0, 0,
            38, 0, 1, 0, 1, 0, 0, 0, 0, 3, 0, 0, 0, 7, 84, 79, 98, 106, 101, 99, 116, 17, 66, 97,
            115, 105, 99, 32, 82, 79, 79, 84, 32, 111, 98, 106, 101, 99, 116, 0, 0, 0, 66, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 144, 27, 192, 45, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 4, 66, 65, 83, 69, 0, 0, 0, 1, 64, 0, 0, 116, 255, 255, 255, 255, 84, 83, 116,
            114, 101, 97, 109, 101, 114, 83, 116, 114, 105, 110, 103, 0, 64, 0, 0, 92, 0, 2, 64, 0,
            0, 86, 0, 4, 64, 0, 0, 36, 0, 1, 0, 1, 0, 0, 0, 0, 3, 0, 0, 0, 5, 102, 78, 97, 109,
            101, 17, 111, 98, 106, 101, 99, 116, 32, 105, 100, 101, 110, 116, 105, 102, 105, 101,
            114, 0, 0, 0, 65, 0, 0, 0, 24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 84, 83, 116, 114, 105, 110, 103, 64, 0, 0, 96, 128, 0,
            18, 227, 64, 0, 0, 88, 0, 2, 64, 0, 0, 82, 0, 4, 64, 0, 0, 32, 0, 1, 0, 1, 0, 0, 0, 0,
            3, 0, 0, 0, 6, 102, 84, 105, 116, 108, 101, 12, 111, 98, 106, 101, 99, 116, 32, 116,
            105, 116, 108, 101, 0, 0, 0, 65, 0, 0, 0, 24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 84, 83, 116, 114, 105, 110, 103,
        ];
        let i = i.as_slice();
        let (i, _ci) = classinfo(i).unwrap();
        // The error manifests in the entire input being (wrongly)
        // consumed, instead of having some left overs
        assert_eq!(i.len(), 352);
    }
}
