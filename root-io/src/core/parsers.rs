use nom::branch::alt;
use nom::combinator::cond;
use nom::error::{ContextError, FromExternalError, VerboseError};
use nom::multi::length_count;
use nom::HexDisplay;
use nom::Slice;
use nom::{
    self,
    bytes::complete::{take, take_until},
    combinator::rest,
    error::ParseError,
    multi::{count, length_data, length_value},
    number::complete::{be_i32, be_u16, be_u32, be_u8},
    sequence::{pair, tuple},
    IResult, Parser,
};
use nom_locate::LocatedSpan;
use nom_supreme::parser_ext::ParserExt;
use nom_supreme::tag::TagError;
use ouroboros::self_referencing;

/// Parsers of the ROOT core types. Note that objects in ROOT files
/// are often, but not always, preceeded by their size. The parsers in
/// this module do therefore not included this leading size
/// information. Usually, the user will want to do that with something
/// along the lines of `length_value!(checked_byte_count, tobject)`
/// themselves.
use std::convert::{TryFrom, TryInto};
use std::fmt::Debug;
use std::result::Result::Ok;
use std::str;

use crate::core::compression::DecompressionError;
use crate::core::*;

pub trait RootError<I>:
    ParseError<I>
    + ContextError<I>
    + TagError<I, &'static str>
    + FromExternalError<I, std::str::Utf8Error>
    + FromExternalError<I, TypeIdError>
    + FromExternalError<I, SemanticError>
    + FromExternalError<I, DecompressionError>
    + Debug
{
}

impl<
        I,
        T: ParseError<I>
            + ContextError<I>
            + TagError<I, &'static str>
            + FromExternalError<I, std::str::Utf8Error>
            + FromExternalError<I, TypeIdError>
            + FromExternalError<I, SemanticError>
            + FromExternalError<I, DecompressionError>
            + Debug,
    > RootError<I> for T
{
}

pub type Span<'s> = LocatedSpan<&'s [u8]>;
pub type RResult<'s, O, E> = IResult<Span<'s>, O, E>;

pub trait RParser<'s, O, E: RootError<Span<'s>>>: Parser<Span<'s>, O, E> {}

impl<'s, O, E: RootError<Span<'s>>, T: Parser<Span<'s>, O, E>> RParser<'s, O, E> for T {}

/// Corerce a closure to a Fn, for use with map_res et al.
pub(crate) fn make_fn<T, U, F: Fn(T) -> U>(f: F) -> F {
    f
}

#[self_referencing]
#[derive(Debug)]
pub struct VerboseErrorInfo {
    input: Vec<u8>,
    #[borrows(input)]
    #[covariant]
    error: VerboseError<Span<'this>>,
}

