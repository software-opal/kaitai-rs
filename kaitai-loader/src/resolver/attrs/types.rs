use crate::raw::attrs::{Attribute, AttributeRepeat, AttributeType};
use std::{
    collections::BTreeMap,
    convert::{TryFrom, TryInto},
};

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum IntegerAttributeType {
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
}
impl TryFrom<&str> for IntegerAttributeType {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "u1" => Ok(IntegerAttributeType::U8),
            "u2" => Ok(IntegerAttributeType::U16),
            "u4" => Ok(IntegerAttributeType::U32),
            "u8" => Ok(IntegerAttributeType::U64),
            "i1" => Ok(IntegerAttributeType::I8),
            "i2" => Ok(IntegerAttributeType::I16),
            "i4" => Ok(IntegerAttributeType::I32),
            "i8" => Ok(IntegerAttributeType::I64),
            _ => Err(()),
        }
    }
}
impl TryFrom<&Option<AttributeType>> for IntegerAttributeType {
    type Error = ();
    fn try_from(value: &Option<AttributeType>) -> Result<Self, Self::Error> {
        match value {
            None => Ok(IntegerAttributeType::U8),
            Some(AttributeType::String(inner)) => Self::try_from(&inner[..]),
            _ => Err(()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum FloatAttributeType {
    F32,
    F64,
}
impl TryFrom<&str> for FloatAttributeType {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "f4" => Ok(FloatAttributeType::F32),
            "f8" => Ok(FloatAttributeType::F64),
            _ => Err(()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum BaseAttributeType {
    Integer(IntegerAttributeType),
    Float(FloatAttributeType),
    String,
    Switch(BTreeMap<String, BaseAttributeType>),
    Enum(IntegerAttributeType, String),
    User(String),
}
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ResolvedAttributeType {
    Base(BaseAttributeType),
    Repeat(AttributeRepeat, BaseAttributeType),
    Optional(BaseAttributeType),
}

impl ResolvedAttributeType {
    pub fn enum_type(&self) -> Option<(&IntegerAttributeType, &String)> {
        match self.base_attr_type() {
            BaseAttributeType::Enum(a, b) => Some((a, b)),
            _ => None,
        }
    }
    pub fn switch_type(&self) -> Option<&BTreeMap<String, BaseAttributeType>> {
        match self.base_attr_type() {
            BaseAttributeType::Switch(r) => Some(r),
            _ => None,
        }
    }
    pub fn base_attr_type(&self) -> &BaseAttributeType {
        match self {
            Self::Base(t) => t,
            Self::Repeat(_, t) => t,
            Self::Optional(t) => t,
        }
    }
}

pub fn get_switch_attr_type(attr: &Attribute) -> Result<BaseAttributeType, String> {
    let switch = match &attr.type_ {
        Some(AttributeType::Switch(sw)) => sw,
        _ => unreachable!("Huh"),
    };

    Ok(BaseAttributeType::Switch(
        switch
            .cases
            .iter()
            .map(|(case, case_type)| {
                let case_type = match (&case_type[..]).try_into() {
                    Ok(int_type) => BaseAttributeType::Integer(int_type),
                    Err(()) => match (&case_type[..]).try_into() {
                        Ok(float_type) => BaseAttributeType::Float(float_type),
                        Err(()) => BaseAttributeType::User(case_type.to_owned()),
                    },
                };
                (case.clone(), case_type)
            })
            .collect(),
    ))
}

pub fn get_base_attr_type(attr: &Attribute) -> Result<BaseAttributeType, String> {
    match &attr.type_ {
        Some(AttributeType::String(inner)) => match &attr.enum_ {
            Some(enum_name) => match (&inner[..]).try_into() {
                Ok(int_type) => return Ok(BaseAttributeType::Enum(int_type, enum_name.clone())),
                Err(()) => {
                    return Err(format!(
                        "Unable to define an enum on a non-integer type. Got type: {:?}",
                        attr.type_
                    ))
                }
            },
            None => match (&inner[..]).try_into() {
                Ok(int_type) => return Ok(BaseAttributeType::Integer(int_type)),
                Err(()) => {
                    if let Ok(float_type) = (&inner[..]).try_into() {
                        Ok(BaseAttributeType::Float(float_type))
                    } else {
                        match &inner[..] {
                            "str" => Ok(BaseAttributeType::String),
                            "strz" => Ok(BaseAttributeType::String),
                            other => Ok(BaseAttributeType::User(other.to_owned())),
                        }
                    }
                }
            },
        },
        Some(AttributeType::Switch(_)) => get_switch_attr_type(attr),
        _ => unreachable!("Code path already dealt with by get_integer_attr_type"),
    }
}

pub fn get_attr_type(attr: &Attribute) -> Result<ResolvedAttributeType, String> {
    let base_type = get_base_attr_type(attr)?;
    if let Some(_repeat) = &attr.repeat {
        Err("Oops".to_owned())
    } else if let Some(_) = attr.if_ {
        Ok(ResolvedAttributeType::Optional(base_type))
    } else {
        Ok(ResolvedAttributeType::Base(base_type))
    }
}
