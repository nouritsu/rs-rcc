pub fn lit_int(s: &str) -> Result<u64, ()> {
    {
        if s.starts_with("0x") {
            u64::from_str_radix(&s[2..], 16)
        } else if s.starts_with("0b") {
            println!("{}\t{}", s, &s[2..]);
            u64::from_str_radix(&s[2..], 2)
        } else if s.starts_with("0") && s.len() >= 2 {
            u64::from_str_radix(&s[1..], 8)
        } else {
            u64::from_str_radix(&s, 10)
        }
    }
    .map_err(|_| ())
}

#[cfg(test)]
mod tests {
    #[test]
    fn lit_int() {
        assert_eq!(super::lit_int("0xF0"), Ok(240));
        assert_eq!(super::lit_int("0x1G"), Err(()));

        assert_eq!(super::lit_int("0123"), Ok(83));
        assert_eq!(super::lit_int("0811"), Err(()));

        assert_eq!(super::lit_int("0b1001"), Ok(9));
        assert_eq!(super::lit_int("0b0102"), Err(()));

        assert_eq!(super::lit_int("1024"), Ok(1024));
        assert_eq!(super::lit_int("12AB"), Err(()));
    }
}
