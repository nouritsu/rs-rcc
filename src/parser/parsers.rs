use super::{expr::Spanned, Expr, Stmt};
use crate::{common::Token, lexer::Span};
use chumsky::prelude::*;

type ParserInput<'tokens, 'src> =
    chumsky::input::SpannedInput<Token<'src>, Span, &'tokens [(Token<'src>, Span)]>;

/// Parses a C Program
pub fn parser<'tokens, 'src: 'tokens>() -> impl Parser<
    'tokens,
    ParserInput<'tokens, 'src>,
    Vec<Spanned<Stmt<'src>>>,
    extra::Err<Rich<'tokens, Token<'src>, Span>>,
> + Clone {
    stmt().repeated().collect().labelled("program")
}

/* Statements */
fn stmt<'tokens, 'src: 'tokens>() -> impl Parser<
    'tokens,
    ParserInput<'tokens, 'src>,
    Spanned<Stmt<'src>>,
    extra::Err<Rich<'tokens, Token<'src>, Span>>,
> + Clone {
    let ident = select! { Token::Identifier(s) => s }.labelled("identifier");

    let stmt_return = just(Token::Return)
        .ignore_then(expr())
        .then_ignore(just(Token::Semicolon))
        .map_with(|expr, e| (Stmt::Return(expr), e.span()))
        .boxed();

    let stmt_declare = just(Token::Int)
        .then(ident)
        .then(just(Token::Equals).ignore_then(expr()).or_not())
        .then_ignore(just(Token::Semicolon))
        .map_with(|((_ty, var), expr), e| (Stmt::Declare(var, expr), e.span()))
        .boxed();

    let stmt_assign = ident
        .then_ignore(just(Token::Equals))
        .then(expr())
        .then_ignore(just(Token::Semicolon))
        .map_with(|(name, expr), e| (Stmt::Assign(name, expr), e.span()))
        .boxed();

    let stmt_expr = expr()
        .then_ignore(just(Token::Semicolon))
        .map_with(|expr, e| (Stmt::Expression(expr), e.span()))
        .boxed();

    let stmt_fun = just(Token::Int)
        .then(ident)
        .then_ignore(just(Token::OpenParen))
        .then_ignore(just(Token::CloseParen))
        .then(
            choice((stmt_assign, stmt_declare, stmt_return, stmt_expr))
                .repeated()
                .collect()
                .delimited_by(just(Token::OpenBrace), just(Token::CloseBrace)),
        )
        .map_with(|((_ty, name), body), e| (Stmt::Function(name, body), e.span()))
        .labelled("function")
        .boxed();

    stmt_fun
}

fn expr<'tokens, 'src: 'tokens>() -> impl Parser<
    'tokens,
    ParserInput<'tokens, 'src>,
    Spanned<Expr<'src>>,
    extra::Err<Rich<'tokens, Token<'src>, Span>>,
> + Clone {
    let literal = select! {
        Token::LitInteger(i) => Expr::LiteralInteger(i),
    }
    .map_with(|expr, e| (expr, e.span()))
    .labelled("value");

    let variable = select! {
        Token::Identifier(v) => Expr::Variable(v),
    }
    .map_with(|expr, e| (expr, e.span()))
    .labelled("variable");

    recursive(|expr| {
        let atom = literal
            .or(expr.delimited_by(just(Token::OpenParen), just(Token::CloseParen)))
            .or(variable)
            .boxed();

        let unary = just(Token::Minus)
            .or(just(Token::Exclamation))
            .or(just(Token::Tilde))
            .repeated()
            .foldr_with(atom, |op, rhs, e| {
                (
                    Expr::Unary(op.try_into().expect("infallible"), Box::new(rhs)),
                    e.span(),
                )
            })
            .boxed();

        let binary = {
            let product = unary
                .clone()
                .foldl_with(
                    choice((just(Token::Slash), just(Token::Star)))
                        .then(unary)
                        .repeated(),
                    |lhs, (op, rhs), e| {
                        (
                            Expr::Binary(
                                Box::new(lhs),
                                op.try_into().expect("infallible"),
                                Box::new(rhs),
                            ),
                            e.span(),
                        )
                    },
                )
                .boxed();

            let sum = product
                .clone()
                .foldl_with(
                    choice((just(Token::Plus), just(Token::Minus)))
                        .then(product)
                        .repeated(),
                    |lhs, (op, rhs), e| {
                        (
                            Expr::Binary(
                                Box::new(lhs),
                                op.try_into().expect("infallible"),
                                Box::new(rhs),
                            ),
                            e.span(),
                        )
                    },
                )
                .boxed();

            let comparison1 = sum
                .clone()
                .foldl_with(
                    choice((
                        just(Token::GreaterEqual),
                        just(Token::LesserEqual),
                        just(Token::GreaterThan),
                        just(Token::LesserThan),
                    ))
                    .then(sum)
                    .repeated(),
                    |lhs, (op, rhs), e| {
                        (
                            Expr::Binary(
                                Box::new(lhs),
                                op.try_into().expect("infallible"),
                                Box::new(rhs),
                            ),
                            e.span(),
                        )
                    },
                )
                .boxed();

            let comparison2 = comparison1
                .clone()
                .foldl_with(
                    choice((just(Token::NotEquals), just(Token::EqualsEquals)))
                        .then(comparison1)
                        .repeated(),
                    |lhs, (op, rhs), e| {
                        (
                            Expr::Binary(
                                Box::new(lhs),
                                op.try_into().expect("infallible"),
                                Box::new(rhs),
                            ),
                            e.span(),
                        )
                    },
                )
                .boxed();

            let logical_and = comparison2
                .clone()
                .foldl_with(
                    just(Token::AndAnd).then(comparison2).repeated(),
                    |lhs, (op, rhs), e| {
                        (
                            Expr::Binary(
                                Box::new(lhs),
                                op.try_into().expect("infallible"),
                                Box::new(rhs),
                            ),
                            e.span(),
                        )
                    },
                )
                .boxed();

            let logical_or = logical_and
                .clone()
                .foldl_with(
                    just(Token::OrOr).then(logical_and).repeated(),
                    |lhs, (op, rhs), e| {
                        (
                            Expr::Binary(
                                Box::new(lhs),
                                op.try_into().expect("infallible"),
                                Box::new(rhs),
                            ),
                            e.span(),
                        )
                    },
                )
                .boxed();

            logical_or
        };

        binary.labelled("expression")
    })
}
