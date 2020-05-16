use crate::raw::attrs::Attribute;
use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::raw::{
    attrs::Attributes,
    base::{Doc, DocRef},
    enums::EnumsSpec,
    meta::TypeMetaSpec,
    params::ParamsSpec,
};

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct TypeSpec {
    #[serde(default)]
    pub doc: Option<Doc>,
    #[serde(rename = "doc-ref")]
    pub doc_ref: Option<DocRef>,
    #[serde(default)]
    pub enums: EnumsSpec,
    #[serde(default)]
    pub instances: InstancesSpec,
    #[serde(default)]
    pub meta: Option<TypeMetaSpec>,
    #[serde(default)]
    pub params: ParamsSpec,
    #[serde(default)]
    pub seq: Attributes,
    #[serde(default)]
    pub types: TypesSpec,
}

pub type InstancesSpec = BTreeMap<String, Attribute>;
pub type TypesSpec = BTreeMap<String, TypeSpec>;