impl std::fmt::Display for VerboseErrorInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        use nom::error::ErrorKind as Kind;
        use nom::error::VerboseErrorKind::*;

        writeln!(f, "Error while parsing this block of data:")?;
        if self.borrow_input().len() > 0x100 {
            write!(f, "{}", self.borrow_input()[..0x100].to_hex(16))?;
            writeln!(
                f,
                "        \t[0x{:x} of 0x{:x} bytes omitted]",
                self.borrow_input().len() - 0x100,
                self.borrow_input().len()
            )?;
        } else {
            write!(f, "{}", self.borrow_input().to_hex(16))?;
        }

        for (span, kind) in self.borrow_error().errors.iter().rev() {
            match kind {
                Context(context) => write!(f, "\nWhile trying to parse {}:", context)?,
                Char(c) => write!(f, "\nWhile trying to match a '{}':", c)?,
                Nom(Kind::Verify) => continue,
                Nom(Kind::Complete) => {
                    write!(f, "\nExpected length exceeds buffer")?;
                    continue;
                }
                Nom(Kind::Eof) => {
                    if span.fragment().is_empty() {
                        // Yes, EOF is returned both for parsers expecting more input (the be_uXX
                        // parsers for us, mostly), but also by parsers expecting *no more* input
                        // such as all_consuming.
                        // We distinguish based on the remaining input - if everything was consumed,
                        // it must have been a premature EOF
                        write!(f, "\nUnexpected EOF")?
                    } else {
                        write!(f, "\nExpected EOF, but found excess data")?
                    }
                }
                Nom(kind) => write!(f, "\nIn {:?}:", kind)?,
            };

            let fragment_begin = span.location_offset();
            let fragment_end = match kind {
                Context(_) | Nom(_) => {
                    span.location_offset()
                        + std::cmp::max(1, std::cmp::min(0x100, span.fragment().len()))
                }
                Char(_) => span.location_offset() + 1,
            };
            // Align hexdump to 16-byte blocks
            let hexdump_begin = fragment_begin / 16 * 16;
            let hexdump_first_line_end =
                std::cmp::min(self.borrow_input().len(), hexdump_begin + 16);
            let hexdump_end = (fragment_end + 16) / 16 * 16;
            let hexdump_end = std::cmp::min(self.borrow_input().len(), hexdump_end);

            // 2 letters per byte + one space
            let fragment_begin_in_dump = 3 * (fragment_begin % 16);
            let fragment_end_in_dump = 3 * ((fragment_end - 1) % 16) + 1;

            write!(
                f,
                "\n{}",
                self.borrow_input()[hexdump_begin..hexdump_first_line_end]
                    .to_hex_from(16, hexdump_begin)
            )?;
            if fragment_begin == self.borrow_input().len() {
                write!(
                    f,
                    "        \t{: >skip$} [at end of input]",
                    '^',
                    skip = fragment_begin_in_dump + 1
                )?;
            } else if fragment_begin / 16 == fragment_end / 16 {
                write!(
                    f,
                    "        \t{: >skip$}{:~>len$}",
                    '^',
                    '~',
                    skip = fragment_begin_in_dump + 1,
                    len = fragment_end_in_dump - fragment_begin_in_dump
                )?
            } else {
                write!(
                    f,
                    "        \t{: >skip$}{:~>len$}",
                    '^',
                    '~',
                    skip = fragment_begin_in_dump + 1,
                    len = (3 * 15 + 1) - fragment_begin_in_dump
                )?;
                write!(
                    f,
                    "\n{}",
                    self.borrow_input()[hexdump_begin + 16..hexdump_end]
                        .to_hex_from(16, hexdump_begin + 16)
                )?;
                if span.fragment().len() > 0x100 {
                    write!(
                        f,
                        "        \t[0x{:x} bytes omitted]",
                        span.fragment().len() - 0x100
                    )?;
                } else {
                    write!(
                        f,
                        "        \t{:~>len$}",
                        '~',
                        len = fragment_end_in_dump + 1
                    )?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

fn reborrow_spans<'s, 't>(
    new_base: &'s [u8],
    error: VerboseError<Span<'t>>,
) -> VerboseError<Span<'s>> {
    let reborrow = |span: &Span<'_>| unsafe {
        Span::new_from_raw_offset(
            span.location_offset(),
            span.location_line(),
            &new_base[span.location_offset()..span.location_offset() + span.fragment().len()],
            (),
        )
    };
    VerboseError {
        errors: error
            .errors
            .iter()
            .map(|(span, kind)| (reborrow(span), kind.clone()))
            .collect::<Vec<_>>(),
    }
}

pub fn wrap_parser<'s, O>(
    parser: impl Parser<Span<'s>, O, VerboseError<Span<'s>>>,
) -> impl FnMut(&'s [u8]) -> Result<O, VerboseErrorInfo> {
    let mut parser = parser.complete();

    move |input| match parser.parse(Span::new(input)) {
        Ok((_, parsed)) => Ok(parsed),
        Err(nom::Err::Error(err)) | Err(nom::Err::Failure(err)) => {
            let info = VerboseErrorInfoBuilder {
                input: input.to_vec(),
                error_builder: |input| reborrow_spans(input, err),
            };
            Err(info.build())
        }
        Err(nom::Err::Incomplete(..)) => {
            unreachable!("Complete combinator should make this impossible")
        }
    }
}

