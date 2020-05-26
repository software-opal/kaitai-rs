use serde::{Deserialize, Serialize};

use crate::raw::{
    base::{Doc, DocRef, Identifier, Switch},
    serde_util::{optional_primitive_as_string, primitive_as_string},
};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum ContentsType {
    String(String),
    Array(Vec<StringOrByte>),
}
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum StringOrByte {
    String(String),
    Integer(u8),
}
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum AttributeType {
    String(String),
    Switch(Switch),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[serde(tag = "repeat", rename_all = "lowercase")]
pub enum AttributeRepeat {
    EOS,
    Expr {
        #[serde(rename = "repeat-expr", deserialize_with = "primitive_as_string")]
        expression: String,
    },
    Until {
        #[serde(rename = "repeat-expr", deserialize_with = "primitive_as_string")]
        expression: String,
    },
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Attribute {
    pub id: Option<Identifier>,
    #[serde(default)]
    pub doc: Option<Doc>,
    #[serde(default, rename = "doc-ref")]
    pub doc_ref: Option<DocRef>,
    #[serde(default)]
    pub contents: Option<ContentsType>,
    #[serde(default, rename = "type")]
    pub type_: Option<AttributeType>,
    #[serde(default, flatten)]
    pub repeat: Option<AttributeRepeat>,
    #[serde(
        default,
        rename = "if",
        deserialize_with = "optional_primitive_as_string"
    )]
    pub if_: Option<String>,
    #[serde(default, deserialize_with = "optional_primitive_as_string")]
    pub size: Option<String>,
    #[serde(default, rename = "size-eos")]
    pub size_eos: Option<bool>,
    // process
    #[serde(default, rename = "enum")]
    pub enum_: Option<String>,
    #[serde(default)]
    pub encoding: Option<String>,
    // pad-right
    #[serde(default)]
    pub terminator: Option<u8>,
    #[serde(default)]
    pub consume: Option<bool>,
    #[serde(default)]
    pub include: Option<bool>,
    #[serde(default, rename = "eos-error")]
    pub eos_error: Option<bool>,
    //pos
    //io
    //value
}

pub type Attributes = Vec<Attribute>;
