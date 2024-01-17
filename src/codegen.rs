use crate::common::{Expr, Operator, Stmt};

pub type Program = Vec<Stmt>;

pub trait Codegen {
    fn code_gen(&self) -> String;
}

impl Codegen for Expr {
    fn code_gen(&self) -> String {
        match self {
            Expr::LiteralInteger(i) => format!("movl ${}, %eax\n", i),
            Expr::Unary(op, expr) => {
                expr.code_gen()
                    + match op {
                        Operator::Minus => "neg %eax\n",
                        Operator::LogicalNot => "cmpl $0, %eax\nmovl $0, %eax\nsete %al\n",
                        Operator::BitwiseNot => "not %eax\n",
                    }
            }
            Expr::Variable(_) => todo!(),
        }
    }
}

impl Codegen for Stmt {
    fn code_gen(&self) -> String {
        match self {
            Stmt::Return(expr) => expr.code_gen() + "ret\n",
            Stmt::Assign(_, _) => todo!(),
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
