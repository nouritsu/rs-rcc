use super::helper::*;
use logos::Logos;

#[derive(Logos, Debug, Clone)]
#[logos(skip r"[ \t\f\n]+")]
pub enum Token<'src> {
    /* Keywords */
    #[token("int")]
    KwInt,

    #[token("return")]
    KwReturn,

    /* Literals */
    #[regex(r"(0x[0-9a-fA-F]+)|(0[0-7]+)|(0b[01]+)|([0-9]+)", |lex| lit_int(lex.slice()))]
    LitInteger(u128),

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
    SemiColon,
}
