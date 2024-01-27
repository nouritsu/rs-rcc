use super::Operator;
use crate::{codegen::Codegen, lexer::Span, parser::helper::label};

pub type Spanned<T> = (T, Span);

#[derive(Debug, Clone)]
pub enum Expr<'src> {
    LiteralInteger(u64),
    Variable(&'src str),
    Unary(Operator, Box<Spanned<Self>>),
    Binary(Box<Spanned<Self>>, Operator, Box<Spanned<Self>>),
    Error,
}

impl<'src> Codegen for Spanned<Expr<'src>> {
    fn code_gen(&self) -> String {
        let mut i = 0usize;
        match &self.0 {
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
                Operator::Plus | Operator::Multiply => {
                    format!(
                        "{}push %rax\n{}pop %rcx\n{} %rcx, %rax\n",
                        lhs.code_gen(),
                        rhs.code_gen(),
                        match op {
                            Operator::Plus => "add",
                            Operator::Multiply => "imul",
                            _ => unreachable!(),
                        }
                    )
                }

                Operator::Minus => {
                    format!(
                        "{}push %rax\n{}pop %rcx\nsub %rcx, %rax\n",
                        rhs.code_gen(),
                        lhs.code_gen()
                    )
                }

                Operator::Divide => {
                    format!(
                        "{}push %rax\n{}pop %rcx\ncqo\nidiv %rcx\n",
                        rhs.code_gen(),
                        lhs.code_gen()
                    )
                }

                Operator::Eq
                | Operator::Ne
                | Operator::Ge
                | Operator::Gt
                | Operator::Le
                | Operator::Lt => {
                    format!(
                        "{}push %rax\n{}pop %rcx\ncmp %rax, %rcx\nmov $0, %rax\n{} %al\n",
                        lhs.code_gen(),
                        rhs.code_gen(),
                        match op {
                            Operator::Eq => "sete",
                            Operator::Ne => "setne",
                            Operator::Ge => "setge",
                            Operator::Gt => "setg",
                            Operator::Le => "setle",
                            Operator::Lt => "setl",
                            _ => unreachable!(),
                        }
                    )
                }

                Operator::LogicalAnd => {
                    let l1 = label(&mut i);
                    let l2 = label(&mut i);

                    format!(
                        "{}cmp $0, %rax\njne {}\njmp {}\n{}:\n{}cmp $0, %rax\nmov $0, %rax\nsetne %al\n{}:\n",
                        lhs.code_gen(),
                        l1, l2, l1,
                        rhs.code_gen(),
                        l2,
                    )
                }

                Operator::LogicalOr => {
                    let l1 = label(&mut i);
                    let l2 = label(&mut i);

                    format!(
                        "{}cmp $0, %rax\nje {}\nmov $1, %rax\njmp {}\n{}:\n{}cmp $0, %rax\nmov $0, %rax\n setne %al\n{}:\n",
                        lhs.code_gen(),
                        l1, l2, l1,
                        rhs.code_gen(),
                        l2
                    )
                }
                _ => unreachable!("reached default branch of binary expr codegen"),
            },
            Expr::Variable(_) => todo!("variable expression"),
            Expr::Error => unreachable!("reached error branch of expr codegen"),
        }
    }
}
