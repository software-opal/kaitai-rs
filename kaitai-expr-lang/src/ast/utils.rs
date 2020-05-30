use super::Expression;

impl Expression {
    pub fn is_constant(&self) -> bool {
        match self {
            Expression::IntNum(_) => true,
            Expression::FloatNum(_) => true,
            Expression::Str(_) => true,
            Expression::Bool(_) => true,
            _ => false,
            // Expression::ByteSizeOfType(_) => {}
            // Expression::BitSizeOfType(_) => {}
            // Expression::Name(_) => {}
            // Expression::List(_) => {}
            // Expression::BoolOp { op, values } => {}
            // Expression::BinOp { left, op, right } => {}
            // Expression::UnaryOp { op, operand } => {}
            // Expression::IfExp { condition, if_true, if_false } => {}
            // Expression::Compare { left, ops, right } => {}
            // Expression::Call { func, args } => {}
            // Expression::EnumByLabel { enum_name, label, in_type } => {}
            // Expression::EnumById { enum_name, id, in_type } => {}
            // Expression::Attribute { value, attr } => {}
            // Expression::CastToType { value, type_name } => {}
            // Expression::Subscript { value, idx } => {}
        }
    }
    pub fn simplify(self) -> Self {
        match self {
            _ => self,
            // Expression::IntNum(_) => {}
            // Expression::FloatNum(_) => {}
            // Expression::Str(_) => {}
            // Expression::Bool(_) => {}
            // Expression::ByteSizeOfType(_) => {}
            // Expression::BitSizeOfType(_) => {}
            // Expression::Name(_) => {}
            // Expression::List(_) => {}
            // Expression::BoolOp { op, values } => {}
            // Expression::BinOp { left, op, right } => {}
            // Expression::UnaryOp { op, operand } => {}
            // Expression::IfExp { condition, if_true, if_false } => {}
            // Expression::Compare { left, ops, right } => {}
            // Expression::Call { func, args } => {}
            // Expression::EnumByLabel { enum_name, label, in_type } => {}
            // Expression::EnumById { enum_name, id, in_type } => {}
            // Expression::Attribute { value, attr } => {}
            // Expression::CastToType { value, type_name } => {}
            // Expression::Subscript { value, idx } => {}
        }
    }
}
