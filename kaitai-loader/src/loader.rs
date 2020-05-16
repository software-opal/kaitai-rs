use crate::raw::root::KsySpec;
use std::io::Read;

pub fn from_string(input: impl ToString) -> serde_yaml::Result<KsySpec> {
    serde_yaml::from_str(&input.to_string())
}
pub fn from_reader<R>(reader: R) -> serde_yaml::Result<KsySpec>
where
    R: Read,
{
    serde_yaml::from_reader(reader)
}

#[cfg(test)]
mod test_generate {
    use super::*;
    use crate::raw::attrs::{
        Attribute, AttributeRepeat, AttributeType, ContentsType, StringOrByte,
    };
    use crate::raw::base::endian::Endian;
    use crate::raw::base::scalar::AnyNonNullScalar;
    use crate::raw::base::switch::Switch;
    use crate::raw::enums::EnumValueSpec;
    use crate::raw::meta::CommonMetaSpec;
    use crate::raw::meta::RootMetaSpec;
    use crate::raw::types::TypeSpec;
    use difference::{Changeset, Difference};
    use std::collections::BTreeMap;

    macro_rules! map {
        ($($key:expr => $value:expr),* $(,)?) => {
            vec![$( ($key, $value), )*].into_iter().collect()
        };
    }

    macro_rules! assert_eq_diff {
        ($a: expr, $b: expr) => {{
            let a = $a;
            let b = $b;
            if a != b {
                let a_repr = format!("{:#?}", a);
                let b_repr = format!("{:#?}", b);
                Changeset::new(&a_repr, &b_repr, "\n")
                    .diffs
                    .into_iter()
                    .for_each(|diff| match diff {
                        Difference::Same(_) => {}
                        Difference::Add(add) => println!("+ {}", add),
                        Difference::Rem(add) => println!("- {}", add),
                    });
                assert!(false);
            }
        }};
    }

    #[test]
    fn test_map() {
        let map: BTreeMap<u8, u8> = map! {1 => 2, 3 => 4,};
        assert_eq!(map.get(&1_u8), Some(&2_u8));
        assert_eq!(map.get(&3_u8), Some(&4_u8));
        assert_eq!(map.get(&5_u8), None);
    }

