#![recursion_limit = "128"]

use proc_macro::TokenStream;

#[proc_macro]
pub fn include_ksy(input: TokenStream) -> TokenStream {
    kaitai_codegen::include_ksy(input)
}
// #[proc_macro]
// pub fn codegen_ksy(input: TokenStream) -> TokenStream {
//     kaitai_codegen::codegen_ksy(input)
// }

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
