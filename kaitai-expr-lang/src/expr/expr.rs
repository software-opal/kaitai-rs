use super::{
    arith::expr_expr,
    atom::atom_expr,
    name::{identifier_ident, type_name},
    test::test_expr,
    utils::maybe_ws_tag,
};
use crate::ast;
use crate::expr::{IResult, Span};
use nom::{
    branch::alt,
    combinator::{map, opt},
    multi::{many0, separated_list, separated_nonempty_list},
    sequence::{delimited, preceded, terminated, tuple},
};

enum Trailer {
    Call(std::vec::Vec<ast::Expression>),
    Subscript(ast::Expression),
    Cast(ast::TypeId),
    Attr(ast::Identifier),
}

fn trailer_expr_map(input: Span) -> IResult<Trailer> {
    // val trailer: P[Ast.expr => Ast.expr] = {
    //   val call = P("(" ~ arglist ~ ")").map{ case (args) => (lhs: Ast.expr) => Ast.expr.Call(lhs, args)}
    //   val slice = P("[" ~ test ~ "]").map{ case (args) => (lhs: Ast.expr) => Ast.expr.Subscript(lhs, args)}
    //   val cast = P( "." ~ "as" ~ "<" ~ TYPE_NAME ~ ">" ).map(
    //     typeName => (lhs: Ast.expr) => Ast.expr.CastToType(lhs, typeName)
    //   )
    //   val attr = P("." ~ NAME).map(id => (lhs: Ast.expr) => Ast.expr.Attribute(lhs, id))
    //   P( call | slice | cast | attr )
    // }
    alt((
        map(
            delimited(maybe_ws_tag("("), arg_list_exprs, maybe_ws_tag(")")),
            |args| Trailer::Call(args),
        ),
        map(
            delimited(maybe_ws_tag("["), test_expr, maybe_ws_tag("]")),
            |args| Trailer::Subscript(args),
        ),
        map(
            preceded(
                tuple((maybe_ws_tag("."), maybe_ws_tag("as"))),
                delimited(maybe_ws_tag("<"), type_name, maybe_ws_tag(">")),
            ),
            |name| Trailer::Cast(name),
        ),
        map(preceded(maybe_ws_tag("."), identifier_ident), |name| {
            Trailer::Attr(name)
        }),
    ))(input)
}

pub fn power_expr(input: Span) -> IResult<ast::Expression> {
    // val power: P[Ast.expr] = P( atom ~ trailer.rep ).map {
    //   case (lhs, trailers) =>
    //     trailers.foldLeft(lhs)((l, t) => t(l))
    // }
    map(
        tuple((atom_expr, many0(trailer_expr_map))),
        |(lhs, rest): (_, Vec<Trailer>)| {
            rest.into_iter().fold(lhs, |lhs, trailer| match trailer {
                Trailer::Call(args) => ast::Expression::Call {
                    func: Box::new(lhs),
                    args,
                },
                Trailer::Subscript(idx) => ast::Expression::Subscript {
                    value: Box::new(lhs),
                    idx: Box::new(idx),
                },
                Trailer::Cast(type_name) => ast::Expression::CastToType {
                    value: Box::new(lhs),
                    type_name,
                },
                Trailer::Attr(attr) => ast::Expression::Attribute {
                    value: Box::new(lhs),
                    attr: attr,
                },
            })
        },
    )(input)
}
pub fn expr_list_exprs(input: Span) -> IResult<Vec<ast::Expression>> {
    // val exprlist: P[Seq[Ast.expr]] = P( expr.rep(1, sep = ",") ~ ",".? )
    terminated(
        separated_nonempty_list(maybe_ws_tag(","), expr_expr),
        opt(maybe_ws_tag(",")),
    )(input)
}
pub fn test_list_exprs(input: Span) -> IResult<Vec<ast::Expression>> {
    // val testlist: P[Seq[Ast.expr]] = P( test.rep(1, sep = ",") ~ ",".? )
    terminated(
        separated_nonempty_list(maybe_ws_tag(","), test_expr),
        opt(maybe_ws_tag(",")),
    )(input)
}
pub fn arg_list_exprs(input: Span) -> IResult<Vec<ast::Expression>> {
    // val arglist: P[Seq[Ast.expr]] = P( (test).rep(0, ",") )
    separated_list(maybe_ws_tag(","), test_expr)(input)
}

pub fn list_contents_exprs(input: Span) -> IResult<Vec<ast::Expression>> {
// val list_contents = P( test.rep(1, ",") ~ ",".? )
    terminated(
        separated_nonempty_list(maybe_ws_tag(","), test_expr),
        opt(maybe_ws_tag(",")),
    )(input)
}
pub fn list_expr(input: Span) -> IResult<ast::Expression> {
    // val list = P( list_contents ).map(Ast.expr.List(_))
    map(list_contents_exprs, |exprs| ast::Expression::List(exprs))(input)
}
pub fn enum_by_name_expr(input: Span) -> IResult<ast::Expression> {
// val enumByName: P[Ast.expr.EnumByLabel] = P("::".!.? ~ NAME.rep(2, "::")).map {
//   case (first, names: Seq[Ast.identifier]) =>
//     val isAbsolute = first.nonEmpty
//     val (enumName, enumLabel) = names.takeRight(2) match {
//     case Seq(a, b) => (a, b)
//     }
//     val typePath = names.dropRight(2)
//     if (typePath.isEmpty) {
//     Ast.expr.EnumByLabel(enumName, enumLabel, Ast.EmptyTypeId)
//     } else {
//     Ast.expr.EnumByLabel(enumName, enumLabel, Ast.typeId(isAbsolute, typePath.map(_.name)))
//     }
// }

}
pub fn byte_size_of_type_expr(input: Span) -> IResult<ast::Expression> {}
pub fn bit_size_of_type_expr(input: Span) -> IResult<ast::Expression> {}
