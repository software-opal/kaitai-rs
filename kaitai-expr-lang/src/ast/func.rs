
    #![allow(non_snake_case)]
    pub use super::Expression::{
        BitSizeOfType, Bool, ByteSizeOfType, FloatNum, IntNum, List, Name, Str,
    };
    use super::*;

    pub fn BoolOp(op: BooleanOperator, values: Vec<Expression>) -> Expression {
        Expression::BoolOp { op, values }
    }
    pub fn BinOp(left: Box<Expression>, op: Operator, right: Box<Expression>) -> Expression {
        Expression::BinOp { left, op, right }
    }
    pub fn UnaryOp(op: UnaryOperator, operand: Box<Expression>) -> Expression {
        Expression::UnaryOp { op, operand }
    }
    pub fn IfExp(
        condition: Box<Expression>,
        if_true: Box<Expression>,
        if_false: Box<Expression>,
    ) -> Expression {
        Expression::IfExp {
            condition,
            if_true,
            if_false,
        }
    }

    pub fn Compare(
        left: Box<Expression>,
        ops: ComparisonOperator,
        right: Box<Expression>,
    ) -> Expression {
        Expression::Compare { left, ops, right }
    }
    pub fn Call(func: Box<Expression>, args: Vec<Expression>) -> Expression {
        Expression::Call { func, args }
    }
    pub fn EnumByLabel(enum_name: Identifier, label: Identifier, in_type: TypeId) -> Expression {
        Expression::EnumByLabel {
            enum_name,
            label,
            in_type,
        }
    }
    pub fn EnumById(enum_name: Identifier, id: Box<Expression>, in_type: TypeId) -> Expression {
        Expression::EnumById {
            enum_name,
            id,
            in_type,
        }
    }
    pub fn Attribute(value: Box<Expression>, attr: Identifier) -> Expression {
        Expression::Attribute { value, attr }
    }
    pub fn CastToType(value: Box<Expression>, type_name: TypeId) -> Expression {
        Expression::CastToType { value, type_name }
    }
    pub fn Subscript(value: Box<Expression>, idx: Box<Expression>) -> Expression {
        Expression::Subscript { value, idx }
    }
