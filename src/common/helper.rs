pub fn label(i: &mut usize) -> String {
    format!("l_{}", {
        *i += 1;
        *i - 1
    })
}
