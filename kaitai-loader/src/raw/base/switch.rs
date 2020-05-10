use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use super::scalar::AnyScalar;

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Switch {
    #[serde(rename = "switch-on")]
    pub switch_on: AnyScalar,
    pub cases: BTreeMap<String, AnyScalar>,
}
