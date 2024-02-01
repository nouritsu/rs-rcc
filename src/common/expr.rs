use super::{env::Environment, helper::label, Codegen, CodegenError, Desugar, Operator, Spanned};

#[derive(Debug, Clone)]
pub enum Expr<'src> {
    LiteralInteger(u64),
    Variable(&'src str),
    Unary(Operator, Box<Spanned<Self>>),
    Binary(Box<Spanned<Self>>, Operator, Box<Spanned<Self>>),
}

impl<'src> Codegen<'src> for Vec<Spanned<Expr<'src>>> {
    fn code_gen(
        self,
        i: &mut usize,
        env: &mut Environment<'src>,
    ) -> Result<String, super::Spanned<CodegenError<'src>>> {
        Ok(self
            .into_iter()
            .map(|expr| expr.code_gen(i, env))
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .fold(String::new(), |s, x| s + &x))
    }
}

impl<'src> Codegen<'src> for Spanned<Expr<'src>> {
    fn code_gen(
        self,
        i: &mut usize,
        env: &mut Environment<'src>,
    ) -> Result<String, Spanned<CodegenError<'src>>> {
        Ok(match self {
            (Expr::LiteralInteger(i), _) => format!("mov ${}, %rax\n", i),

            (Expr::Variable(name), span) => {
                let (offset, _) = env
                    .get(name)
                    .ok_or((CodegenError::UndeclaredVariable(name), span))?;

                format!("mov {}(%rbp), %rax\n", offset)
            }

            /* Unary */
            (Expr::Unary(Operator::Minus, rhs), _) => rhs.code_gen(i, env)? + "neg %rax\n",

            (Expr::Unary(Operator::LogicalNot, rhs), _) => {
                rhs.code_gen(i, env)? + "cmpl $0, %rax\nmov $0, %rax\nsete %al\n"
            }

            (Expr::Unary(Operator::BitwiseNot, rhs), _) => rhs.code_gen(i, env)? + "not %rax\n",

            (Expr::Unary(_, _), _) => unreachable!("reached unary _ branch in codegen"),

            /* Binary */
            // Math Os
            (Expr::Binary(lhs, Operator::Plus, rhs), _) => format!(
                "{}push %rax\n{}pop %rcx\nadd %rcx, %rax\n",
                lhs.code_gen(i, env)?,
                rhs.code_gen(i, env)?,
            ),

            (Expr::Binary(lhs, Operator::Multiply, rhs), _) => format!(
                "{}push %rax\n{}pop %rcx\nimul %rcx, %rax\n",
                lhs.code_gen(i, env)?,
                rhs.code_gen(i, env)?,
            ),

            (Expr::Binary(lhs, Operator::Minus, rhs), _) => format!(
                "{}push %rax\n{}pop %rcx\nsub %rcx, %rax\n",
                rhs.code_gen(i, env)?,
                lhs.code_gen(i, env)?,
            ),

            (Expr::Binary(lhs, Operator::Divide, rhs), _) => format!(
                "{}push %rax\n{}pop %rcx\ncqo\nidiv %rcx\n",
                rhs.code_gen(i, env)?,
                lhs.code_gen(i, env)?,
            ),

            (Expr::Binary(lhs, Operator::Mod, rhs), _) => format!(
                "{}push %rax\n{}pop %rcx\ncqo\nidiv %rcx\nmov %rdx, %rax\n",
                rhs.code_gen(i, env)?,
                lhs.code_gen(i, env)?,
            ),

            (Expr::Binary(lhs, Operator::EqEq, rhs), _) => format!(
                "{}push %rax\n{}pop %rcx\ncmp %rax, %rcx\nmov $0, %rax\nsete %al\n",
                lhs.code_gen(i, env)?,
                rhs.code_gen(i, env)?,
            ),

            (Expr::Binary(lhs, Operator::Ne, rhs), _) => format!(
                "{}push %rax\n{}pop %rcx\ncmp %rax, %rcx\nmov $0, %rax\nsetne %al\n",
                lhs.code_gen(i, env)?,
                rhs.code_gen(i, env)?,
            ),

            (Expr::Binary(lhs, Operator::Ge, rhs), _) => format!(
                "{}push %rax\n{}pop %rcx\ncmp %rax, %rcx\nmov $0, %rax\nsetge %al\n",
                lhs.code_gen(i, env)?,
                rhs.code_gen(i, env)?,
            ),

            (Expr::Binary(lhs, Operator::Gt, rhs), _) => format!(
                "{}push %rax\n{}pop %rcx\ncmp %rax, %rcx\nmov $0, %rax\nsetg %al\n",
                lhs.code_gen(i, env)?,
                rhs.code_gen(i, env)?,
            ),

            (Expr::Binary(lhs, Operator::Le, rhs), _) => format!(
                "{}push %rax\n{}pop %rcx\ncmp %rax, %rcx\nmov $0, %rax\nsetle %al\n",
                lhs.code_gen(i, env)?,
                rhs.code_gen(i, env)?,
            ),

            (Expr::Binary(lhs, Operator::Lt, rhs), _) => format!(
                "{}push %rax\n{}pop %rcx\ncmp %rax, %rcx\nmov $0, %rax\nsetl %al\n",
                lhs.code_gen(i, env)?,
                rhs.code_gen(i, env)?,
            ),

            (Expr::Binary(lhs, Operator::LogicalAnd, rhs), _) => {
                let l1 = label(i);
                let l2 = label(i);

                format!(
                        "{}cmp $0, %rax\njne {}\njmp {}\n{}:\n{}cmp $0, %rax\nmov $0, %rax\nsetne %al\n{}:\n",
                        lhs.code_gen(i, env)?,
                        l1, l2, l1,
                        rhs.code_gen(i, env)?,
                        l2,
                    )
            }

            (Expr::Binary(lhs, Operator::LogicalOr, rhs), _) => {
                let l1 = label(i);
                let l2 = label(i);

                format!(
                        "{}cmp $0, %rax\nje {}\nmov $1, %rax\njmp {}\n{}:\n{}cmp $0, %rax\nmov $0, %rax\n setne %al\n{}:\n",
                        lhs.code_gen(i, env)?,
                        l1, l2, l1,
                        rhs.code_gen(i, env)?,
                        l2
                    )
            }

            (Expr::Binary(lhs, Operator::BitwiseAnd, rhs), _) => format!(
                "{}push %rax\n{}pop %rcx\nand %rcx, %rax\n",
                lhs.code_gen(i, env)?,
                rhs.code_gen(i, env)?,
            ),

            (Expr::Binary(_lhs, Operator::BitwiseOr, _rhs), _) => todo!(),

            (Expr::Binary(_lhs, Operator::BitwiseXor, _rhs), _) => todo!(),

            (Expr::Binary(_lhs, Operator::LeftShift, _rhs), _) => todo!(),

            (Expr::Binary(_lhs, Operator::RightShift, _rhs), _) => todo!(),

            (Expr::Binary(lhs, Operator::Eq, rhs), _) => {
                let var = lhs
                    .0
                    .as_lvalue()
                    .ok_or((CodegenError::InvalidAssignmentTarget, lhs.1))?;

                let (var_offset, _) = env
                    .get(var)
                    .ok_or((CodegenError::UndeclaredVariable(var), lhs.1))?;

                format!("{}mov %rax, {}(%rbp)\n", rhs.code_gen(i, env)?, var_offset,)
            }

            (Expr::Binary(lhs, op, rhs), _) if op.is_compound_assignment() => {
                (Expr::Binary(lhs, op, rhs), self.1)
                    .desugar()
                    .expect("infallible")
                    .code_gen(i, env)?
            }

            (Expr::Binary(_, _, _), _) => unreachable!("reached binary _ branch in codegen"),
        })
    }
}

impl<'src> Desugar<Spanned<Expr<'src>>> for Spanned<Expr<'src>> {
    fn desugar(self) -> Option<Vec<Spanned<Expr<'src>>>> {
        Some(match self {
            (Expr::Binary(lhs, op, rhs), span) if op.is_compound_assignment() => {
                vec![(
                    Expr::Binary(
                        lhs.clone(),
                        Operator::Eq,
                        Box::new((Expr::Binary(lhs, op.compound_to_operator()?, rhs), span)),
                    ),
                    span,
                )]
            }

            _ => return None,
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
