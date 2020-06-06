use super::utils::ws;
use crate::ast;
use crate::expr::{IResult, Span};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, one_of},
    combinator::{map, opt, recognize},
    multi::{many0, separated_nonempty_list},
    sequence::tuple,
};

// val identifier: P[Ast.identifier] =
// P( (letter|"_") ~ (letter | digit | "_").rep ).!.map(Ast.identifier)
// val letter     = P( lowercase | uppercase )
// val lowercase  = P( CharIn('a' to 'z') )
// val uppercase  = P( CharIn('A' to 'Z') )
// val digit      = P( CharIn('0' to '9') )

fn letter(input: Span) -> IResult<char> {
    alt((lower_case_letter, upper_case_letter))(input)
}
fn lower_case_letter(input: Span) -> IResult<char> {
    one_of("abcdefghijklmnopqrstuvwxyz")(input)
}
fn upper_case_letter(input: Span) -> IResult<char> {
    one_of("ABCDEFGHIJKLMNOPQRSTUVWXYZ")(input)
}
fn digit_letter(input: Span) -> IResult<char> {
    one_of("0123456789")(input)
}

pub fn identifier(input: Span) -> IResult<Span> {
    let ident_start = alt((lower_case_letter, upper_case_letter, char('_')));
    let ident_rest = alt((
        lower_case_letter,
        upper_case_letter,
        digit_letter,
        char('_'),
    ));
    recognize(tuple((ident_start, (many0(ident_rest)))))(input)
}
pub fn identifier_ident(input: Span) -> IResult<ast::Identifier> {
    map(identifier, |name| ast::Identifier {
        name: name.to_owned(),
    })(input)
}

pub fn type_name(input: Span) -> IResult<ast::TypeId> {
    // val TYPE_NAME: P[Ast.typeId] = P("::".!.? ~ NAME.rep(1, "::") ~ ("[" ~ "]").!.?).map {
    //     case (first, names: Seq[Ast.identifier], arrayStr) =>
    //       Ast.typeId(first.nonEmpty, names.map((el) => el.name), arrayStr.nonEmpty)
    // }
    let maybe_absolute = map(opt(tag("::")), |option: Option<_>| option.is_some());
    let maybe_array = map(opt(tuple((tag("["), ws, tag("]")))), |option: Option<_>| {
        option.is_some()
    });
    map(
        tuple((
            maybe_absolute,
            separated_nonempty_list(tag("::"), identifier),
            maybe_array,
        )),
        |(absolute, names, is_array)| ast::TypeId {
            absolute,
            names: names.into_iter().map(|name| name.to_owned()).collect(),
            is_array,
        },
    )(input)
}

pub fn name_expr(input: Span) -> IResult<ast::Expression> {
    map(identifier, |name| match name {
        "true" => ast::Expression::Bool(true),
        "false" => ast::Expression::Bool(true),
        name => ast::Expression::Name(ast::Identifier {
            name: name.to_owned(),
        }),
    })(input)
}
