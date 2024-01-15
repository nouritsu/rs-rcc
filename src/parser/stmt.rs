use super::Expr;

#[derive(Debug)]
pub enum Stmt {
    Return(Expr),
    Assign(String, Expr),
    Function(String, Vec<Self>),
}
