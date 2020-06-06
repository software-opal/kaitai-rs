use super::{
    expr::power_expr,
    utils::{maybe_ws, maybe_ws_op},
};
use crate::ast;
use crate::expr::{IResult, Span};
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::map,
    multi::many0,
    sequence::{preceded, tuple},
};

macro_rules! chain_bin_op_expr {
    ($expr:expr, $sep: expr ) => {
        map(
            tuple(($expr, many0(tuple(($sep, $expr))))),
            |(first, rest): (ast::Expression, Vec<(ast::Operator, ast::Expression)>)| {
                rest.into_iter()
                    .fold(first, |left, (op, right)| ast::Expression::BinOp {
                        left: Box::new(left),
                        op,
                        right: Box::new(right),
                    })
            },
        )
    };
}

pub fn expr_expr(input: Span) -> IResult<ast::Expression> {
    // val expr: P[Ast.expr] = P( Chain(xor_expr, BitOr) )
    chain_bin_op_expr!(xor_expr_expr, maybe_ws_op(ast::Operator::BitOr, "|"))(input)
}
pub fn xor_expr_expr(input: Span) -> IResult<ast::Expression> {
    // val xor_expr: P[Ast.expr] = P( Chain(and_expr, BitXor) )
    chain_bin_op_expr!(and_expr_expr, maybe_ws_op(ast::Operator::BitAnd, "^"))(input)
}
pub fn and_expr_expr(input: Span) -> IResult<ast::Expression> {
    // val and_expr: P[Ast.expr] = P( Chain(shift_expr, BitAnd) )
    chain_bin_op_expr!(shift_expr_expr, maybe_ws_op(ast::Operator::BitAnd, "&"))(input)
}
pub fn shift_expr_expr(input: Span) -> IResult<ast::Expression> {
    // val shift_expr: P[Ast.expr] = P( Chain(arith_expr, LShift | RShift) )
    chain_bin_op_expr!(
        arith_expr_expr,
        alt((
            maybe_ws_op(ast::Operator::LShift, "<<"),
            maybe_ws_op(ast::Operator::RShift, ">>")
        ))
    )(input)
}

pub fn arith_expr_expr(input: Span) -> IResult<ast::Expression> {
    // val arith_expr: P[Ast.expr] = P( Chain(term, Add | Sub) )
    chain_bin_op_expr!(
        term_expr,
        alt((
            maybe_ws_op(ast::Operator::Add, "+"),
            maybe_ws_op(ast::Operator::Sub, "-")
        ))
    )(input)
}
pub fn term_expr(input: Span) -> IResult<ast::Expression> {
    // val term: P[Ast.expr] = P( Chain(factor, Mult | Div | Mod) )
    chain_bin_op_expr!(
        factor_expr,
        alt((
            maybe_ws_op(ast::Operator::Mult, "*"),
            maybe_ws_op(ast::Operator::Div, "/"),
            maybe_ws_op(ast::Operator::Mod, "%")
        ))
    )(input)
}
pub fn factor_expr(input: Span) -> IResult<ast::Expression> {
    // val factor: P[Ast.expr] = P(
    //   ("+" ~ factor) |
    //   ("-" ~ factor).map(Ast.expr.UnaryOp(Ast.unaryop.Minus, _)) |
    //   ("~" ~ factor).map(Ast.expr.UnaryOp(Ast.unaryop.Invert, _)) |
    //   power
    // )
    alt((
        preceded(tuple((maybe_ws, tag("+"))), factor_expr),
        map(preceded(tuple((maybe_ws, tag("-"))), factor_expr), |expr| {
            ast::Expression::UnaryOp {
                op: ast::UnaryOperator::Minus,
                operand: Box::new(expr),
            }
        }),
        map(preceded(tuple((maybe_ws, tag("~"))), factor_expr), |expr| {
            ast::Expression::UnaryOp {
                op: ast::UnaryOperator::Invert,
                operand: Box::new(expr),
            }
        }),
        power_expr,
    ))(input)
}
