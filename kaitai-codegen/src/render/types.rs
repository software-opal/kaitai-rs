use convert_case::{Case, Casing};
use kaitai_loader::raw::attrs::{Attribute, AttributeType, Attributes};
use kaitai_loader::raw::base::AnyNonNullScalar;
use kaitai_loader::raw::types::TypeSpec;
use kaitai_loader::resolver::ResolvedKsySpec;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use std::collections::BTreeMap;

pub fn attribute_enum_name(type_name: &str, attr_name: &str) -> String {
    type_name.to_owned() + &attr_name.to_case(Case::UpperCamel)
}

const DEFAULT_ATTRIBUTE_NAME: &str = "unknown";

pub fn resolve_attr_names(attributes: &Attributes) -> Vec<(String, &Attribute)> {
    let mut attribute_nums: BTreeMap<&str, usize> = BTreeMap::new();
    let names: Vec<(_, usize, &Attribute)> = attributes
        .iter()
        .map(|attr| {
            let name = match &attr.id {
                Some(string) => &string,
                None => DEFAULT_ATTRIBUTE_NAME,
            };
            let name_idx = attribute_nums
                .entry(name)
                .and_modify(|v| *v += 1)
                .or_insert(1);
            (name, *name_idx, attr)
        })
        .collect();

    names
        .into_iter()
        .map(|(name, idx, attr)| {
            if attribute_nums[&name[..]] == 1 {
                (name.to_owned(), attr)
            } else {
                (format!("{}{}", name, idx), attr)
            }
        })
        .collect()
}

pub fn render_type(name: String, spec: &TypeSpec, resolved: &ResolvedKsySpec) -> TokenStream {
    let attr_names = resolve_attr_names(&spec.seq);

    let attr_enums = attr_names
        .iter()
        .map(|(attr_name, attr)| attribute_enum(&name, attr_name, attr, resolved))
        .collect::<Vec<_>>();
    for (_attr_name, _attr) in attr_names {}

    quote! {
        #(#attr_enums)*
    }
}

pub fn attribute_enum(
    type_name: &str,
    attr_name: &str,
    attr: &Attribute,
    resolved: &ResolvedKsySpec,
) -> TokenStream {
    match &attr.type_ {
        Some(AttributeType::Switch(switch)) => {
            let enum_name = Ident::new(
                &attribute_enum_name(type_name, attr_name),
                Span::call_site(),
            );

            let variants = switch
                .cases
                .iter()
                .filter_map(|(case, type_name)| match type_name {
                    Some(AnyNonNullScalar::String(string)) => resolved
                        .resolve_type(string)
                        .map(|resolved| (case, resolved)),
                    _ => None,
                })
                .map(|(case, resolved_type)| {
                    let case_name = Ident::new(
                        case.trim_end_matches('"').trim_start_matches('"'),
                        Span::call_site(),
                    );
                    let res_type_name = Ident::new(&resolved_type, Span::call_site());
                    return quote! {
                        #case_name(#res_type_name)
                    };
                });

            // Cannot derive Eq and Ord as floats don't implement them.
            quote! {
                #[derive(Debug, PartialEq, PartialOrd)]
                pub enum #enum_name {
                    #(#variants),*
                }
            }
        }
        _ => TokenStream::new(),
    }
}
