use nom::{combinator::verify, error::ParseError, IResult, multi::length_value, number::complete::*, Parser};
use nom::branch::alt;
use nom::sequence::tuple;
use nom_supreme::ParserExt;
use quote::{Ident, Tokens};

use std::fmt::Debug;

use crate::{code_gen::rust::ToRustType, core::*};

/// Parse a bool from a big endian u8
fn be_bool<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> IResult<&[u8], bool, E> {
    let (i, byte) = verify(be_u8, |&byte| byte == 0 || byte == 1)(i)?;
    Ok((i, byte == 1))
}

// Wrap everything once more to avoid exporting the enum variants.
#[derive(Debug, Clone)]
pub struct TLeaf {
    variant: TLeafVariant,
}

impl TLeaf {
    pub fn parse<'s, E>(
        raw: Raw<'s>,
        context: &'s Context,
    ) -> IResult<Raw<'s>, Self, E>
        where
            E: RootError<&'s [u8]>,
    {
        TLeafVariant::parse(raw, context).map(|(i, var)| (i, Self { variant: var }))
    }

    // A helper function to get around some lifetime issues on the caller sider
    pub(crate) fn parse_from_raw<'s, E>(
        ctxt: &'s Context,
    ) -> impl Parser<Raw<'s>, Self, E>
        where
            E: RootError<&'s [u8]>,
    {
        move |raw: Raw<'s>| {
            Self::parse(raw, ctxt)
        }
    }
}

#[derive(Debug, Clone)]
enum TLeafVariant {
    TLeafB(TLeafB),
    TLeafS(TLeafS),
    TLeafI(TLeafI),
    TLeafL(TLeafL),
    TLeafF(TLeafF),
    TLeafD(TLeafD),
    TLeafC(TLeafC),
    TLeafO(TLeafO),
    TLeafD32(TLeafD32),
    TLeafElement(TLeafElement),
}

impl TLeafVariant {
    fn parse<'s, E>(raw: Raw<'s>, context: &'s Context) -> IResult<Raw<'s>, Self, E>
        where
            E: RootError<&'s [u8]> + Debug,
    {
        let i = raw.obj;
        let mk_raw = |i| Raw { obj: i, classinfo: raw.classinfo };
        match raw.classinfo {
            "TLeafB" => TLeafB::parse(i, context).map(|(i, l)| (mk_raw(i), TLeafVariant::TLeafB(l))),
            "TLeafS" => TLeafS::parse(i, context).map(|(i, l)| (mk_raw(i), TLeafVariant::TLeafS(l))),
            "TLeafI" => TLeafI::parse(i, context).map(|(i, l)| (mk_raw(i), TLeafVariant::TLeafI(l))),
            "TLeafL" => TLeafL::parse(i, context).map(|(i, l)| (mk_raw(i), TLeafVariant::TLeafL(l))),
            "TLeafF" => TLeafF::parse(i, context).map(|(i, l)| (mk_raw(i), TLeafVariant::TLeafF(l))),
            "TLeafD" => TLeafD::parse(i, context).map(|(i, l)| (mk_raw(i), TLeafVariant::TLeafD(l))),
            "TLeafC" => TLeafC::parse(i, context).map(|(i, l)| (mk_raw(i), TLeafVariant::TLeafC(l))),
            "TLeafO" => TLeafO::parse(i, context).map(|(i, l)| (mk_raw(i), TLeafVariant::TLeafO(l))),
            "TLeafD32" => TLeafD32::parse(i, context).map(|(i, l)| (mk_raw(i), TLeafVariant::TLeafD32(l))),
            "TLeafElement" => {
                TLeafElement::parse(context).map(TLeafVariant::TLeafElement).parse(i).map(|(i, l)| (mk_raw(i), l))
            }
            name => unimplemented!("Unexpected Leaf type {}", name),
        }
    }
}

macro_rules! make_tleaf_variant {
    // Usually the element size ish what we also use for min/max, but not always
    ($struct_name:ident, $field_type:ty, $parser:ident) => {
        make_tleaf_variant! {$struct_name, $field_type, $parser, std::mem::size_of::<$field_type>()}
    };
    ($struct_name:ident, $field_type:ty, $parser:ident, $size_of_el:expr) => {
        #[derive(Debug, Clone)]
        #[allow(dead_code)]
        struct $struct_name {
            base: TLeafBase,
            fminimum: $field_type,
            fmaximum: $field_type,
        }
        impl $struct_name {
            fn parse<'s, E>(i: &'s [u8], context: &'s Context) -> IResult<&'s [u8], Self, E>
            where
                E: RootError<&'s [u8]>,
            {
                // All known descendens have version 1
                let (i, _) = verify(be_u16, |&ver| ver == 1)(i)?;
                let (i, base) =
                    length_value(checked_byte_count, TLeafBase::parse(context))(i)?;
                let (i, fminimum) = $parser(i)?;
                let (i, fmaximum) = $parser(i)?;
                let obj = Self {
                    base,
                    fminimum,
                    fmaximum,
                };
                obj.verify_consistency().unwrap();
                Ok((i, obj))
            }

            fn verify_consistency(&self) -> Result<(), String> {
                if self.base.flentype as usize != $size_of_el {
                    return Err(String::from("Unexpected type length"));
                }
                if self.base.fisunsigned {
                    // The minimum and maximum values are possibly wrong
                    // return Err(String::from("Expected signed value"));
                }
                Ok(())
            }
        }
    };
}

