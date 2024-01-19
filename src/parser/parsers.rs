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
        .then_ignore(just(Token::Semicolon))
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
        .then_ignore(just(Token::Semicolon))
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
        Token::LitInteger(i) => Expr::LiteralInteger(i),
    }
    .boxed();

    let variable = select! {
        Token::Identifier(v) => Expr::Variable(v.to_owned()),
    }
    .boxed();

    recursive(|expr| {
        let atom = literal
            .or(expr.delimited_by(just(Token::OpenParen), just(Token::CloseParen)))
            .or(variable)
            .boxed();

        let unary = just(Token::Minus)
            .or(just(Token::Exclamation))
            .or(just(Token::Tilde))
            .repeated()
            .foldr(atom, |op, rhs| {
                Expr::Unary(op.try_into().expect("infallible"), Box::new(rhs))
            });

        let product = unary
            .clone()
            .foldl(
                choice((just(Token::Slash), just(Token::Star)))
                    .then(unary)
                    .repeated(),
                |lhs, (op, rhs)| {
                    Expr::Binary(
                        Box::new(lhs),
                        op.try_into().expect("infallible"),
                        Box::new(rhs),
                    )
                },
            )
            .boxed();

        let sum = product
            .clone()
            .foldl(
                choice((just(Token::Plus), just(Token::Minus)))
                    .then(product)
                    .repeated(),
                |lhs, (op, rhs)| {
                    Expr::Binary(
                        Box::new(lhs),
                        op.try_into().expect("infallible"),
                        Box::new(rhs),
                    )
                },
            )
            .boxed();

        sum
    })
}