pub fn wrap_parser_ctx<'s, O, F, P>(
    parser_gen: F,
) -> impl FnMut(&'s Context) -> Result<O, VerboseErrorInfo>
where
    P: Parser<Span<'s>, O, VerboseError<Span<'s>>>,
    F: Fn(&'s Context) -> P,
{
    move |ctx| match parser_gen(ctx).complete().parse(ctx.span()) {
        Ok((_, parsed)) => Ok(parsed),
        Err(nom::Err::Error(err)) | Err(nom::Err::Failure(err)) => {
            let info = VerboseErrorInfoBuilder {
                input: ctx.s.to_vec(),
                error_builder: |input| reborrow_spans(input, err),
            };
            Err(info.build())
        }
        Err(nom::Err::Incomplete(..)) => {
            unreachable!("Complete combinator should make this impossible")
        }
    }
}

#[allow(clippy::trivially_copy_pass_by_ref)]
fn is_byte_count(v: &u32) -> bool {
    Flags::from_bits_truncate(*v).intersects(Flags::BYTE_COUNT_MASK)
}

/// Return the size in bytes of the following object in the input. The
/// count is the remainder of this object minus the size of the count.
pub fn checked_byte_count<'s, E>(input: Span<'s>) -> RResult<'s, u32, E>
where
    E: RootError<Span<'s>>,
{
    be_u32
        .verify(is_byte_count)
        .context("assertion: byte count matches bytecount mask")
        .map(|v| v & !Flags::BYTE_COUNT_MASK.bits())
        .verify(|&v| v != 0)
        .context("assertion: byte count must not be 0")
        .verify(|&v| v < 0x8000_0000)
        .context("assertion: highest bit in byte count must be unset")
        .parse(input)
}

/// Read ROOT's string length prefix, which is usually a u8, but can be extended
/// to a u32 (for a total of 5 bytes) if the first byte is 255
fn string_length_prefix<'s, E>(input: Span<'s>) -> RResult<'s, u32, E>
where
    E: RootError<Span<'s>>,
{
    alt((
        be_u8
            .verify(|&v| v == 255)
            .precedes(be_u32)
            .context("extended string length prefix"),
        be_u8
            .verify(|&v| v != 255)
            .map(|v| v as u32)
            .context("short string length prefix"),
    ))(input)
}

/// Read ROOT's version of short and long strings (preceeded by u8). Does not read null terminated!
pub fn string<'s, E>(input: Span<'s>) -> RResult<'s, &'s str, E>
where
    E: RootError<Span<'s>>,
{
    length_data(string_length_prefix)
        .map_res(|s| str::from_utf8(&s))
        .context("length-prefixed string")
        .parse(input)
}

/// Parser for the most basic of ROOT types
pub fn tobject<'s, E>(input: Span<'s>) -> RResult<'s, TObject, E>
where
    E: RootError<Span<'s>>,
{
    tuple((
        be_u16.context("tobject version"),
        be_u32.context("tobject id"),
        be_u32
            .context("tobject flags")
            .map(|v| TObjectFlags::from_bits_truncate(v | TObjectFlags::IS_ON_HEAP.bits())),
    ))
    .flat_map(make_fn(|(ver, id, bits): (u16, u32, TObjectFlags)| {
        cond(
            bits.intersects(TObjectFlags::IS_REFERENCED),
            be_u16.context("tobject reference"),
        )
        .map(move |_ref| TObject {
            ver,
            id,
            bits,
            _ref,
        })
    }))
    .parse(input)
}