make_tleaf_variant! {TLeafB, i8, be_i8}
make_tleaf_variant! {TLeafS, i16, be_i16}
make_tleaf_variant! {TLeafI, i32, be_i32}
make_tleaf_variant! {TLeafL, i64, be_i64}
make_tleaf_variant! {TLeafF, f32, be_f32}
make_tleaf_variant! {TLeafD, f64, be_f64}
// TLeafC has chars as elements
make_tleaf_variant! {TLeafC, i32, be_i32, 1}
make_tleaf_variant! {TLeafO, bool, be_bool}
make_tleaf_variant! {TLeafD32, f32, be_f32}

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct TLeafElement {
    base: TLeafBase,
    fid: i32,
    ftype: TypeId,
}

impl TLeafElement {
    fn parse<'s, E>(context: &'s Context) -> impl Parser<&'s [u8], Self, E>
        where
            E: RootError<&'s [u8]>,
    {
        be_u16.verify(|&ver| ver == 1).precedes(
            tuple((
                length_value(checked_byte_count, TLeafBase::parse(context)),
                be_i32,
                be_i32.map_res(TypeId::new)
            )).map(make_fn(|(base, fid, ftype)| Self { base, fid, ftype }))
        ).context("TLeaf")
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct TLeafBase {
    /// Version of the read layout
    ver: u16,
    /// The basis for a named object (name, title)
    tnamed: TNamed,
    /// Number of fixed length elements
    flen: i32,
    /// Number of bytes for this data type
    flentype: i32,
    /// Offset in ClonesArray object (if one)
    foffset: i32,
    /// (=kTRUE if leaf has a range, kFALSE otherwise)
    fisrange: bool,
    /// (=kTRUE if unsigned, kFALSE otherwise)
    fisunsigned: bool,
    /// Pointer to Leaf count if variable length (we do not own the counter)
    fleafcount: Option<Box<TLeafVariant>>,
}

impl TLeafBase {
    fn parse<'s, E>(context: &'s Context) -> impl Parser<&'s [u8], Self, E>
        where
            E: RootError<&'s [u8]>,
    {
        move |i| {
            let (i, ver) = be_u16(i)?;
            let (i, tnamed) = length_value(checked_byte_count, tnamed)(i)?;
            let (i, flen) = be_i32(i)?;
            let (i, flentype) = be_i32(i)?;
            let (i, foffset) = be_i32(i)?;
            let (i, fisrange) = be_bool(i)?;
            let (i, fisunsigned) = be_bool(i)?;
            let (i, fleafcount): (&'s [u8], Option<Box<TLeafVariant>>) = {
                alt((
                    be_u32.verify(|&v| v == 0).map(|_| None),
                    raw(context)
                        .and_then(|r: Raw<'s>| TLeafVariant::parse(r, context))
                        .map(|leaf| Some(Box::new(leaf)))
                )).parse(i)?
            };
            Ok((
                i,
                Self {
                    ver,
                    tnamed,
                    flen,
                    flentype,
                    foffset,
                    fisrange,
                    fisunsigned,
                    fleafcount,
                },
            ))
        }
    }
}

/// If we have more than one element make this
fn arrayfy_maybe(ty_name: &str, len: usize) -> Tokens {
    // not an array
    let t = Ident::new(ty_name);
    if len == 1 {
        quote! {#t}
    } else {
        // array
        quote! {[#t; #len]}
    }
}

impl ToRustType for TLeaf {
    fn type_name(&self) -> Tokens {
        use TLeafVariant::*;
        let (type_name, len) = match &self.variant {
            TLeafO(l) => ("bool", l.base.flen),
            TLeafB(l) => (if l.base.fisunsigned { "u8" } else { "i8" }, l.base.flen),
            TLeafS(l) => (if l.base.fisunsigned { "u16" } else { "i16" }, l.base.flen),
            TLeafI(l) => (if l.base.fisunsigned { "u32" } else { "i32" }, l.base.flen),
            TLeafL(l) => (if l.base.fisunsigned { "u64" } else { "i64" }, l.base.flen),
            TLeafF(l) => ("f32", l.base.flen),
            TLeafD(l) => ("f64", l.base.flen),
            TLeafC(l) => ("String", l.base.flen),
            l => todo!("{:?}", l),
        };
        arrayfy_maybe(type_name, len as usize)
    }
}
