use quote::*;
use nom::HexDisplay;
use nom::{IResult, be_u16, be_u32};

use ::core::{TStreamer, tstreamer, TNamed, Context};
use ::core::parsers::*;
use ::code_gen::rust::{ToRustType, ToRustParser, ToNamedRustParser, ToRustStruct};
use ::code_gen::utils::{type_is_core};


#[derive(Debug)]
pub struct TStreamerInfo {
    tstreamerinfo_ver: u16,
    named: TNamed,
    checksum: u32,
    new_class_version: u32,
    data_members: Vec<TStreamer>,
}

/// Parse one `TStreamerInfo` object (as found in the `TList`)
pub(crate) fn tstreamerinfo<'s, 'c>(input: &'s[u8], context: &'c Context) -> IResult<&'s[u8], TStreamerInfo>
    where 's: 'c
{
    let wrapped_tobjarray = |i| tobjarray(i, &context);
    do_parse!(input,
              tstreamerinfo_ver: be_u16 >>
              named: length_value!(checked_byte_count, tnamed) >>
              checksum: be_u32 >>
              new_class_version: be_u32 >>
              _size_tobjarray_with_class_info: checked_byte_count >>
              _class_info_objarray: classinfo >>
              data_members: length_value!(checked_byte_count, wrapped_tobjarray) >>
              _eof: eof!() >>
              ({
                  let data_members = data_members.iter()
                      .filter_map(|el| {
                          match tstreamer(el) {
                              Ok((_, v)) => Some(v),
                              _ => {println!("Failed to parse TStreamer for {}:\n{}",
                                             el.classinfo, el.obj.to_hex(16));
                                    None
                              },
                          }
                      })
                      .collect();
                  TStreamerInfo {
                      tstreamerinfo_ver,
                      named,
                      checksum,
                      new_class_version,
                      data_members,
                  }})
    )
}


impl ToRustParser for TStreamerInfo {
    /// Generate a parser that can parse an an object described by this TStreamer
    fn to_inline_parser(&self) -> Tokens {
        if type_is_core(self.named.name.as_str()) {
            // Don't generate a parser if its a core type
            return quote!(#(self.named.name.to_lowercase()));
        }
        let struct_name = Ident::new(self.named.name.as_str());
        let member_names: &Vec<Ident> = &self.data_members.iter()
            .map(|m| m.member_name())
            .collect();
        let member_parsers: &Vec<Tokens> = &self.data_members.iter()
            .map(|m| m.to_inline_parser())
            .collect();
        quote!{
            do_parse!(ver: be_u16 >>
                      #(#member_names : #member_parsers >> )*
                      ({let phantom = PhantomData;
                        #struct_name {
                            phantom,
                            ver,
                            #(#member_names),*
                        }})
            )}
    }
}

impl ToNamedRustParser for TStreamerInfo {
    fn parser_name(&self) -> Tokens {
        let ret = Ident::new(self.named.name.to_lowercase());
        quote!(#ret)
    }

    fn to_named_parser(&self) -> Tokens {
        if type_is_core(self.named.name.as_str()) {
            // Don't generate a parser if its a core type
            return quote!{};
        }
        let parser_name = self.parser_name();
        let parser = self.to_inline_parser();
        let struct_name = self.type_name();
        quote!{
            pub fn #parser_name<'s>(input: &'s[u8], context: &'s Context<'s>)
                                    -> IResult<&'s[u8], #struct_name<'s>> {
                value!(input, #parser)
            }
        }
    }
}

impl ToRustStruct for TStreamerInfo {
    /// Generate a struct corresponding to this TStreamerInfo
    fn to_struct(&self) -> Tokens {
        if type_is_core(self.named.name.as_str()) {
            return quote!{};
        }
        let name = self.type_name();
        let fields = &self.data_members;
        let ver_comment = self.type_doc();
        quote! {
            #[derive(Debug)]
            pub struct #name<'s> {
                /// Gurantee that this object does not outlive its underlying slice
                phantom: PhantomData<&'s[u8]>,
                #ver_comment
                ver: u16,
                #(#fields), *
            }
        }
    }
}

impl ToRustType for TStreamerInfo {
    fn type_doc(&self) -> Tokens {
        let ret = Ident::new("\n/// Version of the read layout\n");
        quote!(#ret)
    }
    fn type_name(&self) -> Tokens {
        let ret = Ident::new(self.named.name.as_str());
        quote!(#ret)
    }
}

impl TStreamerInfo {
    pub(crate) fn to_yaml(&self) -> String {
        if type_is_core(self.named.name.as_str()) {
            return "".to_string();
        };
        let mut s = "".to_string();
        s += format!("{}:\n", self.named.name).as_str();
        s += format!("  version: {}\n", self.new_class_version).as_str();
        s += "  members:\n";
        for obj in &self.data_members {
            s += format!("      # {}\n", obj.member_comment()).as_str();
            s += format!("      {}: {}\n",
                         obj.member_name().to_string(),
                         obj.type_name().to_string()).as_str();
        }
        s += "\n";
        s
    }
}