/// Parse a `TList`
pub fn tlist<'s, E>(ctx: &'s Context) -> impl RParser<'s, Vec<Raw<'s>>, E>
where
    E: RootError<Span<'s>>,
{
    let parser = move |inpt| {
        let (i, _ver) = be_u16
            .context("tlist version")
            .verify(|&v| v == 5)
            .context("assertion: tlist version must be 5")
            .parse(inpt)?;
        let (i, (_tobj, _name, num_obj)) = tuple((
            tobject.context("tlist object header"),
            string.context("tlist name"),
            be_i32.context("tlist element count"),
        ))(i)?;

        let (i, objs) = count(
            |i: Span<'s>| {
                let (i, obj) = length_value(checked_byte_count, raw(ctx))
                    .complete()
                    .context("length-prefixed data")
                    .parse(i)?;
                // TODO verify remaining entry data
                // TODO u8 prefix or extended string prefix?
                let (i, _x) = length_data(be_u8).complete().parse(i)?;
                Ok((i, obj))
            },
            num_obj as usize,
        )(i)?;

        // TODO: Verify rest
        let (i, _) = rest(i)?;
        Ok((i, objs))
    };
    parser.context("tlist")
}

/// Parser for `TNamed` objects
#[rustfmt::skip::macros(do_parse)]
pub fn tnamed<'s, E>(input: Span<'s>) -> RResult<'s, TNamed, E>
where
    E: RootError<Span<'s>>,
{
    tuple((
        be_u16.context("version"),
        tobject.context("object header"),
        string.context("name"),
        string.context("title"),
    ))
    .context("named tobject")
    .map(|(_, _, name, title)| TNamed {
        name: name.to_string(),
        title: title.to_string(),
    })
    .parse(input)
}

/// Parse a `TObjArray`
pub fn tobjarray<'s, E, P, O>(parser: P) -> impl RParser<'s, Vec<O>, E>
where
    P: RParser<'s, O, E> + Copy,
    E: RootError<Span<'s>>,
{
    let parser = move |i| {
        let (i, _ver) = be_u16(i)?;
        let (i, _tobj) = tobject(i)?;
        let (i, _name) = c_string(i)?;
        let (i, size) = be_i32(i)?;
        let (i, _low) = be_i32(i)?;
        let (i, objs): (_, Vec<O>) = count(parser, size as usize)(i)?;
        Ok((i, objs))
    };

    parser.context("tobjarray")
}

/// Parse a `TObjArray` which does not have references pointing outside of the input buffer
pub fn tobjarray_no_context<'s, E>(input: Span<'s>) -> RResult<'s, Vec<(ClassInfo, Span<'s>)>, E>
where
    E: RootError<Span<'s>>,
{
    tuple((
        be_u16.context("TObjArray header version"),
        tobject.context("TObjArray object header"),
        c_string.context("TObjArray name"),
        be_i32.context("TObjArray num objects"),
        be_i32.context("TObjArray unknown"),
    ))
    .flat_map(make_fn(
        |(_, _, _, num_objects, _): (u16, TObject, &str, i32, i32)| {
            count(raw_no_context, num_objects.try_into().unwrap())
        },
    ))
    .context("TObjArray")
    .parse(input)
    //                     |v| v.into_iter().map(|(ci, s)| (ci, s)).collect()) >>
}

pub fn tobjstring<'s, E>(input: Span<'s>) -> RResult<'s, &'s str, E>
where
    E: RootError<Span<'s>>,
{
    // TODO move all_consuming to call site
    tuple((
        be_u16.context("tobjstring version"),
        tobject.context("tobjstring object"),
        string.context("tobjstring name"),
    ))
    .all_consuming()
    .context("tobjstring")
    .map(|(_, _, name)| name)
    .parse(input)
}

/// Parse a so-called `TArray`. Note that ROOT's `TArray`s are actually not fixed size.
/// Example usage for TArrayI: `tarray(nom::complete::be_i32).parse(input_slice)`
pub fn tarray<'s, E, F, O>(parser: F) -> impl RParser<'s, Vec<O>, E>
where
    F: RParser<'s, O, E>,
    E: RootError<Span<'s>>,
{
    length_count(be_u32, parser).context("tarray")
}

/// Parse a null terminated string
pub fn c_string<'s, E>(input: Span<'s>) -> RResult<'s, &str, E>
where
    E: RootError<Span<'s>>,
{
    take_until(b"\x00".as_ref())
        .terminated(be_u8.verify(|&v| v == 0))
        .map_res(|s: Span| str::from_utf8(&s))
        .context("c string")
        .parse(input)
}

