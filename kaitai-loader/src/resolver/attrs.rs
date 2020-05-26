mod types;

pub use self::types::{
    get_attr_type, BaseAttributeType, FloatAttributeType, IntegerAttributeType,
    ResolvedAttributeType,
};

// pub fn get_str_attr_type(attr: &Attribute) -> Result<BaseAttributeType, String> {
//     assert_eq!(attr.type_, Some(AttributeType::String("str".to_owned())));
//     let size = match (&attr.size_eos, &attr.size) {
//         (Some(true), None) => None,
//         (Some(true), Some(size)) => return Err(format!("Cannot use size_eos: true and size: {}", size)),
//         (_, None) => return Err(format!("`str` type must define either a `size` or `size_eos: true`: {:?}", attr)),
//         (_, Some(size)) => Some(size.clone())
//     };
//     return Ok(BaseAttributeType::String {
//         size,
//         encoding: attr.encoding.clone()
//     })
// }
// pub fn get_strz_attr_type(attr: &Attribute) -> Result<BaseAttributeType, String> {
//     assert_eq!(attr.type_, Some(AttributeType::String("strz".to_owned())));
//     unreachable!("WTF");
// }
