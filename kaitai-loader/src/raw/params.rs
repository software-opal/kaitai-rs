use serde::{Deserialize, Serialize};

use crate::raw::base::{Doc, DocRef, Identifier};

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct ParamSpec {
    pub doc: Option<Doc>,
    #[serde(rename = "doc-ref")]
    pub doc_ref: Option<DocRef>,
    /// path to enum type, which will become the type of the parameter
    ///
    /// only integer-based enums are supported, so `type` must be an integer type (`type: uX`,
    /// `type: sX` or `type: bX`, but not `type: b1` = boolean) for this property to work
    #[serde(rename = "enum")]
    pub enum_: Option<String>,
    pub id: Identifier,
    /// specifies "pure" type of the parameter, without any serialization details (like endianness,
    /// sizes, encodings)
    ///
    /// | Value                  | Description
    /// |-
    /// | `u1`, `u2`, `u4`, `u8` | unsigned integer
    /// | `s1`, `s2`, `s4`, `s8` | signed integer
    /// | `bX`                   | bit-sized integer (if `X` != 1)
    /// | `f4`, `f8`             | floating point number
    /// | no value<br>or `bytes` | byte array
    /// | `str`                  | string
    /// | `bool` (or `b1`)       | boolean
    /// | `struct`               | arbitrary KaitaiStruct-compatible user type
    /// | `io`                   | KaitaiStream-compatible IO stream
    /// | `any`                  | allow any type (if target language supports that)
    /// | other identifier       | user-defined type, without parameters<br>a nested type can be
    /// referenced with double colon (e.g. `type: 'foo::bar'`)
    ///
    /// one can specify arrays by appending `[]` after the type identifier (e.g. `type: u2[]`,
    /// `type: 'foo::bar[]'`, `type: struct[]` etc.)
    #[serde(rename = "type")]
    pub type_: Option<String>,
}
pub type ParamsSpec = Vec<ParamSpec>;
