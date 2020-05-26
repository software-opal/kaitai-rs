kaitai_derive::codegen_ksy!(
    r#"
meta:
    id: ds_store
    title: macOS '.DS_Store' format
    license: MIT
    ks-version: 0.8
    encoding: UTF-8
    endian: be
types:
    record:
      seq:
      - id: type
        type: str
      - id: fn
        type: str
      - id: match
        type: str
      - id: let
        type: str
    "#
);

fn main() {
    let _record = Record {
        r#type: "".to_owned(),
        r#fn: "".to_owned(),
        r#match: "".to_owned(),
        r#let: "".to_owned(),
    };
}
