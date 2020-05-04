use crate::raw;

pub fn from_string(input: impl ToString) -> serde_yaml::Result<raw::KaitaiStruct> {
    serde_yaml::from_str(&input.to_string())
}

#[cfg(test)]
mod test_generate {
    use super::*;

    #[test]
    fn test_() {
        let ksy_struct =
            from_string(include_str!("../test/test_data/kaitai-sample-png.ksy")).unwrap();

        assert_eq!(
            ksy_struct.meta,
            Some(raw::MetaSpec {
                application: vec![],
                encoding: None,
                endian: Some(serde_json::Value::String("be".to_owned())),
                file_extension: vec!["png".to_owned()],
                id: Some(serde_json::Value::String("png".to_owned())),
                ..raw::MetaSpec::default()
            })
        );
        assert_eq!(ksy_struct.types, Some(raw::TypesSpec {}));
        assert_eq!(
            ksy_struct.enums,
            Some(raw::EnumsSpec {
                ..raw::EnumsSpec::default()
            })
        );
        assert!(ksy_struct.seq.is_some());
        let expected_sequence = vec![
            raw::Attribute::default()
        ];
        for ((i, actual), expected) in ksy_struct.seq.as_ref().unwrap().iter().enumerate().zip(&expected_sequence) {
            println!("{:?}", actual);
            assert_eq!(actual, expected, "Index {}", i);
        }
        assert_eq!(ksy_struct.seq, Some(expected_sequence));
    }
}
