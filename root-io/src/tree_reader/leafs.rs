use std::fmt;

use nom::number::complete::*;
use nom::*;
use quote::{Ident, Tokens};

use crate::{code_gen::rust::ToRustType, core::*};

#[derive(Debug, Clone)]
pub struct TLeafBase {
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
    fisrange: u8,
    /// (=kTRUE if unsigned, kFALSE otherwise)
    fisunsigned: u8,
    /// Pointer to Leaf count if variable length (we do not own the counter)
    fleafcount: Option<Box<TLeaf>>,
}

#[derive(Debug, Clone)]
pub struct TLeafElement {
    /// `TLeaf` header
    base: TLeafBase,
    /// Element serial number in fInfo; No idea what this does...
    id: i32,
    /// TStreamerType number
    type_id: TypeID,
}

#[derive(Clone)]
pub enum TLeaf {
    /// TLeafX describing a `primitive` type such as u16. The first
    /// element of the tupel is the name of the TLeaf class such as
    /// TLeafF.
    Primitive(String, TLeafBase),
    /// Variant describing a `TLeafC`, which describes a variable length string
    String(TLeafBase),
    /// TLeafElement describes more complicated types such as fixed size arrays
    Element(TLeafElement),
    /// `TLeafObject`s are used if a custom object is streamed in one chunk
    /// For now, this is just treated as a blob of &[u8]
    Object(String, TLeafBase),
}

impl fmt::Debug for TLeaf {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Rust leaf type `{}`;", self.type_name())?;
        match *self {
            TLeaf::Primitive(ref leaf_name, ref leaf) => {
                writeln!(f, "`{}`: {:#?}", leaf_name, leaf)
            }
            TLeaf::String(ref leaf) => writeln!(f, "String: {:#?}", leaf),
            TLeaf::Element(ref leaf) => writeln!(f, "Element: {:#?}", leaf),
            TLeaf::Object(ref leaf_name, ref leaf) => writeln!(f, "`{}`: {:#?}", leaf_name, leaf),
        }
    }
}

