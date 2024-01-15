#[derive(Debug)]
pub enum Expr {
    LiteralInteger(u32),
    Variable(String),
}
