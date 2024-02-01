use super::{env::Environment, Codegen, CodegenError};
use super::{Expr, Spanned};
use clap::error::Result;

#[derive(Debug)]
pub enum Stmt<'src> {
    Return(Spanned<Expr<'src>>),
    Declare(Spanned<&'src str>, Option<Spanned<Expr<'src>>>),
    Expression(Spanned<Expr<'src>>),
    Function(&'src str, Vec<Spanned<Self>>),
}

impl<'src> Codegen<'src> for Vec<Spanned<Stmt<'src>>> {
    fn code_gen(
        self,
        i: &mut usize,
        env: &mut Environment<'src>,
    ) -> Result<String, Spanned<CodegenError<'src>>> {
        Ok(self
            .into_iter()
            .map(|stmt| stmt.code_gen(i, env))
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .fold(String::new(), |s, x| s + &x))
    }
}

impl<'src> Codegen<'src> for Spanned<Stmt<'src>> {
    fn code_gen(
        self,
        i: &mut usize,
        env: &mut Environment<'src>,
    ) -> Result<String, Spanned<CodegenError<'src>>> {
        Ok(match self {
            (Stmt::Return(expr), _) => expr.code_gen(i, env)? + "mov %rbp, %rsp\npop %rbp\nret\n",

            (Stmt::Declare((name, name_span), expr), span) => {
                if env.contains(name) {
                    let (_, init_span) = env.get(name).expect("infallible");
                    return Err((CodegenError::RedeclaredVariable(name, init_span), span));
                }

                let asm = format!(
                    "{}push %rax\n",
                    expr.map(|e| e.code_gen(i, env))
                        .transpose()?
                        .unwrap_or("mov $0, %rax\n".to_string())
                );

                env.put(name, name_span);

                asm
            }

            (Stmt::Expression(expr), _) => expr.code_gen(i, env)?,

            (Stmt::Function(name, body), _) => {
                format!(
                    ".globl {}\n{}:\npush %rbp\nmov %rsp, %rbp\n{}",
                    name,
                    name,
                    body.into_iter()
                        .map(|stmt| stmt.code_gen(i, env))
                        .collect::<Result<Vec<_>, _>>()?
                        .into_iter()
                        .fold(String::new(), |s, x| s + &x)
                )
            }
        })
    }
}
