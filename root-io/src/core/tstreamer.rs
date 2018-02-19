use quote::*;
use nom::*;

use ::core::*;
use ::code_gen::utils::{sanitize, alias_or_lifetime, type_is_core};
use ::code_gen::rust::{ToRustType, ToRustParser};


/// Union of all posible `TStreamers`. See figure at
/// <https://root.cern.ch/doc/master/classTStreamerElement.html>
/// for inheritence of ROOT classes
#[derive(Debug)]
pub(crate) enum TStreamer {
    Base {
        el: TStreamerElement,
        /// version number of the base class
        version_base: i32
    },
    BasicType {el: TStreamerElement},
    BasicPointer {
        el: TStreamerElement,
        /// version number of the class with the counter
	cvers: i32,
        /// name of data member holding the array count
	cname: String,
        /// name of the class with the counter
	ccls:  String,
    },
    Loop {
        el: TStreamerElement,
        /// version number of the class with the counter
	cvers: i32,
        /// name of data member holding the array count
	cname: String,
        /// name of the class with the counter
	ccls:  String,
    },
    Object {
        el: TStreamerElement,
    },
    ObjectPointer {
        el: TStreamerElement,
    },
    ObjectAny {
        el: TStreamerElement,
    },
    ObjectAnyPointer {
        el: TStreamerElement,
    },
    String {
        el: TStreamerElement,
    },
    Stl {
        el: TStreamerElement,
        /// type of STL vector
	vtype: StlTypeID,
        /// STL contained type
	ctype: TypeID,
    },
    StlString {
        el: TStreamerElement,
        /// type of STL vector
	vtype: StlTypeID,
        /// STL contained type
	ctype: TypeID,
    }
}


/// Every `TStreamer` inherits from `TStreamerElement`
#[derive(Debug)]
pub(crate) struct TStreamerElement {
    ver: u16,
    name: TNamed,
    el_type: TypeID,
    size: i32,
    array_len: i32,
    array_dim: i32,
    max_idx: Vec<u32>,
    type_name: String,
    // For ver == 3
    // pub(crate) xmin: f32,
    // pub(crate) xmax: f32,
    // pub(crate) factor: f32,
}


/// Parse a `TStreamer` from a `Raw` buffer. This is usually the case
/// after reading the `TList` of `TStreamerInfo`s from a ROOT file
pub(crate) fn tstreamer<'c>(raw: &Raw<'c>) -> IResult<&'c[u8], TStreamer>
{
    let wrapped_tstreamerelem = |i| length_value!(i, checked_byte_count, tstreamerelement);
    match raw.classinfo.as_str() {
        "TStreamerBase" => do_parse!(raw.obj,
                                     _ver: be_u16 >>
                                     el: wrapped_tstreamerelem >>
                                     version_base: be_i32 >>
                                     (TStreamer::Base {el, version_base})),
        "TStreamerBasicType" => do_parse!(raw.obj,
                                          _ver: be_u16 >>
                                          el: wrapped_tstreamerelem >>
                                          (TStreamer::BasicType {el})),
        "TStreamerBasicPointer" => do_parse!(raw.obj,
                                             _ver: be_u16 >>
                                             el: wrapped_tstreamerelem >>
                                             cvers: be_i32 >>
                                             cname: string >>
                                             ccls: string >>
                                             (TStreamer::BasicPointer {el, cvers, cname, ccls})),
        "TStreamerLoop" => do_parse!(raw.obj,
                                     _ver: be_u16 >>
                                     el: wrapped_tstreamerelem >>
                                     cvers: be_i32 >>
                                     cname: string >>
                                     ccls: string >>
                                     (TStreamer::Loop {el, cvers, cname, ccls})),
        "TStreamerObject" => do_parse!(raw.obj,
                                       _ver: be_u16 >>
                                       el: wrapped_tstreamerelem >>
                                       (TStreamer::Object {el})),
        "TStreamerObjectPointer" => do_parse!(raw.obj,
                                              _ver: be_u16 >>
                                              el: wrapped_tstreamerelem >>
                                              (TStreamer::ObjectPointer {el})),
        "TStreamerObjectAny" => do_parse!(raw.obj,
                                    _ver: be_u16 >>
                                    el: wrapped_tstreamerelem >>
                                          (TStreamer::ObjectAny {el})),
        "TStreamerObjectAnyPointer" => do_parse!(raw.obj,
                                                 _ver: be_u16 >>
                                                 el: wrapped_tstreamerelem >>
                                                 (TStreamer::ObjectAnyPointer {el})),
        "TStreamerString" => do_parse!(raw.obj,
                                       _ver: be_u16 >>
                                       el: wrapped_tstreamerelem >>
                                       (TStreamer::String {el})),
        "TStreamerSTL" => do_parse!(raw.obj,
                                    _ver: be_u16 >>
                                    el: wrapped_tstreamerelem >>
                                    vtype: map!(be_i32, StlTypeID::new) >>
                                    ctype: map_res!(be_i32, TypeID::new) >>
                                    (TStreamer::Stl {el, vtype, ctype})),
        "TStreamerSTLstring" => do_parse!(raw.obj,
                                          // Two version bcs `stlstring` derives from `stl`
                                          _ver: be_u16 >>
                                          _ver: be_u16 >>
                                          el: wrapped_tstreamerelem >>
                                          vtype: map!(be_i32, StlTypeID::new) >>
                                          ctype: map_res!(be_i32, TypeID::new) >>
                                          (TStreamer::StlString {el, vtype, ctype})),
        ci => unimplemented!("Unknown TStreamer {}", ci)
    }
}


