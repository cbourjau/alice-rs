use std::f64::consts::PI;

use failure::Error;
use quote::*;
use regex::Regex;

use crate::code_gen::rust::{ToRustParser, ToRustType};

/// Integer ID describing a streamed type in a `TStreamer`
#[derive(Debug, Clone)]
pub(crate) enum TypeID {
    InvalidOrCounter(i32),
    Primitive(PrimitiveID),
    Offset(PrimitiveID),
    Array(PrimitiveID),
    Base,
    Object,
    Named,
    TObject,
    Any,
    Objectp,
    ObjectP,
    String,
    AnyP,
    Stl,
    StlString,
    Streamer,
    Unknown(i32),
}

/// Type of a streamed STL container
#[derive(Debug, Clone)]
pub(crate) enum StlTypeID {
    Vector,
    Bitset,
    String,
    Map,
    MultiMap,
}

/// ID describing a primitive type. This is a subset (1..19) of the integers used for `TypeID`.
#[derive(Debug, Clone)]
pub(crate) enum PrimitiveID {
    KChar,                    // 1 => "i8"
    KShort,                   // 2 => "i16"
    KInt,                     // 3 => i32
    KCounter,                 // 6 => i32
    KLong,                    // 4 => "i64"
    KFloat,                   // 5 => "f32"
    KCharStar,                // 7 => "&'s str"
    KDouble,                  // 8 => "f64"
    KDouble32(f64, f64, u32), // 9 => "f64"
    KLegacyChar,              // 10 => unimplemented!()
    KUChar,                   // 11 => "u8"
    KUShort,                  // 12 => "u16"
    KUInt,                    // 13 => "u32"
    KULong,                   // 14 => "u64"
    KBits,                    // 15 => "u32"
    KLong64,                  // 16 => "i64"
    KULong64,                 // 17 => "u64"
    KBool,                    // 18 => "u8"
    KFloat16,                 // 19 => unimplemented!()
}

impl PrimitiveID {
    fn new(id: i32, comment_str: &str) -> Result<PrimitiveID, Error> {
        use PrimitiveID::*;
        Ok(match id {
            1 => KChar,
            2 => KShort,
            3 => KInt,
            6 => KCounter,
            4 => KLong,
            5 => KFloat,
            7 => KCharStar,
            8 => KDouble,
            9 => {
                // https://root.cern/doc/master/classTBufferFile.html#acdff906aa
                let re = Regex::new(r"^(\s*\[\w+\]\s*)?\[([^,]+),([^,]+)(,([^,]+))?\]").unwrap();
                match re.captures(comment_str) {
                    Some(caps) => {
                        let (min, max, nbits) = (
                            evaluate_range_element(&caps[2])?,
                            evaluate_range_element(&caps[3])?,
                            match caps.get(5) {
                                Some(cap) => cap.as_str().trim().parse().map(|val| {
                                    if !(2..=32).contains(&val) {
                                        32
                                    } else {
                                        val
                                    }
                                })?,
                                None => 32,
                            },
                        );
                        let mod_min = {
                            if min >= max && nbits < 15 {
                                nbits as f64 + 0.1
                            } else {
                                min
                            }
                        };

                        KDouble32(mod_min, max, nbits)
                    }
                    // No range specified. This is a normal f32.
                    None => KFloat,
                }
            }
            10 => KLegacyChar,
            11 => KUChar,
            12 => KUShort,
            13 => KUInt,
            14 => KULong,
            15 => KBits,
            16 => KLong64,
            17 => KULong64,
            18 => KBool,
            19 => KFloat16,
            id => Err(format_err!("Invalid base type id {}", id))?,
        })
    }
}

impl TypeID {
    pub(crate) fn new(id: i32, comment_str: &str) -> Result<TypeID, Error> {
        use self::TypeID::*;
        Ok(match id {
            // -1 may mean that this branch / leaf has no data, or that it has an elements-per-entry array...
            -1 => InvalidOrCounter(id),
            0 => Base,
            id @ 1..=19 => Primitive(PrimitiveID::new(id, comment_str)?),
            id @ 21..=39 => Offset(PrimitiveID::new(id - 20, comment_str)?),
            id @ 41..=59 => Array(PrimitiveID::new(id - 40, comment_str)?),
            61 => Object,
            62 => Any,
            63 => Objectp,
            64 => ObjectP,
            65 => String,
            66 => TObject,
            67 => Named,
            69 => AnyP,
            300 => Stl,
            365 => StlString,
            500 => Streamer,
            id => Unknown(id),
        })
    }
}

impl StlTypeID {
    pub(crate) fn new(id: i32) -> StlTypeID {
        match id {
            1 => StlTypeID::Vector,
            4 => StlTypeID::Map,
            5 => StlTypeID::MultiMap,
            8 => StlTypeID::Bitset,
            365 => StlTypeID::String,
            _ => unimplemented!("`StlTypeID` {} not implemented.", id),
        }
    }
}

