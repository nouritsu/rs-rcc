use super::{
    label_tracker::{self, LabelTracker},
    Codegen, CodegenError, Environment, Spanned, Stmt,
};

#[derive(Debug)]
pub struct FnDeclaration<'src>(pub &'src str, pub Vec<Spanned<Stmt<'src>>>);

impl<'src> Codegen<'src> for Vec<Spanned<FnDeclaration<'src>>> {
    fn code_gen(
        self,
        lt: &mut LabelTracker,
        env: &mut Environment<'src>,
    ) -> Result<String, Spanned<CodegenError<'src>>> {
        Ok(self
            .into_iter()
            .map(|decl| decl.code_gen(lt, env))
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .fold(String::new(), |s, x| s + &x))
    }
}

impl<'src> Codegen<'src> for Spanned<FnDeclaration<'src>> {
    fn code_gen(
        self,
        lt: &mut label_tracker::LabelTracker,
        env: &mut Environment<'src>,
    ) -> Result<String, Spanned<CodegenError<'src>>> {
        let (FnDeclaration(name, body), _) = self;

        Ok(format!(
            "\t.globl {}\n{}:\n\tpush %rbp\n\tmov %rsp, %rbp\n{}",
            name,
            name,
            body.code_gen(lt, env)?,
        ))
    }
}
