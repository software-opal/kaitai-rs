use crate::raw::root::KsySpec;

pub fn from_string(input: impl ToString) -> serde_yaml::Result<KsySpec> {
    serde_yaml::from_str(&input.to_string())
}

#[cfg(test)]
mod test_generate {
    use super::*;

    #[test]
    fn test_() {
        let ksy_struct =
            from_string(include_str!("../test/test_data/kaitai-sample-png.ksy")).unwrap();

            println!("{:#?}", ksy_struct);
            assert!(false);
    }
}
