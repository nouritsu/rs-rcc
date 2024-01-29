use super::Operator;
use crate::{
    codegen::{Codegen, CodegenError},
    lexer::Span,
    parser::helper::label,
};
use std::collections::HashMap;

pub type Spanned<T> = (T, Span);

#[derive(Debug, Clone)]
pub enum Expr<'src> {
    LiteralInteger(u64),
    Variable(&'src str),
    Unary(Operator, Box<Spanned<Self>>),
    Binary(Box<Spanned<Self>>, Operator, Box<Spanned<Self>>),
}

impl<'src> Codegen<'src> for Spanned<Expr<'src>> {
    fn code_gen(
        self,
        i: &mut usize,
        sp: &mut isize,
        env: &mut HashMap<String, (isize, Span)>,
    ) -> Result<String, Spanned<CodegenError<'src>>> {
        Ok(match self.0 {
            Expr::LiteralInteger(i) => format!("mov ${}, %rax\n", i),
            Expr::Variable(name) => {
                let offset = match env.get(name) {
                    Some((offset, _)) => offset,
                    None => return Err((CodegenError::UndeclaredVariable(name), self.1)),
                };

                format!("mov {}(%rbp), %rax\n", offset)
            }

            /* Unary Expressions */
            Expr::Unary(Operator::Minus, rhs) => rhs.code_gen(i, sp, env)? + "neg %rax\n",
            Expr::Unary(Operator::LogicalNot, rhs) => {
                rhs.code_gen(i, sp, env)? + "cmpl $0, %rax\nmov $0, %rax\nsete %al\n"
            }
            Expr::Unary(Operator::BitwiseNot, rhs) => rhs.code_gen(i, sp, env)? + "not %rax\n",
            Expr::Unary(_, _) => unreachable!("reached unary _ branch in codegen"),

            /* Binary Expressions */
            Expr::Binary(lhs, Operator::Plus, rhs) => format!(
                "{}push %rax\n{}pop %rcx\nadd %rcx, %rax\n",
                lhs.code_gen(i, sp, env)?,
                rhs.code_gen(i, sp, env)?,
            ),
            Expr::Binary(lhs, Operator::Multiply, rhs) => format!(
                "{}push %rax\n{}pop %rcx\nimul %rcx, %rax\n",
                lhs.code_gen(i, sp, env)?,
                rhs.code_gen(i, sp, env)?,
            ),
            Expr::Binary(lhs, Operator::Minus, rhs) => format!(
                "{}push %rax\n{}pop %rcx\nsub %rcx, %rax\n",
                rhs.code_gen(i, sp, env)?,
                lhs.code_gen(i, sp, env)?,
            ),
            Expr::Binary(lhs, Operator::Divide, rhs) => format!(
                "{}push %rax\n{}pop %rcx\ncqo\nidiv %rcx\n",
                rhs.code_gen(i, sp, env)?,
                lhs.code_gen(i, sp, env)?,
            ),
            Expr::Binary(lhs, Operator::EqEq, rhs) => format!(
                "{}push %rax\n{}pop %rcx\ncmp %rax, %rcx\nmov $0, %rax\nsete %al\n",
                lhs.code_gen(i, sp, env)?,
                rhs.code_gen(i, sp, env)?,
            ),
            Expr::Binary(lhs, Operator::Ne, rhs) => format!(
                "{}push %rax\n{}pop %rcx\ncmp %rax, %rcx\nmov $0, %rax\nsetne %al\n",
                lhs.code_gen(i, sp, env)?,
                rhs.code_gen(i, sp, env)?,
            ),
            Expr::Binary(lhs, Operator::Ge, rhs) => format!(
                "{}push %rax\n{}pop %rcx\ncmp %rax, %rcx\nmov $0, %rax\nsetge %al\n",
                lhs.code_gen(i, sp, env)?,
                rhs.code_gen(i, sp, env)?,
            ),
            Expr::Binary(lhs, Operator::Gt, rhs) => format!(
                "{}push %rax\n{}pop %rcx\ncmp %rax, %rcx\nmov $0, %rax\nsetg %al\n",
                lhs.code_gen(i, sp, env)?,
                rhs.code_gen(i, sp, env)?,
            ),
            Expr::Binary(lhs, Operator::Le, rhs) => format!(
                "{}push %rax\n{}pop %rcx\ncmp %rax, %rcx\nmov $0, %rax\nsetle %al\n",
                lhs.code_gen(i, sp, env)?,
                rhs.code_gen(i, sp, env)?,
            ),
            Expr::Binary(lhs, Operator::Lt, rhs) => format!(
                "{}push %rax\n{}pop %rcx\ncmp %rax, %rcx\nmov $0, %rax\nsetl %al\n",
                lhs.code_gen(i, sp, env)?,
                rhs.code_gen(i, sp, env)?,
            ),
            Expr::Binary(lhs, Operator::LogicalAnd, rhs) => {
                let l1 = label(i);
                let l2 = label(i);

                format!(
                        "{}cmp $0, %rax\njne {}\njmp {}\n{}:\n{}cmp $0, %rax\nmov $0, %rax\nsetne %al\n{}:\n",
                        lhs.code_gen(i, sp, env)?,
                        l1, l2, l1,
                        rhs.code_gen(i, sp, env)?,
                        l2,
                    )
            }
            Expr::Binary(lhs, Operator::LogicalOr, rhs) => {
                let l1 = label(i);
                let l2 = label(i);

                format!(
                        "{}cmp $0, %rax\nje {}\nmov $1, %rax\njmp {}\n{}:\n{}cmp $0, %rax\nmov $0, %rax\n setne %al\n{}:\n",
                        lhs.code_gen(i, sp, env)?,
                        l1, l2, l1,
                        rhs.code_gen(i, sp, env)?,
                        l2
                    )
            }
            Expr::Binary(lhs, Operator::Eq, rhs) => {
                let var = match lhs.0.as_lvalue() {
                    Some(var) => var,
                    None => return Err((CodegenError::InvalidAssignmentTarget, lhs.1)),
                };

                let var_offset = match env.get(var) {
                    Some((offset, _)) => *offset,
                    None => return Err((CodegenError::UndeclaredVariable(var), lhs.1)),
                };

                format!(
                    "{}mov %rax, {}(%rbp)\n",
                    rhs.code_gen(i, sp, env)?,
                    var_offset,
                )
            }
            Expr::Binary(_, _, _) => unreachable!("reached binary _ branch in codegen"),
        })
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
