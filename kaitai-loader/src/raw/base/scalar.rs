use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum StringOrNumber {
    String(String),
    Number(serde_json::Number),
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum StringOrBoolean {
    String(String),
    Boolean(bool),
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum AnyNonNullScalar {
    String(String),
    Boolean(bool),
    Number(serde_json::Number),
}

pub type AnyScalar = Option<AnyNonNullScalar>;
