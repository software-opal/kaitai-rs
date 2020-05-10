use std::fmt;

use serde::{
    de::{self, MapAccess, Visitor},
    ser::Serializer,
    Deserialize, Deserializer, Serialize,
};

use super::switch::Switch;

#[derive(Debug, PartialEq)]
pub enum Endian {
    Little,
    Big,
    Switch(Switch),
}

impl<'de> Deserialize<'de> for Endian {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(EndianVisitor {})
    }
}
impl Serialize for Endian {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Big => serializer.serialize_str("be"),
            Self::Little => serializer.serialize_str("le"),
            Self::Switch(switch) => switch.serialize(serializer),
        }
    }
}

struct EndianVisitor;

impl<'de> Visitor<'de> for EndianVisitor {
    type Value = Endian;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(r#""le", "be" or map"#)
    }

    fn visit_str<E>(self, value: &str) -> Result<Endian, E>
    where
        E: de::Error,
    {
        match value {
            "be" => Ok(Endian::Big),
            "le" => Ok(Endian::Little),
            _ => Err(de::Error::invalid_type(de::Unexpected::Enum, &self)),
        }
    }

    fn visit_map<M>(self, map: M) -> Result<Endian, M::Error>
    where
        M: MapAccess<'de>,
    {
        Ok(Endian::Switch(Deserialize::deserialize(
            de::value::MapAccessDeserializer::new(map),
        )?))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::raw::base::AnyNonNullScalar;

    #[test]
    fn test_endian_loads() {
        assert_eq!(
            serde_json::from_str::<Endian>(r#""be""#).unwrap(),
            Endian::Big
        );
        assert_eq!(
            serde_json::from_str::<Endian>(r#""le""#).unwrap(),
            Endian::Little
        );
        assert_eq!(
            serde_json::from_str::<Endian>(r#"{ "switch-on": "a", "cases": { "a": "b" } }"#)
                .unwrap(),
            Endian::Switch(Switch {
                switch_on: Some(AnyNonNullScalar::String("a".to_owned())),
                cases: vec![(
                    "a".to_owned(),
                    Some(AnyNonNullScalar::String("b".to_owned()))
                )]
                .into_iter()
                .collect()
            })
        );
    }
    #[test]
    fn test_endian_saves() {
        assert_eq!(
            serde_json::to_string(&Endian::Big).unwrap(),
            r#""be""#.to_owned()
        );
        assert_eq!(serde_json::to_string(&Endian::Little).unwrap(), r#""le""#);
        assert_eq!(
            serde_json::to_string(&Endian::Switch(Switch {
                switch_on: Some(AnyNonNullScalar::String("a".to_owned())),
                cases: vec![(
                    "a".to_owned(),
                    Some(AnyNonNullScalar::String("b".to_owned()))
                )]
                .into_iter()
                .collect()
            }))
            .unwrap(),
            r#"{"switch-on":"a","cases":{"a":"b"}}"#
        );
    }
}
