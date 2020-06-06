use super::{
    arith::expr_expr,
    utils::{maybe_ws, maybe_ws_op, right_ws_kw, ws_kw},
};
use crate::ast;
use crate::expr::{IResult, Span};
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{map, opt},
    multi::separated_nonempty_list,
    sequence::{preceded, separated_pair, tuple},
};

pub fn test_expr(input: Span) -> IResult<ast::Expression> {
    // val test: P[Ast.expr] = P( or_test ~ ("?" ~ test ~ ":" ~ test).? ).map{
    //   case (x, None) => x
    //   case (condition, Some((ifTrue, ifFalse))) => Ast.expr.IfExp(condition, ifTrue, ifFalse)
    // }
    let conditional = preceded(
        tuple((maybe_ws, tag("?"), maybe_ws)),
        separated_pair(test_expr, tuple((maybe_ws, tag(":"), maybe_ws)), test_expr),
    );

    map(tuple((or_test_expr, opt(conditional))), |b| match b {
        (condition, None) => condition,
        (condition, Some((if_true, if_false))) => ast::Expression::IfExp {
            condition: Box::new(condition),
            if_true: Box::new(if_true),
            if_false: Box::new(if_false),
        },
    })(input)
}
pub fn or_test_expr(input: Span) -> IResult<ast::Expression> {
    // val or_test = P( and_test.rep(1, kw("or")) ).map{
    //   case Seq(x) => x
    //   case xs => Ast.expr.BoolOp(Ast.boolop.Or, xs)
    // }
    map(
        separated_nonempty_list(ws_kw("or"), and_test_expr),
        |exprs| {
            if exprs.len() == 1 {
                exprs.into_iter().nth(0).unwrap()
            } else {
                ast::Expression::BoolOp {
                    op: ast::BooleanOperator::Or,
                    values: exprs,
                }
            }
        },
    )(input)
}

pub fn and_test_expr(input: Span) -> IResult<ast::Expression> {
    // val and_test = P( not_test.rep(1, kw("and")) ).map{
    //   case Seq(x) => x
    //   case xs => Ast.expr.BoolOp(Ast.boolop.And, xs)
    // }
    map(
        separated_nonempty_list(ws_kw("and"), not_test_expr),
        |exprs| {
            if exprs.len() == 1 {
                exprs.into_iter().nth(0).unwrap()
            } else {
                ast::Expression::BoolOp {
                    op: ast::BooleanOperator::And,
                    values: exprs,
                }
            }
        },
    )(input)
}

pub fn not_test_expr(input: Span) -> IResult<ast::Expression> {
    // val not_test: P[Ast.expr] = P( (kw("not") ~ not_test).map(Ast.expr.UnaryOp(Ast.unaryop.Not, _)) | comparison )
    alt((
        map(tuple((right_ws_kw("not"), not_test_expr)), |(_, expr)| {
            ast::Expression::UnaryOp {
                op: ast::UnaryOperator::Not,
                operand: Box::new(expr),
            }
        }),
        comparison_expr,
    ))(input)
}

pub fn comparison_operator(input: Span) -> IResult<ast::ComparisonOperator> {
    // def op[T](s: P0, rhs: T) = s.!.map(_ => rhs)
    // val Lt = op("<", Ast.cmpop.Lt)
    // val Gt = op(">", Ast.cmpop.Gt)
    // val Eq = op("==", Ast.cmpop.Eq)
    // val GtE = op(">=", Ast.cmpop.GtE)
    // val LtE = op("<=", Ast.cmpop.LtE)
    // val NotEq = op("<>" | "!=", Ast.cmpop.NotEq)
    // val comp_op = P( LtE|GtE|Eq|Gt|Lt|NotEq )

    alt((
        maybe_ws_op(ast::ComparisonOperator::LtE, "<="),
        maybe_ws_op(ast::ComparisonOperator::GtE, ">="),
        maybe_ws_op(ast::ComparisonOperator::Eq, "=="),
        maybe_ws_op(ast::ComparisonOperator::Lt, "<"),
        maybe_ws_op(ast::ComparisonOperator::Gt, ">"),
        maybe_ws_op(ast::ComparisonOperator::NotEq, "<>"),
        maybe_ws_op(ast::ComparisonOperator::NotEq, "!="),
    ))(input)
}

// val LShift = op("<<", Ast.operator.LShift)
// val RShift = op(">>", Ast.operator.RShift)

pub fn comparison_expr(input: Span) -> IResult<ast::Expression> {
    // val comparison: P[Ast.expr] = P( expr ~ (comp_op ~ expr).? ).map{
    //   case (lhs, None) => lhs
    //   case (lhs, Some(chunks)) =>
    //     val (op, rhs) = chunks
    //     Ast.expr.Compare(lhs, op, rhs)
    // }
    map(
        tuple((expr_expr, opt(tuple((comparison_operator, expr_expr))))),
        |e| match e {
            (lhs, None) => lhs,
            (lhs, Some((op, rhs))) => ast::Expression::Compare {
                left: Box::new(lhs),
                ops: op,
                right: Box::new(rhs),
            },
        },
    )(input)
}
