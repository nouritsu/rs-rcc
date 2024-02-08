use super::{
    emitter::Emitter, env::Environment, label_tracker::LabelTracker, Codegen, CodegenError,
};
use super::{Expr, Spanned};
use crate::common::label_tracker::LabelKind;
use clap::error::Result;

#[derive(Debug)]
pub enum Stmt<'src> {
    Block(Vec<Spanned<Self>>),
    Expression(Spanned<Expr<'src>>),
    Declare(Spanned<&'src str>, Option<Spanned<Expr<'src>>>),
    If(
        Spanned<Expr<'src>>,
        Box<Spanned<Self>>,
        Option<Box<Spanned<Self>>>,
    ),
    Return(Spanned<Expr<'src>>),
    Empty,
}

impl<'src> Codegen<'src> for Vec<Spanned<Stmt<'src>>> {
    fn code_gen(
        self,
        lt: &mut LabelTracker,
        em: &mut Emitter,
        env: &mut Environment<'src>,
    ) -> Result<(), Spanned<CodegenError<'src>>> {
        self.into_iter()
            .map(|stmt| stmt.code_gen(lt, em, env))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(())
    }
}

impl<'src> Codegen<'src> for Spanned<Stmt<'src>> {
    fn code_gen(
        self,
        lt: &mut LabelTracker,
        em: &mut Emitter,
        env: &mut Environment<'src>,
    ) -> Result<(), Spanned<CodegenError<'src>>> {
        match self {
            (Stmt::Block(stmts), _) => {
                env.new_scope();
                stmts.code_gen(lt, em, env)?;
                env.end_scope();
            }

            (Stmt::Expression(expr), _) => expr.code_gen(lt, em, env)?,

            (Stmt::Declare((name, name_span), expr), _) => {
                if env.contains(name) {
                    let (_, init_span) = env.get(name).expect("infallible");
                    return Err((CodegenError::RedeclaredVariable(name, init_span), name_span));
                }

                match expr {
                    Some(expr) => expr.code_gen(lt, em, env)?,
                    None => em.emit_instr("mov $0, %rax"),
                }
                em.emit_instr("push %rax");

                env.put(name, name_span);
            }

            (Stmt::If(condition, then, r#else), _) => {
                let els = &lt.create(LabelKind::TernaryElse);
                let end = &lt.create(LabelKind::TernaryEnd);
                let else_exists = r#else.is_some();

                condition.code_gen(lt, em, env)?;
                em.emit_instr("cmp $0, %rax");
                em.emit_instr(&format!("je {}", if else_exists { els } else { end }));
                then.code_gen(lt, em, env)?;
                em.emit_instr(&format!("jmp {}", end));
                if let Some(r#else) = r#else {
                    em.emit_label(els);
                    r#else.code_gen(lt, em, env)?;
                }
                em.emit_label(end);
            }

            (Stmt::Return(expr), _) => {
                expr.code_gen(lt, em, env)?;
                em.emit_instr("mov %rbp, %rsp");
                em.emit_instr("pop %rbp");
                em.emit_instr("ret");
            }

            (Stmt::Empty, _) => {}
        }
        Ok(())
    }
}
