#![feature(prelude_import)]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std;
pub mod raw {
    pub use :: serde;
    use serde::{Deserialize, Serialize};
    pub type AnyScalar = serde_json::Value;
    pub struct Attribute {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for Attribute {
        #[inline]
        fn clone(&self) -> Attribute {
            match *self {
                Attribute {} => Attribute {},
            }
        }
    }
    impl ::core::marker::StructuralPartialEq for Attribute {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for Attribute {
        #[inline]
        fn eq(&self, other: &Attribute) -> bool {
            match *other {
                Attribute {} => match *self {
                    Attribute {} => true,
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for Attribute {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                Attribute {} => {
                    let mut debug_trait_builder = f.debug_struct("Attribute");
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for Attribute {
        #[inline]
        fn default() -> Attribute {
            Attribute {}
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DESERIALIZE_FOR_Attribute: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Attribute {
            fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"field index 0 <= i < 0",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::export::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::export::PhantomData<Attribute>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Attribute;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "struct Attribute")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        _: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        _serde::export::Ok(Attribute {})
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        while let _serde::export::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            }
                        {
                            match __key {
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        _serde::export::Ok(Attribute {})
                    }
                }
                const FIELDS: &'static [&'static str] = &[];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "Attribute",
                    FIELDS,
                    __Visitor {
                        marker: _serde::export::PhantomData::<Attribute>,
                        lifetime: _serde::export::PhantomData,
                    },
                )
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_SERIALIZE_FOR_Attribute: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Attribute {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::export::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "Attribute",
                    false as usize,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    pub type Attributes = Vec<Attribute>;
    /// used to give a more detailed description of a user-defined type. In most languages, it will be
    /// used as a docstring compatible with tools like Javadoc, Doxygen, JSDoc, etc.
    pub type Doc = String;
    /// used to provide reference to original documentation (if the ksy file is actually an
    /// implementation of some documented format).
    ///
    /// Contains:
    /// 1. URL as text,
    /// 2. arbitrary string, or
    /// 3. URL as text + space + arbitrary string
    pub type DocRef = Vec<String>;
    pub struct EnumSpec {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for EnumSpec {
        #[inline]
        fn clone(&self) -> EnumSpec {
            match *self {
                EnumSpec {} => EnumSpec {},
            }
        }
    }
    impl ::core::marker::StructuralPartialEq for EnumSpec {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for EnumSpec {
        #[inline]
        fn eq(&self, other: &EnumSpec) -> bool {
            match *other {
                EnumSpec {} => match *self {
                    EnumSpec {} => true,
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for EnumSpec {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                EnumSpec {} => {
                    let mut debug_trait_builder = f.debug_struct("EnumSpec");
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for EnumSpec {
        #[inline]
        fn default() -> EnumSpec {
            EnumSpec {}
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DESERIALIZE_FOR_EnumSpec: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for EnumSpec {
            fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"field index 0 <= i < 0",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::export::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::export::PhantomData<EnumSpec>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = EnumSpec;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "struct EnumSpec")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        _: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        _serde::export::Ok(EnumSpec {})
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        while let _serde::export::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            }
                        {
                            match __key {
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        _serde::export::Ok(EnumSpec {})
                    }
                }
                const FIELDS: &'static [&'static str] = &[];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "EnumSpec",
                    FIELDS,
                    __Visitor {
                        marker: _serde::export::PhantomData::<EnumSpec>,
                        lifetime: _serde::export::PhantomData,
                    },
                )
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_SERIALIZE_FOR_EnumSpec: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for EnumSpec {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::export::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "EnumSpec",
                    false as usize,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    pub type EnumValueSpec = serde_json::Value;
    pub struct EnumsSpec {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for EnumsSpec {
        #[inline]
        fn clone(&self) -> EnumsSpec {
            match *self {
                EnumsSpec {} => EnumsSpec {},
            }
        }
    }
    impl ::core::marker::StructuralPartialEq for EnumsSpec {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for EnumsSpec {
        #[inline]
        fn eq(&self, other: &EnumsSpec) -> bool {
            match *other {
                EnumsSpec {} => match *self {
                    EnumsSpec {} => true,
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for EnumsSpec {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                EnumsSpec {} => {
                    let mut debug_trait_builder = f.debug_struct("EnumsSpec");
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for EnumsSpec {
        #[inline]
        fn default() -> EnumsSpec {
            EnumsSpec {}
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DESERIALIZE_FOR_EnumsSpec: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for EnumsSpec {
            fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"field index 0 <= i < 0",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::export::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::export::PhantomData<EnumsSpec>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = EnumsSpec;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "struct EnumsSpec")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        _: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        _serde::export::Ok(EnumsSpec {})
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        while let _serde::export::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            }
                        {
                            match __key {
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        _serde::export::Ok(EnumsSpec {})
                    }
                }
                const FIELDS: &'static [&'static str] = &[];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "EnumsSpec",
                    FIELDS,
                    __Visitor {
                        marker: _serde::export::PhantomData::<EnumsSpec>,
                        lifetime: _serde::export::PhantomData,
                    },
                )
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_SERIALIZE_FOR_EnumsSpec: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for EnumsSpec {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::export::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "EnumsSpec",
                    false as usize,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    pub type Identifier = serde_json::Value;
    pub struct InstancesSpec {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for InstancesSpec {
        #[inline]
        fn clone(&self) -> InstancesSpec {
            match *self {
                InstancesSpec {} => InstancesSpec {},
            }
        }
    }
    impl ::core::marker::StructuralPartialEq for InstancesSpec {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for InstancesSpec {
        #[inline]
        fn eq(&self, other: &InstancesSpec) -> bool {
            match *other {
                InstancesSpec {} => match *self {
                    InstancesSpec {} => true,
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for InstancesSpec {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                InstancesSpec {} => {
                    let mut debug_trait_builder = f.debug_struct("InstancesSpec");
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for InstancesSpec {
        #[inline]
        fn default() -> InstancesSpec {
            InstancesSpec {}
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DESERIALIZE_FOR_InstancesSpec: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for InstancesSpec {
            fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"field index 0 <= i < 0",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::export::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::export::PhantomData<InstancesSpec>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = InstancesSpec;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "struct InstancesSpec")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        _: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        _serde::export::Ok(InstancesSpec {})
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        while let _serde::export::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            }
                        {
                            match __key {
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        _serde::export::Ok(InstancesSpec {})
                    }
                }
                const FIELDS: &'static [&'static str] = &[];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "InstancesSpec",
                    FIELDS,
                    __Visitor {
                        marker: _serde::export::PhantomData::<InstancesSpec>,
                        lifetime: _serde::export::PhantomData,
                    },
                )
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_SERIALIZE_FOR_InstancesSpec: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for InstancesSpec {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::export::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "InstancesSpec",
                    false as usize,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    pub type IsoIdentifier = String;
    pub type LocIdentifier = String;
    pub type MediaWikiPageName = String;
    pub struct MetaSpecItemXref {
        /// article name at [Forensics Wiki](https://forensicswiki.xyz/page/Main_Page), which is a
        /// CC-BY-SA-licensed wiki with information on digital forensics, file formats and tools
        ///
        /// full link name could be generated as `https://forensicswiki.xyz/page/` + this value
        #[serde(default)]
        #[serde(with = "::schemafy_core::one_or_many")]
        pub forensicswiki: Vec<MediaWikiPageName>,
        /// ISO/IEC standard number, reference to a standard accepted and published by
        /// [ISO](https://www.iso.org/) (International Organization for Standardization).
        ///
        /// ISO standards typically have clear designations like "ISO/IEC 15948:2004", so value should
        /// be citing everything except for "ISO/IEC", i.e. `15948:2004`
        #[serde(default)]
        #[serde(with = "::schemafy_core::one_or_many")]
        pub iso: Vec<IsoIdentifier>,
        /// article name at ["Just Solve the File Format Problem"
        /// wiki](http://fileformats.archiveteam.org/wiki/Main_Page), a wiki that collects information
        /// on many file formats
        ///
        /// full link name could be generated as `http://fileformats.archiveteam.org/wiki/` + this
        /// value
        #[serde(default)]
        #[serde(with = "::schemafy_core::one_or_many")]
        pub justsolve: Vec<MediaWikiPageName>,
        /// identifier in [Digital
        /// Formats](https://www.loc.gov/preservation/digital/formats/fdd/browse_list.shtml) database
        /// of [US Library of Congress](https://www.loc.gov/)
        ///
        /// value typically looks like `fddXXXXXX`, where `XXXXXX` is a 6-digit identifier
        #[serde(default)]
        #[serde(with = "::schemafy_core::one_or_many")]
        pub loc: Vec<LocIdentifier>,
        /// MIME type (IANA media type), a string typically used in various Internet protocols to
        /// specify format of binary payload
        ///
        /// there is a [central registry of media
        /// types](https://www.iana.org/assignments/media-types/media-types.xhtml) managed by IANA
        ///
        /// value must specify full MIME type (both parts), e.g. `image/png`
        #[serde(default)]
        #[serde(with = "::schemafy_core::one_or_many")]
        pub mime: Vec<MimeType>,
        /// format identifier in [PRONOM Technical
        /// Registry](https://www.nationalarchives.gov.uk/PRONOM/Default.aspx) of [UK National
        /// Archives](https://www.nationalarchives.gov.uk/), which is a massive file formats database
        /// that catalogues many file formats for digital preservation purposes
        #[serde(default)]
        #[serde(with = "::schemafy_core::one_or_many")]
        pub pronom: Vec<PronomIdentifier>,
        /// reference to [RFC](https://en.wikipedia.org/wiki/Request_for_Comments), "Request for
        /// Comments" documents maintained by ISOC (Internet Society)
        ///
        /// RFCs are typically treated as global, Internet-wide standards, and, for example, many
        /// networking / interoperability protocols are specified in RFCs
        ///
        /// value should be just raw RFC number, without any prefixes, e.g. `1234`
        #[serde(default)]
        #[serde(with = "::schemafy_core::one_or_many")]
        pub rfc: Vec<RfcIdentifier>,
        /// item identifier at Wikidata, a global knowledge base
        ///
        /// value typically follows `Qxxx` pattern, where `xxx` is a number generated by Wikidata, e.g.
        /// `Q535473`
        #[serde(default)]
        #[serde(with = "::schemafy_core::one_or_many")]
        pub wikidata: Vec<WikidataIdentifier>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for MetaSpecItemXref {
        #[inline]
        fn clone(&self) -> MetaSpecItemXref {
            match *self {
                MetaSpecItemXref {
                    forensicswiki: ref __self_0_0,
                    iso: ref __self_0_1,
                    justsolve: ref __self_0_2,
                    loc: ref __self_0_3,
                    mime: ref __self_0_4,
                    pronom: ref __self_0_5,
                    rfc: ref __self_0_6,
                    wikidata: ref __self_0_7,
                } => MetaSpecItemXref {
                    forensicswiki: ::core::clone::Clone::clone(&(*__self_0_0)),
                    iso: ::core::clone::Clone::clone(&(*__self_0_1)),
                    justsolve: ::core::clone::Clone::clone(&(*__self_0_2)),
                    loc: ::core::clone::Clone::clone(&(*__self_0_3)),
                    mime: ::core::clone::Clone::clone(&(*__self_0_4)),
                    pronom: ::core::clone::Clone::clone(&(*__self_0_5)),
                    rfc: ::core::clone::Clone::clone(&(*__self_0_6)),
                    wikidata: ::core::clone::Clone::clone(&(*__self_0_7)),
                },
            }
        }
    }
    impl ::core::marker::StructuralPartialEq for MetaSpecItemXref {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for MetaSpecItemXref {
        #[inline]
        fn eq(&self, other: &MetaSpecItemXref) -> bool {
            match *other {
                MetaSpecItemXref {
                    forensicswiki: ref __self_1_0,
                    iso: ref __self_1_1,
                    justsolve: ref __self_1_2,
                    loc: ref __self_1_3,
                    mime: ref __self_1_4,
                    pronom: ref __self_1_5,
                    rfc: ref __self_1_6,
                    wikidata: ref __self_1_7,
                } => match *self {
                    MetaSpecItemXref {
                        forensicswiki: ref __self_0_0,
                        iso: ref __self_0_1,
                        justsolve: ref __self_0_2,
                        loc: ref __self_0_3,
                        mime: ref __self_0_4,
                        pronom: ref __self_0_5,
                        rfc: ref __self_0_6,
                        wikidata: ref __self_0_7,
                    } => {
                        (*__self_0_0) == (*__self_1_0)
                            && (*__self_0_1) == (*__self_1_1)
                            && (*__self_0_2) == (*__self_1_2)
                            && (*__self_0_3) == (*__self_1_3)
                            && (*__self_0_4) == (*__self_1_4)
                            && (*__self_0_5) == (*__self_1_5)
                            && (*__self_0_6) == (*__self_1_6)
                            && (*__self_0_7) == (*__self_1_7)
                    }
                },
            }
        }
        #[inline]
        fn ne(&self, other: &MetaSpecItemXref) -> bool {
            match *other {
                MetaSpecItemXref {
                    forensicswiki: ref __self_1_0,
                    iso: ref __self_1_1,
                    justsolve: ref __self_1_2,
                    loc: ref __self_1_3,
                    mime: ref __self_1_4,
                    pronom: ref __self_1_5,
                    rfc: ref __self_1_6,
                    wikidata: ref __self_1_7,
                } => match *self {
                    MetaSpecItemXref {
                        forensicswiki: ref __self_0_0,
                        iso: ref __self_0_1,
                        justsolve: ref __self_0_2,
                        loc: ref __self_0_3,
                        mime: ref __self_0_4,
                        pronom: ref __self_0_5,
                        rfc: ref __self_0_6,
                        wikidata: ref __self_0_7,
                    } => {
                        (*__self_0_0) != (*__self_1_0)
                            || (*__self_0_1) != (*__self_1_1)
                            || (*__self_0_2) != (*__self_1_2)
                            || (*__self_0_3) != (*__self_1_3)
                            || (*__self_0_4) != (*__self_1_4)
                            || (*__self_0_5) != (*__self_1_5)
                            || (*__self_0_6) != (*__self_1_6)
                            || (*__self_0_7) != (*__self_1_7)
                    }
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for MetaSpecItemXref {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                MetaSpecItemXref {
                    forensicswiki: ref __self_0_0,
                    iso: ref __self_0_1,
                    justsolve: ref __self_0_2,
                    loc: ref __self_0_3,
                    mime: ref __self_0_4,
                    pronom: ref __self_0_5,
                    rfc: ref __self_0_6,
                    wikidata: ref __self_0_7,
                } => {
                    let mut debug_trait_builder = f.debug_struct("MetaSpecItemXref");
                    let _ = debug_trait_builder.field("forensicswiki", &&(*__self_0_0));
                    let _ = debug_trait_builder.field("iso", &&(*__self_0_1));
                    let _ = debug_trait_builder.field("justsolve", &&(*__self_0_2));
                    let _ = debug_trait_builder.field("loc", &&(*__self_0_3));
                    let _ = debug_trait_builder.field("mime", &&(*__self_0_4));
                    let _ = debug_trait_builder.field("pronom", &&(*__self_0_5));
                    let _ = debug_trait_builder.field("rfc", &&(*__self_0_6));
                    let _ = debug_trait_builder.field("wikidata", &&(*__self_0_7));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DESERIALIZE_FOR_MetaSpecItemXref: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for MetaSpecItemXref {
            fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                    __field4,
                    __field5,
                    __field6,
                    __field7,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::export::Ok(__Field::__field0),
                            1u64 => _serde::export::Ok(__Field::__field1),
                            2u64 => _serde::export::Ok(__Field::__field2),
                            3u64 => _serde::export::Ok(__Field::__field3),
                            4u64 => _serde::export::Ok(__Field::__field4),
                            5u64 => _serde::export::Ok(__Field::__field5),
                            6u64 => _serde::export::Ok(__Field::__field6),
                            7u64 => _serde::export::Ok(__Field::__field7),
                            _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"field index 0 <= i < 8",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "forensicswiki" => _serde::export::Ok(__Field::__field0),
                            "iso" => _serde::export::Ok(__Field::__field1),
                            "justsolve" => _serde::export::Ok(__Field::__field2),
                            "loc" => _serde::export::Ok(__Field::__field3),
                            "mime" => _serde::export::Ok(__Field::__field4),
                            "pronom" => _serde::export::Ok(__Field::__field5),
                            "rfc" => _serde::export::Ok(__Field::__field6),
                            "wikidata" => _serde::export::Ok(__Field::__field7),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"forensicswiki" => _serde::export::Ok(__Field::__field0),
                            b"iso" => _serde::export::Ok(__Field::__field1),
                            b"justsolve" => _serde::export::Ok(__Field::__field2),
                            b"loc" => _serde::export::Ok(__Field::__field3),
                            b"mime" => _serde::export::Ok(__Field::__field4),
                            b"pronom" => _serde::export::Ok(__Field::__field5),
                            b"rfc" => _serde::export::Ok(__Field::__field6),
                            b"wikidata" => _serde::export::Ok(__Field::__field7),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::export::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::export::PhantomData<MetaSpecItemXref>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = MetaSpecItemXref;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "struct MetaSpecItemXref")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match {
                            struct __DeserializeWith<'de> {
                                value: Vec<MediaWikiPageName>,
                                phantom: _serde::export::PhantomData<MetaSpecItemXref>,
                                lifetime: _serde::export::PhantomData<&'de ()>,
                            }
                            impl<'de> _serde::Deserialize<'de> for __DeserializeWith<'de> {
                                fn deserialize<__D>(
                                    __deserializer: __D,
                                ) -> _serde::export::Result<Self, __D::Error>
                                where
                                    __D: _serde::Deserializer<'de>,
                                {
                                    _serde::export::Ok(__DeserializeWith {
                                        value: match ::schemafy_core::one_or_many::deserialize(
                                            __deserializer,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                        phantom: _serde::export::PhantomData,
                                        lifetime: _serde::export::PhantomData,
                                    })
                                }
                            }
                            _serde::export::Option::map(
                                match _serde::de::SeqAccess::next_element::<__DeserializeWith<'de>>(
                                    &mut __seq,
                                ) {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                },
                                |__wrap| __wrap.value,
                            )
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => _serde::export::Default::default(),
                        };
                        let __field1 = match {
                            struct __DeserializeWith<'de> {
                                value: Vec<IsoIdentifier>,
                                phantom: _serde::export::PhantomData<MetaSpecItemXref>,
                                lifetime: _serde::export::PhantomData<&'de ()>,
                            }
                            impl<'de> _serde::Deserialize<'de> for __DeserializeWith<'de> {
                                fn deserialize<__D>(
                                    __deserializer: __D,
                                ) -> _serde::export::Result<Self, __D::Error>
                                where
                                    __D: _serde::Deserializer<'de>,
                                {
                                    _serde::export::Ok(__DeserializeWith {
                                        value: match ::schemafy_core::one_or_many::deserialize(
                                            __deserializer,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                        phantom: _serde::export::PhantomData,
                                        lifetime: _serde::export::PhantomData,
                                    })
                                }
                            }
                            _serde::export::Option::map(
                                match _serde::de::SeqAccess::next_element::<__DeserializeWith<'de>>(
                                    &mut __seq,
                                ) {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                },
                                |__wrap| __wrap.value,
                            )
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => _serde::export::Default::default(),
                        };
                        let __field2 = match {
                            struct __DeserializeWith<'de> {
                                value: Vec<MediaWikiPageName>,
                                phantom: _serde::export::PhantomData<MetaSpecItemXref>,
                                lifetime: _serde::export::PhantomData<&'de ()>,
                            }
                            impl<'de> _serde::Deserialize<'de> for __DeserializeWith<'de> {
                                fn deserialize<__D>(
                                    __deserializer: __D,
                                ) -> _serde::export::Result<Self, __D::Error>
                                where
                                    __D: _serde::Deserializer<'de>,
                                {
                                    _serde::export::Ok(__DeserializeWith {
                                        value: match ::schemafy_core::one_or_many::deserialize(
                                            __deserializer,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                        phantom: _serde::export::PhantomData,
                                        lifetime: _serde::export::PhantomData,
                                    })
                                }
                            }
                            _serde::export::Option::map(
                                match _serde::de::SeqAccess::next_element::<__DeserializeWith<'de>>(
                                    &mut __seq,
                                ) {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                },
                                |__wrap| __wrap.value,
                            )
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => _serde::export::Default::default(),
                        };
                        let __field3 = match {
                            struct __DeserializeWith<'de> {
                                value: Vec<LocIdentifier>,
                                phantom: _serde::export::PhantomData<MetaSpecItemXref>,
                                lifetime: _serde::export::PhantomData<&'de ()>,
                            }
                            impl<'de> _serde::Deserialize<'de> for __DeserializeWith<'de> {
                                fn deserialize<__D>(
                                    __deserializer: __D,
                                ) -> _serde::export::Result<Self, __D::Error>
                                where
                                    __D: _serde::Deserializer<'de>,
                                {
                                    _serde::export::Ok(__DeserializeWith {
                                        value: match ::schemafy_core::one_or_many::deserialize(
                                            __deserializer,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                        phantom: _serde::export::PhantomData,
                                        lifetime: _serde::export::PhantomData,
                                    })
                                }
                            }
                            _serde::export::Option::map(
                                match _serde::de::SeqAccess::next_element::<__DeserializeWith<'de>>(
                                    &mut __seq,
                                ) {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                },
                                |__wrap| __wrap.value,
                            )
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => _serde::export::Default::default(),
                        };
                        let __field4 = match {
                            struct __DeserializeWith<'de> {
                                value: Vec<MimeType>,
                                phantom: _serde::export::PhantomData<MetaSpecItemXref>,
                                lifetime: _serde::export::PhantomData<&'de ()>,
                            }
                            impl<'de> _serde::Deserialize<'de> for __DeserializeWith<'de> {
                                fn deserialize<__D>(
                                    __deserializer: __D,
                                ) -> _serde::export::Result<Self, __D::Error>
                                where
                                    __D: _serde::Deserializer<'de>,
                                {
                                    _serde::export::Ok(__DeserializeWith {
                                        value: match ::schemafy_core::one_or_many::deserialize(
                                            __deserializer,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                        phantom: _serde::export::PhantomData,
                                        lifetime: _serde::export::PhantomData,
                                    })
                                }
                            }
                            _serde::export::Option::map(
                                match _serde::de::SeqAccess::next_element::<__DeserializeWith<'de>>(
                                    &mut __seq,
                                ) {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                },
                                |__wrap| __wrap.value,
                            )
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => _serde::export::Default::default(),
                        };
                        let __field5 = match {
                            struct __DeserializeWith<'de> {
                                value: Vec<PronomIdentifier>,
                                phantom: _serde::export::PhantomData<MetaSpecItemXref>,
                                lifetime: _serde::export::PhantomData<&'de ()>,
                            }
                            impl<'de> _serde::Deserialize<'de> for __DeserializeWith<'de> {
                                fn deserialize<__D>(
                                    __deserializer: __D,
                                ) -> _serde::export::Result<Self, __D::Error>
                                where
                                    __D: _serde::Deserializer<'de>,
                                {
                                    _serde::export::Ok(__DeserializeWith {
                                        value: match ::schemafy_core::one_or_many::deserialize(
                                            __deserializer,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                        phantom: _serde::export::PhantomData,
                                        lifetime: _serde::export::PhantomData,
                                    })
                                }
                            }
                            _serde::export::Option::map(
                                match _serde::de::SeqAccess::next_element::<__DeserializeWith<'de>>(
                                    &mut __seq,
                                ) {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                },
                                |__wrap| __wrap.value,
                            )
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => _serde::export::Default::default(),
                        };
                        let __field6 = match {
                            struct __DeserializeWith<'de> {
                                value: Vec<RfcIdentifier>,
                                phantom: _serde::export::PhantomData<MetaSpecItemXref>,
                                lifetime: _serde::export::PhantomData<&'de ()>,
                            }
                            impl<'de> _serde::Deserialize<'de> for __DeserializeWith<'de> {
                                fn deserialize<__D>(
                                    __deserializer: __D,
                                ) -> _serde::export::Result<Self, __D::Error>
                                where
                                    __D: _serde::Deserializer<'de>,
                                {
                                    _serde::export::Ok(__DeserializeWith {
                                        value: match ::schemafy_core::one_or_many::deserialize(
                                            __deserializer,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                        phantom: _serde::export::PhantomData,
                                        lifetime: _serde::export::PhantomData,
                                    })
                                }
                            }
                            _serde::export::Option::map(
                                match _serde::de::SeqAccess::next_element::<__DeserializeWith<'de>>(
                                    &mut __seq,
                                ) {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                },
                                |__wrap| __wrap.value,
                            )
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => _serde::export::Default::default(),
                        };
                        let __field7 = match {
                            struct __DeserializeWith<'de> {
                                value: Vec<WikidataIdentifier>,
                                phantom: _serde::export::PhantomData<MetaSpecItemXref>,
                                lifetime: _serde::export::PhantomData<&'de ()>,
                            }
                            impl<'de> _serde::Deserialize<'de> for __DeserializeWith<'de> {
                                fn deserialize<__D>(
                                    __deserializer: __D,
                                ) -> _serde::export::Result<Self, __D::Error>
                                where
                                    __D: _serde::Deserializer<'de>,
                                {
                                    _serde::export::Ok(__DeserializeWith {
                                        value: match ::schemafy_core::one_or_many::deserialize(
                                            __deserializer,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                        phantom: _serde::export::PhantomData,
                                        lifetime: _serde::export::PhantomData,
                                    })
                                }
                            }
                            _serde::export::Option::map(
                                match _serde::de::SeqAccess::next_element::<__DeserializeWith<'de>>(
                                    &mut __seq,
                                ) {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                },
                                |__wrap| __wrap.value,
                            )
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => _serde::export::Default::default(),
                        };
                        _serde::export::Ok(MetaSpecItemXref {
                            forensicswiki: __field0,
                            iso: __field1,
                            justsolve: __field2,
                            loc: __field3,
                            mime: __field4,
                            pronom: __field5,
                            rfc: __field6,
                            wikidata: __field7,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::export::Option<Vec<MediaWikiPageName>> =
                            _serde::export::None;
                        let mut __field1: _serde::export::Option<Vec<IsoIdentifier>> =
                            _serde::export::None;
                        let mut __field2: _serde::export::Option<Vec<MediaWikiPageName>> =
                            _serde::export::None;
                        let mut __field3: _serde::export::Option<Vec<LocIdentifier>> =
                            _serde::export::None;
                        let mut __field4: _serde::export::Option<Vec<MimeType>> =
                            _serde::export::None;
                        let mut __field5: _serde::export::Option<Vec<PronomIdentifier>> =
                            _serde::export::None;
                        let mut __field6: _serde::export::Option<Vec<RfcIdentifier>> =
                            _serde::export::None;
                        let mut __field7: _serde::export::Option<Vec<WikidataIdentifier>> =
                            _serde::export::None;
                        while let _serde::export::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::export::Option::is_some(&__field0) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "forensicswiki",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::export::Some({
                                        struct __DeserializeWith<'de> {
                                            value: Vec<MediaWikiPageName>,
                                            phantom: _serde::export::PhantomData<MetaSpecItemXref>,
                                            lifetime: _serde::export::PhantomData<&'de ()>,
                                        }
                                        impl<'de> _serde::Deserialize<'de> for __DeserializeWith<'de> {
                                            fn deserialize<__D>(
                                                __deserializer: __D,
                                            ) -> _serde::export::Result<Self, __D::Error>
                                            where
                                                __D: _serde::Deserializer<'de>,
                                            {
                                                _serde :: export :: Ok ( __DeserializeWith { value : match :: schemafy_core :: one_or_many :: deserialize ( __deserializer ) { _serde :: export :: Ok ( __val ) => __val , _serde :: export :: Err ( __err ) => { return _serde :: export :: Err ( __err ) ; } } , phantom : _serde :: export :: PhantomData , lifetime : _serde :: export :: PhantomData , } )
                                            }
                                        }
                                        match _serde::de::MapAccess::next_value::<
                                            __DeserializeWith<'de>,
                                        >(&mut __map)
                                        {
                                            _serde::export::Ok(__wrapper) => __wrapper.value,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        }
                                    });
                                }
                                __Field::__field1 => {
                                    if _serde::export::Option::is_some(&__field1) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "iso",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::export::Some({
                                        struct __DeserializeWith<'de> {
                                            value: Vec<IsoIdentifier>,
                                            phantom: _serde::export::PhantomData<MetaSpecItemXref>,
                                            lifetime: _serde::export::PhantomData<&'de ()>,
                                        }
                                        impl<'de> _serde::Deserialize<'de> for __DeserializeWith<'de> {
                                            fn deserialize<__D>(
                                                __deserializer: __D,
                                            ) -> _serde::export::Result<Self, __D::Error>
                                            where
                                                __D: _serde::Deserializer<'de>,
                                            {
                                                _serde :: export :: Ok ( __DeserializeWith { value : match :: schemafy_core :: one_or_many :: deserialize ( __deserializer ) { _serde :: export :: Ok ( __val ) => __val , _serde :: export :: Err ( __err ) => { return _serde :: export :: Err ( __err ) ; } } , phantom : _serde :: export :: PhantomData , lifetime : _serde :: export :: PhantomData , } )
                                            }
                                        }
                                        match _serde::de::MapAccess::next_value::<
                                            __DeserializeWith<'de>,
                                        >(&mut __map)
                                        {
                                            _serde::export::Ok(__wrapper) => __wrapper.value,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        }
                                    });
                                }
                                __Field::__field2 => {
                                    if _serde::export::Option::is_some(&__field2) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "justsolve",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::export::Some({
                                        struct __DeserializeWith<'de> {
                                            value: Vec<MediaWikiPageName>,
                                            phantom: _serde::export::PhantomData<MetaSpecItemXref>,
                                            lifetime: _serde::export::PhantomData<&'de ()>,
                                        }
                                        impl<'de> _serde::Deserialize<'de> for __DeserializeWith<'de> {
                                            fn deserialize<__D>(
                                                __deserializer: __D,
                                            ) -> _serde::export::Result<Self, __D::Error>
                                            where
                                                __D: _serde::Deserializer<'de>,
                                            {
                                                _serde :: export :: Ok ( __DeserializeWith { value : match :: schemafy_core :: one_or_many :: deserialize ( __deserializer ) { _serde :: export :: Ok ( __val ) => __val , _serde :: export :: Err ( __err ) => { return _serde :: export :: Err ( __err ) ; } } , phantom : _serde :: export :: PhantomData , lifetime : _serde :: export :: PhantomData , } )
                                            }
                                        }
                                        match _serde::de::MapAccess::next_value::<
                                            __DeserializeWith<'de>,
                                        >(&mut __map)
                                        {
                                            _serde::export::Ok(__wrapper) => __wrapper.value,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        }
                                    });
                                }
                                __Field::__field3 => {
                                    if _serde::export::Option::is_some(&__field3) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "loc",
                                            ),
                                        );
                                    }
                                    __field3 = _serde::export::Some({
                                        struct __DeserializeWith<'de> {
                                            value: Vec<LocIdentifier>,
                                            phantom: _serde::export::PhantomData<MetaSpecItemXref>,
                                            lifetime: _serde::export::PhantomData<&'de ()>,
                                        }
                                        impl<'de> _serde::Deserialize<'de> for __DeserializeWith<'de> {
                                            fn deserialize<__D>(
                                                __deserializer: __D,
                                            ) -> _serde::export::Result<Self, __D::Error>
                                            where
                                                __D: _serde::Deserializer<'de>,
                                            {
                                                _serde :: export :: Ok ( __DeserializeWith { value : match :: schemafy_core :: one_or_many :: deserialize ( __deserializer ) { _serde :: export :: Ok ( __val ) => __val , _serde :: export :: Err ( __err ) => { return _serde :: export :: Err ( __err ) ; } } , phantom : _serde :: export :: PhantomData , lifetime : _serde :: export :: PhantomData , } )
                                            }
                                        }
                                        match _serde::de::MapAccess::next_value::<
                                            __DeserializeWith<'de>,
                                        >(&mut __map)
                                        {
                                            _serde::export::Ok(__wrapper) => __wrapper.value,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        }
                                    });
                                }
                                __Field::__field4 => {
                                    if _serde::export::Option::is_some(&__field4) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "mime",
                                            ),
                                        );
                                    }
                                    __field4 = _serde::export::Some({
                                        struct __DeserializeWith<'de> {
                                            value: Vec<MimeType>,
                                            phantom: _serde::export::PhantomData<MetaSpecItemXref>,
                                            lifetime: _serde::export::PhantomData<&'de ()>,
                                        }
                                        impl<'de> _serde::Deserialize<'de> for __DeserializeWith<'de> {
                                            fn deserialize<__D>(
                                                __deserializer: __D,
                                            ) -> _serde::export::Result<Self, __D::Error>
                                            where
                                                __D: _serde::Deserializer<'de>,
                                            {
                                                _serde :: export :: Ok ( __DeserializeWith { value : match :: schemafy_core :: one_or_many :: deserialize ( __deserializer ) { _serde :: export :: Ok ( __val ) => __val , _serde :: export :: Err ( __err ) => { return _serde :: export :: Err ( __err ) ; } } , phantom : _serde :: export :: PhantomData , lifetime : _serde :: export :: PhantomData , } )
                                            }
                                        }
                                        match _serde::de::MapAccess::next_value::<
                                            __DeserializeWith<'de>,
                                        >(&mut __map)
                                        {
                                            _serde::export::Ok(__wrapper) => __wrapper.value,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        }
                                    });
                                }
                                __Field::__field5 => {
                                    if _serde::export::Option::is_some(&__field5) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "pronom",
                                            ),
                                        );
                                    }
                                    __field5 = _serde::export::Some({
                                        struct __DeserializeWith<'de> {
                                            value: Vec<PronomIdentifier>,
                                            phantom: _serde::export::PhantomData<MetaSpecItemXref>,
                                            lifetime: _serde::export::PhantomData<&'de ()>,
                                        }
                                        impl<'de> _serde::Deserialize<'de> for __DeserializeWith<'de> {
                                            fn deserialize<__D>(
                                                __deserializer: __D,
                                            ) -> _serde::export::Result<Self, __D::Error>
                                            where
                                                __D: _serde::Deserializer<'de>,
                                            {
                                                _serde :: export :: Ok ( __DeserializeWith { value : match :: schemafy_core :: one_or_many :: deserialize ( __deserializer ) { _serde :: export :: Ok ( __val ) => __val , _serde :: export :: Err ( __err ) => { return _serde :: export :: Err ( __err ) ; } } , phantom : _serde :: export :: PhantomData , lifetime : _serde :: export :: PhantomData , } )
                                            }
                                        }
                                        match _serde::de::MapAccess::next_value::<
                                            __DeserializeWith<'de>,
                                        >(&mut __map)
                                        {
                                            _serde::export::Ok(__wrapper) => __wrapper.value,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        }
                                    });
                                }
                                __Field::__field6 => {
                                    if _serde::export::Option::is_some(&__field6) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "rfc",
                                            ),
                                        );
                                    }
                                    __field6 = _serde::export::Some({
                                        struct __DeserializeWith<'de> {
                                            value: Vec<RfcIdentifier>,
                                            phantom: _serde::export::PhantomData<MetaSpecItemXref>,
                                            lifetime: _serde::export::PhantomData<&'de ()>,
                                        }
                                        impl<'de> _serde::Deserialize<'de> for __DeserializeWith<'de> {
                                            fn deserialize<__D>(
                                                __deserializer: __D,
                                            ) -> _serde::export::Result<Self, __D::Error>
                                            where
                                                __D: _serde::Deserializer<'de>,
                                            {
                                                _serde :: export :: Ok ( __DeserializeWith { value : match :: schemafy_core :: one_or_many :: deserialize ( __deserializer ) { _serde :: export :: Ok ( __val ) => __val , _serde :: export :: Err ( __err ) => { return _serde :: export :: Err ( __err ) ; } } , phantom : _serde :: export :: PhantomData , lifetime : _serde :: export :: PhantomData , } )
                                            }
                                        }
                                        match _serde::de::MapAccess::next_value::<
                                            __DeserializeWith<'de>,
                                        >(&mut __map)
                                        {
                                            _serde::export::Ok(__wrapper) => __wrapper.value,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        }
                                    });
                                }
                                __Field::__field7 => {
                                    if _serde::export::Option::is_some(&__field7) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "wikidata",
                                            ),
                                        );
                                    }
                                    __field7 = _serde::export::Some({
                                        struct __DeserializeWith<'de> {
                                            value: Vec<WikidataIdentifier>,
                                            phantom: _serde::export::PhantomData<MetaSpecItemXref>,
                                            lifetime: _serde::export::PhantomData<&'de ()>,
                                        }
                                        impl<'de> _serde::Deserialize<'de> for __DeserializeWith<'de> {
                                            fn deserialize<__D>(
                                                __deserializer: __D,
                                            ) -> _serde::export::Result<Self, __D::Error>
                                            where
                                                __D: _serde::Deserializer<'de>,
                                            {
                                                _serde :: export :: Ok ( __DeserializeWith { value : match :: schemafy_core :: one_or_many :: deserialize ( __deserializer ) { _serde :: export :: Ok ( __val ) => __val , _serde :: export :: Err ( __err ) => { return _serde :: export :: Err ( __err ) ; } } , phantom : _serde :: export :: PhantomData , lifetime : _serde :: export :: PhantomData , } )
                                            }
                                        }
                                        match _serde::de::MapAccess::next_value::<
                                            __DeserializeWith<'de>,
                                        >(&mut __map)
                                        {
                                            _serde::export::Ok(__wrapper) => __wrapper.value,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        }
                                    });
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::export::Some(__field0) => __field0,
                            _serde::export::None => _serde::export::Default::default(),
                        };
                        let __field1 = match __field1 {
                            _serde::export::Some(__field1) => __field1,
                            _serde::export::None => _serde::export::Default::default(),
                        };
                        let __field2 = match __field2 {
                            _serde::export::Some(__field2) => __field2,
                            _serde::export::None => _serde::export::Default::default(),
                        };
                        let __field3 = match __field3 {
                            _serde::export::Some(__field3) => __field3,
                            _serde::export::None => _serde::export::Default::default(),
                        };
                        let __field4 = match __field4 {
                            _serde::export::Some(__field4) => __field4,
                            _serde::export::None => _serde::export::Default::default(),
                        };
                        let __field5 = match __field5 {
                            _serde::export::Some(__field5) => __field5,
                            _serde::export::None => _serde::export::Default::default(),
                        };
                        let __field6 = match __field6 {
                            _serde::export::Some(__field6) => __field6,
                            _serde::export::None => _serde::export::Default::default(),
                        };
                        let __field7 = match __field7 {
                            _serde::export::Some(__field7) => __field7,
                            _serde::export::None => _serde::export::Default::default(),
                        };
                        _serde::export::Ok(MetaSpecItemXref {
                            forensicswiki: __field0,
                            iso: __field1,
                            justsolve: __field2,
                            loc: __field3,
                            mime: __field4,
                            pronom: __field5,
                            rfc: __field6,
                            wikidata: __field7,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] = &[
                    "forensicswiki",
                    "iso",
                    "justsolve",
                    "loc",
                    "mime",
                    "pronom",
                    "rfc",
                    "wikidata",
                ];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "MetaSpecItemXref",
                    FIELDS,
                    __Visitor {
                        marker: _serde::export::PhantomData::<MetaSpecItemXref>,
                        lifetime: _serde::export::PhantomData,
                    },
                )
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_SERIALIZE_FOR_MetaSpecItemXref: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for MetaSpecItemXref {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::export::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "MetaSpecItemXref",
                    false as usize + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "forensicswiki",
                    {
                        struct __SerializeWith<'__a> {
                            values: (&'__a Vec<MediaWikiPageName>,),
                            phantom: _serde::export::PhantomData<MetaSpecItemXref>,
                        }
                        impl<'__a> _serde::Serialize for __SerializeWith<'__a> {
                            fn serialize<__S>(
                                &self,
                                __s: __S,
                            ) -> _serde::export::Result<__S::Ok, __S::Error>
                            where
                                __S: _serde::Serializer,
                            {
                                ::schemafy_core::one_or_many::serialize(self.values.0, __s)
                            }
                        }
                        &__SerializeWith {
                            values: (&self.forensicswiki,),
                            phantom: _serde::export::PhantomData::<MetaSpecItemXref>,
                        }
                    },
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state, "iso", {
                    struct __SerializeWith<'__a> {
                        values: (&'__a Vec<IsoIdentifier>,),
                        phantom: _serde::export::PhantomData<MetaSpecItemXref>,
                    }
                    impl<'__a> _serde::Serialize for __SerializeWith<'__a> {
                        fn serialize<__S>(
                            &self,
                            __s: __S,
                        ) -> _serde::export::Result<__S::Ok, __S::Error>
                        where
                            __S: _serde::Serializer,
                        {
                            ::schemafy_core::one_or_many::serialize(self.values.0, __s)
                        }
                    }
                    &__SerializeWith {
                        values: (&self.iso,),
                        phantom: _serde::export::PhantomData::<MetaSpecItemXref>,
                    }
                }) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "justsolve",
                    {
                        struct __SerializeWith<'__a> {
                            values: (&'__a Vec<MediaWikiPageName>,),
                            phantom: _serde::export::PhantomData<MetaSpecItemXref>,
                        }
                        impl<'__a> _serde::Serialize for __SerializeWith<'__a> {
                            fn serialize<__S>(
                                &self,
                                __s: __S,
                            ) -> _serde::export::Result<__S::Ok, __S::Error>
                            where
                                __S: _serde::Serializer,
                            {
                                ::schemafy_core::one_or_many::serialize(self.values.0, __s)
                            }
                        }
                        &__SerializeWith {
                            values: (&self.justsolve,),
                            phantom: _serde::export::PhantomData::<MetaSpecItemXref>,
                        }
                    },
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state, "loc", {
                    struct __SerializeWith<'__a> {
                        values: (&'__a Vec<LocIdentifier>,),
                        phantom: _serde::export::PhantomData<MetaSpecItemXref>,
                    }
                    impl<'__a> _serde::Serialize for __SerializeWith<'__a> {
                        fn serialize<__S>(
                            &self,
                            __s: __S,
                        ) -> _serde::export::Result<__S::Ok, __S::Error>
                        where
                            __S: _serde::Serializer,
                        {
                            ::schemafy_core::one_or_many::serialize(self.values.0, __s)
                        }
                    }
                    &__SerializeWith {
                        values: (&self.loc,),
                        phantom: _serde::export::PhantomData::<MetaSpecItemXref>,
                    }
                }) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state, "mime", {
                    struct __SerializeWith<'__a> {
                        values: (&'__a Vec<MimeType>,),
                        phantom: _serde::export::PhantomData<MetaSpecItemXref>,
                    }
                    impl<'__a> _serde::Serialize for __SerializeWith<'__a> {
                        fn serialize<__S>(
                            &self,
                            __s: __S,
                        ) -> _serde::export::Result<__S::Ok, __S::Error>
                        where
                            __S: _serde::Serializer,
                        {
                            ::schemafy_core::one_or_many::serialize(self.values.0, __s)
                        }
                    }
                    &__SerializeWith {
                        values: (&self.mime,),
                        phantom: _serde::export::PhantomData::<MetaSpecItemXref>,
                    }
                }) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state, "pronom", {
                    struct __SerializeWith<'__a> {
                        values: (&'__a Vec<PronomIdentifier>,),
                        phantom: _serde::export::PhantomData<MetaSpecItemXref>,
                    }
                    impl<'__a> _serde::Serialize for __SerializeWith<'__a> {
                        fn serialize<__S>(
                            &self,
                            __s: __S,
                        ) -> _serde::export::Result<__S::Ok, __S::Error>
                        where
                            __S: _serde::Serializer,
                        {
                            ::schemafy_core::one_or_many::serialize(self.values.0, __s)
                        }
                    }
                    &__SerializeWith {
                        values: (&self.pronom,),
                        phantom: _serde::export::PhantomData::<MetaSpecItemXref>,
                    }
                }) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state, "rfc", {
                    struct __SerializeWith<'__a> {
                        values: (&'__a Vec<RfcIdentifier>,),
                        phantom: _serde::export::PhantomData<MetaSpecItemXref>,
                    }
                    impl<'__a> _serde::Serialize for __SerializeWith<'__a> {
                        fn serialize<__S>(
                            &self,
                            __s: __S,
                        ) -> _serde::export::Result<__S::Ok, __S::Error>
                        where
                            __S: _serde::Serializer,
                        {
                            ::schemafy_core::one_or_many::serialize(self.values.0, __s)
                        }
                    }
                    &__SerializeWith {
                        values: (&self.rfc,),
                        phantom: _serde::export::PhantomData::<MetaSpecItemXref>,
                    }
                }) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "wikidata",
                    {
                        struct __SerializeWith<'__a> {
                            values: (&'__a Vec<WikidataIdentifier>,),
                            phantom: _serde::export::PhantomData<MetaSpecItemXref>,
                        }
                        impl<'__a> _serde::Serialize for __SerializeWith<'__a> {
                            fn serialize<__S>(
                                &self,
                                __s: __S,
                            ) -> _serde::export::Result<__S::Ok, __S::Error>
                            where
                                __S: _serde::Serializer,
                            {
                                ::schemafy_core::one_or_many::serialize(self.values.0, __s)
                            }
                        }
                        &__SerializeWith {
                            values: (&self.wikidata,),
                            phantom: _serde::export::PhantomData::<MetaSpecItemXref>,
                        }
                    },
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    pub struct MetaSpec {
        #[serde(default)]
        #[serde(with = "::schemafy_core::one_or_many")]
        pub application: Vec<String>,
        pub encoding: Option<String>,
        pub endian: Option<serde_json::Value>,
        #[serde(default)]
        #[serde(with = "::schemafy_core::one_or_many")]
        #[serde(rename = "file-extension")]
        pub file_extension: Vec<String>,
        pub id: Option<Identifier>,
        pub imports: Option<Vec<String>>,
        /// advise the Kaitai Struct Compiler (KSC) to use debug mode
        #[serde(rename = "ks-debug")]
        pub ks_debug: Option<bool>,
        /// advise the Kaitai Struct Compiler (KSC) to ignore missing types in the .ksy file, and
        /// assume that these types are already provided externally by the environment the classes are
        /// generated for
        #[serde(rename = "ks-opaque-types")]
        pub ks_opaque_types: Option<bool>,
        #[serde(rename = "ks-version")]
        pub ks_version: Option<serde_json::Value>,
        pub license: Option<String>,
        pub title: Option<String>,
        pub xref: Option<MetaSpecItemXref>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for MetaSpec {
        #[inline]
        fn clone(&self) -> MetaSpec {
            match *self {
                MetaSpec {
                    application: ref __self_0_0,
                    encoding: ref __self_0_1,
                    endian: ref __self_0_2,
                    file_extension: ref __self_0_3,
                    id: ref __self_0_4,
                    imports: ref __self_0_5,
                    ks_debug: ref __self_0_6,
                    ks_opaque_types: ref __self_0_7,
                    ks_version: ref __self_0_8,
                    license: ref __self_0_9,
                    title: ref __self_0_10,
                    xref: ref __self_0_11,
                } => MetaSpec {
                    application: ::core::clone::Clone::clone(&(*__self_0_0)),
                    encoding: ::core::clone::Clone::clone(&(*__self_0_1)),
                    endian: ::core::clone::Clone::clone(&(*__self_0_2)),
                    file_extension: ::core::clone::Clone::clone(&(*__self_0_3)),
                    id: ::core::clone::Clone::clone(&(*__self_0_4)),
                    imports: ::core::clone::Clone::clone(&(*__self_0_5)),
                    ks_debug: ::core::clone::Clone::clone(&(*__self_0_6)),
                    ks_opaque_types: ::core::clone::Clone::clone(&(*__self_0_7)),
                    ks_version: ::core::clone::Clone::clone(&(*__self_0_8)),
                    license: ::core::clone::Clone::clone(&(*__self_0_9)),
                    title: ::core::clone::Clone::clone(&(*__self_0_10)),
                    xref: ::core::clone::Clone::clone(&(*__self_0_11)),
                },
            }
        }
    }
    impl ::core::marker::StructuralPartialEq for MetaSpec {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for MetaSpec {
        #[inline]
        fn eq(&self, other: &MetaSpec) -> bool {
            match *other {
                MetaSpec {
                    application: ref __self_1_0,
                    encoding: ref __self_1_1,
                    endian: ref __self_1_2,
                    file_extension: ref __self_1_3,
                    id: ref __self_1_4,
                    imports: ref __self_1_5,
                    ks_debug: ref __self_1_6,
                    ks_opaque_types: ref __self_1_7,
                    ks_version: ref __self_1_8,
                    license: ref __self_1_9,
                    title: ref __self_1_10,
                    xref: ref __self_1_11,
                } => match *self {
                    MetaSpec {
                        application: ref __self_0_0,
                        encoding: ref __self_0_1,
                        endian: ref __self_0_2,
                        file_extension: ref __self_0_3,
                        id: ref __self_0_4,
                        imports: ref __self_0_5,
                        ks_debug: ref __self_0_6,
                        ks_opaque_types: ref __self_0_7,
                        ks_version: ref __self_0_8,
                        license: ref __self_0_9,
                        title: ref __self_0_10,
                        xref: ref __self_0_11,
                    } => {
                        (*__self_0_0) == (*__self_1_0)
                            && (*__self_0_1) == (*__self_1_1)
                            && (*__self_0_2) == (*__self_1_2)
                            && (*__self_0_3) == (*__self_1_3)
                            && (*__self_0_4) == (*__self_1_4)
                            && (*__self_0_5) == (*__self_1_5)
                            && (*__self_0_6) == (*__self_1_6)
                            && (*__self_0_7) == (*__self_1_7)
                            && (*__self_0_8) == (*__self_1_8)
                            && (*__self_0_9) == (*__self_1_9)
                            && (*__self_0_10) == (*__self_1_10)
                            && (*__self_0_11) == (*__self_1_11)
                    }
                },
            }
        }
        #[inline]
        fn ne(&self, other: &MetaSpec) -> bool {
            match *other {
                MetaSpec {
                    application: ref __self_1_0,
                    encoding: ref __self_1_1,
                    endian: ref __self_1_2,
                    file_extension: ref __self_1_3,
                    id: ref __self_1_4,
                    imports: ref __self_1_5,
                    ks_debug: ref __self_1_6,
                    ks_opaque_types: ref __self_1_7,
                    ks_version: ref __self_1_8,
                    license: ref __self_1_9,
                    title: ref __self_1_10,
                    xref: ref __self_1_11,
                } => match *self {
                    MetaSpec {
                        application: ref __self_0_0,
                        encoding: ref __self_0_1,
                        endian: ref __self_0_2,
                        file_extension: ref __self_0_3,
                        id: ref __self_0_4,
                        imports: ref __self_0_5,
                        ks_debug: ref __self_0_6,
                        ks_opaque_types: ref __self_0_7,
                        ks_version: ref __self_0_8,
                        license: ref __self_0_9,
                        title: ref __self_0_10,
                        xref: ref __self_0_11,
                    } => {
                        (*__self_0_0) != (*__self_1_0)
                            || (*__self_0_1) != (*__self_1_1)
                            || (*__self_0_2) != (*__self_1_2)
                            || (*__self_0_3) != (*__self_1_3)
                            || (*__self_0_4) != (*__self_1_4)
                            || (*__self_0_5) != (*__self_1_5)
                            || (*__self_0_6) != (*__self_1_6)
                            || (*__self_0_7) != (*__self_1_7)
                            || (*__self_0_8) != (*__self_1_8)
                            || (*__self_0_9) != (*__self_1_9)
                            || (*__self_0_10) != (*__self_1_10)
                            || (*__self_0_11) != (*__self_1_11)
                    }
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for MetaSpec {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                MetaSpec {
                    application: ref __self_0_0,
                    encoding: ref __self_0_1,
                    endian: ref __self_0_2,
                    file_extension: ref __self_0_3,
                    id: ref __self_0_4,
                    imports: ref __self_0_5,
                    ks_debug: ref __self_0_6,
                    ks_opaque_types: ref __self_0_7,
                    ks_version: ref __self_0_8,
                    license: ref __self_0_9,
                    title: ref __self_0_10,
                    xref: ref __self_0_11,
                } => {
                    let mut debug_trait_builder = f.debug_struct("MetaSpec");
                    let _ = debug_trait_builder.field("application", &&(*__self_0_0));
                    let _ = debug_trait_builder.field("encoding", &&(*__self_0_1));
                    let _ = debug_trait_builder.field("endian", &&(*__self_0_2));
                    let _ = debug_trait_builder.field("file_extension", &&(*__self_0_3));
                    let _ = debug_trait_builder.field("id", &&(*__self_0_4));
                    let _ = debug_trait_builder.field("imports", &&(*__self_0_5));
                    let _ = debug_trait_builder.field("ks_debug", &&(*__self_0_6));
                    let _ = debug_trait_builder.field("ks_opaque_types", &&(*__self_0_7));
                    let _ = debug_trait_builder.field("ks_version", &&(*__self_0_8));
                    let _ = debug_trait_builder.field("license", &&(*__self_0_9));
                    let _ = debug_trait_builder.field("title", &&(*__self_0_10));
                    let _ = debug_trait_builder.field("xref", &&(*__self_0_11));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DESERIALIZE_FOR_MetaSpec: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for MetaSpec {
            fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                    __field4,
                    __field5,
                    __field6,
                    __field7,
                    __field8,
                    __field9,
                    __field10,
                    __field11,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::export::Ok(__Field::__field0),
                            1u64 => _serde::export::Ok(__Field::__field1),
                            2u64 => _serde::export::Ok(__Field::__field2),
                            3u64 => _serde::export::Ok(__Field::__field3),
                            4u64 => _serde::export::Ok(__Field::__field4),
                            5u64 => _serde::export::Ok(__Field::__field5),
                            6u64 => _serde::export::Ok(__Field::__field6),
                            7u64 => _serde::export::Ok(__Field::__field7),
                            8u64 => _serde::export::Ok(__Field::__field8),
                            9u64 => _serde::export::Ok(__Field::__field9),
                            10u64 => _serde::export::Ok(__Field::__field10),
                            11u64 => _serde::export::Ok(__Field::__field11),
                            _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"field index 0 <= i < 12",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "application" => _serde::export::Ok(__Field::__field0),
                            "encoding" => _serde::export::Ok(__Field::__field1),
                            "endian" => _serde::export::Ok(__Field::__field2),
                            "file-extension" => _serde::export::Ok(__Field::__field3),
                            "id" => _serde::export::Ok(__Field::__field4),
                            "imports" => _serde::export::Ok(__Field::__field5),
                            "ks-debug" => _serde::export::Ok(__Field::__field6),
                            "ks-opaque-types" => _serde::export::Ok(__Field::__field7),
                            "ks-version" => _serde::export::Ok(__Field::__field8),
                            "license" => _serde::export::Ok(__Field::__field9),
                            "title" => _serde::export::Ok(__Field::__field10),
                            "xref" => _serde::export::Ok(__Field::__field11),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"application" => _serde::export::Ok(__Field::__field0),
                            b"encoding" => _serde::export::Ok(__Field::__field1),
                            b"endian" => _serde::export::Ok(__Field::__field2),
                            b"file-extension" => _serde::export::Ok(__Field::__field3),
                            b"id" => _serde::export::Ok(__Field::__field4),
                            b"imports" => _serde::export::Ok(__Field::__field5),
                            b"ks-debug" => _serde::export::Ok(__Field::__field6),
                            b"ks-opaque-types" => _serde::export::Ok(__Field::__field7),
                            b"ks-version" => _serde::export::Ok(__Field::__field8),
                            b"license" => _serde::export::Ok(__Field::__field9),
                            b"title" => _serde::export::Ok(__Field::__field10),
                            b"xref" => _serde::export::Ok(__Field::__field11),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::export::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::export::PhantomData<MetaSpec>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = MetaSpec;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "struct MetaSpec")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match {
                            struct __DeserializeWith<'de> {
                                value: Vec<String>,
                                phantom: _serde::export::PhantomData<MetaSpec>,
                                lifetime: _serde::export::PhantomData<&'de ()>,
                            }
                            impl<'de> _serde::Deserialize<'de> for __DeserializeWith<'de> {
                                fn deserialize<__D>(
                                    __deserializer: __D,
                                ) -> _serde::export::Result<Self, __D::Error>
                                where
                                    __D: _serde::Deserializer<'de>,
                                {
                                    _serde::export::Ok(__DeserializeWith {
                                        value: match ::schemafy_core::one_or_many::deserialize(
                                            __deserializer,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                        phantom: _serde::export::PhantomData,
                                        lifetime: _serde::export::PhantomData,
                                    })
                                }
                            }
                            _serde::export::Option::map(
                                match _serde::de::SeqAccess::next_element::<__DeserializeWith<'de>>(
                                    &mut __seq,
                                ) {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                },
                                |__wrap| __wrap.value,
                            )
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => _serde::export::Default::default(),
                        };
                        let __field1 = match match _serde::de::SeqAccess::next_element::<
                            Option<String>,
                        >(&mut __seq)
                        {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct MetaSpec with 12 elements",
                                ));
                            }
                        };
                        let __field2 = match match _serde::de::SeqAccess::next_element::<
                            Option<serde_json::Value>,
                        >(&mut __seq)
                        {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    2usize,
                                    &"struct MetaSpec with 12 elements",
                                ));
                            }
                        };
                        let __field3 = match {
                            struct __DeserializeWith<'de> {
                                value: Vec<String>,
                                phantom: _serde::export::PhantomData<MetaSpec>,
                                lifetime: _serde::export::PhantomData<&'de ()>,
                            }
                            impl<'de> _serde::Deserialize<'de> for __DeserializeWith<'de> {
                                fn deserialize<__D>(
                                    __deserializer: __D,
                                ) -> _serde::export::Result<Self, __D::Error>
                                where
                                    __D: _serde::Deserializer<'de>,
                                {
                                    _serde::export::Ok(__DeserializeWith {
                                        value: match ::schemafy_core::one_or_many::deserialize(
                                            __deserializer,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                        phantom: _serde::export::PhantomData,
                                        lifetime: _serde::export::PhantomData,
                                    })
                                }
                            }
                            _serde::export::Option::map(
                                match _serde::de::SeqAccess::next_element::<__DeserializeWith<'de>>(
                                    &mut __seq,
                                ) {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                },
                                |__wrap| __wrap.value,
                            )
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => _serde::export::Default::default(),
                        };
                        let __field4 = match match _serde::de::SeqAccess::next_element::<
                            Option<Identifier>,
                        >(&mut __seq)
                        {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    4usize,
                                    &"struct MetaSpec with 12 elements",
                                ));
                            }
                        };
                        let __field5 = match match _serde::de::SeqAccess::next_element::<
                            Option<Vec<String>>,
                        >(&mut __seq)
                        {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    5usize,
                                    &"struct MetaSpec with 12 elements",
                                ));
                            }
                        };
                        let __field6 = match match _serde::de::SeqAccess::next_element::<Option<bool>>(
                            &mut __seq,
                        ) {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    6usize,
                                    &"struct MetaSpec with 12 elements",
                                ));
                            }
                        };
                        let __field7 = match match _serde::de::SeqAccess::next_element::<Option<bool>>(
                            &mut __seq,
                        ) {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    7usize,
                                    &"struct MetaSpec with 12 elements",
                                ));
                            }
                        };
                        let __field8 = match match _serde::de::SeqAccess::next_element::<
                            Option<serde_json::Value>,
                        >(&mut __seq)
                        {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    8usize,
                                    &"struct MetaSpec with 12 elements",
                                ));
                            }
                        };
                        let __field9 = match match _serde::de::SeqAccess::next_element::<
                            Option<String>,
                        >(&mut __seq)
                        {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    9usize,
                                    &"struct MetaSpec with 12 elements",
                                ));
                            }
                        };
                        let __field10 = match match _serde::de::SeqAccess::next_element::<
                            Option<String>,
                        >(&mut __seq)
                        {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    10usize,
                                    &"struct MetaSpec with 12 elements",
                                ));
                            }
                        };
                        let __field11 = match match _serde::de::SeqAccess::next_element::<
                            Option<MetaSpecItemXref>,
                        >(&mut __seq)
                        {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    11usize,
                                    &"struct MetaSpec with 12 elements",
                                ));
                            }
                        };
                        _serde::export::Ok(MetaSpec {
                            application: __field0,
                            encoding: __field1,
                            endian: __field2,
                            file_extension: __field3,
                            id: __field4,
                            imports: __field5,
                            ks_debug: __field6,
                            ks_opaque_types: __field7,
                            ks_version: __field8,
                            license: __field9,
                            title: __field10,
                            xref: __field11,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::export::Option<Vec<String>> =
                            _serde::export::None;
                        let mut __field1: _serde::export::Option<Option<String>> =
                            _serde::export::None;
                        let mut __field2: _serde::export::Option<Option<serde_json::Value>> =
                            _serde::export::None;
                        let mut __field3: _serde::export::Option<Vec<String>> =
                            _serde::export::None;
                        let mut __field4: _serde::export::Option<Option<Identifier>> =
                            _serde::export::None;
                        let mut __field5: _serde::export::Option<Option<Vec<String>>> =
                            _serde::export::None;
                        let mut __field6: _serde::export::Option<Option<bool>> =
                            _serde::export::None;
                        let mut __field7: _serde::export::Option<Option<bool>> =
                            _serde::export::None;
                        let mut __field8: _serde::export::Option<Option<serde_json::Value>> =
                            _serde::export::None;
                        let mut __field9: _serde::export::Option<Option<String>> =
                            _serde::export::None;
                        let mut __field10: _serde::export::Option<Option<String>> =
                            _serde::export::None;
                        let mut __field11: _serde::export::Option<Option<MetaSpecItemXref>> =
                            _serde::export::None;
                        while let _serde::export::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::export::Option::is_some(&__field0) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "application",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::export::Some({
                                        struct __DeserializeWith<'de> {
                                            value: Vec<String>,
                                            phantom: _serde::export::PhantomData<MetaSpec>,
                                            lifetime: _serde::export::PhantomData<&'de ()>,
                                        }
                                        impl<'de> _serde::Deserialize<'de> for __DeserializeWith<'de> {
                                            fn deserialize<__D>(
                                                __deserializer: __D,
                                            ) -> _serde::export::Result<Self, __D::Error>
                                            where
                                                __D: _serde::Deserializer<'de>,
                                            {
                                                _serde :: export :: Ok ( __DeserializeWith { value : match :: schemafy_core :: one_or_many :: deserialize ( __deserializer ) { _serde :: export :: Ok ( __val ) => __val , _serde :: export :: Err ( __err ) => { return _serde :: export :: Err ( __err ) ; } } , phantom : _serde :: export :: PhantomData , lifetime : _serde :: export :: PhantomData , } )
                                            }
                                        }
                                        match _serde::de::MapAccess::next_value::<
                                            __DeserializeWith<'de>,
                                        >(&mut __map)
                                        {
                                            _serde::export::Ok(__wrapper) => __wrapper.value,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        }
                                    });
                                }
                                __Field::__field1 => {
                                    if _serde::export::Option::is_some(&__field1) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "encoding",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<Option<String>>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::export::Option::is_some(&__field2) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "endian",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            Option<serde_json::Value>,
                                        >(&mut __map)
                                        {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field3 => {
                                    if _serde::export::Option::is_some(&__field3) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "file-extension",
                                            ),
                                        );
                                    }
                                    __field3 = _serde::export::Some({
                                        struct __DeserializeWith<'de> {
                                            value: Vec<String>,
                                            phantom: _serde::export::PhantomData<MetaSpec>,
                                            lifetime: _serde::export::PhantomData<&'de ()>,
                                        }
                                        impl<'de> _serde::Deserialize<'de> for __DeserializeWith<'de> {
                                            fn deserialize<__D>(
                                                __deserializer: __D,
                                            ) -> _serde::export::Result<Self, __D::Error>
                                            where
                                                __D: _serde::Deserializer<'de>,
                                            {
                                                _serde :: export :: Ok ( __DeserializeWith { value : match :: schemafy_core :: one_or_many :: deserialize ( __deserializer ) { _serde :: export :: Ok ( __val ) => __val , _serde :: export :: Err ( __err ) => { return _serde :: export :: Err ( __err ) ; } } , phantom : _serde :: export :: PhantomData , lifetime : _serde :: export :: PhantomData , } )
                                            }
                                        }
                                        match _serde::de::MapAccess::next_value::<
                                            __DeserializeWith<'de>,
                                        >(&mut __map)
                                        {
                                            _serde::export::Ok(__wrapper) => __wrapper.value,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        }
                                    });
                                }
                                __Field::__field4 => {
                                    if _serde::export::Option::is_some(&__field4) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "id",
                                            ),
                                        );
                                    }
                                    __field4 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<Option<Identifier>>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field5 => {
                                    if _serde::export::Option::is_some(&__field5) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "imports",
                                            ),
                                        );
                                    }
                                    __field5 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<Option<Vec<String>>>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field6 => {
                                    if _serde::export::Option::is_some(&__field6) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "ks-debug",
                                            ),
                                        );
                                    }
                                    __field6 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<Option<bool>>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field7 => {
                                    if _serde::export::Option::is_some(&__field7) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "ks-opaque-types",
                                            ),
                                        );
                                    }
                                    __field7 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<Option<bool>>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field8 => {
                                    if _serde::export::Option::is_some(&__field8) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "ks-version",
                                            ),
                                        );
                                    }
                                    __field8 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            Option<serde_json::Value>,
                                        >(&mut __map)
                                        {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field9 => {
                                    if _serde::export::Option::is_some(&__field9) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "license",
                                            ),
                                        );
                                    }
                                    __field9 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<Option<String>>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field10 => {
                                    if _serde::export::Option::is_some(&__field10) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "title",
                                            ),
                                        );
                                    }
                                    __field10 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<Option<String>>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field11 => {
                                    if _serde::export::Option::is_some(&__field11) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "xref",
                                            ),
                                        );
                                    }
                                    __field11 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            Option<MetaSpecItemXref>,
                                        >(&mut __map)
                                        {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::export::Some(__field0) => __field0,
                            _serde::export::None => _serde::export::Default::default(),
                        };
                        let __field1 = match __field1 {
                            _serde::export::Some(__field1) => __field1,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("encoding") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::export::Some(__field2) => __field2,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("endian") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field3 = match __field3 {
                            _serde::export::Some(__field3) => __field3,
                            _serde::export::None => _serde::export::Default::default(),
                        };
                        let __field4 = match __field4 {
                            _serde::export::Some(__field4) => __field4,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("id") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field5 = match __field5 {
                            _serde::export::Some(__field5) => __field5,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("imports") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field6 = match __field6 {
                            _serde::export::Some(__field6) => __field6,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("ks-debug") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field7 = match __field7 {
                            _serde::export::Some(__field7) => __field7,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("ks-opaque-types") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field8 = match __field8 {
                            _serde::export::Some(__field8) => __field8,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("ks-version") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field9 = match __field9 {
                            _serde::export::Some(__field9) => __field9,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("license") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field10 = match __field10 {
                            _serde::export::Some(__field10) => __field10,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("title") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field11 = match __field11 {
                            _serde::export::Some(__field11) => __field11,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("xref") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::export::Ok(MetaSpec {
                            application: __field0,
                            encoding: __field1,
                            endian: __field2,
                            file_extension: __field3,
                            id: __field4,
                            imports: __field5,
                            ks_debug: __field6,
                            ks_opaque_types: __field7,
                            ks_version: __field8,
                            license: __field9,
                            title: __field10,
                            xref: __field11,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] = &[
                    "application",
                    "encoding",
                    "endian",
                    "file-extension",
                    "id",
                    "imports",
                    "ks-debug",
                    "ks-opaque-types",
                    "ks-version",
                    "license",
                    "title",
                    "xref",
                ];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "MetaSpec",
                    FIELDS,
                    __Visitor {
                        marker: _serde::export::PhantomData::<MetaSpec>,
                        lifetime: _serde::export::PhantomData,
                    },
                )
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_SERIALIZE_FOR_MetaSpec: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for MetaSpec {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::export::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "MetaSpec",
                    false as usize + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "application",
                    {
                        struct __SerializeWith<'__a> {
                            values: (&'__a Vec<String>,),
                            phantom: _serde::export::PhantomData<MetaSpec>,
                        }
                        impl<'__a> _serde::Serialize for __SerializeWith<'__a> {
                            fn serialize<__S>(
                                &self,
                                __s: __S,
                            ) -> _serde::export::Result<__S::Ok, __S::Error>
                            where
                                __S: _serde::Serializer,
                            {
                                ::schemafy_core::one_or_many::serialize(self.values.0, __s)
                            }
                        }
                        &__SerializeWith {
                            values: (&self.application,),
                            phantom: _serde::export::PhantomData::<MetaSpec>,
                        }
                    },
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "encoding",
                    &self.encoding,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "endian",
                    &self.endian,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "file-extension",
                    {
                        struct __SerializeWith<'__a> {
                            values: (&'__a Vec<String>,),
                            phantom: _serde::export::PhantomData<MetaSpec>,
                        }
                        impl<'__a> _serde::Serialize for __SerializeWith<'__a> {
                            fn serialize<__S>(
                                &self,
                                __s: __S,
                            ) -> _serde::export::Result<__S::Ok, __S::Error>
                            where
                                __S: _serde::Serializer,
                            {
                                ::schemafy_core::one_or_many::serialize(self.values.0, __s)
                            }
                        }
                        &__SerializeWith {
                            values: (&self.file_extension,),
                            phantom: _serde::export::PhantomData::<MetaSpec>,
                        }
                    },
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "id",
                    &self.id,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "imports",
                    &self.imports,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "ks-debug",
                    &self.ks_debug,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "ks-opaque-types",
                    &self.ks_opaque_types,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "ks-version",
                    &self.ks_version,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "license",
                    &self.license,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "title",
                    &self.title,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "xref",
                    &self.xref,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    pub type MimeType = String;
    pub struct ParamSpec {
        pub doc: Option<Doc>,
        #[serde(rename = "doc-ref")]
        pub doc_ref: Option<DocRef>,
        /// path to enum type, which will become the type of the parameter
        ///
        /// only integer-based enums are supported, so `type` must be an integer type (`type: uX`,
        /// `type: sX` or `type: bX`, but not `type: b1` = boolean) for this property to work
        #[serde(rename = "enum")]
        pub enum_: Option<String>,
        pub id: Identifier,
        /// specifies "pure" type of the parameter, without any serialization details (like endianness,
        /// sizes, encodings)
        ///
        /// | Value                  | Description
        /// |-
        /// | `u1`, `u2`, `u4`, `u8` | unsigned integer
        /// | `s1`, `s2`, `s4`, `s8` | signed integer
        /// | `bX`                   | bit-sized integer (if `X` != 1)
        /// | `f4`, `f8`             | floating point number
        /// | no value<br>or `bytes` | byte array
        /// | `str`                  | string
        /// | `bool` (or `b1`)       | boolean
        /// | `struct`               | arbitrary KaitaiStruct-compatible user type
        /// | `io`                   | KaitaiStream-compatible IO stream
        /// | `any`                  | allow any type (if target language supports that)
        /// | other identifier       | user-defined type, without parameters<br>a nested type can be
        /// referenced with double colon (e.g. `type: 'foo::bar'`)
        ///
        /// one can specify arrays by appending `[]` after the type identifier (e.g. `type: u2[]`,
        /// `type: 'foo::bar[]'`, `type: struct[]` etc.)
        #[serde(rename = "type")]
        pub type_: Option<String>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for ParamSpec {
        #[inline]
        fn clone(&self) -> ParamSpec {
            match *self {
                ParamSpec {
                    doc: ref __self_0_0,
                    doc_ref: ref __self_0_1,
                    enum_: ref __self_0_2,
                    id: ref __self_0_3,
                    type_: ref __self_0_4,
                } => ParamSpec {
                    doc: ::core::clone::Clone::clone(&(*__self_0_0)),
                    doc_ref: ::core::clone::Clone::clone(&(*__self_0_1)),
                    enum_: ::core::clone::Clone::clone(&(*__self_0_2)),
                    id: ::core::clone::Clone::clone(&(*__self_0_3)),
                    type_: ::core::clone::Clone::clone(&(*__self_0_4)),
                },
            }
        }
    }
    impl ::core::marker::StructuralPartialEq for ParamSpec {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for ParamSpec {
        #[inline]
        fn eq(&self, other: &ParamSpec) -> bool {
            match *other {
                ParamSpec {
                    doc: ref __self_1_0,
                    doc_ref: ref __self_1_1,
                    enum_: ref __self_1_2,
                    id: ref __self_1_3,
                    type_: ref __self_1_4,
                } => match *self {
                    ParamSpec {
                        doc: ref __self_0_0,
                        doc_ref: ref __self_0_1,
                        enum_: ref __self_0_2,
                        id: ref __self_0_3,
                        type_: ref __self_0_4,
                    } => {
                        (*__self_0_0) == (*__self_1_0)
                            && (*__self_0_1) == (*__self_1_1)
                            && (*__self_0_2) == (*__self_1_2)
                            && (*__self_0_3) == (*__self_1_3)
                            && (*__self_0_4) == (*__self_1_4)
                    }
                },
            }
        }
        #[inline]
        fn ne(&self, other: &ParamSpec) -> bool {
            match *other {
                ParamSpec {
                    doc: ref __self_1_0,
                    doc_ref: ref __self_1_1,
                    enum_: ref __self_1_2,
                    id: ref __self_1_3,
                    type_: ref __self_1_4,
                } => match *self {
                    ParamSpec {
                        doc: ref __self_0_0,
                        doc_ref: ref __self_0_1,
                        enum_: ref __self_0_2,
                        id: ref __self_0_3,
                        type_: ref __self_0_4,
                    } => {
                        (*__self_0_0) != (*__self_1_0)
                            || (*__self_0_1) != (*__self_1_1)
                            || (*__self_0_2) != (*__self_1_2)
                            || (*__self_0_3) != (*__self_1_3)
                            || (*__self_0_4) != (*__self_1_4)
                    }
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for ParamSpec {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                ParamSpec {
                    doc: ref __self_0_0,
                    doc_ref: ref __self_0_1,
                    enum_: ref __self_0_2,
                    id: ref __self_0_3,
                    type_: ref __self_0_4,
                } => {
                    let mut debug_trait_builder = f.debug_struct("ParamSpec");
                    let _ = debug_trait_builder.field("doc", &&(*__self_0_0));
                    let _ = debug_trait_builder.field("doc_ref", &&(*__self_0_1));
                    let _ = debug_trait_builder.field("enum_", &&(*__self_0_2));
                    let _ = debug_trait_builder.field("id", &&(*__self_0_3));
                    let _ = debug_trait_builder.field("type_", &&(*__self_0_4));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DESERIALIZE_FOR_ParamSpec: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for ParamSpec {
            fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                    __field4,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::export::Ok(__Field::__field0),
                            1u64 => _serde::export::Ok(__Field::__field1),
                            2u64 => _serde::export::Ok(__Field::__field2),
                            3u64 => _serde::export::Ok(__Field::__field3),
                            4u64 => _serde::export::Ok(__Field::__field4),
                            _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"field index 0 <= i < 5",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "doc" => _serde::export::Ok(__Field::__field0),
                            "doc-ref" => _serde::export::Ok(__Field::__field1),
                            "enum" => _serde::export::Ok(__Field::__field2),
                            "id" => _serde::export::Ok(__Field::__field3),
                            "type" => _serde::export::Ok(__Field::__field4),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"doc" => _serde::export::Ok(__Field::__field0),
                            b"doc-ref" => _serde::export::Ok(__Field::__field1),
                            b"enum" => _serde::export::Ok(__Field::__field2),
                            b"id" => _serde::export::Ok(__Field::__field3),
                            b"type" => _serde::export::Ok(__Field::__field4),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::export::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::export::PhantomData<ParamSpec>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = ParamSpec;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "struct ParamSpec")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match match _serde::de::SeqAccess::next_element::<Option<Doc>>(
                            &mut __seq,
                        ) {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct ParamSpec with 5 elements",
                                ));
                            }
                        };
                        let __field1 = match match _serde::de::SeqAccess::next_element::<
                            Option<DocRef>,
                        >(&mut __seq)
                        {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct ParamSpec with 5 elements",
                                ));
                            }
                        };
                        let __field2 = match match _serde::de::SeqAccess::next_element::<
                            Option<String>,
                        >(&mut __seq)
                        {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    2usize,
                                    &"struct ParamSpec with 5 elements",
                                ));
                            }
                        };
                        let __field3 = match match _serde::de::SeqAccess::next_element::<Identifier>(
                            &mut __seq,
                        ) {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    3usize,
                                    &"struct ParamSpec with 5 elements",
                                ));
                            }
                        };
                        let __field4 = match match _serde::de::SeqAccess::next_element::<
                            Option<String>,
                        >(&mut __seq)
                        {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    4usize,
                                    &"struct ParamSpec with 5 elements",
                                ));
                            }
                        };
                        _serde::export::Ok(ParamSpec {
                            doc: __field0,
                            doc_ref: __field1,
                            enum_: __field2,
                            id: __field3,
                            type_: __field4,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::export::Option<Option<Doc>> =
                            _serde::export::None;
                        let mut __field1: _serde::export::Option<Option<DocRef>> =
                            _serde::export::None;
                        let mut __field2: _serde::export::Option<Option<String>> =
                            _serde::export::None;
                        let mut __field3: _serde::export::Option<Identifier> = _serde::export::None;
                        let mut __field4: _serde::export::Option<Option<String>> =
                            _serde::export::None;
                        while let _serde::export::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::export::Option::is_some(&__field0) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "doc",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<Option<Doc>>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::export::Option::is_some(&__field1) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "doc-ref",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<Option<DocRef>>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::export::Option::is_some(&__field2) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "enum",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<Option<String>>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field3 => {
                                    if _serde::export::Option::is_some(&__field3) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "id",
                                            ),
                                        );
                                    }
                                    __field3 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<Identifier>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field4 => {
                                    if _serde::export::Option::is_some(&__field4) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "type",
                                            ),
                                        );
                                    }
                                    __field4 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<Option<String>>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::export::Some(__field0) => __field0,
                            _serde::export::None => match _serde::private::de::missing_field("doc")
                            {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            },
                        };
                        let __field1 = match __field1 {
                            _serde::export::Some(__field1) => __field1,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("doc-ref") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::export::Some(__field2) => __field2,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("enum") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field3 = match __field3 {
                            _serde::export::Some(__field3) => __field3,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("id") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field4 = match __field4 {
                            _serde::export::Some(__field4) => __field4,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("type") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::export::Ok(ParamSpec {
                            doc: __field0,
                            doc_ref: __field1,
                            enum_: __field2,
                            id: __field3,
                            type_: __field4,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] = &["doc", "doc-ref", "enum", "id", "type"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "ParamSpec",
                    FIELDS,
                    __Visitor {
                        marker: _serde::export::PhantomData::<ParamSpec>,
                        lifetime: _serde::export::PhantomData,
                    },
                )
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_SERIALIZE_FOR_ParamSpec: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for ParamSpec {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::export::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "ParamSpec",
                    false as usize + 1 + 1 + 1 + 1 + 1,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "doc",
                    &self.doc,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "doc-ref",
                    &self.doc_ref,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "enum",
                    &self.enum_,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "id",
                    &self.id,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "type",
                    &self.type_,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    pub type ParamsSpec = Vec<ParamSpec>;
    pub type PronomIdentifier = String;
    pub type RfcIdentifier = serde_json::Value;
    pub type StringOrInteger = serde_json::Value;
    pub struct TypeSpec {
        pub doc: Option<Doc>,
        #[serde(rename = "doc-ref")]
        pub doc_ref: Option<DocRef>,
        pub enums: Option<EnumsSpec>,
        pub instances: Option<InstancesSpec>,
        pub meta: Option<MetaSpec>,
        pub params: Option<ParamsSpec>,
        pub seq: Option<Attributes>,
        pub types: Option<TypesSpec>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for TypeSpec {
        #[inline]
        fn clone(&self) -> TypeSpec {
            match *self {
                TypeSpec {
                    doc: ref __self_0_0,
                    doc_ref: ref __self_0_1,
                    enums: ref __self_0_2,
                    instances: ref __self_0_3,
                    meta: ref __self_0_4,
                    params: ref __self_0_5,
                    seq: ref __self_0_6,
                    types: ref __self_0_7,
                } => TypeSpec {
                    doc: ::core::clone::Clone::clone(&(*__self_0_0)),
                    doc_ref: ::core::clone::Clone::clone(&(*__self_0_1)),
                    enums: ::core::clone::Clone::clone(&(*__self_0_2)),
                    instances: ::core::clone::Clone::clone(&(*__self_0_3)),
                    meta: ::core::clone::Clone::clone(&(*__self_0_4)),
                    params: ::core::clone::Clone::clone(&(*__self_0_5)),
                    seq: ::core::clone::Clone::clone(&(*__self_0_6)),
                    types: ::core::clone::Clone::clone(&(*__self_0_7)),
                },
            }
        }
    }
    impl ::core::marker::StructuralPartialEq for TypeSpec {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for TypeSpec {
        #[inline]
        fn eq(&self, other: &TypeSpec) -> bool {
            match *other {
                TypeSpec {
                    doc: ref __self_1_0,
                    doc_ref: ref __self_1_1,
                    enums: ref __self_1_2,
                    instances: ref __self_1_3,
                    meta: ref __self_1_4,
                    params: ref __self_1_5,
                    seq: ref __self_1_6,
                    types: ref __self_1_7,
                } => match *self {
                    TypeSpec {
                        doc: ref __self_0_0,
                        doc_ref: ref __self_0_1,
                        enums: ref __self_0_2,
                        instances: ref __self_0_3,
                        meta: ref __self_0_4,
                        params: ref __self_0_5,
                        seq: ref __self_0_6,
                        types: ref __self_0_7,
                    } => {
                        (*__self_0_0) == (*__self_1_0)
                            && (*__self_0_1) == (*__self_1_1)
                            && (*__self_0_2) == (*__self_1_2)
                            && (*__self_0_3) == (*__self_1_3)
                            && (*__self_0_4) == (*__self_1_4)
                            && (*__self_0_5) == (*__self_1_5)
                            && (*__self_0_6) == (*__self_1_6)
                            && (*__self_0_7) == (*__self_1_7)
                    }
                },
            }
        }
        #[inline]
        fn ne(&self, other: &TypeSpec) -> bool {
            match *other {
                TypeSpec {
                    doc: ref __self_1_0,
                    doc_ref: ref __self_1_1,
                    enums: ref __self_1_2,
                    instances: ref __self_1_3,
                    meta: ref __self_1_4,
                    params: ref __self_1_5,
                    seq: ref __self_1_6,
                    types: ref __self_1_7,
                } => match *self {
                    TypeSpec {
                        doc: ref __self_0_0,
                        doc_ref: ref __self_0_1,
                        enums: ref __self_0_2,
                        instances: ref __self_0_3,
                        meta: ref __self_0_4,
                        params: ref __self_0_5,
                        seq: ref __self_0_6,
                        types: ref __self_0_7,
                    } => {
                        (*__self_0_0) != (*__self_1_0)
                            || (*__self_0_1) != (*__self_1_1)
                            || (*__self_0_2) != (*__self_1_2)
                            || (*__self_0_3) != (*__self_1_3)
                            || (*__self_0_4) != (*__self_1_4)
                            || (*__self_0_5) != (*__self_1_5)
                            || (*__self_0_6) != (*__self_1_6)
                            || (*__self_0_7) != (*__self_1_7)
                    }
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for TypeSpec {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                TypeSpec {
                    doc: ref __self_0_0,
                    doc_ref: ref __self_0_1,
                    enums: ref __self_0_2,
                    instances: ref __self_0_3,
                    meta: ref __self_0_4,
                    params: ref __self_0_5,
                    seq: ref __self_0_6,
                    types: ref __self_0_7,
                } => {
                    let mut debug_trait_builder = f.debug_struct("TypeSpec");
                    let _ = debug_trait_builder.field("doc", &&(*__self_0_0));
                    let _ = debug_trait_builder.field("doc_ref", &&(*__self_0_1));
                    let _ = debug_trait_builder.field("enums", &&(*__self_0_2));
                    let _ = debug_trait_builder.field("instances", &&(*__self_0_3));
                    let _ = debug_trait_builder.field("meta", &&(*__self_0_4));
                    let _ = debug_trait_builder.field("params", &&(*__self_0_5));
                    let _ = debug_trait_builder.field("seq", &&(*__self_0_6));
                    let _ = debug_trait_builder.field("types", &&(*__self_0_7));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for TypeSpec {
        #[inline]
        fn default() -> TypeSpec {
            TypeSpec {
                doc: ::core::default::Default::default(),
                doc_ref: ::core::default::Default::default(),
                enums: ::core::default::Default::default(),
                instances: ::core::default::Default::default(),
                meta: ::core::default::Default::default(),
                params: ::core::default::Default::default(),
                seq: ::core::default::Default::default(),
                types: ::core::default::Default::default(),
            }
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DESERIALIZE_FOR_TypeSpec: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for TypeSpec {
            fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                    __field4,
                    __field5,
                    __field6,
                    __field7,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::export::Ok(__Field::__field0),
                            1u64 => _serde::export::Ok(__Field::__field1),
                            2u64 => _serde::export::Ok(__Field::__field2),
                            3u64 => _serde::export::Ok(__Field::__field3),
                            4u64 => _serde::export::Ok(__Field::__field4),
                            5u64 => _serde::export::Ok(__Field::__field5),
                            6u64 => _serde::export::Ok(__Field::__field6),
                            7u64 => _serde::export::Ok(__Field::__field7),
                            _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"field index 0 <= i < 8",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "doc" => _serde::export::Ok(__Field::__field0),
                            "doc-ref" => _serde::export::Ok(__Field::__field1),
                            "enums" => _serde::export::Ok(__Field::__field2),
                            "instances" => _serde::export::Ok(__Field::__field3),
                            "meta" => _serde::export::Ok(__Field::__field4),
                            "params" => _serde::export::Ok(__Field::__field5),
                            "seq" => _serde::export::Ok(__Field::__field6),
                            "types" => _serde::export::Ok(__Field::__field7),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"doc" => _serde::export::Ok(__Field::__field0),
                            b"doc-ref" => _serde::export::Ok(__Field::__field1),
                            b"enums" => _serde::export::Ok(__Field::__field2),
                            b"instances" => _serde::export::Ok(__Field::__field3),
                            b"meta" => _serde::export::Ok(__Field::__field4),
                            b"params" => _serde::export::Ok(__Field::__field5),
                            b"seq" => _serde::export::Ok(__Field::__field6),
                            b"types" => _serde::export::Ok(__Field::__field7),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::export::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::export::PhantomData<TypeSpec>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = TypeSpec;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "struct TypeSpec")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match match _serde::de::SeqAccess::next_element::<Option<Doc>>(
                            &mut __seq,
                        ) {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct TypeSpec with 8 elements",
                                ));
                            }
                        };
                        let __field1 = match match _serde::de::SeqAccess::next_element::<
                            Option<DocRef>,
                        >(&mut __seq)
                        {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct TypeSpec with 8 elements",
                                ));
                            }
                        };
                        let __field2 = match match _serde::de::SeqAccess::next_element::<
                            Option<EnumsSpec>,
                        >(&mut __seq)
                        {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    2usize,
                                    &"struct TypeSpec with 8 elements",
                                ));
                            }
                        };
                        let __field3 = match match _serde::de::SeqAccess::next_element::<
                            Option<InstancesSpec>,
                        >(&mut __seq)
                        {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    3usize,
                                    &"struct TypeSpec with 8 elements",
                                ));
                            }
                        };
                        let __field4 = match match _serde::de::SeqAccess::next_element::<
                            Option<MetaSpec>,
                        >(&mut __seq)
                        {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    4usize,
                                    &"struct TypeSpec with 8 elements",
                                ));
                            }
                        };
                        let __field5 = match match _serde::de::SeqAccess::next_element::<
                            Option<ParamsSpec>,
                        >(&mut __seq)
                        {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    5usize,
                                    &"struct TypeSpec with 8 elements",
                                ));
                            }
                        };
                        let __field6 = match match _serde::de::SeqAccess::next_element::<
                            Option<Attributes>,
                        >(&mut __seq)
                        {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    6usize,
                                    &"struct TypeSpec with 8 elements",
                                ));
                            }
                        };
                        let __field7 = match match _serde::de::SeqAccess::next_element::<
                            Option<TypesSpec>,
                        >(&mut __seq)
                        {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    7usize,
                                    &"struct TypeSpec with 8 elements",
                                ));
                            }
                        };
                        _serde::export::Ok(TypeSpec {
                            doc: __field0,
                            doc_ref: __field1,
                            enums: __field2,
                            instances: __field3,
                            meta: __field4,
                            params: __field5,
                            seq: __field6,
                            types: __field7,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::export::Option<Option<Doc>> =
                            _serde::export::None;
                        let mut __field1: _serde::export::Option<Option<DocRef>> =
                            _serde::export::None;
                        let mut __field2: _serde::export::Option<Option<EnumsSpec>> =
                            _serde::export::None;
                        let mut __field3: _serde::export::Option<Option<InstancesSpec>> =
                            _serde::export::None;
                        let mut __field4: _serde::export::Option<Option<MetaSpec>> =
                            _serde::export::None;
                        let mut __field5: _serde::export::Option<Option<ParamsSpec>> =
                            _serde::export::None;
                        let mut __field6: _serde::export::Option<Option<Attributes>> =
                            _serde::export::None;
                        let mut __field7: _serde::export::Option<Option<TypesSpec>> =
                            _serde::export::None;
                        while let _serde::export::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::export::Option::is_some(&__field0) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "doc",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<Option<Doc>>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::export::Option::is_some(&__field1) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "doc-ref",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<Option<DocRef>>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::export::Option::is_some(&__field2) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "enums",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<Option<EnumsSpec>>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field3 => {
                                    if _serde::export::Option::is_some(&__field3) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "instances",
                                            ),
                                        );
                                    }
                                    __field3 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            Option<InstancesSpec>,
                                        >(&mut __map)
                                        {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field4 => {
                                    if _serde::export::Option::is_some(&__field4) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "meta",
                                            ),
                                        );
                                    }
                                    __field4 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<Option<MetaSpec>>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field5 => {
                                    if _serde::export::Option::is_some(&__field5) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "params",
                                            ),
                                        );
                                    }
                                    __field5 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<Option<ParamsSpec>>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field6 => {
                                    if _serde::export::Option::is_some(&__field6) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "seq",
                                            ),
                                        );
                                    }
                                    __field6 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<Option<Attributes>>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field7 => {
                                    if _serde::export::Option::is_some(&__field7) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "types",
                                            ),
                                        );
                                    }
                                    __field7 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<Option<TypesSpec>>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::export::Some(__field0) => __field0,
                            _serde::export::None => match _serde::private::de::missing_field("doc")
                            {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            },
                        };
                        let __field1 = match __field1 {
                            _serde::export::Some(__field1) => __field1,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("doc-ref") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::export::Some(__field2) => __field2,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("enums") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field3 = match __field3 {
                            _serde::export::Some(__field3) => __field3,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("instances") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field4 = match __field4 {
                            _serde::export::Some(__field4) => __field4,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("meta") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field5 = match __field5 {
                            _serde::export::Some(__field5) => __field5,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("params") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field6 = match __field6 {
                            _serde::export::Some(__field6) => __field6,
                            _serde::export::None => match _serde::private::de::missing_field("seq")
                            {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            },
                        };
                        let __field7 = match __field7 {
                            _serde::export::Some(__field7) => __field7,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("types") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::export::Ok(TypeSpec {
                            doc: __field0,
                            doc_ref: __field1,
                            enums: __field2,
                            instances: __field3,
                            meta: __field4,
                            params: __field5,
                            seq: __field6,
                            types: __field7,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] = &[
                    "doc",
                    "doc-ref",
                    "enums",
                    "instances",
                    "meta",
                    "params",
                    "seq",
                    "types",
                ];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "TypeSpec",
                    FIELDS,
                    __Visitor {
                        marker: _serde::export::PhantomData::<TypeSpec>,
                        lifetime: _serde::export::PhantomData,
                    },
                )
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_SERIALIZE_FOR_TypeSpec: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for TypeSpec {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::export::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "TypeSpec",
                    false as usize + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "doc",
                    &self.doc,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "doc-ref",
                    &self.doc_ref,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "enums",
                    &self.enums,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "instances",
                    &self.instances,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "meta",
                    &self.meta,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "params",
                    &self.params,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "seq",
                    &self.seq,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "types",
                    &self.types,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    pub struct TypesSpec {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for TypesSpec {
        #[inline]
        fn clone(&self) -> TypesSpec {
            match *self {
                TypesSpec {} => TypesSpec {},
            }
        }
    }
    impl ::core::marker::StructuralPartialEq for TypesSpec {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for TypesSpec {
        #[inline]
        fn eq(&self, other: &TypesSpec) -> bool {
            match *other {
                TypesSpec {} => match *self {
                    TypesSpec {} => true,
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for TypesSpec {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                TypesSpec {} => {
                    let mut debug_trait_builder = f.debug_struct("TypesSpec");
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for TypesSpec {
        #[inline]
        fn default() -> TypesSpec {
            TypesSpec {}
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DESERIALIZE_FOR_TypesSpec: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for TypesSpec {
            fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"field index 0 <= i < 0",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::export::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::export::PhantomData<TypesSpec>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = TypesSpec;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "struct TypesSpec")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        _: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        _serde::export::Ok(TypesSpec {})
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        while let _serde::export::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            }
                        {
                            match __key {
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        _serde::export::Ok(TypesSpec {})
                    }
                }
                const FIELDS: &'static [&'static str] = &[];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "TypesSpec",
                    FIELDS,
                    __Visitor {
                        marker: _serde::export::PhantomData::<TypesSpec>,
                        lifetime: _serde::export::PhantomData,
                    },
                )
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_SERIALIZE_FOR_TypesSpec: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for TypesSpec {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::export::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "TypesSpec",
                    false as usize,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    pub type WikidataIdentifier = String;
    pub struct KaitaiStruct {
        pub doc: Option<Doc>,
        #[serde(rename = "doc-ref")]
        pub doc_ref: Option<DocRef>,
        /// allows for the setup of named enums, mappings of integer constants to symbolic names. Can
        /// be used with integer attributes using the enum key.
        ///
        /// would be represented as enum-like construct (or closest equivalent, if target language
        /// doesn’t support enums), nested or namespaced in current type/class
        pub enums: Option<EnumsSpec>,
        /// Purpose: description of data that lies outside of normal sequential parsing flow (for
        /// example, that requires seeking somewhere in the file) or just needs to be loaded only by
        /// special request
        ///
        /// Influences: would be translated into distinct methods (that read desired data on demand) in
        /// current class
        pub instances: Option<InstancesSpec>,
        pub meta: Option<MetaSpec>,
        pub params: Option<ParamsSpec>,
        /// identifier for a primary structure described in top-level map
        pub seq: Option<Attributes>,
        /// maps of strings to user-defined types
        ///
        /// declares types for substructures that can be referenced in the attributes of seq or
        /// instances element
        ///
        /// would be directly translated into classes
        pub types: Option<TypesSpec>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for KaitaiStruct {
        #[inline]
        fn clone(&self) -> KaitaiStruct {
            match *self {
                KaitaiStruct {
                    doc: ref __self_0_0,
                    doc_ref: ref __self_0_1,
                    enums: ref __self_0_2,
                    instances: ref __self_0_3,
                    meta: ref __self_0_4,
                    params: ref __self_0_5,
                    seq: ref __self_0_6,
                    types: ref __self_0_7,
                } => KaitaiStruct {
                    doc: ::core::clone::Clone::clone(&(*__self_0_0)),
                    doc_ref: ::core::clone::Clone::clone(&(*__self_0_1)),
                    enums: ::core::clone::Clone::clone(&(*__self_0_2)),
                    instances: ::core::clone::Clone::clone(&(*__self_0_3)),
                    meta: ::core::clone::Clone::clone(&(*__self_0_4)),
                    params: ::core::clone::Clone::clone(&(*__self_0_5)),
                    seq: ::core::clone::Clone::clone(&(*__self_0_6)),
                    types: ::core::clone::Clone::clone(&(*__self_0_7)),
                },
            }
        }
    }
    impl ::core::marker::StructuralPartialEq for KaitaiStruct {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for KaitaiStruct {
        #[inline]
        fn eq(&self, other: &KaitaiStruct) -> bool {
            match *other {
                KaitaiStruct {
                    doc: ref __self_1_0,
                    doc_ref: ref __self_1_1,
                    enums: ref __self_1_2,
                    instances: ref __self_1_3,
                    meta: ref __self_1_4,
                    params: ref __self_1_5,
                    seq: ref __self_1_6,
                    types: ref __self_1_7,
                } => match *self {
                    KaitaiStruct {
                        doc: ref __self_0_0,
                        doc_ref: ref __self_0_1,
                        enums: ref __self_0_2,
                        instances: ref __self_0_3,
                        meta: ref __self_0_4,
                        params: ref __self_0_5,
                        seq: ref __self_0_6,
                        types: ref __self_0_7,
                    } => {
                        (*__self_0_0) == (*__self_1_0)
                            && (*__self_0_1) == (*__self_1_1)
                            && (*__self_0_2) == (*__self_1_2)
                            && (*__self_0_3) == (*__self_1_3)
                            && (*__self_0_4) == (*__self_1_4)
                            && (*__self_0_5) == (*__self_1_5)
                            && (*__self_0_6) == (*__self_1_6)
                            && (*__self_0_7) == (*__self_1_7)
                    }
                },
            }
        }
        #[inline]
        fn ne(&self, other: &KaitaiStruct) -> bool {
            match *other {
                KaitaiStruct {
                    doc: ref __self_1_0,
                    doc_ref: ref __self_1_1,
                    enums: ref __self_1_2,
                    instances: ref __self_1_3,
                    meta: ref __self_1_4,
                    params: ref __self_1_5,
                    seq: ref __self_1_6,
                    types: ref __self_1_7,
                } => match *self {
                    KaitaiStruct {
                        doc: ref __self_0_0,
                        doc_ref: ref __self_0_1,
                        enums: ref __self_0_2,
                        instances: ref __self_0_3,
                        meta: ref __self_0_4,
                        params: ref __self_0_5,
                        seq: ref __self_0_6,
                        types: ref __self_0_7,
                    } => {
                        (*__self_0_0) != (*__self_1_0)
                            || (*__self_0_1) != (*__self_1_1)
                            || (*__self_0_2) != (*__self_1_2)
                            || (*__self_0_3) != (*__self_1_3)
                            || (*__self_0_4) != (*__self_1_4)
                            || (*__self_0_5) != (*__self_1_5)
                            || (*__self_0_6) != (*__self_1_6)
                            || (*__self_0_7) != (*__self_1_7)
                    }
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for KaitaiStruct {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                KaitaiStruct {
                    doc: ref __self_0_0,
                    doc_ref: ref __self_0_1,
                    enums: ref __self_0_2,
                    instances: ref __self_0_3,
                    meta: ref __self_0_4,
                    params: ref __self_0_5,
                    seq: ref __self_0_6,
                    types: ref __self_0_7,
                } => {
                    let mut debug_trait_builder = f.debug_struct("KaitaiStruct");
                    let _ = debug_trait_builder.field("doc", &&(*__self_0_0));
                    let _ = debug_trait_builder.field("doc_ref", &&(*__self_0_1));
                    let _ = debug_trait_builder.field("enums", &&(*__self_0_2));
                    let _ = debug_trait_builder.field("instances", &&(*__self_0_3));
                    let _ = debug_trait_builder.field("meta", &&(*__self_0_4));
                    let _ = debug_trait_builder.field("params", &&(*__self_0_5));
                    let _ = debug_trait_builder.field("seq", &&(*__self_0_6));
                    let _ = debug_trait_builder.field("types", &&(*__self_0_7));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for KaitaiStruct {
        #[inline]
        fn default() -> KaitaiStruct {
            KaitaiStruct {
                doc: ::core::default::Default::default(),
                doc_ref: ::core::default::Default::default(),
                enums: ::core::default::Default::default(),
                instances: ::core::default::Default::default(),
                meta: ::core::default::Default::default(),
                params: ::core::default::Default::default(),
                seq: ::core::default::Default::default(),
                types: ::core::default::Default::default(),
            }
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DESERIALIZE_FOR_KaitaiStruct: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for KaitaiStruct {
            fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                    __field4,
                    __field5,
                    __field6,
                    __field7,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::export::Ok(__Field::__field0),
                            1u64 => _serde::export::Ok(__Field::__field1),
                            2u64 => _serde::export::Ok(__Field::__field2),
                            3u64 => _serde::export::Ok(__Field::__field3),
                            4u64 => _serde::export::Ok(__Field::__field4),
                            5u64 => _serde::export::Ok(__Field::__field5),
                            6u64 => _serde::export::Ok(__Field::__field6),
                            7u64 => _serde::export::Ok(__Field::__field7),
                            _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"field index 0 <= i < 8",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "doc" => _serde::export::Ok(__Field::__field0),
                            "doc-ref" => _serde::export::Ok(__Field::__field1),
                            "enums" => _serde::export::Ok(__Field::__field2),
                            "instances" => _serde::export::Ok(__Field::__field3),
                            "meta" => _serde::export::Ok(__Field::__field4),
                            "params" => _serde::export::Ok(__Field::__field5),
                            "seq" => _serde::export::Ok(__Field::__field6),
                            "types" => _serde::export::Ok(__Field::__field7),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"doc" => _serde::export::Ok(__Field::__field0),
                            b"doc-ref" => _serde::export::Ok(__Field::__field1),
                            b"enums" => _serde::export::Ok(__Field::__field2),
                            b"instances" => _serde::export::Ok(__Field::__field3),
                            b"meta" => _serde::export::Ok(__Field::__field4),
                            b"params" => _serde::export::Ok(__Field::__field5),
                            b"seq" => _serde::export::Ok(__Field::__field6),
                            b"types" => _serde::export::Ok(__Field::__field7),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::export::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::export::PhantomData<KaitaiStruct>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = KaitaiStruct;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "struct KaitaiStruct")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match match _serde::de::SeqAccess::next_element::<Option<Doc>>(
                            &mut __seq,
                        ) {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct KaitaiStruct with 8 elements",
                                ));
                            }
                        };
                        let __field1 = match match _serde::de::SeqAccess::next_element::<
                            Option<DocRef>,
                        >(&mut __seq)
                        {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct KaitaiStruct with 8 elements",
                                ));
                            }
                        };
                        let __field2 = match match _serde::de::SeqAccess::next_element::<
                            Option<EnumsSpec>,
                        >(&mut __seq)
                        {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    2usize,
                                    &"struct KaitaiStruct with 8 elements",
                                ));
                            }
                        };
                        let __field3 = match match _serde::de::SeqAccess::next_element::<
                            Option<InstancesSpec>,
                        >(&mut __seq)
                        {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    3usize,
                                    &"struct KaitaiStruct with 8 elements",
                                ));
                            }
                        };
                        let __field4 = match match _serde::de::SeqAccess::next_element::<
                            Option<MetaSpec>,
                        >(&mut __seq)
                        {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    4usize,
                                    &"struct KaitaiStruct with 8 elements",
                                ));
                            }
                        };
                        let __field5 = match match _serde::de::SeqAccess::next_element::<
                            Option<ParamsSpec>,
                        >(&mut __seq)
                        {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    5usize,
                                    &"struct KaitaiStruct with 8 elements",
                                ));
                            }
                        };
                        let __field6 = match match _serde::de::SeqAccess::next_element::<
                            Option<Attributes>,
                        >(&mut __seq)
                        {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    6usize,
                                    &"struct KaitaiStruct with 8 elements",
                                ));
                            }
                        };
                        let __field7 = match match _serde::de::SeqAccess::next_element::<
                            Option<TypesSpec>,
                        >(&mut __seq)
                        {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    7usize,
                                    &"struct KaitaiStruct with 8 elements",
                                ));
                            }
                        };
                        _serde::export::Ok(KaitaiStruct {
                            doc: __field0,
                            doc_ref: __field1,
                            enums: __field2,
                            instances: __field3,
                            meta: __field4,
                            params: __field5,
                            seq: __field6,
                            types: __field7,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::export::Option<Option<Doc>> =
                            _serde::export::None;
                        let mut __field1: _serde::export::Option<Option<DocRef>> =
                            _serde::export::None;
                        let mut __field2: _serde::export::Option<Option<EnumsSpec>> =
                            _serde::export::None;
                        let mut __field3: _serde::export::Option<Option<InstancesSpec>> =
                            _serde::export::None;
                        let mut __field4: _serde::export::Option<Option<MetaSpec>> =
                            _serde::export::None;
                        let mut __field5: _serde::export::Option<Option<ParamsSpec>> =
                            _serde::export::None;
                        let mut __field6: _serde::export::Option<Option<Attributes>> =
                            _serde::export::None;
                        let mut __field7: _serde::export::Option<Option<TypesSpec>> =
                            _serde::export::None;
                        while let _serde::export::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::export::Option::is_some(&__field0) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "doc",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<Option<Doc>>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::export::Option::is_some(&__field1) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "doc-ref",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<Option<DocRef>>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::export::Option::is_some(&__field2) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "enums",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<Option<EnumsSpec>>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field3 => {
                                    if _serde::export::Option::is_some(&__field3) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "instances",
                                            ),
                                        );
                                    }
                                    __field3 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            Option<InstancesSpec>,
                                        >(&mut __map)
                                        {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field4 => {
                                    if _serde::export::Option::is_some(&__field4) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "meta",
                                            ),
                                        );
                                    }
                                    __field4 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<Option<MetaSpec>>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field5 => {
                                    if _serde::export::Option::is_some(&__field5) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "params",
                                            ),
                                        );
                                    }
                                    __field5 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<Option<ParamsSpec>>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field6 => {
                                    if _serde::export::Option::is_some(&__field6) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "seq",
                                            ),
                                        );
                                    }
                                    __field6 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<Option<Attributes>>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field7 => {
                                    if _serde::export::Option::is_some(&__field7) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "types",
                                            ),
                                        );
                                    }
                                    __field7 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<Option<TypesSpec>>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::export::Some(__field0) => __field0,
                            _serde::export::None => match _serde::private::de::missing_field("doc")
                            {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            },
                        };
                        let __field1 = match __field1 {
                            _serde::export::Some(__field1) => __field1,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("doc-ref") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::export::Some(__field2) => __field2,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("enums") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field3 = match __field3 {
                            _serde::export::Some(__field3) => __field3,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("instances") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field4 = match __field4 {
                            _serde::export::Some(__field4) => __field4,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("meta") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field5 = match __field5 {
                            _serde::export::Some(__field5) => __field5,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("params") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field6 = match __field6 {
                            _serde::export::Some(__field6) => __field6,
                            _serde::export::None => match _serde::private::de::missing_field("seq")
                            {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            },
                        };
                        let __field7 = match __field7 {
                            _serde::export::Some(__field7) => __field7,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("types") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::export::Ok(KaitaiStruct {
                            doc: __field0,
                            doc_ref: __field1,
                            enums: __field2,
                            instances: __field3,
                            meta: __field4,
                            params: __field5,
                            seq: __field6,
                            types: __field7,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] = &[
                    "doc",
                    "doc-ref",
                    "enums",
                    "instances",
                    "meta",
                    "params",
                    "seq",
                    "types",
                ];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "KaitaiStruct",
                    FIELDS,
                    __Visitor {
                        marker: _serde::export::PhantomData::<KaitaiStruct>,
                        lifetime: _serde::export::PhantomData,
                    },
                )
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_SERIALIZE_FOR_KaitaiStruct: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for KaitaiStruct {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::export::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "KaitaiStruct",
                    false as usize + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "doc",
                    &self.doc,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "doc-ref",
                    &self.doc_ref,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "enums",
                    &self.enums,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "instances",
                    &self.instances,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "meta",
                    &self.meta,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "params",
                    &self.params,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "seq",
                    &self.seq,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "types",
                    &self.types,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    impl Default for MetaSpec {
        fn default() -> Self {
            Self {
                application: ::alloc::vec::Vec::new(),
                encoding: None,
                endian: None,
                file_extension: ::alloc::vec::Vec::new(),
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
}
