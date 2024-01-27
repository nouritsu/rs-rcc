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
    fn code_gen(&self, i: &mut usize) -> String {
        match &self.0 {
            Expr::LiteralInteger(i) => format!("mov ${}, %rax\n", i),

            Expr::Unary(op, rhs) => {
                rhs.code_gen(i)
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
                        lhs.code_gen(i),
                        rhs.code_gen(i),
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
                        rhs.code_gen(i),
                        lhs.code_gen(i)
                    )
                }
                Operator::Divide => {
                    format!(
                        "{}push %rax\n{}pop %rcx\ncqo\nidiv %rcx\n",
                        rhs.code_gen(i),
                        lhs.code_gen(i)
                    )
                }
                Operator::EqEq
                | Operator::Ne
                | Operator::Ge
                | Operator::Gt
                | Operator::Le
                | Operator::Lt => {
                    format!(
                        "{}push %rax\n{}pop %rcx\ncmp %rax, %rcx\nmov $0, %rax\n{} %al\n",
                        lhs.code_gen(i),
                        rhs.code_gen(i),
                        match op {
                            Operator::EqEq => "sete",
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
                    let l1 = label(i);
                    let l2 = label(i);

                    format!(
                        "{}cmp $0, %rax\njne {}\njmp {}\n{}:\n{}cmp $0, %rax\nmov $0, %rax\nsetne %al\n{}:\n",
                        lhs.code_gen(i),
                        l1, l2, l1,
                        rhs.code_gen(i),
                        l2,
                    )
                }
                Operator::LogicalOr => {
                    let l1 = label(i);
                    let l2 = label(i);

                    format!(
                        "{}cmp $0, %rax\nje {}\nmov $1, %rax\njmp {}\n{}:\n{}cmp $0, %rax\nmov $0, %rax\n setne %al\n{}:\n",
                        lhs.code_gen(i),
                        l1, l2, l1,
                        rhs.code_gen(i),
                        l2
                    )
                }
                Operator::LogicalNot => unreachable!("reached logical not in binary expr codegen"),
                Operator::BitwiseNot => unreachable!("reached bitwise not in binary expr codegen"),
                Operator::Eq => todo!("assignment operator"),
            },

            Expr::Variable(_) => todo!("variable expression"),

            Expr::Error => unreachable!("reached error branch of expr codegen"),
        }
    }
}

impl<'src> Expr<'src> {
    pub fn as_lvalue(&self) -> Option<&'src str> {
        match self {
            Expr::Variable(s) => Some(s),
            _ => None,
        }
    }
}