/// Figure out the class we are looking at. The data might not be
/// saved locally but rather in a reference to some other place in the
/// buffer.This is modeled after ROOT's `TBufferFile::ReadObjectAny` and
/// `TBufferFile::ReadClass`
pub fn classinfo<'s, E>(input: Span<'s>) -> RResult<'s, ClassInfo, E>
where
    E: RootError<Span<'s>>,
{
    let (i, tag) = alt((
        be_u32
            .verify(|&v| !is_byte_count(&v) || v == Flags::NEW_CLASSTAG.bits())
            .context("class info: new classtag or not a valid bytecount"),
        be_u32
            .verify(|&v| is_byte_count(&v) && v != Flags::NEW_CLASSTAG.bits())
            .context("class info: class tag preceded by byte count")
            .precedes(be_u32),
    ))
    .parse(input)?;

    match tag as u32 {
        0xFFFF_FFFF => {
            // new classtag mask
            c_string
                .map(ClassInfo::New)
                .context("new classtag")
                .parse(i)
        }
        tag => {
            if Flags::from_bits_truncate(tag).contains(Flags::CLASS_MASK) {
                Ok((i, ClassInfo::Exists(tag & !Flags::CLASS_MASK.bits())))
            } else {
                Ok((i, ClassInfo::References(tag)))
            }
        }
    }
}

pub fn class_name<'s, E>(ctx: &'s Context) -> impl RParser<'s, &'s str, E>
where
    E: RootError<Span<'s>>,
{
    let parser = move |i| {
        let ctx_offset = u32::try_from(ctx.offset)
            .expect("Encountered pointer larger than 32 bits. Please file a bug.");

        let (i, ci) = classinfo(i)?;
        match ci {
            ClassInfo::New(name) => Ok((i, name)),
            ClassInfo::Exists(tag) => {
                let abs_offset = tag & !Flags::CLASS_MASK.bits();
                // TODO handle insufficient buffer length, abs_offset < ctx_offset
                let s = ctx.span().slice(((abs_offset - ctx_offset) as usize)..);
                let (_, name) = class_name(ctx)
                    .context("pre-existing class name")
                    .parse(s)?;
                Ok((i, name))
            }
            ClassInfo::References(tag) => {
                let abs_offset = tag;
                if abs_offset == 0 {
                    Ok((i, ""))
                } else {
                    // TODO as above
                    let s = ctx.span().slice(((abs_offset - ctx_offset) as usize)..);
                    let (_, name) = class_name(ctx)
                        .context("reference to class name")
                        .parse(s)?;
                    Ok((i, name))
                }
            }
        }
    };
    parser.context("class name")
}

/// Figure out the class we are looking at. This parser immediately
/// resolves possible references returning the name of the object in
/// this buffer and the associated data. This function needs a
/// `Context`, though, which may not be available. If so, have a look
/// at the `classinfo` parser.
pub fn class_name_and_buffer<'s, E>(ctx: &'s Context) -> impl RParser<'s, (&'s str, Span<'s>), E>
where
    E: RootError<Span<'s>>,
{
    let parser = move |i| {
        let ctx_offset = u32::try_from(ctx.offset)
            .expect("Encountered pointer larger than 32 bits. Please file a bug.");
        let (i, ci) = classinfo(i)?;
        Ok(match ci {
            ClassInfo::New(s) => {
                let (i, buf) = length_value(checked_byte_count, rest)
                    .complete()
                    .context("length-prefixed data")
                    .parse(i)?;
                (i, (s, buf))
            }
            ClassInfo::Exists(tag) => {
                let name = {
                    let abs_offset = tag & !Flags::CLASS_MASK.bits();
                    // TODO handle insufficient buffer length, abs_offset < ctx_offset
                    let s = ctx.span().slice(((abs_offset - ctx_offset) as usize)..);
                    let (_, name) = class_name(ctx)
                        .context("pre-existing class name")
                        .parse(s)?;
                    name
                };
                let (i, buf) = length_value(checked_byte_count, rest)
                    .complete()
                    .context("length-prefixed data")
                    .parse(i)?;
                (i, (name, buf))
            }
            ClassInfo::References(tag) => {
                let (name, buf) = {
                    let abs_offset = tag;
                    // Sometimes, the reference points to `0`; so we return an empty slice
                    if abs_offset == 0 {
                        ("", ctx.span().slice(..0))
                    } else {
                        // TODO as above
                        let s = ctx.span().slice(((abs_offset - ctx_offset) as usize)..);
                        let (_, (name, buf)) = class_name_and_buffer(ctx)
                            .context("reference to class")
                            .parse(s)?;
                        (name, buf)
                    }
                };
                (i, (name, buf))
            }
        })
    };
    parser.context("class name and buffer")
}

