use std::collections::BTreeMap;

use crate::{
    raw::{root::KsySpec, types::TypeSpec},
    resolver::utils::prefix_map,
};

pub fn extract_types<'a>(spec: &'a KsySpec) -> BTreeMap<Vec<&'a str>, &'a TypeSpec> {
    let mut map = BTreeMap::new();
    for (key, value) in spec.types.iter() {
        map.append(&mut prefix_map(&key[..], extract_type_types(value)));
    }
    map
}

pub fn extract_type_types<'a>(type_spec: &'a TypeSpec) -> BTreeMap<Vec<&'a str>, &'a TypeSpec> {
    let mut map = BTreeMap::new();
    map.insert(vec![], type_spec);
    for (key, value) in type_spec.types.iter() {
        map.append(&mut prefix_map(&key[..], extract_type_types(value)));
    }
    map
}
