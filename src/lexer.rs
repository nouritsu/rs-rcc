use crate::common::{Span, Token};
use chumsky::{prelude::*, text::digits};

pub fn lexer<'src>(
) -> impl Parser<'src, &'src str, Vec<(Token<'src>, Span)>, extra::Err<Rich<'src, char, Span>>> {
    let literal = {
        let int = choice((
            just("0x").ignore_then(digits(16).to_slice().map(|num| (16, num))),
            just("0b").ignore_then(digits(2).to_slice().map(|num| (2, num))),
            just("0").ignore_then(digits(8).to_slice().map(|num| (8, num))),
            text::int(10).map(|num| (10, num)),
        ))
        .map(|(radix, src)| u64::from_str_radix(src, radix).expect("infallible"))
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
        /* Operators */
        // Compound Assignment Operators
        just("+=").to(Token::PlusEquals),
        just("-=").to(Token::MinusEquals),
        just("*=").to(Token::StarEquals),
        just("/=").to(Token::SlashEquals),
        just("%=").to(Token::PercentEquals),
        just("&=").to(Token::AndEquals),
        just("|=").to(Token::PipeEquals),
        just("^=").to(Token::CaretEquals),
        just("<<=").to(Token::LeftShiftEquals),
        just(">>=").to(Token::RightShiftEquals),
        // Math Operators
        just("+").to(Token::Plus),
        just("-").to(Token::Minus),
        just("*").to(Token::Star),
        just("/").to(Token::Slash),
        just("%").to(Token::Percent),
        // Logical Operators
        just("&&").to(Token::AndAnd),
        just("||").to(Token::PipePipe),
        // Bitwise Operators
        just("~").to(Token::Tilde),
        just("&").to(Token::And),
        just("|").to(Token::Pipe),
        just("^").to(Token::Caret),
        just("<<").to(Token::LeftShift),
        just(">>").to(Token::RightShift),
        // Comparison Operators
        just("==").to(Token::EqualsEquals),
        just("!=").to(Token::NotEquals),
        just(">=").to(Token::GreaterEquals),
        just("<=").to(Token::LesserEquals),
        just(">").to(Token::GreaterThan),
        just("<").to(Token::LesserThan),
        // Conditional Operators
        just("?").to(Token::Question),
        just(":").to(Token::Colon),
        // Assignment Operator
        just("=").to(Token::Equals),
        // Logical Not
        just("!").to(Token::Exclamation),
    ])
    .boxed();

    let ident = text::ascii::ident()
        .map(|ident| match ident {
            "int" => Token::Int,
            "return" => Token::Return,
            "if" => Token::If,
            "else" => Token::Else,
            s => Token::Identifier(s),
        })
        .boxed();

    let comment = {
        let sl = just("//")
            .then(any().and_is(text::newline().not()).repeated())
            .padded()
            .ignored();

        let ml = just("/*")
            .then(any().and_is(just("*/").not()).repeated())
            .map(|_| println!("HEY"))
            .then(just("*/"))
            .padded()
            .ignored();

        ml.or(sl)
    }
    .boxed();

    let token = choice((literal, symbol, ident));

    token
        .map_with(|tok, e| (tok, e.span()))
        .padded_by(comment.repeated())
        .padded()
        .recover_with(skip_then_retry_until(any().ignored(), end()))
        .repeated()
        .collect()
}
