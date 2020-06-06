use crate::expr::{IResult, Span};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::one_of,
    combinator::{map, opt, value},
    multi::many1,
    sequence::delimited,
};

pub fn ws(input: Span) -> IResult<()> {
    let inner_ws = alt((value((), one_of(" \n")), value((), tag("\\\n"))));
    value((), many1(inner_ws))(input)
}
pub fn maybe_ws(input: Span) -> IResult<()> {
    value((), opt(ws))(input)
}

pub fn ws_tag<'a>(tag_val: &'a str) -> impl Fn(Span) -> IResult<Span> + 'a {
    move |input: Span| delimited(ws, tag(tag_val), ws)(input)
}
pub fn right_ws_tag<'a>(tag_val: &'a str) -> impl Fn(Span) -> IResult<Span> + 'a {
    move |input: Span| delimited(maybe_ws, tag(tag_val), ws)(input)
}
pub fn left_ws_tag<'a>(tag_val: &'a str) -> impl Fn(Span) -> IResult<Span> + 'a {
    move |input: Span| delimited(ws, tag(tag_val), maybe_ws)(input)
}
pub fn maybe_ws_tag<'a>(tag_val: &'a str) -> impl Fn(Span) -> IResult<Span> + 'a {
    move |input: Span| delimited(maybe_ws, tag(tag_val), maybe_ws)(input)
}

pub fn ws_op<'a, R: 'a + Clone>(op_value: R, tag_val: &'a str) -> impl Fn(Span) -> IResult<R> + 'a {
    move |input: Span| map(ws_tag(tag_val), |_| op_value.clone())(input)
}
pub fn left_ws_op<'a, R: 'a + Clone>(
    op_value: R,
    tag_val: &'a str,
) -> impl Fn(Span) -> IResult<R> + 'a {
    move |input: Span| map(left_ws_tag(tag_val), |_| op_value.clone())(input)
}
pub fn right_ws_op<'a, R: 'a + Clone>(
    op_value: R,
    tag_val: &'a str,
) -> impl Fn(Span) -> IResult<R> + 'a {
    move |input: Span| map(right_ws_tag(tag_val), |_| op_value.clone())(input)
}
pub fn maybe_ws_op<'a, R: 'a + Clone>(
    op_value: R,
    tag_val: &'a str,
) -> impl Fn(Span) -> IResult<R> + 'a {
    move |input: Span| map(maybe_ws_tag(tag_val), |_| op_value.clone())(input)
}

pub fn ws_kw<'a>(tag_val: &'a str) -> impl Fn(Span) -> IResult<()> + 'a {
    move |input: Span| value((), ws_tag(tag_val))(input)
}
pub fn left_ws_kw<'a>(tag_val: &'a str) -> impl Fn(Span) -> IResult<()> + 'a {
    move |input: Span| value((), left_ws_tag(tag_val))(input)
}
pub fn right_ws_kw<'a>(tag_val: &'a str) -> impl Fn(Span) -> IResult<()> + 'a {
    move |input: Span| value((), right_ws_tag(tag_val))(input)
}
pub fn maybe_ws_kw<'a>(tag_val: &'a str) -> impl Fn(Span) -> IResult<()> + 'a {
    move |input: Span| value((), maybe_ws_tag(tag_val))(input)
}
