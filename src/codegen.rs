use crate::common::{Expr, Stmt};

pub type Program = Vec<Stmt>;

pub trait Codegen {
    fn code_gen(&self) -> String;
}

impl Codegen for Stmt {
    fn code_gen(&self) -> String {
        match self {
            Stmt::Return(expr) => {
                let v = match expr {
                    Expr::LiteralInteger(i) => i,
                    Expr::Variable(_) => todo!(),
                };
                format!("movl ${}, %eax\nret", v)
            }
            Stmt::Assign(_, _) => todo!(),
            Stmt::Function(name, body) => {
                5;
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
