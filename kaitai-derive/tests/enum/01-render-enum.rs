
kaitai_derive::codegen_ksy!(
r#"
meta:
    id: png
    file-extension: png
    endian: be

enums:
    color_type:
        0: greyscale
        2: truecolor
        3: indexed
        4: greyscale_alpha
        6: truecolor_alpha
    phys_unit:
        0: unknown
        1: meter
"#
);

fn main() {
    use std::convert::TryFrom;

    assert_eq!(ColorType::try_from("2").unwrap(), ColorType::Truecolor);
    assert_eq!(ColorType::try_from(2_u8).unwrap(), ColorType::Truecolor);
    assert_eq!(ColorType::try_from(4_u8).unwrap(), ColorType::GreyscaleAlpha);
    assert!(ColorType::Truecolor < ColorType::GreyscaleAlpha);

    assert_eq!(PhysUnit::try_from(0_u8).unwrap(), PhysUnit::Unknown);
    assert_eq!(PhysUnit::try_from(1_u8).unwrap(), PhysUnit::Meter);
    assert!(PhysUnit::Unknown < PhysUnit::Meter);
}
