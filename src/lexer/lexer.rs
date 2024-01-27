use crate::Token;
use chumsky::prelude::*;

pub type Span = SimpleSpan<usize>;

pub fn lexer<'src>(
) -> impl Parser<'src, &'src str, Vec<(Token<'src>, Span)>, extra::Err<Rich<'src, char, Span>>> {
    let literal = {
        let int = choice((
            text::int(10).map(|s| (10, s)),
            just("0").ignore_then(text::int(8)).map(|s| (8, s)),
            just("0b").ignore_then(text::int(2)).map(|s| (2, s)),
            just("0x").ignore_then(text::int(16)).map(|s| (16, s)),
        ))
        .map(|(base, num)| u64::from_str_radix(num, base).expect("infallible"))
        .map(Token::LitInteger);

        int
    }
    .boxed();

    let symbol = choice(vec![
        // Delimiters
        just("(").to(Token::OpenParen),
        just(")").to(Token::CloseParen),
        just("{").to(Token::OpenBrace),
        just("}").to(Token::CloseBrace),
        // Controls
        just(";").to(Token::Semicolon),
        // Math Operators
        just("+").to(Token::Plus),
        just("-").to(Token::Minus),
        just("*").to(Token::Star),
        just("/").to(Token::Slash),
        // Comparison Operators
        just("==").to(Token::EqualsEquals),
        just("!=").to(Token::NotEquals),
        just(">=").to(Token::GreaterEqual),
        just("<=").to(Token::LesserEqual),
        just(">").to(Token::GreaterThan),
        just("<").to(Token::LesserThan),
        // Logical Operators
        just("!").to(Token::Exclamation),
        just("&&").to(Token::AndAnd),
        just("||").to(Token::OrOr),
        // Bitwise Operators
        just("~").to(Token::Tilde),
        // Misc Operators
        just("=").to(Token::Equals),
    ])
    .boxed();

    let ident = text::ascii::ident()
        .map(|ident| match ident {
            "int" => Token::Int,
            "return" => Token::Return,
            s => Token::Identifier(s),
        })
        .boxed();

    //TODO: comment lexer

    let token = choice((literal, symbol, ident));

    token
        .map_with(|tok, e| (tok, e.span()))
        // .padded_by(comment.repeated())
        .padded()
        .recover_with(skip_then_retry_until(any().ignored(), end()))
        .repeated()
        .collect()
}
