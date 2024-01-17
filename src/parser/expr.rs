use super::Operator;

#[derive(Debug)]
pub enum Expr {
    LiteralInteger(u32),
    Unary(Operator, Box<Self>),
    Variable(String),
}