/// Parse a `Raw` chunk from the given input buffer. This is useful when one does not
/// know the exact type at the time of parsing
pub fn raw<'s, E>(context: &'s Context) -> impl RParser<'s, Raw<'s>, E>
where
    E: RootError<Span<'s>>,
{
    class_name_and_buffer(context).map(|(classinfo, obj)| Raw { classinfo, obj })
}

/// Same as `raw` but doesn't require a `Context` as input. Panics if
/// a `Context` is required to parse the underlying buffer (i.e., the
/// given buffer contains a reference to some other part of the file.
pub fn raw_no_context<'s, E>(input: Span<'s>) -> RResult<'s, (ClassInfo, Span<'s>), E>
where
    E: RootError<Span<'s>>,
{
    use super::ClassInfo::*;
    let parser = |input| {
        let (input, ci) = classinfo.parse(input)?;

        match ci {
            // point to beginning of slice
            References(0) => take(0usize).map(|o| (ci, o)).parse(input),
            New(_) | Exists(_) => length_data(checked_byte_count)
                .complete()
                .context("length-prefixed data")
                .map(|o| (ci, o))
                .parse(input),
            // If its a reference to any other thing but 0 it needs a context
            _ => panic!("Object needs context!"),
        }
    };

    parser.context("raw (no context)").parse(input)
}

/// ESD trigger classes are strings describing a particular
/// Trigger. Each event (but in reality every run) might have a
/// different "menu" of available triggers. The trigger menu is saved
/// as an `TObjArray` of `TNamed` objects for each event. This breaks
/// it down to a simple vector
pub fn parse_tobjarray_of_tnameds<'s, E>(input: Span<'s>) -> RResult<'s, Vec<String>, E>
where
    E: RootError<Span<'s>>,
{
    // each element of the tobjarray has a Vec<u8>
    let (input, vals) = length_value(checked_byte_count, tobjarray_no_context)
        .complete()
        .context("length-prefixed array")
        .parse(input)?;
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
pub fn parse_custom_mantissa<'s, E>(input: Span<'s>, nbits: usize) -> RResult<'s, f32, E>
where
    E: RootError<Span<'s>>,
{
    // TODO: Use ByteOrder crate to be cross-platform?
    pair(be_u8, be_u16)
        .map(|(exp, man)| {
            let mut s = u32::from(exp);
            // Move the exponent into the last 23 bits
            s <<= 23;
            s |= (u32::from(man) & ((1 << (nbits + 1)) - 1)) << (23 - nbits);
            f32::from_bits(s)
        })
        .parse(input)
}

#[cfg(test)]
mod classinfo_test {
    use nom::error::VerboseError;

    use crate::core::Span;

    use super::classinfo;

    /// There is an issue where the following is parsed differently on
    /// nightly ( rustc 1.25.0-nightly (79a521bb9 2018-01-15)), than
    /// on stable, if verbose-errors are enabled for nom in the
    /// cargo.toml
    ///
    /// Passes again as of rustc nightly 1.60.0 (2022-01-12)
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
        let i = Span::new(i.as_slice());
        let (i, _ci) = classinfo::<VerboseError<_>>(i).unwrap();
        // The error manifests in the entire input being (wrongly)
        // consumed, instead of having some left overs
        assert_eq!(i.len(), 352);
    }
}
