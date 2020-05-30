use kaitai_expr_lang::{ast::func::*, expr::parse};

#[test]
fn it_parses_single_positive_integer() {
    assert_eq!(parse("123").unwrap(), IntNum(123))
}
// #[test]
// fn it_parses_single_negative_integer() {
//     assert_eq!(parse("-456").unwrap(), UnaryOp(Minus, IntNum(456)))
// }
#[test]
fn it_parses_positive_integer_with_underscores() {
    assert_eq!(parse("100_500").unwrap(), IntNum(100500))
}
#[test]
fn it_parses_hex_integer() {
    assert_eq!(parse("0x1234").unwrap(), IntNum(0x1234))
}
#[test]
fn it_parses_hex_integer_with_underscores() {
    assert_eq!(parse("0x12_34").unwrap(), IntNum(0x1234))
}
#[test]
fn it_parses_octal_integer() {
    assert_eq!(parse("0o644").unwrap(), IntNum(420))
}
#[test]
fn it_parses_octal_integer_with_undescores() {
    assert_eq!(parse("0o06_44").unwrap(), IntNum(420))
}
#[test]
fn it_parses_binary_integer() {
    assert_eq!(parse("0b10101010").unwrap(), IntNum(0xaa))
}
#[test]
fn it_parses_binary_integer_with_undescores() {
    assert_eq!(parse("0b1010_1_010").unwrap(), IntNum(0xaa))
}
#[test]
fn it_parses_simple_float() {
    assert_eq!(parse("1.2345").unwrap(), FloatNum(1.2345))
}
#[test]
fn it_parses_float_with_positive_exponent() {
    assert_eq!(parse("123e4").unwrap(), FloatNum(123e4))
}
#[test]
fn it_parses_float_with_positive_exponent_with_plus_sign() {
    assert_eq!(parse("123e+4").unwrap(), FloatNum(123e4))
}
#[test]
fn it_parses_float_with_negative_exponent() {
    assert_eq!(parse("123e-7").unwrap(), FloatNum(123e-7))
}
#[test]
fn it_parses_float_and_non_integral_part_with_positive_exponent() {
    assert_eq!(parse("1.2345e7").unwrap(), FloatNum(1.2345e7))
}
#[test]
fn it_parses_float_and_non_integral_part_with_positive_exponent_with_plus_sign() {
    assert_eq!(parse("123.45e+7").unwrap(), FloatNum(123.45e7))
}
#[test]
fn it_parses_float_and_non_integral_part_with_negative_exponent() {
    assert_eq!(parse("123.45e-7").unwrap(), FloatNum(123.45e-7))
}
