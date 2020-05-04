use std::fmt;

use serde::de::SeqAccess;
use serde::de::{self,  Visitor};
use serde::{Deserialize, Deserializer};

pub fn string_or_vec_deserialize<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    struct StringOrVec {};

    impl<'de> Visitor<'de> for StringOrVec {
        type Value = Vec<String>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("string or sequence")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(vec![value.to_owned()])
        }

        fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
        where
            A: SeqAccess<'de>,
        {
            // `MapAccessDeserializer` is a wrapper that turns a `MapAccess`
            // into a `Deserializer`, allowing it to be used as the input to T's
            // `Deserialize` implementation. T then deserializes itself using
            // the entries from the map visitor.
            Deserialize::deserialize(de::value::SeqAccessDeserializer::new(seq))
        }
    }

    deserializer.deserialize_any(StringOrVec {})
}
