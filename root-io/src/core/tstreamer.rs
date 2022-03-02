use nom::Parser;
use nom::multi::{count, length_count, length_value};
use nom::number::complete::{be_i32, be_u16, be_u32};
use nom::sequence::{pair, tuple};
use nom_supreme::ParserExt;
use quote::*;

use std::fmt::Debug;

use crate::{
    code_gen::rust::{ToRustParser, ToRustType},
    code_gen::utils::{alias_or_lifetime, sanitize, type_is_core},
    core::*,
};
use crate::core::SemanticError::VersionNotSupported;

/// Union of all posible `TStreamers`. See figure at
/// <https://root.cern.ch/doc/master/classTStreamerElement.html>
/// for inheritence of ROOT classes
#[derive(Debug)]
#[allow(dead_code)]
pub(crate) enum TStreamer {
    Base {
        el: TStreamerElement,
        /// version number of the base class
        version_base: i32,
    },
    BasicType {
        el: TStreamerElement,
    },
    BasicPointer {
        el: TStreamerElement,
        /// version number of the class with the counter
        cvers: i32,
        /// name of data member holding the array count
        cname: String,
        /// name of the class with the counter
        ccls: String,
    },
    Loop {
        el: TStreamerElement,
        /// version number of the class with the counter
        cvers: i32,
        /// name of data member holding the array count
        cname: String,
        /// name of the class with the counter
        ccls: String,
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
        ctype: TypeId,
    },
    StlString {
        el: TStreamerElement,
        /// type of STL vector
        vtype: StlTypeID,
        /// STL contained type
        ctype: TypeId,
    },
}

/// Every `TStreamer` inherits from `TStreamerElement`
#[derive(Debug)]
#[allow(dead_code)]
pub(crate) struct TStreamerElement {
    ver: u16,
    name: TNamed,
    el_type: TypeId,
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
pub(crate) fn tstreamer<'s, E>(ctx: &'s Context) -> impl RParser<'s, TStreamer, E> + Copy
    where
        E: RootError<Span<'s>>,
{
    let parser = move |i| {
        let (i, (classinfo, obj)) = class_name_and_buffer(ctx).parse(i)?;


        let wrapped_tstreamerelem = length_value(checked_byte_count, tstreamerelement);

        let (_, streamer) = match classinfo {
            "TStreamerBase" => tuple((
                be_u16.context("version"),
                wrapped_tstreamerelem,
                be_i32.context("version base")
            )).map(|(_ver, el, version_base)| TStreamer::Base { el, version_base })
                .all_consuming().context("tstreamer (base)").parse(obj),

            "TStreamerBasicType" => pair(
                be_u16.context("version"),
                wrapped_tstreamerelem,
            ).map(|(_ver, el)| TStreamer::BasicType { el })
                .all_consuming().context("tstreamer (basic type)").parse(obj),

            "TStreamerBasicPointer" => tuple((
                be_u16.context("version"),
                wrapped_tstreamerelem,
                be_i32.context("cvers"),
                string.context("cname"),
                string.context("ccls")
            )).map(|(_ver, el, cvers, cname, ccls)| TStreamer::BasicPointer { el, cvers, cname: cname.to_string(), ccls: ccls.to_string() })
                .all_consuming().context("tstreamer (basic pointer)").parse(obj),

            "TStreamerLoop" => tuple((
                be_u16.context("version"),
                wrapped_tstreamerelem,
                be_i32.context("cvers"),
                string.context("cname"),
                string.context("ccls")
            )).map(|(_ver, el, cvers, cname, ccls)| TStreamer::Loop { el, cvers, cname: cname.to_string(), ccls: ccls.to_string() })
                .all_consuming().context("tstreamer (loop)").parse(obj),

            "TStreamerObject" => pair(
                be_u16.context("version"),
                wrapped_tstreamerelem,
            ).map(|(_ver, el)| TStreamer::Object { el })
                .all_consuming().context("tstreamer (object)").parse(obj),

            "TStreamerObjectPointer" => pair(
                be_u16.context("version"),
                wrapped_tstreamerelem,
            ).map(|(_ver, el)| TStreamer::ObjectPointer { el })
                .all_consuming().context("tstreamer (object pointer)").parse(obj),

            "TStreamerObjectAny" => pair(
                be_u16.context("version"),
                wrapped_tstreamerelem,
            ).map(|(_ver, el)| TStreamer::ObjectAny { el })
                .all_consuming().context("tstreamer (object (any))").parse(obj),

            "TStreamerObjectAnyPointer" => pair(
                be_u16.context("version"),
                wrapped_tstreamerelem,
            ).map(|(_ver, el)| TStreamer::ObjectAnyPointer { el })
                .all_consuming().context("tstreamer (object pointer (any))").parse(obj),

            "TStreamerString" => pair(
                be_u16.context("version"),
                wrapped_tstreamerelem,
            ).map(|(_ver, el)| TStreamer::String { el })
                .all_consuming().context("tstreamer (string)").parse(obj),

            "TStreamerSTL" => tuple((
                be_u16.context("version"),
                wrapped_tstreamerelem,
                be_i32.map(StlTypeID::new).context("vtype"),
                be_i32.map_res(TypeId::new).context("ctype")
            )).map(|(_ver, el, vtype, ctype)| TStreamer::Stl { el, vtype, ctype })
                .all_consuming().context("tstreamer (stl)").parse(obj),

            "TStreamerSTLstring" => {
                // Two version bcs `stlstring` derives from `stl`
                be_u16.precedes(length_value(checked_byte_count, tuple((
                    be_u16.context("version"),
                    wrapped_tstreamerelem,
                    be_i32.map(StlTypeID::new).context("vtype"),
                    be_i32.map_res(TypeId::new).context("ctype")
                )))).map(|(_ver, el, vtype, ctype)| TStreamer::StlString { el, vtype, ctype })
                    .all_consuming().context("tstreamer (stl string)").parse(obj)
            }
            ci => unimplemented!("Unknown TStreamer {}", ci),
        }?;

        Ok((i, streamer))
    };

    parser.context("tstreamer")
}

