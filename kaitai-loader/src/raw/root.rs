use serde::{Deserialize, Serialize};

use super::{
    base::{Doc, DocRef},
    enums::EnumsSpec,
    meta::RootMetaSpec,
    params::ParamsSpec
};

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct KsySpec {
    meta: RootMetaSpec,
    #[serde(default)]
    doc: Option<Doc>,
    #[serde(default, rename = "doc-ref")]
    doc_ref: Option<DocRef>,
    #[serde(default)]
    params: ParamsSpec,
    ///"allows for the setup of named enums, mappings of integer constants to symbolic names. Can be used with integer attributes using the enum key.
    ///
    /// Would be represented as enum-like construct (or closest equivalent, if target language doesn’t support enums), nested or namespaced in current type/class
    enums: EnumsSpec,
}

// "properties": {
//     "meta": {
//       "$ref": "#/definitions/MetaSpec",
//       "required": [
//         "id"
//       ]
//     },
//     "doc": {
//       "$ref": "#/definitions/Doc"
//     },
//     "doc-ref": {
//       "$ref": "#/definitions/DocRef"
//     },
//     "params": {
//       "$ref": "#/definitions/ParamsSpec"
//     },
//     "seq": {
//       "description": "identifier for a primary structure described in top-level map",
//       "$ref": "#/definitions/Attributes"
//     },
//     "types": {
//       "description": "maps of strings to user-defined types\n\ndeclares types for substructures that can be referenced in the attributes of seq or instances element\n\nwould be directly translated into classes",
//       "$ref": "#/definitions/TypesSpec"
//     },
//     "instances": {
//       "description": "Purpose: description of data that lies outside of normal sequential parsing flow (for example, that requires seeking somewhere in the file) or just needs to be loaded only by special request\n\nInfluences: would be translated into distinct methods (that read desired data on demand) in current class",
//       "$ref": "#/definitions/InstancesSpec"
//     },
//     "enums": {
//       "description": "allows for the setup of named enums, mappings of integer constants to symbolic names. Can be used with integer attributes using the enum key.\n\nwould be represented as enum-like construct (or closest equivalent, if target language doesn’t support enums), nested or namespaced in current type/class",
//       "$ref": "#/definitions/EnumsSpec"
//     }
//   },
