use eyre::Result;
use std::collections::BTreeSet;

pub mod ast;
pub mod expr;
mod helpers;

#[derive(Debug, PartialEq)]
pub struct Expression<'a> {
    input: &'a str,
    ast_expr: ast::Expression,
}

impl<'a> Expression<'a> {
    pub fn parse(expression: &'a str) -> Result<Self> {
        Ok(Expression {
            input: expression,
            ast_expr: expr::parse(expression)?,
        })
    }

    pub fn is_constant(&self) -> bool {
        self.ast_expr.is_constant()
    }

    pub fn arguments(&self) -> BTreeSet<&'a str> {
        BTreeSet::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_parse_constant_int {
        ($($name:ident: $input:expr => $output:expr;)+) => {
            $(
                #[test]
                fn $name() {
                    let expr = Expression::parse($input).unwrap();
                    assert!(expr.is_constant())
                }
            )+
        };
    }
    macro_rules! test_parse_constant_float {
        ($($name:ident: $input:expr => $output:expr;)+) => {
            $(
                #[test]
                fn $name() {
                    let expr = Expression::parse($input).unwrap();
                    assert!(expr.is_constant())
                }
            )+
        };
    }

    test_parse_constant_int! {
        parse_constant_integer: "10" => 10;
        parse_constant_hex_integer: "0xABC" => 0xABC;
        parse_constant_hex_integer_with_underscores: "0xA_BC" => 0xABC;
    }

    test_parse_constant_float! {
        parse_constant_float: "1.2345" =>  1.2345;
        parse_constant_float_with_exponent: "123e4" =>  123e4;
        parse_constant_float_with_positive_exponent: "123e+4" =>  123e+4;
        parse_constant_float_with_negative_exponent: "123e-4" =>  123e-4;
        parse_constant_float_with_non_integral_part_and_exponent: "1.23e4" =>  123e4;
        parse_constant_float_with_non_integral_part_and_positive_exponent: "1.23e+4" =>  123e+4;
        parse_constant_float_with_non_integral_part_and_negative_exponent: "1.23e-4" =>  123e-4;
    }

    #[test]
    fn parse_constant_str() {
        let expr = Expression::parse(r#""hi""#).unwrap();
        assert!(expr.is_constant());
    }
    // #[test]
    // fn parse_single_var() {
    //     let expr = Expression::parse("a").unwrap();
    //     assert!(!expr.is_constant());
    //     assert_eq!(expr.arguments(), vec!["a"].into_iter().collect());
    // }
    // #[test]
    // fn parse_add_var() {
    //     let expr = Expression::parse("a + b").unwrap();
    //     assert!(!expr.is_constant());
    //     assert_eq!(expr.arguments(), vec!["a", "b"].into_iter().collect());
    // }
    // #[test]
    // fn parse_add_multi_var() {
    //     let expr = Expression::parse("a + b + c").unwrap();
    //     assert!(!expr.is_constant());
    //     assert_eq!(expr.arguments(), vec!["a", "b", "c"].into_iter().collect());
    // }

    // #[test]
    // fn parse_const_index() {
    //     let expr = Expression::parse("a[0]").unwrap();
    //     assert!(!expr.is_constant());
    //     assert_eq!(expr.arguments(), vec!["a"].into_iter().collect());
    // }
    // #[test]
    // fn parse_var_index() {
    //     let expr = Expression::parse("a[b]").unwrap();
    //     assert!(!expr.is_constant());
    //     assert_eq!(expr.arguments(), vec!["a", "b"].into_iter().collect());
    // }
}
