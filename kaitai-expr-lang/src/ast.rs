pub type IntegerType = u64;
pub type FloatType = f64;

pub mod func;
pub mod utils;

#[derive(Clone, Debug, PartialEq)]
pub struct Identifier {
    pub name: String,
}
#[derive(Clone, Debug, PartialEq)]
pub struct TypeId {
    pub absolute: bool,
    pub names: Vec<String>,
    pub is_array: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expression {
    IntNum(IntegerType),
    FloatNum(FloatType),
    Str(String),
    Bool(bool),
    ByteSizeOfType(TypeId),
    BitSizeOfType(TypeId),
    Name(Identifier),
    List(Vec<Expression>),
    BoolOp {
        op: BooleanOperator,
        values: Vec<Expression>,
    },
    BinOp {
        left: Box<Expression>,
        op: Operator,
        right: Box<Expression>,
    },
    UnaryOp {
        op: UnaryOperator,
        operand: Box<Expression>,
    },
    IfExp {
        condition: Box<Expression>,
        if_true: Box<Expression>,
        if_false: Box<Expression>,
    },
    Compare {
        left: Box<Expression>,
        ops: ComparisonOperator,
        right: Box<Expression>,
    },
    Call {
        func: Box<Expression>,
        args: Vec<Expression>,
    },
    EnumByLabel {
        enum_name: Identifier,
        label: Identifier,
        in_type: TypeId,
    },
    EnumById {
        enum_name: Identifier,
        id: Box<Expression>,
        in_type: TypeId,
    },
    Attribute {
        value: Box<Expression>,
        attr: Identifier,
    },
    CastToType {
        value: Box<Expression>,
        type_name: TypeId,
    },
    Subscript {
        value: Box<Expression>,
        idx: Box<Expression>,
    },
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BooleanOperator {
    And,
    Or,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Operator {
    Add,
    Sub,
    Mult,
    Div,
    Mod,
    LShift,
    RShift,
    BitOr,
    BitXor,
    BitAnd,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UnaryOperator {
    Invert,
    Not,
    Minus,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ComparisonOperator {
    Eq,
    NotEq,
    Lt,
    LtE,
    Gt,
    GtE,
}
