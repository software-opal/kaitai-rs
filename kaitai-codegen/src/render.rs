use kaitai_loader::raw::root::KsySpec;
use proc_macro2::TokenStream;

pub mod enums;
pub mod types;

#[derive(Default, Debug)]
pub struct Config {}

pub fn render_spec(spec: KsySpec, _config: Config) -> TokenStream {
    let resolved = kaitai_loader::resolver::ResolvedKsySpec::new(&spec);
    let enums = resolved
        .all_enums()
        .into_iter()
        .map(|(key, enum_spec)| enums::render_enum(key, enum_spec));

    let types = resolved
        .all_types()
        .into_iter()
        .map(|(key, type_spec)| types::render_type(key, type_spec, &resolved));

    quote::quote! {
        #( #enums )*
        #( #types )*
    }
}
