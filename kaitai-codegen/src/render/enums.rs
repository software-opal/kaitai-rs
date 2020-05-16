use convert_case::{Case, Casing};
use kaitai_loader::raw::enums::EnumSpec;
use kaitai_loader::raw::enums::EnumValueSpec;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use quote::ToTokens;
use std::collections::BTreeMap;
use std::str::FromStr;

pub fn map_enum_name(_key: &str, spec: &EnumValueSpec) -> String {
    spec.id.to_case(Case::UpperCamel)
}
pub fn enum_ident(name: &str) -> String {
    name.to_case(Case::UpperCamel)
}
pub fn string_to_ident(name: &str) -> Ident {
    Ident::new(name, Span::call_site())
}

pub type NamedEnumSpec<'a> = BTreeMap<&'a String, (String, &'a EnumValueSpec)>;
pub type IdentEnumSpec<'a> = BTreeMap<&'a String, (Ident, &'a EnumValueSpec)>;

pub fn named_enum_spec(enum_spec: &EnumSpec) -> NamedEnumSpec<'_> {
    enum_spec
        .iter()
        .map(|(key, value)| (key, (map_enum_name(key, value), value)))
        .collect::<BTreeMap<_, _>>()
}
pub fn ident_enum_spec<'a>(enum_name_and_items: &'a NamedEnumSpec<'a>) -> IdentEnumSpec<'a> {
    enum_name_and_items
        .iter()
        .map(|(&key, (name, value))| (key, (string_to_ident(&name), *value)))
        .collect()
}
pub fn render_enum(name: String, enum_spec: &EnumSpec) -> proc_macro2::TokenStream {
    let name_ident = string_to_ident(&enum_ident(&name));
    eprintln!("{:#?} {:#?}", name, name_ident);
    let enum_name_and_items = named_enum_spec(enum_spec);
    let enum_ident_and_items = ident_enum_spec(&enum_name_and_items);

    // let enum_ = enum_spec.iter().map(|(key, value)| {
    //     value.id
    // })
    let enum_block = render_enum_block(&name_ident, &enum_ident_and_items);

    let try_from = [
        render_try_from_block(&name_ident, &enum_ident_and_items, quote! { &str }, |s| {
            Some(s.to_owned())
        }),
        render_try_from_block(
            &name_ident,
            &enum_ident_and_items,
            quote! { u8 },
            |s: &str| u8::from_str(s).ok(),
        ),
        render_try_from_block(
            &name_ident,
            &enum_ident_and_items,
            quote! { i8 },
            |s: &str| i8::from_str(s).ok(),
        ),
    ];

    return quote! {
        #enum_block
        #( #try_from )*
    };
}

fn render_enum_block(name: &Ident, enum_ident_and_items: &IdentEnumSpec) -> TokenStream {
    let mut enums_by_ident = BTreeMap::new();
    let mut enum_ident_unique = Vec::with_capacity(enum_ident_and_items.len());
    for (name, value) in enum_ident_and_items.values() {
        enums_by_ident
            .entry(name)
            .or_insert_with(|| {
                enum_ident_unique.push(name);
                vec![]
            })
            .push(value);
    }

    let inner_tokens: TokenStream = enum_ident_unique
        .into_iter()
        .map(|name| {
            // let enum_specs = enums_by_ident[name];
            quote! {
                #name,
            }
        })
        .collect();

    return quote! {
        pub enum #name {
            #inner_tokens
        }
    };
}

