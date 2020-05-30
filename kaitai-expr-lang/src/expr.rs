use crate::ast;
use nom::{branch::alt, combinator::complete};
use thiserror::Error;

mod utils;
mod numbers;
mod name;
mod strings;
mod atom;

type Span<'a> = &'a str;
type IResult<'a, T> = nom::IResult<Span<'a>, T>;
type Result<T> = std::result::Result<T, ExpressionError>;

#[derive(Error, Debug)]
pub enum ExpressionError {
    #[error("Unable to parse input: {0}")]
    ParseError(String),
}
impl From<nom::Err<(&'_ str, nom::error::ErrorKind)>> for ExpressionError {
    fn from(e: nom::Err<(&'_ str, nom::error::ErrorKind)>) -> Self {
        ExpressionError::ParseError(format!("{:?}", e))
    }
}

fn parser(input: Span) -> IResult<ast::Expression> {
    complete(alt((strings::string_expr, numbers::float_expr, numbers::integer_expr)))(input)
}

pub fn parse(input: &str) -> Result<ast::Expression> {
    Ok(parser(input)?.1)
}
