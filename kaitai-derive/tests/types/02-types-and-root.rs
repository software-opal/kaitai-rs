kaitai_derive::codegen_ksy!(
    r#"
meta:
    id: ds_store
    title: macOS '.DS_Store' format
    license: MIT
    ks-version: 0.8
    encoding: UTF-8
    endian: be
seq:
    - id: something
      type: u16
      repeat: expr
      repeat-expr: 10
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
    // Assert that RecordValue has exactly the following variants, and the inner types are correct.
    match value {
        RecordValue::long(val) => {
            let _other: u32 = val;
        }
        RecordValue::shor(val) => {
            let _other: u32 = val;
        }
        RecordValue::bool(val) => {
            let _other: u8 = val;
        }
        RecordValue::comp(val) => {
            let _other: u64 = val;
        }
        RecordValue::dutc(val) => {
            let _other: u64 = val;
        }
    }

    // Assert that the record object has exactly the following contents.
    let _record = Record {
        filename: "".to_owned(),
        data_type: "".to_owned(),
        value: RecordValue::long(10_u32),
    };

    let _ds_store = DsStore {
        something: vec![0_u16]
    };
}
