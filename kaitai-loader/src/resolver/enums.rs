use crate::{
    raw::{enums::EnumSpec, root::KsySpec, types::TypeSpec},
    resolver::utils::prefix_map,
};
use std::collections::BTreeMap;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum EnumPathSegment<'a> {
    Type(&'a str),
    Enum(&'a str),
}

pub fn extract_enums<'a>(spec: &'a KsySpec) -> BTreeMap<Vec<EnumPathSegment<'a>>, &'a EnumSpec> {
    let mut map = BTreeMap::new();
    for (key, value) in spec.types.iter() {
        map.append(&mut prefix_map(
            EnumPathSegment::Type(key),
            extract_type_enums(value),
        ));
    }
    for (key, value) in spec.enums.iter() {
        map.insert(vec![EnumPathSegment::Enum(key)], value);
    }
    map
}

pub fn extract_type_enums<'a>(
    type_spec: &'a TypeSpec,
) -> BTreeMap<Vec<EnumPathSegment<'a>>, &'a EnumSpec> {
    let mut map = BTreeMap::new();
    for (key, value) in type_spec.types.iter() {
        map.append(&mut prefix_map(
            EnumPathSegment::Type(key),
            extract_type_enums(value),
        ));
    }
    for (key, value) in type_spec.enums.iter() {
        map.insert(vec![EnumPathSegment::Enum(key)], value);
    }
    map
}
