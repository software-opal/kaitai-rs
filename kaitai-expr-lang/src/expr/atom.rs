use super::{
    name::name_expr,
    numbers::{float_expr, integer_expr},
    strings::string_expr,
    test::test_expr,
    utils::{maybe_ws_tag, ws}, expr::{enum_by_name_expr, list_expr, byte_size_of_type_expr, bit_size_of_type_expr},
};
use crate::ast;
use crate::expr::{IResult, Span};
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::map,
    sequence::{delimited, tuple},
};

pub fn empty_list_expr(input: Span) -> IResult<ast::Expression> {
    map(tuple((tag("["), ws, tag("]"))), |_| {
        ast::Expression::List(vec![])
    })(input)
}

pub fn atom_expr(input: Span) -> IResult<ast::Expression> {
    alt((
        empty_list_expr,
        // empty_dict_expr,
        delimited(maybe_ws_tag("("), test_expr, maybe_ws_tag(")")),
        delimited(maybe_ws_tag("["), list_expr, maybe_ws_tag("]")),
        enum_by_name_expr,
        byte_size_of_type_expr,
        bit_size_of_type_expr,
        string_expr,
        name_expr,
        float_expr,
        integer_expr,
    ))(input)
}
//   val atom: P[Ast.expr] = {
//     val empty_list = ("[" ~ "]").map(_ => Ast.expr.List(Nil))
// //    val empty_dict = ("{" ~ "}").map(_ => Ast.expr.Dict(Nil, Nil))
//     P(
//       empty_list |
// //      empty_dict |
//       "(" ~ test ~ ")" |
//       "[" ~ list ~ "]" |
// //      "{" ~ dictorsetmaker ~ "}" |
//       enumByName |
//       byteSizeOfType |
//       bitSizeOfType |
//       STRING.rep(1).map(_.mkString).map(Ast.expr.Str) |
//       NAME.map((x) => x.name match {
//         case "true" => Ast.expr.Bool(true)
//         case "false" => Ast.expr.Bool(false)
//         case _ => Ast.expr.Name(x)
//       }) |
//       FLOAT_NUMBER.map(Ast.expr.FloatNum) |
//       INT_NUMBER.map(Ast.expr.IntNum)
//     )
//   }
