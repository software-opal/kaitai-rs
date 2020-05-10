use serde::{Deserialize, Serialize};

use crate::raw::base::{Doc, DocRef, Identifier, Switch};
use crate::raw::serde_util::{primitive_as_string, optional_primitive_as_string};


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

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(tag = "repeat", rename_all="lowercase")]
pub enum AttributeRepeat {
    EOS,
    Expr {
        #[serde(rename="repeat-expr", deserialize_with="primitive_as_string")]
        expression: String,
    },
    Until {
        #[serde(rename="repeat-expr", deserialize_with="primitive_as_string")]
        expression: String
    }

}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Attribute {
    pub id: Identifier,
    #[serde(default)]
    doc: Option<Doc>,
    #[serde(default, rename = "doc-ref")]
    doc_ref: Option<DocRef>,
    #[serde(default)]
    contents: Option<ContentsType>,
    #[serde(default, rename="type")]
    type_: Option<AttributeType>,
    #[serde(default, flatten)]
    repeat: Option<AttributeRepeat>,
    #[serde(default, rename="if", deserialize_with="optional_primitive_as_string")]
    if_: Option<String>,
    #[serde(default, deserialize_with="optional_primitive_as_string")]
    size: Option<String>,
    #[serde(default, rename = "size-eos")]
    size_eos: Option<bool>,
    // process
    #[serde(default, rename = "enum")]
    enum_: Option<String>,
    #[serde(default)]
    encoding: Option<String>,
    // pad-right
    #[serde(default)]
    terminator: Option<u8>,
    #[serde(default)]
    consume: Option<bool>,
    #[serde(default)]
    include: Option<bool>,
    #[serde(default, rename = "eos-error")]
    eos_error: Option<bool>,
    //pos
    //io
    //value
}

pub type Attributes = Vec<Attribute>;