fn render_try_from_block<F, TT>(
    name: &Ident,
    enum_ident_and_items: &IdentEnumSpec,
    into_type: TokenStream,
    conversion: F,
) -> TokenStream
where
    F: Fn(&str) -> Option<TT>,
    TT: ToTokens,
{
    let mut try_from_case_body = TokenStream::new();
    for (key, (ident, _)) in enum_ident_and_items {
        if let Some(case_expect) = conversion(key) {
            try_from_case_body.extend(quote! {
                #case_expect => Ok(#name::#ident),
            })
        }
    }

    if try_from_case_body.is_empty() {
        return try_from_case_body;
    }

    let error_format =
        format!("Unable to convert value from {} into {}: ", into_type, name) + "{:?}";

    return quote! {
        impl std::convert::TryFrom<#into_type> for #name {
            type Error = String;

            fn try_from(value: #into_type) -> Result<Self, Self::Error> {
                match value {
                    #try_from_case_body
                    other => Err(format!(#error_format, other))
                }
            }
        }
    };
}

#[cfg(test)]
mod test_super {
    use super::*;

    fn sample_enum() -> EnumSpec {
        vec![
            (
                "1".to_owned(),
                EnumValueSpec {
                    id: "test1".to_owned(),
                    ..EnumValueSpec::default()
                },
            ),
            (
                "2".to_owned(),
                EnumValueSpec {
                    id: "test2".to_owned(),
                    ..EnumValueSpec::default()
                },
            ),
            (
                "foo".to_owned(),
                EnumValueSpec {
                    id: "test2".to_owned(),
                    ..EnumValueSpec::default()
                },
            ),
        ]
        .into_iter()
        .collect()
    }

    #[test]
    fn test_named_enum_spec() {
        assert_eq!(
            named_enum_spec(&sample_enum()),
            vec![
                (
                    &"1".to_owned(),
                    (
                        "Test1".to_owned(),
                        &EnumValueSpec {
                            id: "test1".to_owned(),
                            ..EnumValueSpec::default()
                        },
                    )
                ),
                (
                    &"2".to_owned(),
                    (
                        "Test2".to_owned(),
                        &EnumValueSpec {
                            id: "test2".to_owned(),
                            ..EnumValueSpec::default()
                        },
                    )
                ),
                (
                    &"foo".to_owned(),
                    (
                        "Test2".to_owned(),
                        &EnumValueSpec {
                            id: "test2".to_owned(),
                            ..EnumValueSpec::default()
                        },
                    )
                ),
            ]
            .into_iter()
            .collect()
        );
    }
    #[test]
    fn test_ident_enum_spec() {
        assert_eq!(
            ident_enum_spec(&named_enum_spec(&sample_enum())),
            vec![
                (
                    &"1".to_owned(),
                    (
                        Ident::new(&"Test1", Span::call_site()),
                        &EnumValueSpec {
                            id: "test1".to_owned(),
                            ..EnumValueSpec::default()
                        },
                    )
                ),
                (
                    &"2".to_owned(),
                    (
                        Ident::new(&"Test2", Span::call_site()),
                        &EnumValueSpec {
                            id: "test2".to_owned(),
                            ..EnumValueSpec::default()
                        },
                    )
                ),
                (
                    &"foo".to_owned(),
                    (
                        Ident::new(&"Test2", Span::call_site()),
                        &EnumValueSpec {
                            id: "test2".to_owned(),
                            ..EnumValueSpec::default()
                        },
                    )
                ),
            ]
            .into_iter()
            .collect()
        );
    }

    #[test]
    fn test_render_enum() {
        let render = render_enum_block(
            &Ident::new("Test", Span::call_site()),
            &ident_enum_spec(&named_enum_spec(&sample_enum())),
        );
        let expected = quote! {
          pub  enum Test {
                Test1,
                Test2,
            }
        };
        assert_eq!(render.to_string(), expected.to_string());
    }
    #[test]
    fn test_render_enum_try_from_str() {
        let render = render_try_from_block(
            &Ident::new("Test", Span::call_site()),
            &ident_enum_spec(&named_enum_spec(&sample_enum())),
            quote! { &str },
            |s| Some(s.to_owned()),
        );
        let expected = quote! {
            impl std::convert::TryFrom<&str> for Test {
                type Error = String;

                fn try_from(value: &str) -> Result<Self, Self::Error> {
                    match value {
                        "1" => Ok(Test::Test1),
                        "2" => Ok(Test::Test2),
                        "foo" => Ok(Test::Test2),
                        other => Err(format!(
                            "Unable to convert value from & str into Test: {:?}",
                            other
                        ))
                    }
                }
            }
        };
        assert_eq!(render.to_string(), expected.to_string());
    }
    #[test]
    fn test_render_enum_try_from_u8() {
        let render = render_try_from_block(
            &Ident::new("Test", Span::call_site()),
            &ident_enum_spec(&named_enum_spec(&sample_enum())),
            quote! { u8 },
            |s: &str| u8::from_str(s).ok(),
        );
        let expected = quote! {
            impl std::convert::TryFrom<u8> for Test {
                type Error = String;

                fn try_from(value: u8) -> Result<Self, Self::Error> {
                    match value {
                        1u8 => Ok(Test::Test1),
                        2u8 => Ok(Test::Test2),
                        other => Err(format!(
                            "Unable to convert value from u8 into Test: {:?}",
                            other
                        ))
                    }
                }
            }
        };
        assert_eq!(render.to_string(), expected.to_string());
    }
}
