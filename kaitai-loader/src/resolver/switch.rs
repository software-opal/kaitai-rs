use std::collections::BTreeMap;

pub struct SwitchDefinition<'a> {
    pub name: &'a Option<String>,
    pub index: usize,
    pub variants: BTreeMap<&'a str, Vec<&'a str>>,
}

// pub fn extract_switch_types<'a>(
//     spec: &'a BTreeMap<Vec<&'a str>, &'a TypeSpec>,
// ) -> BTreeMap<(Vec<&'a str>, usize), &'a SwitchDefinition> {
//     spec.iter()
//         .enumerate()
//         .flat_map(|(index, (_type_position, type_spec))| {
//             type_spec
//                 .seq
//                 .iter()
//                 .filter_map(|attribute| match &attribute.type_ {
//                     Some(AttributeType::Switch(switch)) => {
//                         println!("{:#?}", switch);
//                         Some(SwitchDefinition {
//                             name: &attribute.id,
//                             index: index,
//                             variants: switch.cases.map(|(case, type_)| {

//                             }),
//                         })
//                     }
//                     _ => None,
//                 })
//         })
//         .collect()
// }