impl ToRustType for TypeID {
    fn type_name(&self) -> Tokens {
        use self::TypeID::*;
        let t = match self {
            Primitive(ref id) | Offset(ref id) => id.type_name().to_string(),
            Array(ref id) => format!("Vec<{}>", id.type_name()),
            // "kObjectP"; might be null!
            ObjectP => "Option<Raw<'s>>".to_string(),
            String => "String".to_string(),
            // Some funky things which we just treat as byte strings for now
            Object | Stl | StlString | Streamer | Unknown(82) => "Vec<u8>".to_string(),
            Any => "Vec<u8>".to_string(),
            AnyP => "Vec<u8>".to_string(),
            InvalidOrCounter(-1) => "u32".to_string(),
            _ => panic!("{:?}: type not implemented, yet", self),
        };
        let t = Ident::new(t);
        quote!(#t)
    }
}

impl ToRustParser for PrimitiveID {
    fn to_inline_parser(&self) -> Tokens {
        match self {
            PrimitiveID::KChar => quote! {nom::number::complete::be_i8},
            PrimitiveID::KShort => quote! {nom::number::complete::be_i16},
            PrimitiveID::KInt => quote! {nom::number::complete::be_i32},
            PrimitiveID::KCounter => quote! {nom::number::complete::be_i32},
            PrimitiveID::KLong => quote! {nom::number::complete::be_i64},
            PrimitiveID::KFloat => quote! {nom::number::complete::be_f32},
            PrimitiveID::KCharStar => quote! { c_string },
            PrimitiveID::KDouble => quote! {nom::number::complete::be_f64},
            // This one is nasty! Check the
            // TFileBuffer.cxx sources in ROOT and:
            // https://root.cern/root/html606/classTBufferFile.html#a44c2adb6fb1194ec999b84aed259e5bc
            // and
            // https://root.cern/root/html606/TStreamerElement_8cxx.html#a4d6c86845bee19cf28c93a531ec50f29
            PrimitiveID::KDouble32(min, max, nbits) => {
                quote!(parse_custom_mantissa(#min, #max, #nbits))
            }
            PrimitiveID::KLegacyChar => unimplemented!("{:?}: type not implemented, yet", self),
            PrimitiveID::KUChar => quote! {nom::number::complete::be_u8},
            PrimitiveID::KUShort => quote! {nom::number::complete::be_u16},
            PrimitiveID::KUInt => quote! {nom::number::complete::be_u32},
            PrimitiveID::KULong => quote! {nom::number::complete::be_u64},
            PrimitiveID::KBits => quote! {nom::number::complete::be_u32},
            PrimitiveID::KLong64 => quote! {nom::number::complete::be_i64},
            PrimitiveID::KULong64 => quote! {nom::number::complete::be_u64},
            PrimitiveID::KBool => quote! {nom::number::complete::be_u8},
            PrimitiveID::KFloat16 => quote! {custom_float16},
        }
    }
}

impl PrimitiveID {
    pub(crate) fn type_name_str(&self) -> &str {
        use PrimitiveID::*;
        match self {
            KChar => "i8",
            KShort => "i16",
            KInt => "i32",
            KCounter => "i32",
            KLong => "i64",
            KFloat => "f32",
            KCharStar => "&'s str",
            KDouble => "f64",
            KDouble32(_, _, _) => "f64",
            KLegacyChar => unimplemented!("{:?}: type not implemented", self),
            KUChar => "u8",
            KUShort => "u16",
            KUInt => "u32",
            KULong => "u64",
            KBits => "u32",
            KLong64 => "i64",
            KULong64 => "u64",
            KBool => "u8",
            KFloat16 => "f16",
        }
    }
}

impl ToRustType for PrimitiveID {
    fn type_name(&self) -> Tokens {
        let t = Ident::new(self.type_name_str());
        quote!(#t)
    }
}

fn remove_whitespace(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}

/// Very primitve logic for evaluating comment ranges
fn evaluate_range_element(comment_str: &str) -> Result<f64, Error> {
    // Remove all whites spaces
    let comment_string = remove_whitespace(comment_str);
    let comment_str = comment_string.as_str();

    // Is a simple float
    if let Ok(float) = comment_str.parse() {
        return Ok(float);
    }

    // Might contain "pi"
    let (negate, comment_str) = {
        match comment_str.strip_prefix('-') {
            Some(rest) => (true, rest),
            None => (false, comment_str),
        }
    };
    let val = match comment_str {
        "2pi" | "2*pi" | "twopi" => 2. * PI,
        "pi/2" => PI / 2.,
        "pi/4" => PI / 4.,
        "pi" => PI,
        s => Err(format_err!("Unrecognized element in comment string {}", s))?,
    };

    if negate {
        Ok(-val)
    } else {
        Ok(val)
    }
}
