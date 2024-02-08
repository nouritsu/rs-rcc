use super::{
    emitter::Emitter, label_tracker::LabelTracker, Codegen, CodegenError, Environment, Spanned,
    Stmt,
};

#[derive(Debug)]
pub struct FnDeclaration<'src>(pub &'src str, pub Vec<Spanned<Stmt<'src>>>);

impl<'src> Codegen<'src> for Vec<Spanned<FnDeclaration<'src>>> {
    fn code_gen(
        self,
        lt: &mut LabelTracker,
        em: &mut Emitter,
        env: &mut Environment<'src>,
    ) -> Result<(), Spanned<CodegenError<'src>>> {
        self.into_iter()
            .map(|decl| decl.code_gen(lt, em, env))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(())
    }
}

impl<'src> Codegen<'src> for Spanned<FnDeclaration<'src>> {
    fn code_gen(
        self,
        lt: &mut LabelTracker,
        em: &mut Emitter,
        env: &mut Environment<'src>,
    ) -> Result<(), Spanned<CodegenError<'src>>> {
        let (FnDeclaration(name, body), _) = self;

        env.new_scope();

        em.emit_directive(&format!(".globl {}", name));
        em.emit_label(name);
        em.emit_instr("push %rbp");
        em.emit_instr("mov %rsp, %rbp");
        body.code_gen(lt, em, env)?;

        env.end_scope();

        Ok(())
    }
}
