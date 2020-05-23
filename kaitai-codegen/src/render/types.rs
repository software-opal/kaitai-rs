use convert_case::{Case, Casing};
use kaitai_loader::raw::attrs::Attribute;
use kaitai_loader::raw::types::TypeSpec;
use kaitai_loader::resolver::{
    attrs::{get_attr_type, BaseAttributeType, ResolvedAttributeType},
    ResolvedKsySpec,
};
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use std::collections::BTreeMap;

pub fn attribute_enum_name(type_name: &str, attr_name: &str) -> String {
    type_name.to_owned() + &attr_name.to_case(Case::UpperCamel)
}

const DEFAULT_ATTRIBUTE_NAME: &str = "unknown";

pub fn base_type_to_tokens(
    type_name: &str, attr_name: &str,
    attr_type: &BaseAttributeType,
    resolved: &ResolvedKsySpec,
) -> Result<TokenStream, String> {
    match attr_type {
        BaseAttributeType::Integer(int) => match int {
            kaitai_loader::resolver::attrs::IntegerAttributeType::U8 => Ok(quote! {u8}),
            kaitai_loader::resolver::attrs::IntegerAttributeType::U16 => Ok(quote! {u16}),
            kaitai_loader::resolver::attrs::IntegerAttributeType::U32 => Ok(quote! {u32}),
            kaitai_loader::resolver::attrs::IntegerAttributeType::U64 => Ok(quote! {u64}),
            kaitai_loader::resolver::attrs::IntegerAttributeType::I8 => Ok(quote! {i8}),
            kaitai_loader::resolver::attrs::IntegerAttributeType::I16 => Ok(quote! {i16}),
            kaitai_loader::resolver::attrs::IntegerAttributeType::I32 => Ok(quote! {i32}),
            kaitai_loader::resolver::attrs::IntegerAttributeType::I64 => Ok(quote! {i64}),
        },
        BaseAttributeType::Float(float) => match float {
            kaitai_loader::resolver::attrs::FloatAttributeType::F32 => Ok(quote! {f32}),
            kaitai_loader::resolver::attrs::FloatAttributeType::F64 => Ok(quote! {f64}),
        },
        BaseAttributeType::String { .. } => Ok(quote! {String}),
        BaseAttributeType::Stringz { .. } => Ok(quote! {String}),
        BaseAttributeType::Switch(_) => {
            let ident = Ident::new(&attribute_enum_name(type_name, attr_name), Span::call_site());
          Ok(quote! {#ident} )},
        BaseAttributeType::Enum(_, name) => resolved.find_enum_named(name).ok_or_else(|| format!("Unable to find type for enum: {:?}", name))
        .map(|(name, _, _)| Ident::new(&name, Span::call_site()))
        .map(|ident| quote! {#ident}),
        BaseAttributeType::User(name) => resolved
            .find_type_named(name)
            .ok_or_else(|| format!("Unable to find type for user type: {:?}", name))
            .map(|(name, _, _)| Ident::new(&name, Span::call_site()))
            .map(|ident| quote! {#ident}),
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
    let attr_names = resolve_attr_names(&spec.seq).unwrap();

    let attr_enums = attr_names
        .iter()
        .map(|(attr_name, _attr, attr_type)| attribute_enum(&name, attr_name, attr_type, resolved))
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    let struct_items = attr_names.iter().map(|(attr_name, attr| {

    })
    // for (_attr_name, _attr) in attr_names {}

    quote! {
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
            let enum_name = Ident::new(
                &attribute_enum_name(type_name, attr_name),
                Span::call_site(),
            );
            let variants = switch_arms.iter().map(|(case, resolved_type)| {
                let case_name = Ident::new(
                    case.trim_end_matches('"').trim_start_matches('"'),
                    Span::call_site(),
                );
                let res_type_name = base_type_to_tokens(resolved_type, resolved).ok_or_else(|| format!("Unable to determine type of switch arm, possibly an unrecognised user defined type. Got: {:?}", resolved_type))?;
                return Ok(quote! {
                    #case_name(#res_type_name)
                });
            }).collect::<Result<Vec<_>, String>>()?;
            // Cannot derive Eq and Ord as floats don't implement them.
            Ok(quote! {
                #[derive(Debug, PartialEq, PartialOrd)]
                pub enum #enum_name {
                    #(#variants),*
                }
            })
        }
        None => Ok(TokenStream::new()),
    }
}
