use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::raw::base::{Doc, DocRef, Identifier};

/// Internal struct used to deserialize the EnumValueSpec correctly
#[derive(Deserialize, Debug, PartialEq)]
#[serde(untagged)]
enum EnumValueSpecIntermediate {
    String(String),
    Structure {
        id: Identifier,
        #[serde(default)]
        doc: Option<Doc>,
        #[serde(default, rename = "doc-ref")]
        doc_ref: Option<DocRef>,
        #[serde(default, rename = "-orig-id")]
        orig_id: Option<String>,
    },
}

impl From<EnumValueSpecIntermediate> for EnumValueSpec {
    fn from(evs: EnumValueSpecIntermediate) -> Self {
        match evs {
            EnumValueSpecIntermediate::String(s) => EnumValueSpec {
                id: s,
                ..EnumValueSpec::default()
            },
            EnumValueSpecIntermediate::Structure {
                id,
                doc,
                doc_ref,
                orig_id,
            } => EnumValueSpec {
                id,
                doc,
                doc_ref,
                orig_id,
            },
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
#[serde(from = "EnumValueSpecIntermediate")]
pub struct EnumValueSpec {
    id: Identifier,
    #[serde(default)]
    doc: Option<Doc>,
    #[serde(default, rename = "doc-ref")]
    doc_ref: Option<DocRef>,
    #[serde(default, rename = "-orig-id")]
    orig_id: Option<String>,
}

pub type EnumSpec = BTreeMap<String, EnumValueSpec>;
pub type EnumsSpec = BTreeMap<String, EnumSpec>;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_enum_value_spec_loads() {}

    #[test]
    fn test_endian_loads() {
        assert_eq!(
            serde_json::from_str::<EnumValueSpec>(r#""enum1""#).unwrap(),
            EnumValueSpec {
                id: "enum1".to_owned(),
                ..EnumValueSpec::default()
            }
        );
        assert_eq!(
            serde_json::from_str::<EnumValueSpec>(r#"{ "id": "enum1", "-orig-id": "teapot" }"#)
                .unwrap(),
            EnumValueSpec {
                id: "enum1".to_owned(),
                orig_id: Some("teapot".to_owned()),
                ..EnumValueSpec::default()
            }
        );
    }
}
