use super::{expr::Spanned, Expr};
use crate::codegen::Codegen;

#[derive(Debug)]
pub enum Stmt<'src> {
    Return(Spanned<Expr<'src>>),
    Declare(&'src str, Option<Spanned<Expr<'src>>>),
    Expression(Spanned<Expr<'src>>),
    Function(&'src str, Vec<Spanned<Self>>),
}

impl<'src> Codegen for Spanned<Stmt<'src>> {
    fn code_gen(&self) -> String {
        match &self.0 {
            Stmt::Return(expr) => expr.code_gen() + "ret\n",
            Stmt::Declare(_, _) => todo!("declare statement"),
            Stmt::Expression(expr) => expr.code_gen(),
            Stmt::Function(name, body) => {
                format!(
                    ".globl {}\n{}:\n{}",
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
