use super::{expr::Spanned, Expr};
use crate::codegen::Codegen;

#[derive(Debug)]
pub enum Stmt<'src> {
    Return(Spanned<Expr<'src>>),
    Declare(&'src str, Option<Spanned<Expr<'src>>>),
    Expression(Spanned<Expr<'src>>),
    Function(&'src str, Vec<Spanned<Self>>),
}

impl<'src> Codegen for Vec<Spanned<Stmt<'src>>> {
    fn code_gen(&self, label_idx: &mut usize) -> String {
        self.iter()
            .map(|stmt| stmt.code_gen(label_idx))
            .fold(String::new(), |s, x| s + &x)
    }
}

impl<'src> Codegen for Spanned<Stmt<'src>> {
    fn code_gen(&self, i: &mut usize) -> String {
        match &self.0 {
            Stmt::Return(expr) => expr.code_gen(i) + "ret\n",
            Stmt::Declare(_, _) => todo!("declare statement"),
            Stmt::Expression(expr) => expr.code_gen(i),
            Stmt::Function(name, body) => {
                format!(
                    ".globl {}\n{}:\n{}",
                    name,
                    name,
                    body.iter()
                        .map(|stmt| stmt.code_gen(i))
                        .fold(String::new(), |acc, s| acc + &s)
                )
            }
        }
    }
}
