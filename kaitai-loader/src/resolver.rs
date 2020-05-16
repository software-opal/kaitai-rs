use std::collections::BTreeMap;

use convert_case::{Case, Casing};

use crate::raw::enums::EnumSpec;
use crate::raw::root::KsySpec;
use crate::raw::types::TypeSpec;
use crate::resolver::enums::EnumPathSegment;

pub mod enums;
pub mod types;
pub mod utils;

pub struct ResolvedKsySpec<'a> {
    pub spec: &'a KsySpec,
    pub enums: BTreeMap<Vec<EnumPathSegment<'a>>, &'a EnumSpec>,
    pub enum_name_mapping: BTreeMap<Vec<EnumPathSegment<'a>>, String>,
    pub types: BTreeMap<Vec<&'a String>, &'a TypeSpec>,
    pub type_name_mapping: BTreeMap<Vec<&'a String>, String>,
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
pub fn default_type_name_mapper(segments: &Vec<&String>) -> String {
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
        TF: Fn(&Vec<&String>) -> String,
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
        TF: Fn(&Vec<&String>) -> String,
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
}