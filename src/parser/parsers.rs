use super::{Expr, Stmt};
use crate::common::Token;
use chumsky::{input::ValueInput, prelude::*};

/// Parses a C Program
pub fn parser<'src, I>() -> impl Parser<'src, I, Vec<Stmt>, extra::Err<Rich<'src, Token<'src>>>>
where
    I: ValueInput<'src, Token = Token<'src>, Span = SimpleSpan>,
{
    stmt_fun().repeated().collect()
}

/* Statements */

fn stmt<'src, I>() -> impl Parser<'src, I, Stmt, extra::Err<Rich<'src, Token<'src>>>>
where
    I: ValueInput<'src, Token = Token<'src>, Span = SimpleSpan>,
{
    choice((stmt_assign(), stmt_return()))
}

fn stmt_return<'src, I>() -> impl Parser<'src, I, Stmt, extra::Err<Rich<'src, Token<'src>>>>
where
    I: ValueInput<'src, Token = Token<'src>, Span = SimpleSpan>,
{
    just(Token::KwReturn)
        .ignore_then(expr())
        .then_ignore(just(Token::SemiColon))
        .map(|exp| Stmt::Return(exp))
        .boxed()
}

fn stmt_assign<'src, I>() -> impl Parser<'src, I, Stmt, extra::Err<Rich<'src, Token<'src>>>>
where
    I: ValueInput<'src, Token = Token<'src>, Span = SimpleSpan>,
{
    just(Token::KwInt)
        .then(select! {
            Token::Identifier(s) => s
        })
        .then(expr())
        .then_ignore(just(Token::SemiColon))
        .map(|((_ty, var), exp)| Stmt::Assign(var.to_owned(), exp))
        .boxed()
}

fn stmt_fun<'src, I>() -> impl Parser<'src, I, Stmt, extra::Err<Rich<'src, Token<'src>>>>
where
    I: ValueInput<'src, Token = Token<'src>, Span = SimpleSpan>,
{
    just(Token::KwInt)
        .then(select! {Token::Identifier(s) => s})
        .then_ignore(just(Token::OpenParen))
        .then_ignore(just(Token::CloseParen))
        .then_ignore(just(Token::OpenBrace))
        .then(stmt().repeated().collect())
        .then_ignore(just(Token::CloseBrace))
        .map(|((_ret_ty, name), body)| Stmt::Function(name.to_owned(), body))
        .boxed()
}

fn expr<'src, I>() -> impl Parser<'src, I, Expr, extra::Err<Rich<'src, Token<'src>>>>
where
    I: ValueInput<'src, Token = Token<'src>, Span = SimpleSpan>,
{
    let literal = select! {
        Token::LitInteger(i) => Expr::LiteralInteger(i as u32),
    }
    .boxed();

    let variable = select! {
        Token::Identifier(v) => Expr::Variable(v.to_owned()),
    }
    .boxed();

    choice((literal, variable))
}
