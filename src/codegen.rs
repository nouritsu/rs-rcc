pub trait Codegen {
    fn code_gen(&self, label_idx: &mut usize) -> String;
}