/// The element which is wrapped in a TStreamer
named!(
    tstreamerelement<&[u8], TStreamerElement>,
    do_parse!(ver: be_u16 >>
              name: length_value!(checked_byte_count, tnamed) >>
              el_type: map_res!(be_i32, TypeID::new) >>
              size: be_i32 >>
              array_len: be_i32 >>
              array_dim: be_i32 >>
              max_idx: switch!(value!(ver),
                               1 => length_count!(be_i32, be_u32) |
                               _ => count!(be_u32, 5)) >>
              type_name: string >>
              _eof: eof!() >>
              ({
                  if ver <= 3 {
                      unimplemented!();
                  }
                  TStreamerElement {
                      ver, name, el_type, size, array_len,
                      array_dim, max_idx, type_name
                  }
              })
    )
);


impl TStreamer {
    pub(crate) fn elem(&self) -> &TStreamerElement {
        use self::TStreamer::*;
        match self {
            &Base{ref el, ..} | &BasicType{ref el} | &BasicPointer{ref el, ..} | &Loop{ref el, ..}
            | &Object{ref el} | &ObjectPointer{ref el} | &ObjectAny{ref el} | &ObjectAnyPointer{ref el}
            | &String{ref el} | &Stl{ref el, ..} | &StlString{ref el, ..} => el,
        }
    }

    /// Get the comment associated with this particular member
    pub(crate) fn member_comment(&self) -> Ident {
        let cmt = &self.elem().name.title;
        Ident::new(format!("\n/// {}\n", cmt))
    }
    /// The name of the member/field to be used in the generated struct
    pub(crate) fn member_name(&self) -> Ident {
        let name = sanitize(&self.elem().name.name.to_lowercase());
        Ident::new(name)
    }
}

impl ToTokens for TStreamer {
    /// Converts TStreamer to "\n///comment \n name: type"
    fn to_tokens(&self, tokens: &mut Tokens) {
        self.member_comment().to_tokens(tokens);
        self.member_name().to_tokens(tokens);
        tokens.append(": ");
        self.type_name().to_tokens(tokens);
    }
}

