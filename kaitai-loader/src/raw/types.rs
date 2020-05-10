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
    pub doc: Option<Doc>,
    #[serde(rename = "doc-ref")]
    pub doc_ref: Option<DocRef>,
    pub enums: EnumsSpec,
    pub instances: Option<InstancesSpec>,
    pub meta: Option<TypeMetaSpec>,
    pub params: ParamsSpec,
    pub seq: Option<Attributes>,
    pub types: TypesSpec,
}

pub type InstancesSpec = BTreeMap<String, Attribute>;
pub type TypesSpec = BTreeMap<String, TypeSpec>;
