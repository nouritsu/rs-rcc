use super::Operator;
use crate::codegen::Codegen;

#[derive(Debug)]
pub enum Expr {
    LiteralInteger(u64),
    Variable(String),
    Unary(Operator, Box<Self>),
    Binary(Box<Self>, Operator, Box<Self>),
}

impl Codegen for Expr {
    fn code_gen(&self) -> String {
        match self {
            Expr::LiteralInteger(i) => format!("mov ${}, %rax\n", i),
            Expr::Unary(op, rhs) => {
                rhs.code_gen()
                    + match op {
                        Operator::Minus => "neg %rax\n",
                        Operator::LogicalNot => "cmpl $0, %rax\nmov $0, %rax\nsete %al\n",
                        Operator::BitwiseNot => "not %rax\n",
                        _ => unreachable!("reached default branch of unary expression codegen"),
                    }
            }
            Expr::Binary(lhs, op, rhs) => match op {
                Operator::Plus => {
                    format!(
                        "{}push %rax\n{}pop %rcx\nadd %rcx, %rax\n",
                        lhs.code_gen(),
                        rhs.code_gen()
                    )
                }
                Operator::Minus => {
                    format!(
                        "{}push %rax\n{}pop %rcx\nsub %rcx, %rax\n",
                        rhs.code_gen(),
                        lhs.code_gen()
                    )
                }
                Operator::Multiply => format!(
                    "{}push %rax\n{}pop %rcx\nimul %rcx, %rax\n",
                    lhs.code_gen(),
                    rhs.code_gen()
                ),
                Operator::Divide => {
                    format!(
                        "{}push %rax\n{}pop %rcx\ncqo\nidiv %rcx\n",
                        rhs.code_gen(),
                        lhs.code_gen()
                    )
                }
                _ => unreachable!("reached default branch of binary expression codegen"),
            },
            Expr::Variable(_) => todo!("variable expression"),
        }
    }
}
