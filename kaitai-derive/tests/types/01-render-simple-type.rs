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
        - id: filename
          type: str
        - id: data_type
          size: 4
          type: str
          doc: Data type of the value.
        - id: value
          type:
            switch-on: data_type
            cases:
              '"long"': u4
              '"shor"': u4
              '"bool"': u1
              '"comp"': u8
              '"dutc"': u8
    "#
);

fn main() {

    use std::convert::TryFrom;

    let value: RecordValue = RecordValue::long(10_u32);
    match value {
        RecordValue::long(val) => {
            let other: u32 = val;
        }
        RecordValue::shor(val) => {
            let other: u32 = val;
        }
        RecordValue::bool(val) => {
            let other: u8 = val;
        }
        RecordValue::comp(val) => {
            let other: u64 = val;
        }
        RecordValue::dutc(val) => {
            let other: u64 = val;
        }
    }
}
