/// Types to map out the inter-dependences of the streamed objects
use quote::Tokens;

pub(crate) trait ToRustType {
    fn type_doc(&self) -> Tokens {quote!()}
    fn type_name(&self) -> Tokens;
}

pub(crate) trait ToRustParser: ToRustType {
    /// The definition of the parser parsing this thing such that it can be used in-line
    fn to_inline_parser(&self) -> Tokens {quote!{#(self.parser_name())}}
}

pub(crate) trait ToNamedRustParser: ToRustParser {
    /// The name of the parser of this thing
    fn parser_name(&self) -> Tokens;
    
    /// The definition of the parser parsing this thing; May be blank if it is build-in
    fn to_named_parser(&self) -> Tokens;
}

pub(crate) trait ToRustStruct: ToRustType {
    fn to_struct(&self) -> Tokens;
}
