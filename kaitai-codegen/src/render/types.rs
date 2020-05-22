use convert_case::{Case, Casing};
use kaitai_loader::raw::attrs::{Attribute, AttributeType};
use kaitai_loader::raw::enums::EnumSpec;
use kaitai_loader::raw::enums::EnumValueSpec;
use kaitai_loader::raw::types::TypeSpec;
use kaitai_loader::resolver::ResolvedKsySpec;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use quote::ToTokens;
use std::collections::BTreeMap;
use std::str::FromStr;

pub fn attribute_enum_name(type_name: &str, attr_name: &str) -> String {
    type_name.to_owned() + &attr_name.to_case(Case::UpperCamel)
}

pub fn render_type(name: String, spec: &TypeSpec, resolved: &ResolvedKsySpec) -> TokenStream {
    quote! {}
}

pub fn attribute_enum(type_name: &str, attr_name: &str, attr: &Attribute) -> TokenStream {
    match &attr.type_ {
        Some(AttributeType::Switch(switch)) => {
            let enum_name = Ident::new(
                &attribute_enum_name(type_name, attr_name),
                Span::call_site(),
            );

            quote! {
                pub enum #enum_name {

                }
            }
        }
        _ => TokenStream::new(),
    }
}
