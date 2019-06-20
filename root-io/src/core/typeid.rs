use failure::Error;
use quote::*;

use ::code_gen::rust::{ToRustType, ToRustParser};

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
    ObjectP,
    String,
    AnyP,
    STL,
    STLString,
    Streamer,
    Unknown(i32)
}

/// ID describing a primitive type. This is a subset (1..19) of the integers used for `TypeID`.
#[derive(Debug, Clone)]
pub(crate) struct PrimitiveID(pub(crate) i32);

/// Type of a streamed STL container
#[derive(Debug, Clone)]
pub(crate) enum StlTypeID {
    Vector,
    Bitset,
}


impl PrimitiveID {
    pub(crate) fn new(id: i32) -> Result<PrimitiveID, Error> {
        match id {
            1...19 => Ok(PrimitiveID(id)),
            id => Err(format_err!("Invalid base type id {}", id)),
        }
    }
}

impl TypeID {
    pub(crate) fn new(id: i32) -> Result<TypeID, Error> {
        use self::TypeID::*;
        Ok(match id {
            // -1 may mean that this branch / leaf has no data, or that it has an elements-per-entry array...
            -1 => InvalidOrCounter(id),
            0 => Base,
            id@1 ... 19 => Primitive(PrimitiveID::new(id)?),
            id@21 ... 39 => Offset(PrimitiveID::new(id - 20)?),
            id@41 ... 59 => Array(PrimitiveID::new(id - 40)?),
            61 => Object,
            62 => Any,
            64 => ObjectP,
            65 => String,
            66 => TObject,
            67 => Named,
            69 => AnyP,
            300 => STL,
            365 => STLString,
            500 => Streamer,
            id => Unknown(id)
        })
    }
}

impl StlTypeID {
    pub(crate) fn new(id: i32) -> StlTypeID {
        use self::StlTypeID::*;
        match id {
            1 => Vector,
            8 => Bitset,
            _ => unimplemented!("`StlTypeID` {} not implemented.", id)
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
                Object | STL | STLString | Streamer | Unknown(82) => "Vec<u8>".to_string(),
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
        let t = match self.0 {
            1 => "be_i8", //"kChar",
            2 => "be_i16", //"kShort",
            3 | 6 => "be_i32", //"kInt", "kCounter",
            4 => "be_i64", //"kLong",
            5 => "be_f32", //"kFloat",
            // "kCharStar"
            7 => unimplemented!("{:?}: type not implemented, yet", self),
            8 => "be_f64", //"kDouble",
            // "kDouble32"; This one is nasty! Check the TFileBuffer.cxx sources in ROOT
            9 => "custom_float",
            // "kLegacyChar"
            10 => unimplemented!("{:?}: type not implemented, yet", self),
            11 => "be_u8", //"kUChar",
            12 => "be_u16", //"kUShort",
            13 => "be_u32", //"kUInt",
            14 => "be_u64", //"kULong",
            15 => "be_u32",  // "kBits",
            16 => "be_i64", //"kLong64",
            17 => "be_u64", //"kULong64",
            18 => "be_u8", //"kBool",
            19 => "be_f16", //"kFloat16",
            id => panic!("Invalid base type id {} which should not be possible here", id),
        };
        let t = Ident::new(t);
        quote!(#t)
    }
}


impl ToRustType for PrimitiveID {
    fn type_name(&self) -> Tokens {
        let t = match self.0 {
            1 => "i8", //"kChar",
            2 => "i16", //"kShort",
            3 | 6 => "i32", //"kInt", "kCounter",
            4 => "i64", //"kLong",
            5 => "f32", //"kFloat",
            // "kCharStar"
            7 => unimplemented!("{:?}: type not implemented, yet", self),
            8 => "f64", //"kDouble",
            // "kDouble32"; This one is nasty! Check the TFileBuffer.cxx sources in ROOT
            9 => "f32",
            // "kLegacyChar"
            10 => unimplemented!("{:?}: type not implemented, yet", self),
            11 => "u8", //"kUChar",
            12 => "u16", //"kUShort",
            13 => "u32", //"kUInt",
            14 => "u64", //"kULong",
            15 => "u32",  // "kBits",
            16 => "i64", //"kLong64",
            17 => "u64", //"kULong64",
            18 => "u8", //"kBool",
            19 => "f16", //"kFloat16",
            id => panic!("Invalid base type id {} which should not be possible here", id),
        };
        let t = Ident::new(t);
        quote!(#t)
    }
}
