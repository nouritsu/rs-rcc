pub fn label(i: &mut usize) -> String {
    *i += 1;
    format!("l_{}", *i - 1)
}
