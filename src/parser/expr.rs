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

            /* Unary Expressions */
            Expr::Unary(Operator::Minus, rhs) => rhs.code_gen(i) + "neg %rax\n",
            Expr::Unary(Operator::LogicalNot, rhs) => {
                rhs.code_gen(i) + "cmpl $0, %rax\nmov $0, %rax\nsete %al\n"
            }
            Expr::Unary(Operator::BitwiseNot, rhs) => rhs.code_gen(i) + "not %rax\n",
            Expr::Unary(_, _) => unreachable!("reached unary _ branch in codegen"),

            /* Binary Expressions */
            Expr::Binary(lhs, Operator::Plus, rhs) => format!(
                "{}push %rax\n{}pop %rcx\nadd %rcx, %rax\n",
                lhs.code_gen(i),
                rhs.code_gen(i),
            ),
            Expr::Binary(lhs, Operator::Multiply, rhs) => format!(
                "{}push %rax\n{}pop %rcx\nimul %rcx, %rax\n",
                lhs.code_gen(i),
                rhs.code_gen(i),
            ),
            Expr::Binary(lhs, Operator::Minus, rhs) => format!(
                "{}push %rax\n{}pop %rcx\nsub %rcx, %rax\n",
                rhs.code_gen(i),
                lhs.code_gen(i)
            ),
            Expr::Binary(lhs, Operator::Divide, rhs) => format!(
                "{}push %rax\n{}pop %rcx\ncqo\nidiv %rcx\n",
                rhs.code_gen(i),
                lhs.code_gen(i)
            ),
            Expr::Binary(lhs, Operator::EqEq, rhs) => format!(
                "{}push %rax\n{}pop %rcx\ncmp %rax, %rcx\nmov $0, %rax\nsete %al\n",
                lhs.code_gen(i),
                rhs.code_gen(i),
            ),
            Expr::Binary(lhs, Operator::Ne, rhs) => format!(
                "{}push %rax\n{}pop %rcx\ncmp %rax, %rcx\nmov $0, %rax\nsetne %al\n",
                lhs.code_gen(i),
                rhs.code_gen(i),
            ),
            Expr::Binary(lhs, Operator::Ge, rhs) => format!(
                "{}push %rax\n{}pop %rcx\ncmp %rax, %rcx\nmov $0, %rax\nsetge %al\n",
                lhs.code_gen(i),
                rhs.code_gen(i),
            ),
            Expr::Binary(lhs, Operator::Gt, rhs) => format!(
                "{}push %rax\n{}pop %rcx\ncmp %rax, %rcx\nmov $0, %rax\nsetg %al\n",
                lhs.code_gen(i),
                rhs.code_gen(i),
            ),
            Expr::Binary(lhs, Operator::Le, rhs) => format!(
                "{}push %rax\n{}pop %rcx\ncmp %rax, %rcx\nmov $0, %rax\nsetle %al\n",
                lhs.code_gen(i),
                rhs.code_gen(i),
            ),
            Expr::Binary(lhs, Operator::Lt, rhs) => format!(
                "{}push %rax\n{}pop %rcx\ncmp %rax, %rcx\nmov $0, %rax\nsetl %al\n",
                lhs.code_gen(i),
                rhs.code_gen(i),
            ),
            Expr::Binary(lhs, Operator::LogicalAnd, rhs) => {
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
            Expr::Binary(lhs, Operator::LogicalOr, rhs) => {
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
            Expr::Binary(_lhs, Operator::Eq, _rhs) => todo!("assignment operator"),
            Expr::Binary(_, _, _) => unreachable!("reached binary _ branch in codegen"),

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
