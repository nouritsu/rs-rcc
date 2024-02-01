use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Token<'src> {
    /* Keywords */
    Int,
    Return,
    If,
    Else,

    /* Literals */
    LitInteger(u64),

    /* Miscellaneous */
    Identifier(&'src str),

    /* Symbols */
    // Delimiters
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,

    // Math Operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,

    // Logical Operators
    Exclamation,
    AndAnd,
    PipePipe,

    // Comparison Operators
    EqualsEquals,
    NotEquals,
    GreaterThan,
    LesserThan,
    GreaterEquals,
    LesserEquals,

    // Bitwise Operators
    Tilde,
    And,
    Pipe,
    Caret,
    LeftShift,
    RightShift,

    // Assignment Operators
    Equals,
    PlusEquals,
    MinusEquals,
    StarEquals,
    SlashEquals,
    PercentEquals,
    AndEquals,
    PipeEquals,
    CaretEquals,
    LeftShiftEquals,
    RightShiftEquals,

    // Conditional Operators
    Colon,
    Question,

    //Control
    Semicolon,
}

impl<'src> Display for Token<'src> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Int => write!(f, "int"),
            Self::Return => write!(f, "return"),
            Self::LitInteger(i) => write!(f, "{}", i),
            Self::Identifier(s) => write!(f, "{}", s),
            Self::OpenParen => write!(f, "("),
            Self::CloseParen => write!(f, ")"),
            Self::OpenBrace => write!(f, "{{"),
            Self::CloseBrace => write!(f, "}}"),
            Self::Semicolon => write!(f, ";"),
            Self::Tilde => write!(f, "~"),
            Self::Plus => write!(f, "+"),
            Self::Minus => write!(f, "-"),
            Self::Star => write!(f, "*"),
            Self::Slash => write!(f, "/"),
            Self::Exclamation => write!(f, "!"),
            Self::AndAnd => write!(f, "&&"),
            Self::PipePipe => write!(f, "||"),
            Self::EqualsEquals => write!(f, "=="),
            Self::NotEquals => write!(f, "!="),
            Self::GreaterThan => write!(f, ">"),
            Self::LesserThan => write!(f, "<"),
            Self::GreaterEquals => write!(f, ">="),
            Self::LesserEquals => write!(f, "<="),
            Self::Equals => write!(f, "="),
            Self::And => write!(f, "&"),
            Self::Pipe => write!(f, "|"),
            Self::Caret => write!(f, "^"),
            Self::PlusEquals => write!(f, "+="),
            Self::MinusEquals => write!(f, "-="),
            Self::StarEquals => write!(f, "*="),
            Self::SlashEquals => write!(f, "/="),
            Self::AndEquals => write!(f, "&="),
            Self::PipeEquals => write!(f, "|="),
            Self::CaretEquals => write!(f, "^="),
            Self::LeftShift => write!(f, "<<"),
            Self::RightShift => write!(f, ">>"),
            Self::LeftShiftEquals => write!(f, "<<="),
            Self::RightShiftEquals => write!(f, ">>="),
            Self::Percent => write!(f, "%"),
            Self::PercentEquals => write!(f, "%="),
            Self::If => write!(f, "if"),
            Self::Else => write!(f, "else"),
            Self::Question => write!(f, "?"),
            Self::Colon => write!(f, ":"),
        }
    }
}
