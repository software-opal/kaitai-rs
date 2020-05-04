
use serde::{Deserialize, Serialize};
use std::collections::HashMap;



#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]

pub type Attributes = Vec<Attribute>;
/// used to give a more detailed description of a user-defined type. In most languages, it will be
/// used as a docstring compatible with tools like Javadoc, Doxygen, JSDoc, etc.
pub type Doc = String;
/// used to provide reference to original documentation (if the ksy file is actually an
/// implementation of some documented format).
///
/// Contains:
/// 1. URL as text,
/// 2. arbitrary string, or
/// 3. URL as text + space + arbitrary string
pub type DocRef = Vec<String>;

pub struct TypeMetadata {
    id: Option<String>,
    title: Option<String>,
    application: Option<String>,
    #[serde(
        rename = "file-extension",
        default,
        deserialize_with = "super::deserializer::string_or_vec_deserialize"
    )]
    file_extension: Vec<String>,
    license: Option<String>,
    #[serde(rename = "ks-version")]
    ks_version: Option<String>,
    #[serde(rename = "file-extension", default)]
    imports: Vec<String>,
    encoding: Option<String>,
    endian: Option<Endian>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Endian {
    #[serde(rename = "be")]
    BigEndian,
    #[serde(rename = "le")]
    LittleEndian,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Attribute {
    id: Option<String>,
    doc: Option<String>,
    #[serde(rename = "doc-ref")]
    doc_ref: Option<String>,
    contents: Option<ContentsEnum>,
    r#type: Option<String>,
    #[serde(flatten)]
    repeat: Option<RepeatEnum>,
    #[serde(rename = "if")]
    r#if: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum ContentsEnum {
    String(String),
    U8(u8),
    Vec(Vec<ContentsEnum>),
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(tag = "repeat")]
pub enum RepeatEnum {
    Eos,
    Expr {
        #[serde(rename = "repeat-expr")]
        repeat_expr: String,
    },
    Until {
        #[serde(rename = "repeat-until")]
        repeat_until: String,
    },
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct RawParamSpec {}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct RawTypeSpec {
    #[serde(default)]
    meta: TypeMetadata,
    doc: Option<String>,
    doc_ref: Option<String>,
    seq: Vec<Attribute>,
    // instances: HashMap<String,
    #[serde(default)]
    enums: HashMap<String, HashMap<i64, String>>,
    #[serde(default)]
    types: HashMap<String, RawTypeSpec>,
    #[serde(default)]
    params: Vec<RawParamSpec>,
}

pub fn parse_string(string: &str) -> serde_yaml::Result<RawTypeSpec> {
    serde_yaml::from_str(string)
}

#[cfg(test)]
mod test_super {
    use super::*;

    const GIF_SAMPLE: &str = r"
meta:
  id: gif
  file-extension: gif
  endian: le
seq:
  - id: header
    type: header
  - id: logical_screen
    type: logical_screen
types:
  header:
    seq:
      - id: magic
        contents: 'GIF'
      - id: version
        size: 3
  logical_screen:
    seq:
      - id: image_width
        type: u2
      - id: image_height
        type: u2
      - id: flags
        type: u1
      - id: bg_color_index
        type: u1
      - id: pixel_aspect_ratio
        type: u1
";

    #[test]
    fn test_load_gif_sample() {
        let format = parse_string(GIF_SAMPLE).unwrap();
        println!("{:?}", format);
        assert_eq!(
            format.meta,
            TypeMetadata {
                id: Some("gif".to_owned()),
                endian: Some(Endian::LittleEndian),
                file_extension: vec!["gif".to_owned()],
                ..TypeMetadata::default()
            }
        );
        assert_eq!(
            format.seq,
            vec![
                Attribute {
                    id: Some("header".to_owned()),
                    r#type: Some("header".to_owned()),
                    ..Attribute::default()
                },
                Attribute {
                    id: Some("logical_screen".to_owned()),
                    r#type: Some("logical_screen".to_owned()),
                    ..Attribute::default()
                }
            ]
        );
    }
}
