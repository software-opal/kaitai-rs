use std::collections::BTreeMap;

use convert_case::{Case, Casing};

use crate::raw::enums::EnumSpec;
use crate::raw::root::KsySpec;
use crate::raw::types::TypeSpec;
use crate::resolver::enums::EnumPathSegment;

pub mod enums;
pub mod switch;
pub mod types;
pub mod utils;

pub struct ResolvedKsySpec<'a> {
    pub spec: &'a KsySpec,
    pub enums: BTreeMap<Vec<EnumPathSegment<'a>>, &'a EnumSpec>,
    pub enum_name_mapping: BTreeMap<Vec<EnumPathSegment<'a>>, String>,
    pub types: BTreeMap<Vec<&'a str>, &'a TypeSpec>,
    pub type_name_mapping: BTreeMap<Vec<&'a str>, String>,
}

pub fn default_enum_name_mapper(segments: &Vec<EnumPathSegment<'_>>) -> String {
    segments.iter().fold("".to_owned(), |acc, item| {
        acc + &(match item {
            EnumPathSegment::Type(v) => v,
            EnumPathSegment::Enum(v) => v,
        })
        .to_case(Case::UpperCamel)
    })
}
pub fn default_type_name_mapper(segments: &Vec<&str>) -> String {
    segments.iter().fold("".to_owned(), |acc, item| {
        acc + &item.to_case(Case::UpperCamel)
    })
}

impl<'a> ResolvedKsySpec<'a> {
    pub fn new(spec: &'a KsySpec) -> Self {
        Self::new_with_converter(spec, default_enum_name_mapper, default_type_name_mapper)
    }
    pub fn new_with_converter<EF, TF>(
        spec: &'a KsySpec,
        enum_name_mapper: EF,
        type_name_mapper: TF,
    ) -> Self
    where
        EF: Fn(&Vec<EnumPathSegment<'_>>) -> String,
        TF: Fn(&Vec<&str>) -> String,
    {
        let mut new = Self {
            spec,
            enums: enums::extract_enums(spec),
            enum_name_mapping: BTreeMap::new(),
            types: types::extract_types(spec),
            type_name_mapping: BTreeMap::new(),
        };
        new.update_enum_mapping(enum_name_mapper);
        new.update_type_mapping(type_name_mapper);
        new
    }

    pub fn update_enum_mapping<EF>(&mut self, enum_name_mapper: EF)
    where
        EF: Fn(&Vec<EnumPathSegment<'_>>) -> String,
    {
        self.enum_name_mapping = self
            .enums
            .keys()
            .map(|key| (key.clone(), enum_name_mapper(key)))
            .collect();
    }

    pub fn update_type_mapping<TF>(&mut self, type_name_mapper: TF)
    where
        TF: Fn(&Vec<&str>) -> String,
    {
        self.type_name_mapping = self
            .types
            .keys()
            .map(|key| (key.clone(), type_name_mapper(key)))
            .collect();
    }

    pub fn all_enums(&self) -> BTreeMap<String, &'a EnumSpec> {
        self.enums
            .iter()
            .map(|(key, &value)| (self.enum_name_mapping[key].clone(), value))
            .collect()
    }

    pub fn all_types(&self) -> BTreeMap<String, &'a TypeSpec> {
        self.types
            .iter()
            .map(|(key, &value)| (self.type_name_mapping[key].clone(), value))
            .collect()
    }

    pub fn resolve_type(&self, type_name: &str) -> Option<String> {
        if type_name.len() == 2 || type_name.len() == 4 {
            // A signed or unsigned integer
            // Note kaitai uses u1 to refer to an 1-byte wide integer; but rust uses u8 to refer to the same size
            match &type_name[..2] {
                "u1" => return Some("u8".to_owned()),
                "u2" => return Some("u16".to_owned()),
                "u4" => return Some("u32".to_owned()),
                "u8" => return Some("u64".to_owned()),
                "s1" => return Some("i8".to_owned()),
                "s2" => return Some("i16".to_owned()),
                "s4" => return Some("i32".to_owned()),
                "s8" => return Some("i64".to_owned()),
                "f4" => return Some("f32".to_owned()),
                "f8" => return Some("f64".to_owned()),
                _ => {}
            }
        }
        if type_name == "str" || type_name == "strz" {
            Some("String".to_owned())
        } else {
            self.find_type_named(type_name).map(|(name, _, _)| name)
        }
    }

    pub fn find_type_named(&'a self, type_name: &str) -> Option<(String, &[&'a str], &TypeSpec)> {
        self.types
            .iter()
            .filter_map(|(key, &value)| {
                if key.last() == Some(&type_name) {
                    Some((self.type_name_mapping[key].clone(), &key[..], value))
                } else {
                    None
                }
            })
            .next()
    }
}
