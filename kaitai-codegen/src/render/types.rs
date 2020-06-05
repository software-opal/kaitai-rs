use crate::utils;
use kaitai_loader::{
    raw::{attrs::Attribute, types::TypeSpec},
    resolver::{
        attrs::{
            get_attr_type, BaseAttributeType, FloatAttributeType, IntegerAttributeType,
            ResolvedAttributeType,
        },
        ResolvedKsySpec,
    },
};
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::BTreeMap;
use heck::CamelCase;

pub fn attribute_enum_name(type_name: &str, attr_name: &str) -> String {
    type_name.to_owned() + &attr_name.to_camel_case()
}

const DEFAULT_ATTRIBUTE_NAME: &str = "unknown";

pub fn int_type_to_tokens(int: &IntegerAttributeType) -> TokenStream {
    match int {
        IntegerAttributeType::U8 => quote! {u8},
        IntegerAttributeType::U16 => quote! {u16},
        IntegerAttributeType::U32 => quote! {u32},
        IntegerAttributeType::U64 => quote! {u64},
        IntegerAttributeType::I8 => quote! {i8},
        IntegerAttributeType::I16 => quote! {i16},
        IntegerAttributeType::I32 => quote! {i32},
        IntegerAttributeType::I64 => quote! {i64},
    }
}

pub fn base_type_to_tokens(
    type_name: &str,
    attr_name: &str,
    attr_type: &BaseAttributeType,
    resolved: &ResolvedKsySpec,
) -> Result<TokenStream, String> {
    match attr_type {
        BaseAttributeType::Integer(int) => Ok(int_type_to_tokens(int)),
        BaseAttributeType::Float(float) => match float {
            FloatAttributeType::F32 => Ok(quote! {f32}),
            FloatAttributeType::F64 => Ok(quote! {f64}),
        },
        BaseAttributeType::String => Ok(quote! {String}),
        BaseAttributeType::Switch(_) => {
            let ident = utils::ident(&attribute_enum_name(type_name, attr_name));
            Ok(quote! {#ident})
        }
        BaseAttributeType::Enum(_, name) => resolved
            .find_enum_named(name)
            .ok_or_else(|| format!("Unable to find type for enum: {:?}", name))
            .map(|(name, _, _)| utils::ident(&name))
            .map(|ident| quote! {#ident}),
        BaseAttributeType::User(name) => resolved
            .find_type_named(name)
            .ok_or_else(|| format!("Unable to find type for user type: {:?}", name))
            .map(|(name, _, _)| utils::ident(&name))
            .map(|ident| quote! {#ident}),
    }
}

pub fn resolved_type_to_tokens(
    type_name: &str,
    attr_name: &str,
    attr_type: &ResolvedAttributeType,
    resolved: &ResolvedKsySpec,
) -> Result<TokenStream, String> {
    match attr_type {
        ResolvedAttributeType::Base(atype) => {
            base_type_to_tokens(type_name, attr_name, atype, resolved)
        }
        ResolvedAttributeType::Repeat(_, rtype) => {
            let inner = base_type_to_tokens(type_name, attr_name, rtype, resolved)?;
            Ok(quote! {Vec<#inner>})
        }
        ResolvedAttributeType::Optional(otype) => {
            let inner = base_type_to_tokens(type_name, attr_name, otype, resolved)?;
            Ok(quote! {Option<#inner>})
        }
    }
}

pub fn resolve_attr_names(
    attributes: &[Attribute],
) -> Result<Vec<(String, &Attribute, ResolvedAttributeType)>, String> {
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
        .map(|(name, attr)| Ok((name, attr, get_attr_type(attr)?)))
        .collect()
}

// pub fn resolve_attr_type(    type_name: &str,attr_name: &str, attr: &Attribute, resolved: &ResolvedKsySpec) {
//     let base_type = match attr.type_ {
//         None => "u8",
//         Some(AttributeType::String(typ)) => resolved.resolve_type(typ).unwrap_or("u8"),
//         Some(AttributeType::Switch(_)) =>
//            attribute_enum_name(type_name, attr_name)
//         ,
//     }
// }

pub fn render_type(name: String, spec: &TypeSpec, resolved: &ResolvedKsySpec) -> TokenStream {
    let struct_name = utils::ident(&name);
    let attr_names = resolve_attr_names(&spec.seq).unwrap();

    let attr_enums = attr_names
        .iter()
        .map(|(attr_name, _attr, attr_type)| attribute_enum(&name, attr_name, attr_type, resolved))
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    let struct_items = attr_names
        .iter()
        .map(|(attr_name, _attr, attr_type)| {
            let type_name = resolved_type_to_tokens(&name, attr_name, attr_type, resolved)?;
            let attr_name = utils::ident(attr_name);
            Ok(quote! {
                pub #attr_name: #type_name,
            })
        })
        .collect::<Result<Vec<_>, String>>()
        .unwrap();

    // for (_attr_name, _attr) in attr_names {}

    quote! {
        #[derive(Clone, Debug, PartialEq, PartialOrd)]
        pub struct #struct_name {
            #(#struct_items)*
        }
        #(#attr_enums)*
    }
}

pub fn attribute_enum(
    type_name: &str,
    attr_name: &str,
    attr_type: &ResolvedAttributeType,
    resolved: &ResolvedKsySpec,
) -> Result<TokenStream, String> {
    match attr_type.switch_type() {
        Some(switch_arms) => {
            let enum_name = utils::ident(&attribute_enum_name(type_name, attr_name));
            let variants = switch_arms
                .iter()
                .map(|(case, resolved_type)| {
                    let case_name =
                        utils::ident(case.trim_end_matches('"').trim_start_matches('"'));
                    let res_type_name =
                        base_type_to_tokens(type_name, attr_name, resolved_type, resolved)
                            .map_err(|err| {
                                format!(
                                    "Unable to determine type of switch arm case {:?}: {}",
                                    case, err
                                )
                            })?;
                    return Ok(quote! {
                        #case_name(#res_type_name)
                    });
                })
                .collect::<Result<Vec<_>, String>>()?;
            // Cannot derive Eq and Ord as floats don't implement them.
            Ok(quote! {
                #[derive(Clone, Debug, PartialEq, PartialOrd)]
                pub enum #enum_name {
                    #(#variants),*
                }
            })
        }
        None => Ok(TokenStream::new()),
    }
}
