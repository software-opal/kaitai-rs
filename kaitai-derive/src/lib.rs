#![recursion_limit = "128"]

use proc_macro::TokenStream;
use syn::parse_macro_input;

#[proc_macro]
pub fn include_ksy(input: TokenStream) -> TokenStream {
    TokenStream::from(kaitai_codegen::include_ksy(parse_macro_input!(
        input as kaitai_codegen::macro_entrypoint::IncludeKsyConfig
    )))
}
#[proc_macro]
pub fn codegen_ksy(input: TokenStream) -> TokenStream {
    TokenStream::from(kaitai_codegen::codegen_ksy(parse_macro_input!(
        input as kaitai_codegen::macro_entrypoint::CodegenKsyConfig
    )))
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