/// Return all `TSreamerInfo` for the data in this file
pub fn streamers<'s, E>(ctx: &'s Context) -> impl RParser<'s, Vec<TStreamerInfo>, E> + 's
    where
        E: RootError<Span<'s>>,
{
    let parser = move |i| {
        // Dunno why we are 4 bytes off with the size of the streamer info...

        // This TList in the payload has a bytecount in front...
        let (i, tlist_objs) = length_value(checked_byte_count, tlist(ctx))(i)?;
        // Mainly this is a TList of `TStreamerInfo`s, but there might
        // be some "rules" in the end
        let _streamers: Result<Vec<_>, _> = tlist_objs
            .iter()
            .filter_map(|raw| match raw.classinfo {
                "TStreamerInfo" => Some(raw.obj),
                _ => None,
            })
            .map(|buf| Ok(tstreamerinfo(ctx).context("in streamers listing").parse(buf)?.1))
            .collect();
        let streamers = _streamers?;

        // Parse the "rules", if any, from the same tlist
        let _rules: Result<Vec<_>, _> = tlist_objs
            .iter()
            .filter_map(|raw| match raw.classinfo {
                "TList" => Some(raw.obj),
                _ => None,
            })
            .map(|buf| {
                let tl = tlist(ctx).parse(buf)?.1;
                // Each `Rule` is a TList of `TObjString`s
                tl.iter()
                    .map(|el| tobjstring::<'s, E>(el.obj))
                    .collect::<Result<Vec<_>, _>>()
            })
            .collect();
        let _rules = _rules?;

        for raw in tlist_objs {
            match raw.classinfo {
                "TStreamerInfo" | "TList" => {}
                other => println!("Got unexpected class in streamers list: {other}")
            }
        };

        Ok((i, streamers))
    };

    parser.context("streamers")
}

/// The element which is wrapped in a TStreamer
fn tstreamerelement<'s, E>(input: Span<'s>) -> RResult<'s, TStreamerElement, E>
    where
        E: RootError<Span<'s>>,
{

    tuple((
        be_u16.context("version"),
        length_value(checked_byte_count, tnamed).context("name"),
        be_i32.map_res(TypeId::new).context("element type"),
        be_i32.context("size"),
        be_i32.context("array length"),
        be_i32.context("array dimensions")
    )).flat_map(make_fn(|(ver, name, el_type, size, array_len, array_dim): (u16, TNamed, TypeId, i32, i32, i32)| {
        let mut optname = Some(name);
        tuple((
            move |i| if ver == 1 { length_count(be_u32, be_u32)(i) } else { count(be_u32, 5)(i) },
            string,
        )).map_res(move |(max_idx, type_name)| {
            if ver <= 3 {
                Err(VersionNotSupported(Component::TStreamerElement, ver as u32, "must be >= 4"))
            } else {
                Ok(TStreamerElement {
                    ver,
                    name: optname.take().unwrap(),
                    el_type,
                    size,
                    array_len,
                    array_dim,
                    max_idx,
                    type_name: type_name.to_string(),
                })
            }
        })
    })).context("tstreamer element").parse(input)
}

