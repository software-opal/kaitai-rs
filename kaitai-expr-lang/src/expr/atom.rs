use crate::ast;
use crate::expr::{IResult, Span};
use nom::{
    branch::alt,
    character::complete::{char, one_of},
    combinator::{map, recognize},
    multi::many0,
    sequence::tuple,
};
use super::{name::name_expr, numbers::{integer_expr, float_expr}, strings::string_expr};


pub fn atom_expr(input: Span) -> IResult<ast::Expression> {


    alt((
        // empty_list_expr,
        // enum_by_name_expr,
        // byte_size_of_type_expr,
        // bit_size_of_type_expr,
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