    #[test]
    fn test_png_sample() {
        let ksy_struct =
            from_string(include_str!("../test/test_data/kaitai-sample-png.ksy")).unwrap();

        assert_eq_diff!(
            ksy_struct,
            KsySpec {
                meta: RootMetaSpec {
                    id: "png".to_owned(),
                    file_extension: vec!["png".to_owned()],
                    common: CommonMetaSpec {
                        endian: Some(Endian::Big),
                        ..CommonMetaSpec::default()
                    },
                    ..RootMetaSpec::default()
                },
                seq: vec![
                    Attribute {
                        id: Some("magic".to_owned()),
                        contents: Some(ContentsType::Array(vec![
                            StringOrByte::Integer(137),
                            StringOrByte::Integer(80),
                            StringOrByte::Integer(78),
                            StringOrByte::Integer(71),
                            StringOrByte::Integer(13),
                            StringOrByte::Integer(10),
                            StringOrByte::Integer(26),
                            StringOrByte::Integer(10)
                        ])),
                        ..Attribute::default()
                    },
                    Attribute {
                        id: Some("ihdr_len".to_owned()),
                        contents: Some(ContentsType::Array(vec![
                            StringOrByte::Integer(0),
                            StringOrByte::Integer(0),
                            StringOrByte::Integer(0),
                            StringOrByte::Integer(13)
                        ])),
                        ..Attribute::default()
                    },
                    Attribute {
                        id: Some("ihdr_type".to_owned()),
                        contents: Some(ContentsType::String("IHDR".to_owned())),
                        ..Attribute::default()
                    },
                    Attribute {
                        id: Some("ihdr".to_owned()),
                        type_: Some(AttributeType::String("ihdr_chunk".to_owned())),
                        ..Attribute::default()
                    },
                    Attribute {
                        id: Some("ihdr_crc".to_owned()),
                        size: Some("4".to_owned()),
                        ..Attribute::default()
                    },
                    Attribute {
                        id: Some("chunks".to_owned()),
                        type_: Some(AttributeType::String("chunk".to_owned())),
                        ..Attribute::default()
                    }
                ],
                types: map! {
                    "bkgd_chunk".to_owned() => TypeSpec {
                        seq: vec![
                            Attribute {
                                id: Some("bkgd".to_owned()),
                                type_: Some(AttributeType::Switch(Switch {
                                    switch_on: Some(AnyNonNullScalar::String("_root.ihdr.color_type".to_owned())),
                                    cases: map!{
                                        "color_type::greyscale".to_owned() => Some(AnyNonNullScalar::String("bkgd_greyscale".to_owned())),
                                        "color_type::greyscale_alpha".to_owned() => Some(AnyNonNullScalar::String("bkgd_greyscale".to_owned())),
                                        "color_type::indexed".to_owned() => Some(AnyNonNullScalar::String("bkgd_indexed".to_owned())),
                                        "color_type::truecolor".to_owned() => Some(AnyNonNullScalar::String("bkgd_truecolor".to_owned())),
                                        "color_type::truecolor_alpha".to_owned() => Some(AnyNonNullScalar::String("bkgd_truecolor".to_owned()))
                                    }
                                })),
                                ..Attribute::default()
                        }],
                        ..TypeSpec::default()
                    },
                    "bkgd_greyscale".to_owned() => TypeSpec {
                        seq: vec![
                            Attribute {
                                id: Some("value".to_owned()),
                                type_: Some(AttributeType::String("u2".to_owned())),
                                ..Attribute::default()
                            }
                        ],
                        ..TypeSpec::default()
                    },
                    "bkgd_indexed".to_owned() => TypeSpec {
                        seq: vec![
                            Attribute {
                                id: Some("palette_index".to_owned()),
                                type_: Some(AttributeType::String("u1".to_owned())),
                                ..Attribute::default()
                            }
                        ],
                        ..TypeSpec::default()
                    },
                    "bkgd_truecolor".to_owned() => TypeSpec {
                        seq: vec![
                            Attribute {
                                id: Some("red".to_owned()),
                                type_: Some(AttributeType::String("u2".to_owned())),
                                ..Attribute::default()
                            },
                            Attribute {
                                id: Some("green".to_owned()),
                                type_: Some(AttributeType::String("u2".to_owned())),
                                ..Attribute::default()
                            },
                            Attribute {
                                id: Some("blue".to_owned()),
                                type_: Some(AttributeType::String("u2".to_owned())),
                                ..Attribute::default()
                            }
                        ],
                        ..TypeSpec::default()
                    },
                    "chrm_chunk".to_owned() => TypeSpec {
                        seq: vec![
                            Attribute {
                                id: Some("white_point".to_owned()),
                                type_: Some(AttributeType::String("point".to_owned())),
                                ..Attribute::default()
                            },
                            Attribute {
                                id: Some("red".to_owned()),
                                type_: Some(AttributeType::String("point".to_owned())),
                                ..Attribute::default()
                            },
                            Attribute {
                                id: Some("green".to_owned()),
                                type_: Some(AttributeType::String("point".to_owned())),
                                ..Attribute::default()
                            },
                            Attribute {
                                id: Some("blue".to_owned()),
                                type_: Some(AttributeType::String("point".to_owned())),
                                ..Attribute::default()
                            }
                        ],
                        ..TypeSpec::default()
                    },
                    "chunk".to_owned() => TypeSpec {
                        seq: vec![
                            Attribute {
                                id: Some("len".to_owned()),
                                type_: Some(AttributeType::String("u4".to_owned())),
                                ..Attribute::default()
                            },
                            Attribute {
                                id: Some("type".to_owned()),
                                type_: Some(AttributeType::String("str".to_owned())),
                                size: Some("4".to_owned()),
                                encoding: Some("UTF-8".to_owned()),
                                ..Attribute::default()
                            },
                            Attribute {
                                id: Some("body".to_owned()),
                                type_: Some(AttributeType::Switch(Switch {
                                    switch_on: Some(AnyNonNullScalar::String("type".to_owned())),
                                    cases: map!{
                                        "\"PLTE\"".to_owned() => Some(AnyNonNullScalar::String("plte_chunk".to_owned())),
                                        "\"bKGD\"".to_owned() => Some(AnyNonNullScalar::String("bkgd_chunk".to_owned())),
                                        "\"cHRM\"".to_owned() => Some(AnyNonNullScalar::String("chrm_chunk".to_owned())),
                                        "\"gAMA\"".to_owned() => Some(AnyNonNullScalar::String("gama_chunk".to_owned())),
                                        "\"iTXt\"".to_owned() => Some(AnyNonNullScalar::String("international_text_chunk".to_owned())),
                                        "\"pHYs\"".to_owned() => Some(AnyNonNullScalar::String("phys_chunk".to_owned())),
                                        "\"sRGB\"".to_owned() => Some(AnyNonNullScalar::String("srgb_chunk".to_owned())),
                                        "\"tEXt\"".to_owned() => Some(AnyNonNullScalar::String("text_chunk".to_owned())),
                                        "\"tIME\"".to_owned() => Some(AnyNonNullScalar::String("time_chunk".to_owned())),
                                        "\"zTXt\"".to_owned() => Some(AnyNonNullScalar::String("compressed_text_chunk".to_owned()))
                                    }
                                })),
                                size: Some("len".to_owned()),
                                ..Attribute::default()
                            },
                            Attribute {
                                id: Some("crc".to_owned()),
                                size: Some("4".to_owned()),
                                ..Attribute::default()
                            }
                        ],
                        ..TypeSpec::default()
                    },
                    "compressed_text_chunk".to_owned() => TypeSpec {
                        seq: vec![
                            Attribute {
                                id: Some("keyword".to_owned()),
                                type_: Some(AttributeType::String("strz".to_owned())),
                                encoding: Some("UTF-8".to_owned()),
                                ..Attribute::default()
                            },
                            Attribute {
                                id: Some("compression_method".to_owned()),
                                type_: Some(AttributeType::String("u1".to_owned())),
                                ..Attribute::default()
                            },
                            Attribute {
                                id: Some("text_datastream".to_owned()),
                                size_eos: Some(true),
                                ..Attribute::default()
                            }
                        ],
                        ..TypeSpec::default()
                    },
                    "gama_chunk".to_owned() => TypeSpec {
                        instances: map!{
                            "gamma_ratio".to_owned() => Attribute::default()
                        },
                        seq: vec![
                            Attribute {
                                id: Some("gamma_int".to_owned()),
                                type_: Some(AttributeType::String("u4".to_owned())),
                                ..Attribute::default()
                            }
                        ],
                        ..TypeSpec::default()
                    },
                    "ihdr_chunk".to_owned() => TypeSpec {
                        seq: vec![
                            Attribute {
                                id: Some("width".to_owned()),
                                type_: Some(AttributeType::String("u4".to_owned())),
                                ..Attribute::default()
                            },
                            Attribute {
                                id: Some("height".to_owned()),
                                type_: Some(AttributeType::String("u4".to_owned())),
                                ..Attribute::default()
                            },
                            Attribute {
                                id: Some("bit_depth".to_owned()),
                                type_: Some(AttributeType::String("u1".to_owned())),
                                ..Attribute::default()
                            },
                            Attribute {
                                id: Some("color_type".to_owned()),
                                type_: Some(AttributeType::String("u1".to_owned())),
                                enum_: Some("color_type".to_owned()),
                                ..Attribute::default()
                            },
                            Attribute {
                                id: Some("compression_method".to_owned()),
                                type_: Some(AttributeType::String("u1".to_owned())),
                                ..Attribute::default()
                            },
                            Attribute {
                                id: Some("filter_method".to_owned()),
                                type_: Some(AttributeType::String("u1".to_owned())),
                                ..Attribute::default()
                            },
                            Attribute {
                                id: Some("interlace_method".to_owned()),
                                type_: Some(AttributeType::String("u1".to_owned())),
                                ..Attribute::default()
                            }
                        ],
                        ..TypeSpec::default()
                    },
                    "international_text_chunk".to_owned() => TypeSpec {
                        seq: vec![
                            Attribute {
                                id: Some("keyword".to_owned()),
                                type_: Some(AttributeType::String("strz".to_owned())),
                                encoding: Some("UTF-8".to_owned()),
                                ..Attribute::default()
                            },
                            Attribute {
                                id: Some("compression_flag".to_owned()),
                                type_: Some(AttributeType::String("u1".to_owned())),
                                ..Attribute::default()
                            },
                            Attribute {
                                id: Some("compression_method".to_owned()),
                                type_: Some(AttributeType::String("u1".to_owned())),
                                ..Attribute::default()
                            },
                            Attribute {
                                id: Some("language_tag".to_owned()),
                                type_: Some(AttributeType::String("strz".to_owned())),
                                encoding: Some("ASCII".to_owned()),
                                ..Attribute::default()
                            },
                            Attribute {
                                id: Some("translated_keyword".to_owned()),
                                type_: Some(AttributeType::String("strz".to_owned())),
                                encoding: Some("UTF-8".to_owned()),
                                ..Attribute::default()
                            },
                            Attribute {
                                id: Some("text".to_owned()),
                                type_: Some(AttributeType::String("str".to_owned())),
                                size_eos: Some(true),
                                encoding: Some("UTF-8".to_owned()),
                                ..Attribute::default()
                            }
                        ],
                        ..TypeSpec::default()
                    },
                    "phys_chunk".to_owned() => TypeSpec {
                        seq: vec![
                            Attribute {
                                id: Some("pixels_per_unit_x".to_owned()),
                                type_: Some(AttributeType::String("u4".to_owned())),
                                ..Attribute::default()
                            },
                            Attribute {
                                id: Some("pixels_per_unit_y".to_owned()),
                                type_: Some(AttributeType::String("u4".to_owned())),
                                ..Attribute::default()
                            },
                            Attribute {
                                id: Some("unit".to_owned()),
                                type_: Some(AttributeType::String("u1".to_owned())),
                                enum_: Some("phys_unit".to_owned()),
                                ..Attribute::default()
                            }
                        ],
                        ..TypeSpec::default()
                     },
                    "plte_chunk".to_owned() => TypeSpec {
                        seq: vec![
                            Attribute {
                                id: Some("entries".to_owned()),
                                type_: Some(AttributeType::String("rgb".to_owned())),
                                repeat: Some(AttributeRepeat::EOS),
                                ..Attribute::default()
                            }
                        ],
                        ..TypeSpec::default()
                    },
                    "point".to_owned() => TypeSpec {
                        instances: map!{
                            "x".to_owned() => Attribute {
                                id: None,
                                type_: None,
                                ..Attribute::default()
                            },
                            "y".to_owned() => Attribute::default()
                        },
                        seq: vec![
                            Attribute {
                                id: Some("x_int".to_owned()),
                                type_: Some(AttributeType::String("u4".to_owned())),
                                ..Attribute::default()
                            },
                            Attribute {
                                id: Some("y_int".to_owned()),
                                type_: Some(AttributeType::String("u4".to_owned())),
                                ..Attribute::default()
                            }
                        ],
                        ..TypeSpec::default()
                     },
                    "rgb".to_owned() => TypeSpec {
                        seq: vec![
                            Attribute {
                                id: Some("r".to_owned()),
                                type_: Some(AttributeType::String("u1".to_owned())),
                                ..Attribute::default()
                            },
                            Attribute {
                                id: Some("g".to_owned()),
                                type_: Some(AttributeType::String("u1".to_owned())),
                                ..Attribute::default()
                            },
                            Attribute {
                                id: Some("b".to_owned()),
                                type_: Some(AttributeType::String("u1".to_owned())),
                                ..Attribute::default()
                            }
                        ],
                        ..TypeSpec::default()
                    },
                    "srgb_chunk".to_owned() => TypeSpec {
                        enums: map!{
                            "intent".to_owned() => map!{
                                "0".to_owned() => EnumValueSpec {
                                    id: "perceptual".to_owned(),
                                    ..EnumValueSpec::default()
                                },
                                "1".to_owned() => EnumValueSpec {
                                    id: "relative_colorimetric".to_owned(),
                                    ..EnumValueSpec::default()
                                },
                                "2".to_owned() => EnumValueSpec {
                                    id: "saturation".to_owned(),
                                    ..EnumValueSpec::default()
                                },
                                "3".to_owned() => EnumValueSpec {
                                    id: "absolute_colorimetric".to_owned(),
                                    ..EnumValueSpec::default()
                                }
                            }
                        },
                        seq: vec![
                            Attribute {
                                id: Some("render_intent".to_owned()),
                                type_: Some(AttributeType::String("u1".to_owned())),
                                enum_: Some("intent".to_owned()),
                                ..Attribute::default()
                            }
                        ],
                        ..TypeSpec::default()
                    },
                    "text_chunk".to_owned() => TypeSpec {
                        seq: vec![
                            Attribute {
                                id: Some("keyword".to_owned()),
                                type_: Some(AttributeType::String("strz".to_owned())),
                                encoding: Some("iso8859-1".to_owned()),
                                ..Attribute::default()
                            },
                            Attribute {
                                id: Some("text".to_owned()),
                                type_: Some(AttributeType::String("str".to_owned())),
                                size_eos: Some(true),
                                encoding: Some("iso8859-1".to_owned()),
                                ..Attribute::default()
                            }
                        ],
                        ..TypeSpec::default()
                    },
                    "time_chunk".to_owned() => TypeSpec {
                        seq: vec![
                            Attribute {
                                id: Some("year".to_owned()),
                                type_: Some(AttributeType::String("u2".to_owned())),
                                ..Attribute::default()
                            },
                            Attribute {
                                id: Some("month".to_owned()),
                                type_: Some(AttributeType::String("u1".to_owned())),
                                ..Attribute::default()
                            },
                            Attribute {
                                id: Some("day".to_owned()),
                                type_: Some(AttributeType::String("u1".to_owned())),
                                ..Attribute::default()
                            },
                            Attribute {
                                id: Some("hour".to_owned()),
                                type_: Some(AttributeType::String("u1".to_owned())),
                                ..Attribute::default()
                            },
                            Attribute {
                                id: Some("minute".to_owned()),
                                type_: Some(AttributeType::String("u1".to_owned())),
                                ..Attribute::default()
                            },
                            Attribute {
                                id: Some("second".to_owned()),
                                type_: Some(AttributeType::String("u1".to_owned())),
                                ..Attribute::default()
                            }
                        ],
                        ..TypeSpec::default()
                     }
                },
                enums: map! {
                    "color_type".to_owned() => map!{
                        "0".to_owned() => EnumValueSpec {
                            id: "greyscale".to_owned(),
                            ..EnumValueSpec::default()
                        },
                        "2".to_owned() => EnumValueSpec {
                            id: "truecolor".to_owned(),
                            ..EnumValueSpec::default()
                        },
                        "3".to_owned() => EnumValueSpec {
                            id: "indexed".to_owned(),
                            ..EnumValueSpec::default()
                        },
                        "4".to_owned() => EnumValueSpec {
                            id: "greyscale_alpha".to_owned(),
                            ..EnumValueSpec::default()
                        },
                        "6".to_owned() => EnumValueSpec {
                            id: "truecolor_alpha".to_owned(),
                            ..EnumValueSpec::default()
                        }
                    },
                    "phys_unit".to_owned() => map!{
                        "0".to_owned() => EnumValueSpec {
                            id: "unknown".to_owned(),
                            ..EnumValueSpec::default()
                        },
                        "1".to_owned() => EnumValueSpec {
                            id: "meter".to_owned(),
                            ..EnumValueSpec::default()
                        }
                    }
                },
                ..KsySpec::default()
            }
        )
    }
}
