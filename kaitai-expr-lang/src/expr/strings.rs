use crate::ast;
use crate::expr::{IResult, Span};
use nom::{
    branch::alt,
    character::complete::{char, none_of, one_of},
    combinator::{map, recognize, value},
    multi::{many0, many1, many_m_n},
    sequence::{preceded, terminated},
};
use std::convert::TryInto;

fn single_quoted_string(input: Span) -> IResult<String> {
    // val singlestring = P("'" ~/ singlestringchar.rep.! ~ "'")
    // val singlestringchar = P( CharsWhile(!"'".contains(_)) )

    let string_char = none_of("'");
    // We use recognize as there are no escape sequences in a single-quoted string.
    let string_content = map(recognize(many0(string_char)), |s: Span| s.to_owned());
    preceded(char('\''), terminated(string_content, char('\'')))(input)
}
fn double_quoted_string(input: Span) -> IResult<String> {
    // val doublestring: P[String] = P("\"" ~/ doublestringitem.rep ~ "\"").map(_.mkString)
    // val doublestringitem = P( doublestringchar.! | escapeseq )
    // val doublestringchar = P( CharsWhile(!"\\\"".contains(_)) )
    let string_char = none_of("\\\"");
    let string_item = alt((string_char, escape_sequence));
    // Cannot use recognize as we have escape sequences.
    let string_content = map(many0(string_item), |chr_vec| chr_vec.into_iter().collect());
    preceded(char('"'), terminated(string_content, char('"')))(input)
}

fn escaped_char(input: Span) -> IResult<char> {
    // val QUOTED_CC = Map(
    // "a" -> "\7",  // bell, ASCII code 7
    // "b" -> "\b",  // backspace, ASCII code 8
    // "t" -> "\t",  // horizontal tab, ASCII code 9
    // "n" -> "\n",  // newline, ASCII code 10
    // "v" -> "\13", // vertical tab, ASCII code 11 = 0o13
    // "f" -> "\14", // form feed, ASCII code 12 = 0o14
    // "r" -> "\r",  // carriage return, ASCII code 13
    // "e" -> "\33", // escape, ASCII code 27 = 0o33
    // "'" -> "'",   // single quote
    // "\"" -> "\"", // double quote
    // "\\" -> "\\"  // backslash
    // )
    // val VALID_QUOTED = QUOTED_CC.keys.toList.sorted.mkString
    // val quotedchar = P( CharIn(VALID_QUOTED).! ).map(QUOTED_CC)

    alt((
        value('\u{07}', char('a')), // bell, ASCII code 7
        value('\u{08}', char('b')), // backspace, ASCII code 8
        value('\t', char('t')),     // horizontal tab, ASCII code 9
        value('\n', char('n')),     // newline, ASCII code 10
        value('\u{0B}', char('v')), // vertical tab, ASCII code 11 = 0o13
        value('\u{0C}', char('f')), // form feed, ASCII code 12 = 0o14
        value('\r', char('r')),     // carriage return, ASCII code 13
        value('\u{1B}', char('e')), // escape, ASCII code 27 = 0o33
        value('\'', char('\'')),    // single quote
        value('\"', char('\"')),    // double quote
        value('\\', char('\\')),    // backslash
    ))(input)
}
fn escaped_octal(input: Span) -> IResult<char> {
    // val quotedoctal: P[String] = P( octdigit.rep(1).! ).map { (digits) =>
    // val code = Integer.parseInt(digits, 8).toChar
    // Character.toString(code)
    // }
    map(recognize(many1(one_of("01234567"))), |octal| {
        u32::from_str_radix(octal, 8).unwrap().try_into().unwrap()
    })(input)
}
fn escaped_hex(input: Span) -> IResult<char> {
    // val quotedhex: P[String] = P( "u" ~/ hexdigit.rep(exactly = 4).! ).map { (digits) =>
    // val code = Integer.parseInt(digits, 16).toChar
    // Character.toString(code)
    // }
    preceded(
        char('u'),
        map(
            recognize(many_m_n(4, 4, one_of("0123456789ABCDEFabcdef"))),
            |hex| u32::from_str_radix(hex, 16).unwrap().try_into().unwrap(),
        ),
    )(input)
}
fn escape_sequence(input: Span) -> IResult<char> {
    // val escapeseq = P( "\\" ~/ (quotedchar | quotedoctal | quotedhex) )
    preceded(char('\\'), alt((escaped_char, escaped_octal, escaped_hex)))(input)
}

pub fn string(input: Span) -> IResult<String> {
    alt((single_quoted_string, double_quoted_string))(input)
}
pub fn string_expr(input: Span) -> IResult<ast::Expression> {
    map(string, |string| ast::Expression::Str(string))(input)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_string() {
        assert_eq!(string(r#""""#).unwrap().1, "".to_owned());
        assert_eq!(string(r#""a""#).unwrap().1, "a".to_owned());
        assert_eq!(string(r#""\0""#).unwrap().1, "\u{00}".to_owned());
        assert_eq!(string(r#""\n\t""#).unwrap().1, "\n\t".to_owned());
        assert_eq!(string(r#""\"\'""#).unwrap().1, "\"'".to_owned());
        assert_eq!(string(r#""\\""#).unwrap().1, "\\".to_owned());
        assert_eq!(string(r#""\u0000""#).unwrap().1, "\u{00}".to_owned());
    }
}

// val stringliteral: P[String] = P( singlestring | doublestring )
