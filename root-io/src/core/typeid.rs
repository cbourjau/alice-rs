use quote::*;
use thiserror::Error;

use crate::code_gen::rust::{ToRustParser, ToRustType};

#[derive(Error, Debug)]
pub enum TypeIdError {
    #[error("Invalid Type Id {0}")]
    InvalidTypeId(i32),
    #[error("Invalid Primitive Id")]
    PrimitiveError(#[from] InvalidPrimitive),
}

#[derive(Error, Debug)]
pub enum InvalidPrimitive {
    #[error("Invalid Primitive Id {0}")]
    Id(i32),
    #[error("Invalid Primitive Offset {0}")]
    Offset(i32),
    #[error("Invalid Primitive Array {0}")]
    Array(i32),
}

/// Integer ID describing a streamed type in a `TStreamer`
#[derive(Debug, Clone, Copy)]
pub(crate) enum TypeId {
    InvalidOrCounter(i32),
    Primitive(PrimitiveId),
    Offset(PrimitiveId),
    Array(PrimitiveId),
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

/// ID describing a primitive type. This is a subset (1..19) of the integers used for `TypeID`.
#[derive(Debug, Clone, Copy)]
pub(crate) struct PrimitiveId(pub(crate) i32);

/// Type of a streamed STL container
#[derive(Debug, Clone, Copy)]
pub(crate) enum StlTypeID {
    Vector,
    Bitset,
    String,
    Map,
    MultiMap,
}

impl PrimitiveId {
    pub(crate) fn new(id: i32) -> Option<PrimitiveId> {
        match id {
            1..=19 => Some(PrimitiveId(id)),
            _id => None,
        }
    }
}

impl TypeId {
    pub(crate) fn new(id: i32) -> Result<TypeId, TypeIdError> {
        use self::TypeId::*;
        Ok(match id {
            // -1 may mean that this branch / leaf has no data, or that it has an elements-per-entry array...
            -1 => InvalidOrCounter(id),
            0 => Base,
            id @ 1..=19 => Primitive(PrimitiveId::new(id).ok_or(InvalidPrimitive::Id(id))?),
            id @ 21..=39 => Offset(PrimitiveId::new(id - 20).ok_or(InvalidPrimitive::Offset(id))?),
            id @ 41..=59 => Array(PrimitiveId::new(id - 40).ok_or(InvalidPrimitive::Array(id))?),
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

impl ToRustType for TypeId {
    fn type_name(&self) -> Tokens {
        use self::TypeId::*;
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

impl ToRustParser for PrimitiveId {
    fn to_inline_parser(&self) -> Tokens {
        let t = match self.0 {
            1 => "be_i8",      //"kChar",
            2 => "be_i16",     //"kShort",
            3 | 6 => "be_i32", //"kInt", "kCounter",
            4 => "be_i64",     //"kLong",
            5 => "be_f32",     //"kFloat",
            // "kCharStar"
            7 => unimplemented!("{:?}: type not implemented, yet", self),
            8 => "be_f64", //"kDouble",
            // "kDouble32"; This one is nasty! Check the TFileBuffer.cxx sources in ROOT
            9 => "custom_float",
            // "kLegacyChar"
            10 => unimplemented!("{:?}: type not implemented, yet", self),
            11 => "be_u8",  //"kUChar",
            12 => "be_u16", //"kUShort",
            13 => "be_u32", //"kUInt",
            14 => "be_u64", //"kULong",
            15 => "be_u32", // "kBits",
            16 => "be_i64", //"kLong64",
            17 => "be_u64", //"kULong64",
            18 => "be_u8",  //"kBool",
            19 => "be_f16", //"kFloat16",
            id => panic!(
                "Invalid base type id {} which should not be possible here",
                id
            ),
        };
        let t = Ident::new(t);
        quote!(#t)
    }
}

impl ToRustType for PrimitiveId {
    fn type_name(&self) -> Tokens {
        let t = match self.0 {
            1 => "i8",      //"kChar",
            2 => "i16",     //"kShort",
            3 | 6 => "i32", //"kInt", "kCounter",
            4 => "i64",     //"kLong",
            5 => "f32",     //"kFloat",
            // "kCharStar"
            7 => unimplemented!("{:?}: type not implemented, yet", self),
            8 => "f64", //"kDouble",
            // "kDouble32"; This one is nasty! Check the TFileBuffer.cxx sources in ROOT
            9 => "f32",
            // "kLegacyChar"
            10 => unimplemented!("{:?}: type not implemented, yet", self),
            11 => "u8",  //"kUChar",
            12 => "u16", //"kUShort",
            13 => "u32", //"kUInt",
            14 => "u64", //"kULong",
            15 => "u32", // "kBits",
            16 => "i64", //"kLong64",
            17 => "u64", //"kULong64",
            18 => "u8",  //"kBool",
            19 => "f16", //"kFloat16",
            id => panic!(
                "Invalid base type id {} which should not be possible here",
                id
            ),
        };
        let t = Ident::new(t);
        quote!(#t)
    }
}
