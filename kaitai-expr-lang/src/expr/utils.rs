use crate::expr::{IResult, Span};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, one_of},
    combinator::{map, recognize, value},
    multi::{many0, many1},
};

pub fn ws(input: Span) -> IResult<()> {
    let inner_ws = alt((value((), one_of(" \n")), value((), tag("\\\n"))));
    value((), many1(inner_ws))(input)
}
