use clap::error::Result;

use super::{expr::Spanned, Expr};
use crate::{
    codegen::{Codegen, CodegenError},
    lexer::Span,
};
use std::collections::HashMap;

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
        sp: &mut isize,
        env: &mut HashMap<String, (isize, Span)>,
    ) -> Result<String, Spanned<CodegenError<'src>>> {
        Ok(self
            .into_iter()
            .map(|stmt| stmt.code_gen(i, sp, env))
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .fold(String::new(), |s, x| s + &x))
    }
}

impl<'src> Codegen<'src> for Spanned<Stmt<'src>> {
    fn code_gen(
        self,
        i: &mut usize,
        sp: &mut isize,
        env: &mut HashMap<String, (isize, Span)>,
    ) -> Result<String, Spanned<CodegenError<'src>>> {
        Ok(match self.0 {
            Stmt::Return(expr) => expr.code_gen(i, sp, env)? + "mov %rbp, %rsp\npop %rbp\nret\n",
            Stmt::Declare((name, name_span), expr) => {
                if env.contains_key(name) {
                    let (_, initial_span) = env.get(name).expect("infallible");
                    return Err((
                        CodegenError::RedeclaredVariable(name, *initial_span),
                        name_span,
                    ));
                }

                let asm = format!(
                    "{}push %rax\n",
                    expr.map(|e| e.code_gen(i, sp, env))
                        .transpose()?
                        .unwrap_or("mov $0, %rax\n".to_string())
                );

                env.insert(name.to_string(), (*sp, name_span));
                *sp -= 8;

                asm
            }
            Stmt::Expression(expr) => expr.code_gen(i, sp, env)?,
            Stmt::Function(name, body) => {
                format!(
                    ".globl {}\n{}:\npush %rbp\nmov %rsp, %rbp\n{}",
                    name,
                    name,
                    body.into_iter()
                        .map(|stmt| stmt.code_gen(i, sp, env))
                        .collect::<Result<Vec<_>, _>>()?
                        .into_iter()
                        .fold(String::new(), |s, x| s + &x)
                )
            }
        })
    }
}
