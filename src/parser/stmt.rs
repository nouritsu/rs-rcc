use super::Expr;
use crate::codegen::Codegen;

#[derive(Debug)]
pub enum Stmt {
    Return(Expr),
    Assign(String, Expr),
    Function(String, Vec<Self>),
}

impl Codegen for Stmt {
    fn code_gen(&self) -> String {
        match self {
            Stmt::Return(expr) => expr.code_gen() + "ret\n",
            Stmt::Assign(_, _) => todo!("assign statement"),
            Stmt::Function(name, body) => {
                format!(
                    " .globl {}\n{}:\n{}",
                    name,
                    name,
                    body.iter()
                        .map(|stmt| stmt.code_gen())
                        .fold(String::new(), |acc, s| acc + &s)
                )
            }
        }
    }
}
