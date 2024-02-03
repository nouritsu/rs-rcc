use super::{
    helper::{LabelKind, LabelTracker},
    BinaryOperator, Codegen, CodegenError, Desugar, Environment, Span, Spanned, UnaryOperator,
};

#[derive(Debug, Clone)]
pub enum Expr<'src> {
    LiteralInteger(u64),
    Variable(&'src str),
    Unary(UnaryOperator, Box<Spanned<Self>>),
    Binary(Box<Spanned<Self>>, BinaryOperator, Box<Spanned<Self>>),
    Ternary(Box<Spanned<Self>>, Box<Spanned<Self>>, Box<Spanned<Self>>),
}

impl<'src> Codegen<'src> for Vec<Spanned<Expr<'src>>> {
    fn code_gen(
        self,
        lt: &mut LabelTracker,
        env: &mut Environment<'src>,
    ) -> Result<String, Spanned<CodegenError<'src>>> {
        Ok(self
            .into_iter()
            .map(|expr| expr.code_gen(lt, env))
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .fold(String::new(), |s, x| s + &x))
    }
}

impl<'src> Codegen<'src> for Spanned<Expr<'src>> {
    fn code_gen(
        self,
        lt: &mut LabelTracker,
        env: &mut Environment<'src>,
    ) -> Result<String, Spanned<CodegenError<'src>>> {
        Ok(match self {
            (Expr::LiteralInteger(i), _) => format!("\tmov ${}, %rax\n", i),

            (Expr::Variable(name), span) => {
                let (offset, _) = env
                    .get(name)
                    .ok_or((CodegenError::UndeclaredVariable(name), span))?;

                format!("\tmov {}(%rbp), %rax\n", offset)
            }

            /* Unary */
            (Expr::Unary(UnaryOperator::Minus, rhs), _) => rhs.code_gen(lt, env)? + "\tneg %rax\n",

            (Expr::Unary(UnaryOperator::LogicalNot, rhs), _) => {
                rhs.code_gen(lt, env)? + "\tcmpl $0, %rax\n\tmov $0, %rax\n\tsete %al\n"
            }

            (Expr::Unary(UnaryOperator::BitwiseNot, rhs), _) => {
                rhs.code_gen(lt, env)? + "\tnot %rax\n"
            }

            /* Binary */
            // Math Os
            (Expr::Binary(lhs, BinaryOperator::Plus, rhs), _) => format!(
                "{}\tpush %rax\n{}\tpop %rcx\n\tadd %rcx, %rax\n",
                lhs.code_gen(lt, env)?,
                rhs.code_gen(lt, env)?,
            ),

            (Expr::Binary(lhs, BinaryOperator::Multiply, rhs), _) => format!(
                "{}\tpush %rax\n{}\tpop %rcx\n\timul %rcx, %rax\n",
                lhs.code_gen(lt, env)?,
                rhs.code_gen(lt, env)?,
            ),

            (Expr::Binary(lhs, BinaryOperator::Minus, rhs), _) => format!(
                "{}\tpush %rax\n{}\tpop %rcx\n\tsub %rcx, %rax\n",
                rhs.code_gen(lt, env)?,
                lhs.code_gen(lt, env)?,
            ),

            (Expr::Binary(lhs, BinaryOperator::Divide, rhs), _) => format!(
                "{}\tpush %rax\n{}\tpop %rcx\n\tcqo\n\tidiv %rcx\n",
                rhs.code_gen(lt, env)?,
                lhs.code_gen(lt, env)?,
            ),

            (Expr::Binary(lhs, BinaryOperator::Mod, rhs), _) => format!(
                "{}\tpush %rax\n{}\tpop %rcx\n\tcqo\n\tidiv %rcx\n\tmov %rdx, %rax\n",
                rhs.code_gen(lt, env)?,
                lhs.code_gen(lt, env)?,
            ),

            (Expr::Binary(lhs, BinaryOperator::EqEq, rhs), _) => format!(
                "{}\tpush %rax\n{}\tpop %rcx\n\tcmp %rax, %rcx\n\tmov $0, %rax\n\tsete %al\n",
                lhs.code_gen(lt, env)?,
                rhs.code_gen(lt, env)?,
            ),

            (Expr::Binary(lhs, BinaryOperator::Ne, rhs), _) => format!(
                "{}\tpush %rax\n{}\tpop %rcx\n\tcmp %rax, %rcx\n\tmov $0, %rax\n\tsetne %al\n",
                lhs.code_gen(lt, env)?,
                rhs.code_gen(lt, env)?,
            ),

            (Expr::Binary(lhs, BinaryOperator::Ge, rhs), _) => format!(
                "{}\tpush %rax\n{}\tpop %rcx\n\tcmp %rax, %rcx\n\tmov $0, %rax\n\tsetge %al\n",
                lhs.code_gen(lt, env)?,
                rhs.code_gen(lt, env)?,
            ),

            (Expr::Binary(lhs, BinaryOperator::Gt, rhs), _) => format!(
                "{}push %rax\n{}\tpop %rcx\n\tcmp %rax, %rcx\n\tmov $0, %rax\n\tsetg %al\n",
                lhs.code_gen(lt, env)?,
                rhs.code_gen(lt, env)?,
            ),

            (Expr::Binary(lhs, BinaryOperator::Le, rhs), _) => format!(
                "{}\tpush %rax\n{}\tpop %rcx\n\tcmp %rax, %rcx\n\tmov $0, %rax\n\tsetle %al\n",
                lhs.code_gen(lt, env)?,
                rhs.code_gen(lt, env)?,
            ),

            (Expr::Binary(lhs, BinaryOperator::Lt, rhs), _) => format!(
                "{}\tpush %rax\n{}\tpop %rcx\n\tcmp %rax, %rcx\n\tmov $0, %rax\n\tsetl %al\n",
                lhs.code_gen(lt, env)?,
                rhs.code_gen(lt, env)?,
            ),

            (Expr::Binary(lhs, BinaryOperator::LogicalAnd, rhs), _) => {
                let l1 = lt.create(LabelKind::And);
                let l2 = lt.create(LabelKind::AndShortCircuit);

                format!(
                        "{}\tcmp $0, %rax\n\tjne {}\n\tjmp {}\n{}:\n{}\tcmp $0, %rax\n\tmov $0, %rax\n\tsetne %al\n{}:\n",
                        lhs.code_gen(lt, env)?,
                        l1, l2, l1,
                        rhs.code_gen(lt, env)?,
                        l2,
                    )
            }

            (Expr::Binary(lhs, BinaryOperator::LogicalOr, rhs), _) => {
                let l1 = lt.create(LabelKind::Or);
                let l2 = lt.create(LabelKind::OrShortCircuit);

                format!(
                        "{}\tcmp $0, %rax\n\tje {}\n\tmov $1, %rax\n\tjmp {}\n{}:\n{}\tcmp $0, %rax\n\tmov $0, %rax\n\tsetne %al\n{}:\n",
                        lhs.code_gen(lt, env)?,
                        l1, l2, l1,
                        rhs.code_gen(lt, env)?,
                        l2
                    )
            }

            (Expr::Binary(lhs, BinaryOperator::BitwiseAnd, rhs), _) => format!(
                "{}\tpush %rax\n{}\tpop %rcx\n\tand %rcx, %rax\n",
                lhs.code_gen(lt, env)?,
                rhs.code_gen(lt, env)?,
            ),

            (Expr::Binary(lhs, BinaryOperator::BitwiseOr, rhs), _) => format!(
                "{}\tpush %rax\n{}\tpop %rcx\n\tor %rcx, %rax\n",
                lhs.code_gen(lt, env)?,
                rhs.code_gen(lt, env)?,
            ),

            (Expr::Binary(lhs, BinaryOperator::BitwiseXor, rhs), _) => format!(
                "{}\tpush %rax\n{}\tpop %rcx\n\txor %rcx, %rax\n",
                lhs.code_gen(lt, env)?,
                rhs.code_gen(lt, env)?,
            ),

            (Expr::Binary(lhs, BinaryOperator::LeftShift, rhs), _) => format!(
                "{}\tpush %rax\n{}\tpop %rcx\n\tshl %rcx, %rax\n",
                rhs.code_gen(lt, env)?,
                lhs.code_gen(lt, env)?,
            ),

            (Expr::Binary(lhs, BinaryOperator::RightShift, rhs), _) => format!(
                "{}\tpush %rax\n{}\tpop %rcx\n\tshr %rcx, %rax\n",
                rhs.code_gen(lt, env)?,
                lhs.code_gen(lt, env)?,
            ),

            (Expr::Binary(lhs, BinaryOperator::Eq, rhs), _) => {
                let var = lhs
                    .0
                    .as_lvalue()
                    .ok_or((CodegenError::InvalidAssignmentTarget, lhs.1))?;

                let (var_offset, _) = env
                    .get(var)
                    .ok_or((CodegenError::UndeclaredVariable(var), lhs.1))?;

                format!(
                    "{}\tmov %rax, {}(%rbp)\n",
                    rhs.code_gen(lt, env)?,
                    var_offset,
                )
            }

            (Expr::Binary(lhs, op, rhs), span) if op.is_compound_assignment() => {
                (Expr::Binary(lhs, op, rhs), span)
                    .desugar()
                    .expect("infallible")
                    .code_gen(lt, env)?
            }

            (Expr::Binary(_, _, _), _) => unreachable!("reached binary _ branch in codegen"),

            (Expr::Ternary(_condition, _a, _b), _span) => todo!(),
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
                        BinaryOperator::Eq,
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
    pub fn new_unary(op: UnaryOperator, rhs: Spanned<Self>, span: Span) -> Spanned<Self> {
        (Expr::Unary(op, Box::new(rhs)), span)
    }

    pub fn new_binary(
        lhs: Spanned<Self>,
        op: BinaryOperator,
        rhs: Spanned<Self>,
        span: Span,
    ) -> Spanned<Self> {
        (Expr::Binary(Box::new(lhs), op, Box::new(rhs)), span)
    }

    pub fn new_ternary(
        cond: Spanned<Self>,
        a: Spanned<Self>,
        b: Spanned<Self>,
        span: Span,
    ) -> Spanned<Self> {
        (
            Expr::Ternary(Box::new(cond), Box::new(a), Box::new(b)),
            span,
        )
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_as_lvalue() {
        let expr = Expr::Variable("x");
        assert_eq!(expr.as_lvalue(), Some("x"));

        let expr = Expr::LiteralInteger(42);
        assert_eq!(expr.as_lvalue(), None);
    }
}