impl TStreamer {
    pub(crate) fn elem(&self) -> &TStreamerElement {
        use self::TStreamer::*;
        // TODO: Move element out of the enum
        match self {
            Base { ref el, .. }
            | BasicType { ref el }
            | BasicPointer { ref el, .. }
            | Loop { ref el, .. }
            | Object { ref el }
            | ObjectPointer { ref el }
            | ObjectAny { ref el }
            | ObjectAnyPointer { ref el }
            | String { ref el }
            | Stl { ref el, .. }
            | StlString { ref el, .. } => el,
        }
    }

    /// Get the comment associated with this particular member
    pub(crate) fn member_comment(&self) -> Ident {
        let cmt = &self.elem().name.title;
        Ident::new(cmt.to_string())
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
        // insert a new line befor and after the comment!
        tokens.append("\n/// ");
        self.member_comment().to_tokens(tokens);
        tokens.append("\n");
        self.member_name().to_tokens(tokens);
        tokens.append(": ");
        self.type_name().to_tokens(tokens);
    }
}

impl ToRustType for TStreamer {
    fn type_name(&self) -> Tokens {
        use self::TypeId::*;
        let name = Ident::new(alias_or_lifetime(&self.elem().name.name.to_owned()));
        match self {
            TStreamer::Base { ref el, .. } => {
                match el.el_type {
                    Object | Base | Named | TObject => quote! {#name},
                    // Not sure about the following branch...
                    InvalidOrCounter(-1) => quote! {#name},
                    _ => panic!("{:#?}", self),
                }
            }
            TStreamer::BasicType { ref el } => match el.el_type {
                Primitive(ref id) => id.type_name(),
                Offset(ref id) => {
                    let s = Ident::new(format!("[{}; {}]", id.type_name(), el.array_len));
                    quote! {#s}
                }
                _ => panic!("{:#?}", self),
            },
            TStreamer::BasicPointer { ref el, .. } => {
                match el.el_type {
                    Array(ref id) => {
                        // Arrays are preceeded by a byte and then have a length given by a
                        // previous member
                        let s = Ident::new(format!("Vec<{}>", id.type_name()));
                        quote! {#s}
                    }
                    _ => panic!("{:#?}", self),
                }
            }
            TStreamer::Object { ref el } => match el.el_type {
                Object => quote! {#name},
                _ => panic!("{:#?}", self),
            },
            TStreamer::ObjectPointer { ref el } => {
                match el.el_type {
                    // Pointers may be null!
                    ObjectP | Objectp => quote! {Option<#name>},
                    _ => panic!("{:#?}", self),
                }
            }
            TStreamer::ObjectAny { ref el } | &TStreamer::ObjectAnyPointer { ref el } => {
                match el.el_type {
                    Any => quote! {#name},
                    AnyP => quote! {#name},
                    // No idea what this is; probably an array of custom type? Found in AliESDs
                    Unknown(82) => quote! {Vec<u8>},
                    _ => panic!("{:#?}", self),
                }
            }
            TStreamer::String { ref el } | TStreamer::StlString { ref el, .. } => {
                match el.el_type {
                    String | Streamer => quote! {String},
                    _ => panic!("{:#?}", self),
                }
            }
            TStreamer::Stl { ref vtype, .. } => match vtype {
                StlTypeID::Vector => {
                    quote! {Stl_vec}
                }
                StlTypeID::Bitset => {
                    quote! {Stl_bitset}
                }
                StlTypeID::String => {
                    quote! {Stl_string}
                }
                StlTypeID::Map => {
                    quote! {Stl_map}
                }
                StlTypeID::MultiMap => {
                    quote! {Stl_map}
                }
            },
            _ => panic!("{:#?}", self),
        }
    }
}

impl ToRustParser for TStreamer {
    fn to_inline_parser(&self) -> Tokens {
        use self::TypeId::*;
        let name = match self {
            //  `Base` types, i.e. types from which the current object inherited;
            // In that case the name is actually the type
            TStreamer::Base { .. } => &self.elem().name.name,
            _ => &self.elem().type_name,
        };
        // Most core-types do not need the context, but some do
        let name = if type_is_core(name.as_str()) && name != "TObjArray" {
            name.to_lowercase()
        } else {
            format!("apply!({}, &context)", name.to_lowercase())
        };

        let name = Ident::new(name);

        match self {
            TStreamer::Base { ref el, .. } => match el.el_type {
                Object | Base | Named => quote! {length_value!(checked_byte_count, #name)},
                TObject => quote! {#name},
                InvalidOrCounter(-1) => {
                    let size = el.size;
                    quote! {map!(take!(#size), |v| v.to_vec())}
                }
                _ => panic!("{:#?}", self),
            },
            TStreamer::BasicType { ref el } => {
                match el.el_type {
                    Primitive(ref id) => id.to_inline_parser(),
                    // Offsets are floating points with a custom mantissa
                    // By default, parse as Vec<u8>
                    Offset(_) => {
                        let size = el.size;
                        quote! {map!(take!(#size), |v| v.to_vec())}
                    }
                    _ => panic!("{:#?}", self),
                }
            }
            TStreamer::BasicPointer {
                ref el, ref cname, ..
            } => {
                let n_entries_array = Ident::new(cname.to_lowercase());
                match el.el_type {
                    Array(ref id) => {
                        // Arrays are preceeded by a byte and then have a length given by a
                        // previous member
                        let b_par = id.to_inline_parser();
                        quote! {preceded!(be_u8, count!(#b_par, #n_entries_array as usize))}
                    }
                    _ => panic!("{:#?}", self),
                }
            }
            TStreamer::Object { ref el } => match el.el_type {
                Object => quote! {length_value!(checked_byte_count, #name)},
                _ => panic!("{:#?}", self),
            },
            TStreamer::ObjectPointer { ref el } => {
                match el.el_type {
                    // Pointers may be null!
                    ObjectP => quote! {switch!(peek!(be_u32),
                    0 => map!(call!(be_u32), |_| None) |
                    _ => map!(call!(_curried_raw), Some))},
                    _ => panic!("{:#?}", self),
                }
            }
            TStreamer::ObjectAny { ref el } | &TStreamer::ObjectAnyPointer { ref el } => {
                match el.el_type {
                    Any => quote! {#name},
                    AnyP => quote! {#name},
                    // No idea what this is; probably an array of custom type? Found in AliESDs
                    Unknown(82) => quote! {map!(eof!(), |o| o.to_vec())},
                    _ => panic!("{:#?}", self),
                }
            }
            TStreamer::String { ref el } | TStreamer::StlString { ref el, .. } => {
                match el.el_type {
                    String | Streamer => quote! {string},
                    _ => panic!("{:#?}", self),
                }
            }
            TStreamer::Stl { ref vtype, .. } => match vtype {
                StlTypeID::Vector => {
                    quote! {stl_vec}
                }
                StlTypeID::Bitset => {
                    quote! {stl_bitset}
                }
                StlTypeID::String => {
                    quote! {stl_string}
                }
                StlTypeID::Map => {
                    quote! {stl_map}
                }
                StlTypeID::MultiMap => {
                    quote! {stl_multimap}
                }
            },
            _ => panic!("{:#?}", self),
        }
    }
}
