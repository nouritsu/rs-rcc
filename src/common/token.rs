use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Token<'src> {
    /* Keywords */
    Int,
    Return,

    /* Literals */
    LitInteger(u64),

    /* Miscellaneous */
    Identifier(&'src str),

    /* Symbols */
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    Semicolon,
    Tilde,
    Plus,
    Minus,
    Star,
    Slash,
    Exclamation,
    AndAnd,
    OrOr,
    EqualsEquals,
    NotEquals,
    GreaterThan,
    LesserThan,
    GreaterEqual,
    LesserEqual,
    Equals,
}

impl<'src> Display for Token<'src> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Int => write!(f, "int"),
            Token::Return => write!(f, "return"),
            Token::LitInteger(i) => write!(f, "{}", i),
            Token::Identifier(s) => write!(f, "{}", s),
            Token::OpenParen => write!(f, "("),
            Token::CloseParen => write!(f, ")"),
            Token::OpenBrace => write!(f, "{{"),
            Token::CloseBrace => write!(f, "}}"),
            Token::Semicolon => write!(f, ";"),
            Token::Tilde => write!(f, "~"),
            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::Star => write!(f, "*"),
            Token::Slash => write!(f, "/"),
            Token::Exclamation => write!(f, "!"),
            Token::AndAnd => write!(f, "&&"),
            Token::OrOr => write!(f, "||"),
            Token::EqualsEquals => write!(f, "=="),
            Token::NotEquals => write!(f, "!="),
            Token::GreaterThan => write!(f, ">"),
            Token::LesserThan => write!(f, "<"),
            Token::GreaterEqual => write!(f, ">="),
            Token::LesserEqual => write!(f, "<="),
            Token::Equals => write!(f, "="),
        }
    }
}
