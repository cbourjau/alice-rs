use std::convert::TryFrom;
use std::io::Read;
/// Parsers of the ROOT core types. Note that objects in ROOT files
/// are often, but not always, preceeded by their size. The parsers in
/// this module do therefore not included this leading size
/// information. Usually, the user will want to do that with something
/// along the lines of `length_value!(checked_byte_count, tobject)`
/// themself.
// Much of this is based on infromation from Much of this is derived
// from streamerinfo.txt which is included in the official ROOT source
// code for (old) layout
use std::str;

use failure::Error;
use flate2::bufread::ZlibDecoder;
use lzma_rs::xz_decompress;
use nom::{
    self,
    bytes::complete::{take, take_until},
    combinator::{map_res, rest},
    multi::{count, length_data, length_value},
    number::complete::{be_f64, be_i32, be_u16, be_u32, be_u8},
    sequence::{pair, tuple},
};

use crate::core::*;

#[allow(clippy::trivially_copy_pass_by_ref)]
fn is_byte_count(v: &u32) -> bool {
    Flags::from_bits_truncate(*v).intersects(Flags::BYTE_COUNT_MASK)
}

// Check that the given byte count is not zero after applying bit mask
named!(
    #[doc="Return the size in bytes of the following object in the
    input. The count is the remainder of this object minus the size
    of the count."],
    pub checked_byte_count<&[u8], u32>,
    verify!(
        map!(verify!(be_u32, is_byte_count),
             |v| v & !Flags::BYTE_COUNT_MASK.bits()),
        |v| *v != 0)
);

/// Read ROOT's version of short and long strings (preceeded by u8). Does not read null terminated!
#[rustfmt::skip::macros(do_parse)]
pub fn string(input: &[u8]) -> nom::IResult<&[u8], String> {
    do_parse!(input,
              len: switch!(be_u8,
                           255 => call!(be_u32) |
                           a => value!(u32::from(a))) >>
              s: map!(
                  map_res!(take!(len), |s| str::from_utf8(s)),
                  |s| s.to_string()) >>
              (s)
    )
}

#[rustfmt::skip::macros(do_parse)]
named!(
    #[doc="Parser for the most basic of ROOT types"],
    pub tobject<&[u8], TObject>,
    do_parse!(ver: be_u16 >> // version_consume_extra_virtual >>
              id: be_u32 >>
              bits: map!(be_u32, |v| {
                  // TObjects read from disc must have the ON_HEAP flag
                  TObjectFlags::from_bits_truncate(v| TObjectFlags::IS_ON_HEAP.bits())}
              ) >>
              _ref: cond!(bits.intersects(TObjectFlags::IS_REFERENCED), be_u16) >>
              ({TObject {
                  ver, id, bits
              }})
    )
);

/// Parse a `TList`
pub fn tlist<'s, 'c>(input: &'s [u8], context: &'c Context) -> nom::IResult<&'s [u8], TList<'c>>
where
    's: 'c,
{
    let (input, (ver, tobj, name, len)) = tuple((be_u16, tobject, string, be_i32))(input)?;
    let (input, objs) = count(
        |i| {
            let wrapped_raw = |i| raw(i, context);
            let (i, obj) = length_value(checked_byte_count, wrapped_raw)(i)?;
            let (i, _) = length_data(be_u8)(i)?;
            Ok((i, obj))
        },
        len as usize,
    )(input)?;

    let (input, _) = rest(input)?;
    Ok((
        input,
        TList {
            ver,
            tobj,
            name,
            len: len as usize,
            objs,
        },
    ))
}

// /// Parse a `TList`
// pub fn tlist<'s, 'c>(input: &'s [u8], context: &'c Context) -> nom::IResult<&'s [u8], TList<'c>>
// where
//     's: 'c,
// {
//     let wrapped_raw = |i| raw(i, context);
//     do_parse!(input,
//               ver: be_u16 >>
//               tobj: tobject >>
//               name: string >>
//               len: be_i32 >>
//               objs: count!(
//                   do_parse!(obj: length_value!(checked_byte_count, wrapped_raw) >>
//                             // It seems like the TList can have gaps
//                             // between the elements. The size of the
//                             // gap is specified with a be_u8 following
//                             // the previous element.
//                             _gap: opt!(complete!(length_data!(be_u8))) >>
//                             (obj)),
//                   len as usize) >>
//               _rest: rest >>
//               ({
//                   TList{
//                       ver: ver,
//                       tobj: tobj,
//                       name: name,
//                       len: len as usize,
//                       objs: objs
//                   }})
//     )
// }

#[rustfmt::skip::macros(do_parse)]
named!(
    #[doc="Parser for `TNamed` objects"],
    pub tnamed<&[u8], TNamed>,
    do_parse!(_ver: be_u16 >>
              _tobj: tobject >>
              name: string >>
              title: string >>
              ({TNamed{name, title}})
    )
);

