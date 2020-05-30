use crate::ast;
use crate::expr::{IResult, Span};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, one_of},
    combinator::{map, opt, recognize, value},
    multi::{many0, many1},
    sequence::{pair, preceded, terminated, tuple},
};
use std::str::FromStr;

// val integer: P[BigInt] = P( octinteger | hexinteger | bininteger | decimalinteger)
// val decimalinteger: P[BigInt] = P( nonzerodigit ~ (digit | "_").rep | "0" ).!.map(parseNum(_, 10))
// val octinteger: P[BigInt] = P( "0" ~ ("o" | "O") ~ octdigit.rep(1).! ).map(parseNum(_, 8))
// val hexinteger: P[BigInt] = P( "0" ~ ("x" | "X") ~ hexdigit.rep(1).! ).map(parseNum(_, 16))
// val bininteger: P[BigInt] = P( "0" ~ ("b" | "B") ~ bindigit.rep(1).! ).map(parseNum(_, 2))
// val nonzerodigit: P0 = P( CharIn('1' to '9') )
// val octdigit: P0 = P( CharIn('0' to '7') | "_" )
// val bindigit: P0 = P( "0" | "1" | "_" )
// val hexdigit: P0 = P( digit | CharIn('a' to 'f', 'A' to 'F') | "_" )
fn integer_radix_parse<'a, F>(
    parser: F,
    radix: u32,
) -> impl Fn(Span<'a>) -> IResult<ast::IntegerType>
where
    F: Fn(Span<'a>) -> IResult<(char, Vec<char>)>,
{
    map(recognize(parser), move |string| {
        ast::IntegerType::from_str_radix(&string.replace("_", ""), radix).unwrap()
    })
}
pub fn decimal_integer(input: Span) -> IResult<ast::IntegerType> {
    let zero = value(0, char('0'));
    let nonzero = integer_radix_parse(
        tuple((one_of("123456789"), many0(one_of("_0123456789")))),
        10,
    );
    let parser = alt((zero, nonzero));
    parser(input)
}
pub fn binary_integer(input: Span) -> IResult<ast::IntegerType> {
    preceded(
        pair(tag("0"), one_of("bB")),
        integer_radix_parse(pair(one_of("01"), many0(one_of("_01"))), 2),
    )(input)
}
pub fn octal_integer(input: Span) -> IResult<ast::IntegerType> {
    preceded(
        pair(tag("0"), one_of("oO")),
        integer_radix_parse(pair(one_of("01234567"), many0(one_of("_01234567"))), 8),
    )(input)
}
pub fn hexadecimal_integer(input: Span) -> IResult<ast::IntegerType> {
    preceded(
        pair(tag("0"), one_of("xX")),
        integer_radix_parse(
            pair(
                one_of("0123456789abcdefABCDEF"),
                many0(one_of("_0123456789abcdefABCDEF")),
            ),
            16,
        ),
    )(input)
}
pub fn integer(input: Span) -> IResult<ast::IntegerType> {
    // Note the order matters here, the decimal must come last as it matches the `0` used to prefix the other types
    alt((
        binary_integer,
        octal_integer,
        hexadecimal_integer,
        decimal_integer,
    ))(input)
}
pub fn integer_expr(input: Span) -> IResult<ast::Expression> {
    map(integer, ast::Expression::IntNum)(input)
}

// val floatnumber: P[BigDecimal] = P( exponentfloat | pointfloat )
// val pointfloat: P[BigDecimal] = P( intpart.? ~ fraction | intpart ~ "." ).!.map(BigDecimal(_))
// val exponentfloat: P[BigDecimal] = P( (pointfloat | intpart) ~ exponent ).!.map(BigDecimal(_))
// val intpart: P[BigDecimal] = P( digit.rep(1) ).!.map(BigDecimal(_))
// val fraction: P0 = P( "." ~ digit.rep(1) )
// val exponent: P0 = P( ("e" | "E") ~ ("+" | "-").? ~ digit.rep(1) )
fn integer_part_float(input: Span) -> IResult<Vec<char>> {
    many1(one_of("0123456789"))(input)
}
fn fraction_part_float(input: Span) -> IResult<(char, std::vec::Vec<char>)> {
    tuple((char('.'), many1(one_of("0123456789"))))(input)
}
fn point_float(input: Span) -> IResult<Span> {
    let true_float = tuple((opt(integer_part_float), fraction_part_float));
    let integer_dot = terminated(integer_part_float, char('.'));
    alt((recognize(true_float), recognize(integer_dot)))(input)
}
fn exponent_float(input: Span) -> IResult<Span> {
    let coefficient = alt((point_float, recognize(integer_part_float)));
    let exponent = tuple((opt(one_of("+-")), integer_part_float));
    recognize(tuple((coefficient, one_of("eE"), exponent)))(input)
}
pub fn float(input: Span) -> IResult<ast::FloatType> {
    map(alt((exponent_float, point_float)), |f| {
        ast::FloatType::from_str(f).unwrap()
    })(input)
}
pub fn float_expr(input: Span) -> IResult<ast::Expression> {
    map(float, ast::Expression::FloatNum)(input)
}