impl ToRustType for TStreamer {
    fn type_name(&self) -> Tokens {
        use self::TypeID::*;
        let name = Ident::new(alias_or_lifetime(&self.elem().name.name.to_owned()));
        match self {
            &TStreamer::Base {ref el, ..} => {
                match el.el_type {
                    Object | Base | Named | TObject => quote!{#name},
                    // Not sure about the following branch...
                    InvalidOrCounter(-1) => quote!{#name},
                    _ => panic!("{:#?}", self),
                }
            },
            &TStreamer::BasicType {ref el} => {
                match el.el_type {
                    Primitive(ref id) => id.type_name(),
                    Offset(ref id) => {
                        let s = Ident::new(format!("[{}; {}]", id.type_name().to_string(), el.array_len));
                        quote!{#s}
                    },
                    _ => panic!("{:#?}", self),
                }
            },
            &TStreamer::BasicPointer {ref el, ..} => {
                match el.el_type {
                    Array(ref id) => {
                        // Arrays are preceeded by a byte and then have a length given by a
                        // previous member
                        let s = Ident::new(format!("Vec<{}>", id.type_name().to_string()));
                        quote!{#s}
                    },
                    _ => panic!("{:#?}", self),
                }
            },
            &TStreamer::Object {ref el} => {
                match el.el_type {
                    Object => quote!{#name},
                    _ => panic!("{:#?}", self),
                }
            },
            &TStreamer::ObjectPointer {ref el} => {
                match el.el_type {
                    // Pointers may be null!
                    ObjectP => quote!{Option<#name>},
                    _ => panic!("{:#?}", self),
                }
            },
            &TStreamer::ObjectAny {ref el} | &TStreamer::ObjectAnyPointer {ref el} => {
                match el.el_type {
                    Any => quote!{#name},
                    AnyP => quote!{#name},
                    // No idea what this is; probably an array of custom type? Found in AliESDs
                    Unknown(82) => quote!{Vec<u8>},
                    _ => panic!("{:#?}", self),
                }
            },
            &TStreamer::String {ref el} => {
                match el.el_type {
                    String => quote!{String},
                    _ => panic!("{:#?}", self),
                }
            },
            &TStreamer::Stl {ref vtype, ..} => {
                match vtype {
                    &StlTypeID::Vector => {
                        quote!{Stl_vec}
                    },
                    &StlTypeID::Bitset => {
                        quote!{Stl_bitset}
                    }
                }
            },
            _ => panic!("{:#?}", self),
        }
    }
}

impl ToRustParser for TStreamer {
    fn to_inline_parser(&self) -> Tokens {
        use self::TypeID::*;
        let name = match self {
            //  `Base` types, i.e. types from which the current object inherited;
            // In that case the name is actually the type
            &TStreamer::Base{..} => &self.elem().name.name,
            _ => &self.elem().type_name,
        };
        // Most core-types do not need the context, but some do
        let name =
            if type_is_core(name.as_str()) && name != "TObjArray" {
                name.to_lowercase()
            } else {
                format!("apply!({}, &context)", name.to_lowercase())
            };
        
        let name = Ident::new(name);

        match self {
            &TStreamer::Base {ref el, ..} => {
                match el.el_type {
                    Object | Base | Named => quote!{length_value!(checked_byte_count, #name)},
                    TObject => quote!{#name},
                    InvalidOrCounter(-1) => {
                        let size = el.size;
                        quote!{map!(take!(#size), |v| v.to_vec())}
                    },
                    _ => panic!("{:#?}", self),
                }
            },
            &TStreamer::BasicType {ref el} => {
                match el.el_type {
                    Primitive(ref id) => id.to_inline_parser(),
                    // Offsets are floating points with a custom mantissa
                    // By default, parse as Vec<u8>
                    Offset(_) => {
                        let size = el.size;
                        quote!{map!(take!(#size), |v| v.to_vec())}
                    },
                    _ => panic!("{:#?}", self),
                }
            },
            &TStreamer::BasicPointer {ref el, ref cname, ..} => {
                let n_entries_array = Ident::new(cname.to_lowercase());
                match el.el_type {
                    Array(ref id) => {
                        // Arrays are preceeded by a byte and then have a length given by a
                        // previous member
                        let b_par = id.to_inline_parser();
                        quote!{preceded!(be_u8, count!(#b_par, #n_entries_array as usize))}
                    },
                    _ => panic!("{:#?}", self),
                }
            },
            &TStreamer::Object {ref el} => {
                match el.el_type {
                    Object => quote!{length_value!(checked_byte_count, #name)},
                    _ => panic!("{:#?}", self),
                }
            },
            &TStreamer::ObjectPointer {ref el} => {
                match el.el_type {
                    // Pointers may be null!
                    ObjectP => quote!{switch!(peek!(be_u32),
                                      0 => map!(call!(be_u32), |_| None) |
                                      _ => map!(call!(_curried_raw), Some))},
                    _ => panic!("{:#?}", self),
                }
            },
            &TStreamer::ObjectAny {ref el} | &TStreamer::ObjectAnyPointer {ref el} => {
                match el.el_type {
                    Any => quote!{#name},
                    AnyP => quote!{#name},
                    // No idea what this is; probably an array of custom type? Found in AliESDs
                    Unknown(82) => quote!{map!(eof!(), |o| o.to_vec())},
                    _ => panic!("{:#?}", self),
                }
            },
            &TStreamer::String {ref el} => {
                match el.el_type {
                    String => quote!{string},
                    _ => panic!("{:#?}", self),
                }
            },
            &TStreamer::Stl {ref vtype, ..} => {
                match vtype {
                    &StlTypeID::Vector => {
                        quote!{stl_vec}
                    },
                    &StlTypeID::Bitset => {
                        quote!{stl_bitset}
                    }
                }
            },
            _ => panic!("{:#?}", self),
        }
    }
}