/// Parse a `TObjArray`
#[rustfmt::skip::macros(do_parse)]
pub fn tobjarray<'s, 'c>(
    input: &'s [u8],
    context: &'c Context,
) -> nom::IResult<&'s [u8], Vec<Raw<'c>>>
where
    's: 'c,
{
    let wrapped_raw = |i| raw(i, context);
    do_parse!(input,
              _ver: be_u16 >>
              _tobj: tobject >>
              _name: c_string >>
              _size: be_i32 >>
              _low: be_i32 >>
              objs: count!(wrapped_raw, _size as usize) >>
              (objs)
    )
}

/// Parse a `TObjArray` which does not have references pointing outside of the input buffer
#[rustfmt::skip::macros(do_parse)]
pub fn tobjarray_no_context(input: &[u8]) -> nom::IResult<&[u8], Vec<(ClassInfo, &[u8])>> {
    do_parse!(input,
              _ver: be_u16 >>
              _tobj: tobject >>
              _name: c_string >>
              _size: be_i32 >>
              _low: be_i32 >>
              objs: map!(count!(raw_no_context, _size as usize),
                         |v| v.into_iter().map(|(ci, s)| (ci, s)).collect()) >>
              (objs)
    )
}

#[rustfmt::skip::macros(do_parse)]
named!(
    #[doc="Parser for `TObjString`"],
    pub tobjstring<&[u8], String>,
    do_parse!(_ver: be_u16 >>
              _tobj: tobject >>
              name: string >>
              _eof: eof!() >>
              ({name})
    )
);

named!(
    #[doc="Parse a so-called `TArrayI`. Note that ROOT's `TArray`s are actually not fixed size..."],
    pub tarrayi<&[u8], Vec<i32>>,
    length_count!(be_i32, be_i32)
);
named!(
    #[doc="Parse a so-called `TArrayI`. Note that ROOT's `TArray`s are actually not fixed size..."],
    pub tarrayd<&[u8], Vec<f64>>,
    length_count!(be_i32, be_f64)
);

fn decode_reader(bytes: &[u8], magic: &str) -> Result<Vec<u8>, Error> {
    let mut ret = vec![];
    match magic {
        "ZL" => {
            let mut decoder = ZlibDecoder::new(&bytes[..]);
            decoder.read_to_end(&mut ret)?;
        }
        "XZ" => {
            let mut reader = std::io::BufReader::new(bytes);
            xz_decompress(&mut reader, &mut ret).unwrap();
        }
        m => return Err(format_err!("Unsupported compression format `{}`", m)),
    };
    Ok(ret)
}

/// Decompress the given buffer. Figures out the compression algorithm from the preceeding \"magic\" bytes
pub fn decompress(input: &[u8]) -> nom::IResult<&[u8], Vec<u8>> {
    map_res(
        tuple((|i| take_str!(i, 2usize), take(7usize), rest)),
        |(magic, _header, comp_buf)| decode_reader(comp_buf, magic),
    )(input)
}

/// Parse a null terminated string
pub fn c_string(i: &[u8]) -> nom::IResult<&[u8], String> {
    let (i, s) = map_res(take_until(b"\x00".as_ref()), str::from_utf8)(i)?;
    // consume the null tag
    let (i, _) = take(1usize)(i)?;
    Ok((i, s.to_string()))
}

/// Figure out the class we are looking at. The data might not be
/// saved locally but rather in a reference to some other place in the
/// buffer.This is modeled after ROOT's `TBufferFile::ReadObjectAny` and
/// `TBufferFile::ReadClass`
#[rustfmt::skip::macros(do_parse)]
pub fn classinfo(input: &[u8]) -> nom::IResult<&[u8], ClassInfo> {
    do_parse!(input,
              bcnt: be_u32 >>
              tag: switch!(
                  value!(!is_byte_count(&bcnt) || bcnt == Flags::NEW_CLASSTAG.bits()),
                  true => value!(bcnt) |
                  false => call!(be_u32)) >>
              cl: switch!(
                  value!(tag as u32), // If new class, this should be like TClass::Load
                  // 0xFFFFFFFF is new class tag
                  0xFFFF_FFFF => map!(c_string, ClassInfo::New) |
                  // Is this an existing class or is it another tag (pointer elswhere)
                  tag => switch!(
                      value!(Flags::from_bits_truncate(tag).contains(Flags::CLASS_MASK)),
                      false => value!(ClassInfo::References(tag)) |
                      true => value!(ClassInfo::Exists(tag & !Flags::CLASS_MASK.bits()))
                  )) >>
              (cl)
    )
}

