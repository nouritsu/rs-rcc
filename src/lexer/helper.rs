pub fn lit_int(s: &str) -> Result<u128, ()> {
    {
        if s.starts_with("0x") {
            u128::from_str_radix(&s[2..], 16)
        } else if s.starts_with("0") {
            u128::from_str_radix(&s[1..], 8)
        } else if s.starts_with("0b") {
            u128::from_str_radix(&s[2..], 2)
        } else {
            u128::from_str_radix(&s, 10)
        }
    }
    .map_err(|_| ())
}