impl ToRustType for TLeaf {
    fn type_name(&self) -> Tokens {
        match *self {
            TLeaf::Primitive(ref leaf_name, ref leaf) => {
                if leaf.flen != 1 || leaf.foffset != 0 || leaf.fleafcount.is_some() {
                    panic!("Unexpected TLeaf: \n{:#?}", leaf);
                }
                let t = match leaf_name.as_str() {
                    "TLeafO" => {
                        if leaf.flentype != 1 {
                            panic!("Unexpected type length")
                        }
                        "bool"
                    }
                    "TLeafB" => {
                        if leaf.flentype != 1 {
                            panic!("Unexpected type length")
                        };
                        if leaf.fisunsigned == 1 {
                            "u8"
                        } else {
                            "i8"
                        }
                    }
                    "TLeafS" => {
                        if leaf.flentype != 2 {
                            panic!("Unexpected type length")
                        };
                        if leaf.fisunsigned == 1 {
                            "u16"
                        } else {
                            "i16"
                        }
                    }
                    "TLeafI" => {
                        if leaf.flentype != 4 {
                            panic!("Unexpected type length")
                        };
                        if leaf.fisunsigned == 1 {
                            "u32"
                        } else {
                            "i32"
                        }
                    }
                    "TLeafL" => {
                        if leaf.flentype != 8 {
                            panic!("Unexpected type length")
                        };
                        if leaf.fisunsigned == 1 {
                            "u64"
                        } else {
                            "i64"
                        }
                    }
                    "TLeafF" => {
                        if leaf.flentype != 4 || leaf.fisunsigned != 0 {
                            panic!("Unexpected type length or sign: {:#?}", leaf);
                        };
                        "f32"
                    }
                    "TLeafD" => {
                        if leaf.flentype != 8 || leaf.fisunsigned != 0 {
                            panic!("Unexpected type length or sign: {:#?}", leaf);
                        };
                        "f64"
                    }
                    name => panic!("Unexpected TLeaf type name {}", name),
                };
                // not an array
                let t = Ident::new(t);
                if leaf.flen == 1 {
                    quote! {#t}
                } else {
                    // array
                    let s = format!("[{}; {}]", t, leaf.flen);
                    quote! {#s}
                }
            }
            TLeaf::String(_) => quote!(String),
            TLeaf::Element(ref tleaf_el) => {
                match &tleaf_el.type_id {
                    &TypeID::Primitive(ref id) | &TypeID::Offset(ref id) => {
                        let t = id.type_name().to_string();
                        if tleaf_el.base.flen > 1 {
                            let t = Ident::new(format!("[{}; {}]", t, tleaf_el.base.flen));
                            quote! {#t}
                        } else {
                            let t = Ident::new(t);
                            quote! {#t}
                        }
                    }
                    id @ &TypeID::InvalidOrCounter(_) => {
                        // If this is used as a counter, its type id is
                        // -1 but its "serial id" is 0 (else -2)...
                        if tleaf_el.id == 0 {
                            quote! {u32}
                        } else {
                            id.type_name()
                        }
                    }
                    id => id.type_name(),
                }
            }
            // Treating streamed objects as blobs of &[u8] for now
            TLeaf::Object(_, _) => quote! {Vec<u8>},
        }
    }
}

/// Helper function to parse the header of a `TLeaf`; Note that each
/// `TLeaf` also has a type specific part which is ingnored here!
pub(crate) fn tleaf<'s>(
    i: &'s [u8],
    context: &'s Context,
    c_name: &str,
) -> IResult<&'s [u8], TLeaf> {
    // let c_name = c_name.as_bytes();
    match c_name {
        // Variable length string; has same layout as `Primitive`
        "TLeafC" => map!(i, call!(tleafprimitive, context), TLeaf::String),
        "TLeafElement" => map!(i, call!(tleafelement, context), TLeaf::Element),
        "TLeafObject" => map!(i, call!(tleafobject, context), |v| TLeaf::Object(
            c_name.to_string(),
            v
        )),
        _ => map!(i, call!(tleafprimitive, context), |v| TLeaf::Primitive(
            c_name.to_string(),
            v
        )),
    }
}

#[rustfmt::skip::macros(do_parse)]
fn tleafbase<'s>(input: &'s [u8], context: &'s Context<'s>) -> IResult<&'s [u8], TLeafBase> {
    let _curried_raw = |i| raw(i, context);
    do_parse!(input,
              ver: be_u16 >>
              tnamed: length_value!(checked_byte_count, tnamed) >>
              flen: be_i32 >>
              flentype: be_i32 >>
              foffset: be_i32 >>
              fisrange: be_u8 >>
              fisunsigned: be_u8 >>
              fleafcount:
              switch!(
                  peek!(be_u32),
                  0 => map!(call!(be_u32), | _ | None) |
                  _ => map!(
                      map!(
                          call!(_curried_raw),
                          |r| tleaf(r.obj, context, &r.classinfo).unwrap().1
                      ),
                      |i| Some(Box::new(i)))
              ) >>
              ({
                  TLeafBase {
                      ver,
                      tnamed,
                      flen,
                      flentype,
                      foffset,
                      fisrange,
                      fisunsigned,
                      fleafcount } }))
}

#[rustfmt::skip::macros(do_parse)]
fn tleafelement<'s>(input: &'s [u8], context: &'s Context<'s>) -> IResult<&'s [u8], TLeafElement> {
    do_parse!(input,
              _ver: be_u16 >>
              base: length_value!(checked_byte_count, call!(tleafbase, context)) >>
              id: be_i32 >>
              type_id: map_res!(be_i32, TypeID::new) >>
              (TLeafElement {base, id, type_id})
    )
}

#[rustfmt::skip::macros(do_parse)]
fn tleafprimitive<'s>(input: &'s [u8], context: &'s Context<'s>) -> IResult<&'s [u8], TLeafBase> {
    do_parse!(input,
              _ver: be_u16 >>
              base: length_value!(checked_byte_count, call!(tleafbase, context)) >>
              _fmaximum: be_f32 >>
              _fminimum: be_f32 >>
              (base)
    )
}

#[rustfmt::skip::macros(do_parse)]
fn tleafobject<'s>(input: &'s [u8], context: &'s Context<'s>) -> IResult<&'s [u8], TLeafBase> {
    do_parse!(input,
              _ver: be_u16 >>
              base: length_value!(checked_byte_count, call!(tleafbase, context)) >>
              _fvirtual: be_u8 >>
              (base)
    )
}