/// Figure out the class we are looking at. This parser immediately
/// resolves possible references returning the name of the object in
/// this buffer and the associated data. This function needs a
/// `Context`, though, which may not be avialable. If so, have a look
/// at the `classinfo` parser.
#[rustfmt::skip::macros(do_parse)]
pub fn class_name_and_buffer<'s, 'c>(
    input: &'s [u8],
    context: &'c Context,
) -> nom::IResult<&'s [u8], (String, &'c [u8])>
where
    's: 'c,
{
    let ctx_offset = u32::try_from(context.offset)
        .expect("Encountered pointer larger than 32 bits. Please file a bug.");
    let get_name_elsewhere = |tag| {
        let abs_offset = tag & !Flags::CLASS_MASK.bits();
        let s = &context.s[((abs_offset - ctx_offset) as usize)..];
        let (_, (name, _)) = class_name_and_buffer(s, context).unwrap();
        name
    };
    let get_name_and_buf_elsewhere = |tag| {
        let abs_offset = tag;
        // Sometimes, the reference points to `0`; so we return an empty slice
        if abs_offset == 0 {
            return ("".to_string(), &context.s[..0]);
        }
        let s = &context.s[((abs_offset - ctx_offset) as usize)..];
        let (_, (name, buf)) = class_name_and_buffer(s, context).unwrap();
        (name, buf)
    };
    do_parse!(input,
              ci: switch!(classinfo,
                          ClassInfo::New(s) => tuple!(value!(s), length_value!(checked_byte_count, call!(rest))) |
                          ClassInfo::Exists(tag) => tuple!(value!(get_name_elsewhere(tag)),
                                                             length_value!(checked_byte_count, call!(rest))) |
                          ClassInfo::References(tag) => value!(get_name_and_buf_elsewhere(tag))) >>
              (ci)
    )
}

/// Parse a `Raw` chunk from the given input buffer. This is usefull when one does not know the exact type at the time of parsing
#[rustfmt::skip::macros(do_parse)]
pub fn raw<'s, 'c>(input: &'s [u8], context: &'c Context) -> nom::IResult<&'s [u8], Raw<'c>>
where
    's: 'c,
{
    do_parse!(input,
              string_and_obj: call!(class_name_and_buffer, context) >>
              // obj: length_value!(checked_byte_count, call!(nom::rest)) >>
              ({let (classinfo, obj) = string_and_obj;
                Raw{classinfo, obj}})
    )
}

/// Same as `raw` but doesn't require a `Context` as input. Panics if
/// a `Context` is required to parse the underlying buffer (i.e., the
/// given buffer contains a reference to some other part of the file.
pub fn raw_no_context(input: &[u8]) -> nom::IResult<&[u8], (ClassInfo, &[u8])> {
    use super::ClassInfo::*;
    let (input, ci) = classinfo(input)?;
    let obj = match ci {
        // point to beginning of slice
        References(0) => value!(input, &input[..0]),
        New(_) | Exists(_) => length_value!(input, checked_byte_count, call!(rest)),
        // If its a reference to any other thing but 0 it needs a context
        _ => panic!("Object needs context!"),
    };
    obj.map(|(i, o)| (i, (ci, o)))
}

/// ESD trigger classes are strings describing a particular
/// Trigger. Each event (but in reality every run) might have a
/// different "menu" of available triggers. The trigger menu is saved
/// as an `TObjArray` of `TNamed` objects for each event. This breaks
/// it down to a simple vector
pub fn parse_tobjarray_of_tnameds(input: &[u8]) -> nom::IResult<&[u8], Vec<String>> {
    // each element of the tobjarray has a Vec<u8>
    let (input, vals) = length_value!(input, checked_byte_count, tobjarray_no_context)?;
    let strings = vals
        .into_iter()
        .map(|(ci, el)| {
            if let ClassInfo::References(0) = ci {
                Ok("".to_string())
            } else {
                tnamed(&el).map(|(_input, tn)| tn.name)
            }
        })
        .collect::<Result<Vec<String>, _>>();
    strings.map(|ss| (input, ss))
}

/// Some Double_* values are saved with a custom mantissa... The
/// number of bytes can be found in the comment string of the
/// generated YAML code (for ALICE ESD files at least).  This function
/// reconstructs a float from the exponent and mantissa
pub fn parse_custom_mantissa(input: &[u8], nbits: usize) -> nom::IResult<&[u8], f32> {
    // TODO: Use ByteOrder crate to be cross-platform?
    pair(be_u8, be_u16)(input).map(|(input, (exp, man))| {
        let mut s = u32::from(exp);
        // Move the exponent into the last 23 bits
        s <<= 23;
        s |= (u32::from(man) & ((1 << (nbits + 1)) - 1)) << (23 - nbits);
        (input, f32::from_bits(s))
    })
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
        // The error manifests in the entire input being (wrongly) consumed, instead of having some left overs
        assert_eq!(i.len(), 352);
    }
}
