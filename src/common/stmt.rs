use super::{env::Environment, label_tracker::LabelTracker, Codegen, CodegenError};
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
        env: &mut Environment<'src>,
    ) -> Result<String, Spanned<CodegenError<'src>>> {
        Ok(self
            .into_iter()
            .map(|stmt| stmt.code_gen(lt, env))
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .fold(String::new(), |s, x| s + &x))
    }
}

impl<'src> Codegen<'src> for Spanned<Stmt<'src>> {
    fn code_gen(
        self,
        lt: &mut LabelTracker,
        env: &mut Environment<'src>,
    ) -> Result<String, Spanned<CodegenError<'src>>> {
        Ok(match self {
            (Stmt::Block(stmts), _) => stmts.code_gen(lt, env)?,

            (Stmt::Expression(expr), _) => expr.code_gen(lt, env)?,

            (Stmt::Declare(name, expr), _) => {
                let (name, name_span) = name;
                if env.contains(name) {
                    let (_, init_span) = env.get(name).expect("infallible");
                    return Err((CodegenError::RedeclaredVariable(name, init_span), name_span));
                }

                let asm = format!(
                    "{}\tpush %rax\n",
                    expr.map(|e| e.code_gen(lt, env))
                        .transpose()?
                        .unwrap_or("\tmov $0, %rax\n".to_string())
                );

                env.put(name, name_span);

                asm
            }

            (Stmt::If(condition, then, r#else), _span) => {
                let els = &lt.create(LabelKind::TernaryElse);
                let end = &lt.create(LabelKind::TernaryEnd);
                let else_exists = r#else.is_some();

                format!(
                    "{}\tcmp $0, %rax\n\tje {}\n{}\tjmp {}\n{}{}:\n",
                    condition.code_gen(lt, env)?,
                    if else_exists { els } else { end },
                    then.code_gen(lt, env)?,
                    end,
                    r#else
                        .map(|stmt| stmt.code_gen(lt, env))
                        .transpose()?
                        .map(|asm| format!("{}:\n{}", els, asm))
                        .unwrap_or(String::new()),
                    end,
                )
            }

            (Stmt::Return(expr), _) => {
                expr.code_gen(lt, env)? + "\tmov %rbp, %rsp\n\tpop %rbp\n\tret\n"
            }

            (Stmt::Empty, _) => String::new(),
        })
    }
}
