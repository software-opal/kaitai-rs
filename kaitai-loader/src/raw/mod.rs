pub use ::serde;
use serde::{Deserialize, Serialize};

schemafy::schemafy!(
    root: KaitaiStruct
    "src/raw/ksy_schema.json"
);

impl Default for MetaSpec {
    fn default() -> Self {
        Self {
            application: vec![],
            encoding: None,
            endian: None,
            file_extension: vec![],
            id: None,
            imports: None,
            ks_debug: None,
            ks_opaque_types: None,
            ks_version: None,
            license: None,
             title: None,
             xref: None,

        }
    }
}
