use super::helper::*;
use logos::Logos;

#[derive(Logos, Debug, Clone, Copy, PartialEq, Eq)]
#[logos(skip r"[ \t\f\r\n]+")] // Spaces
#[logos(skip r"//[^\r\n]*(\r\n|\n)?")] // Single Line Comments
#[logos(skip r"/\*([^*]|\*[^/])+\*/")] // Multi Line Comments
pub enum Token<'src> {
    /* Keywords */
    #[token("int")]
    KwInt,

    #[token("return")]
    KwReturn,

    /* Literals */
    #[regex(r"[0-9]+|0x[0-9a-fA-F]+|0b[01]+", |lex| lit_int(lex.slice()))]
    LitInteger(u64),

    /* Miscellaneous */
    #[regex(r"[a-zA-Z_][0-9a-zA-Z_]*")]
    Identifier(&'src str),

    /* Symbols */
    #[token("(")]
    OpenParen,

    #[token(")")]
    CloseParen,

    #[token("{")]
    OpenBrace,

    #[token("}")]
    CloseBrace,

    #[token(";")]
    Semicolon,

    #[token("~")]
    Tilde,

    #[token("-")]
    Minus,

    #[token("+")]
    Plus,

    #[token("*")]
    Star,

    #[token("/")]
    Slash,

    #[token("!")]
    Exclamation,

    #[token("&&")]
    AndAnd,

    #[token("||")]
    OrOr,

    #[token("==")]
    EqualsEquals,

    #[token("!=")]
    NotEquals,

    #[token(">")]
    GreaterThan,

    #[token("<")]
    LesserThan,

    #[token(">=")]
    GreaterEqual,

    #[token("<=")]
    LesserEqual,
}
