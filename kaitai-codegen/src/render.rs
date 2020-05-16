use kaitai_loader::raw::root::KsySpec;
use proc_macro2::TokenStream;

pub mod enums;

#[derive(Default, Debug)]
pub struct Config {}

pub fn render_spec(spec: KsySpec, _config: Config) -> TokenStream {
    let resolved = kaitai_loader::resolver::ResolvedKsySpec::new(&spec);
    let enums = resolved
        .all_enums()
        .into_iter()
        .map(|(key, enum_spec)| enums::render_enum(key, enum_spec));

    quote::quote! {
        #( #enums )*
    }
}
