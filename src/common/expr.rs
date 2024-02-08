use super::{
    emitter::Emitter,
    label_tracker::{LabelKind, LabelTracker},
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
        em: &mut Emitter,
        env: &mut Environment<'src>,
    ) -> Result<(), Spanned<CodegenError<'src>>> {
        self.into_iter()
            .map(|expr| expr.code_gen(lt, em, env))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(())
    }
}

impl<'src> Codegen<'src> for Spanned<Expr<'src>> {
    fn code_gen(
        self,
        lt: &mut LabelTracker,
        em: &mut Emitter,
        env: &mut Environment<'src>,
    ) -> Result<(), Spanned<CodegenError<'src>>> {
        match self {
            (Expr::LiteralInteger(i), _) => {
                em.emit_instr(&format!("mov ${}, %rax", i));
            }

            (Expr::Variable(name), span) => {
                let (offset, _) = env
                    .get(name)
                    .ok_or((CodegenError::UndeclaredVariable(name), span))?;

                em.emit_instr(&format!("mov {}(%rbp), %rax", offset));
            }

            /* Unary */
            (Expr::Unary(UnaryOperator::Plus, rhs), _) => rhs.code_gen(lt, em, env)?,

            (Expr::Unary(UnaryOperator::Minus, rhs), _) => {
                rhs.code_gen(lt, em, env)?;
                em.emit_instr("neg %rax");
            }

            (Expr::Unary(UnaryOperator::LogicalNot, rhs), _) => {
                rhs.code_gen(lt, em, env)?;
                em.emit_instr("cmpl $0, %rax");
                em.emit_instr("mov $0, %rax");
                em.emit_instr("sete %al");
            }

            (Expr::Unary(UnaryOperator::BitwiseNot, rhs), _) => {
                rhs.code_gen(lt, em, env)?;
                em.emit_instr("not %rax");
            }

            /* Binary */
            // Math Ops
            (Expr::Binary(lhs, BinaryOperator::Plus, rhs), _) => {
                lhs.code_gen(lt, em, env)?;
                em.emit_instr("push %rax");
                rhs.code_gen(lt, em, env)?;
                em.emit_instr("pop %rcx");
                em.emit_instr("add %rcx, %rax");
            }

            (Expr::Binary(lhs, BinaryOperator::Multiply, rhs), _) => {
                lhs.code_gen(lt, em, env)?;
                em.emit_instr("push %rax");
                rhs.code_gen(lt, em, env)?;
                em.emit_instr("pop %rcx");
                em.emit_instr("imul %rcx, %rax");
            }

            (Expr::Binary(lhs, BinaryOperator::Minus, rhs), _) => {
                rhs.code_gen(lt, em, env)?;
                em.emit_instr("push %rax");
                lhs.code_gen(lt, em, env)?;
                em.emit_instr("pop %rcx");
                em.emit_instr("sub %rax, %rcx");
            }

            (Expr::Binary(lhs, BinaryOperator::Divide, rhs), _) => {
                rhs.code_gen(lt, em, env)?;
                em.emit_instr("push %rax");
                lhs.code_gen(lt, em, env)?;
                em.emit_instr("pop %rcx");
                em.emit_instr("cqo");
                em.emit_instr("idiv %rcx");
            }

            (Expr::Binary(lhs, BinaryOperator::Mod, rhs), _) => {
                rhs.code_gen(lt, em, env)?;
                em.emit_instr("push %rax");
                lhs.code_gen(lt, em, env)?;
                em.emit_instr("pop %rcx");
                em.emit_instr("cqo");
                em.emit_instr("idiv %rcx");
                em.emit_instr("mov %rdx, %rax");
            }

            (Expr::Binary(lhs, BinaryOperator::EqEq, rhs), _) => {
                lhs.code_gen(lt, em, env)?;
                em.emit_instr("push %rax");
                rhs.code_gen(lt, em, env)?;
                em.emit_instr("pop %rcx");
                em.emit_instr("cmp %rax, %rcx");
                em.emit_instr("mov $0, %rax");
                em.emit_instr("sete %al");
            }

            (Expr::Binary(lhs, BinaryOperator::Ne, rhs), _) => {
                lhs.code_gen(lt, em, env)?;
                em.emit_instr("push %rax");
                rhs.code_gen(lt, em, env)?;
                em.emit_instr("pop %rcx");
                em.emit_instr("cmp %rax, %rcx");
                em.emit_instr("mov $0, %rax");
                em.emit_instr("setne %al");
            }

            (Expr::Binary(lhs, BinaryOperator::Ge, rhs), _) => {
                lhs.code_gen(lt, em, env)?;
                em.emit_instr("push %rax");
                rhs.code_gen(lt, em, env)?;
                em.emit_instr("pop %rcx");
                em.emit_instr("cmp %rax, %rcx");
                em.emit_instr("mov $0, %rax");
                em.emit_instr("setge %al");
            }

            (Expr::Binary(lhs, BinaryOperator::Gt, rhs), _) => {
                lhs.code_gen(lt, em, env)?;
                em.emit_instr("push %rax");
                rhs.code_gen(lt, em, env)?;
                em.emit_instr("pop %rcx");
                em.emit_instr("cmp %rax, %rcx");
                em.emit_instr("mov $0, %rax");
                em.emit_instr("setg %al");
            }

            (Expr::Binary(lhs, BinaryOperator::Le, rhs), _) => {
                lhs.code_gen(lt, em, env)?;
                em.emit_instr("push %rax");
                rhs.code_gen(lt, em, env)?;
                em.emit_instr("pop %rcx");
                em.emit_instr("cmp %rax, %rcx");
                em.emit_instr("mov $0, %rax");
                em.emit_instr("setle %al");
            }

            (Expr::Binary(lhs, BinaryOperator::Lt, rhs), _) => {
                lhs.code_gen(lt, em, env)?;
                em.emit_instr("push %rax");
                rhs.code_gen(lt, em, env)?;
                em.emit_instr("pop %rcx");
                em.emit_instr("cmp %rax, %rcx");
                em.emit_instr("mov $0, %rax");
                em.emit_instr("setl %al");
            }

            (Expr::Binary(lhs, BinaryOperator::LogicalAnd, rhs), _) => {
                let l1 = lt.create(LabelKind::And);
                let l2 = lt.create(LabelKind::AndShortCircuit);

                lhs.code_gen(lt, em, env)?;
                em.emit_instr("cmp $0, %rax");
                em.emit_instr(&format!("jne {}", l1));
                em.emit_instr(&format!("jmp {}", l2));
                em.emit_label(&l1);
                rhs.code_gen(lt, em, env)?;
                em.emit_instr("cmp $0, %rax");
                em.emit_instr("mov $0, %rax");
                em.emit_instr("setne %al");
                em.emit_label(&l2);
            }

            (Expr::Binary(lhs, BinaryOperator::LogicalOr, rhs), _) => {
                let l1 = lt.create(LabelKind::Or);
                let l2 = lt.create(LabelKind::OrShortCircuit);

                lhs.code_gen(lt, em, env)?;
                em.emit_instr("cmp $0, %rax");
                em.emit_instr(&format!("je {}", l1));
                em.emit_instr("mov $1, %rax");
                em.emit_instr(&format!("jmp {}", l2));
                em.emit_label(&l1);
                rhs.code_gen(lt, em, env)?;
                em.emit_instr("cmp $0, %rax");
                em.emit_instr("mov $0, %rax");
                em.emit_instr("setne %al");
                em.emit_label(&l2);
            }

            (Expr::Binary(lhs, BinaryOperator::BitwiseAnd, rhs), _) => {
                lhs.code_gen(lt, em, env)?;
                em.emit_instr("push %rax");
                rhs.code_gen(lt, em, env)?;
                em.emit_instr("pop %rcx");
                em.emit_instr("and %rcx, %rax");
            }

            (Expr::Binary(lhs, BinaryOperator::BitwiseOr, rhs), _) => {
                lhs.code_gen(lt, em, env)?;
                em.emit_instr("push %rax");
                rhs.code_gen(lt, em, env)?;
                em.emit_instr("pop %rcx");
                em.emit_instr("or %rcx, %rax");
            }

            (Expr::Binary(lhs, BinaryOperator::BitwiseXor, rhs), _) => {
                lhs.code_gen(lt, em, env)?;
                em.emit_instr("push %rax");
                rhs.code_gen(lt, em, env)?;
                em.emit_instr("pop %rcx");
                em.emit_instr("xor %rcx, %rax");
            }

            (Expr::Binary(lhs, BinaryOperator::LeftShift, rhs), _) => {
                rhs.code_gen(lt, em, env)?;
                em.emit_instr("push %rax");
                lhs.code_gen(lt, em, env)?;
                em.emit_instr("pop %rcx");
                em.emit_instr("shl %rcx, %rax");
            }

            (Expr::Binary(lhs, BinaryOperator::RightShift, rhs), _) => {
                rhs.code_gen(lt, em, env)?;
                em.emit_instr("push %rax");
                lhs.code_gen(lt, em, env)?;
                em.emit_instr("pop %rcx");
                em.emit_instr("shr %rcx, %rax");
            }

            (Expr::Binary(lhs, BinaryOperator::Eq, rhs), _) => {
                let var = lhs
                    .0
                    .as_lvalue()
                    .ok_or((CodegenError::InvalidAssignmentTarget, lhs.1))?;

                let (var_offset, _) = env
                    .get(var)
                    .ok_or((CodegenError::UndeclaredVariable(var), lhs.1))?;

                rhs.code_gen(lt, em, env)?;
                em.emit_instr(&format!("mov %rax, {}(%rbp)", var_offset));
            }

            (Expr::Binary(lhs, op, rhs), span) if op.is_compound_assignment() => {
                (Expr::Binary(lhs, op, rhs), span)
                    .desugar()
                    .expect("infallible")
                    .code_gen(lt, em, env)?
            }

            (Expr::Binary(_, _, _), _) => unreachable!("reached binary _ branch in codegen"),

            (Expr::Ternary(condition, a, b), _span) => {
                let els = lt.create(LabelKind::TernaryElse);
                let end = lt.create(LabelKind::TernaryEnd);

                condition.code_gen(lt, em, env)?;
                em.emit_instr("cmp $0, %rax");
                em.emit_instr(&format!("je {}", els));
                a.code_gen(lt, em, env)?;
                em.emit_instr(&format!("jmp {}", end));
                em.emit_label(&els);
                b.code_gen(lt, em, env)?;
                em.emit_label(&end);
            }
        }
        Ok(())
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
