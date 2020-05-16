
kaitai_derive::include_ksy!("../../../kaitai-derive/tests/enum/01-render-enum.ksy");

fn main() {
    assert_eq!(ColorType::try_from("2").unwrap(), ColorType::Truecolor);
    assert_eq!(ColorType::try_from(2_u8).unwrap(), ColorType::Truecolor);
    assert_eq!(ColorType::try_from(4_u8).unwrap(), ColorType::GreyscaleAlpha);

    assert_eq!(PhysUnit::try_from(0_u8).unwrap(), PhysUnit::Unknown);
    assert_eq!(PhysUnit::try_from(1_u8).unwrap(), PhysUnit::Meter);
}
