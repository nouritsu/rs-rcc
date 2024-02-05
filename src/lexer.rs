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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keywords() {
        let result = lexer().parse("int return if else").into_result();
        assert_eq!(
            result,
            Ok(vec![
                (Token::Int, Span::new(0, 3)),
                (Token::Return, Span::new(4, 10)),
                (Token::If, Span::new(11, 13)),
                (Token::Else, Span::new(14, 18))
            ])
        );
    }

    #[test]
    fn lit_int() {
        let result = lexer().parse("0 123 0x123 0b101 0123").into_result();
        assert_eq!(
            result,
            Ok(vec![
                (Token::LitInteger(0), Span::new(0, 1)),
                (Token::LitInteger(123), Span::new(2, 5)),
                (Token::LitInteger(0x123), Span::new(6, 11)),
                (Token::LitInteger(0b101), Span::new(12, 17)),
                (Token::LitInteger(0o123), Span::new(18, 22))
            ])
        );
    }

    #[test]
    fn ident() {
        let result = lexer()
            .parse("hello hello_world hello123 _123 123a")
            .into_result();
        assert_eq!(
            result,
            Ok(vec![
                (Token::Identifier("hello"), Span::new(0, 5)),
                (Token::Identifier("hello_world"), Span::new(6, 17)),
                (Token::Identifier("hello123"), Span::new(18, 26)),
                (Token::Identifier("_123"), Span::new(27, 31)),
                (Token::LitInteger(123), Span::new(32, 35)),
                (Token::Identifier("a"), Span::new(35, 36))
            ])
        );
    }

    #[test]
    fn delimiters() {
        let result = lexer().parse(r"(){}").into_result();
        assert_eq!(
            result,
            Ok(vec![
                (Token::OpenParen, Span::new(0, 1)),
                (Token::CloseParen, Span::new(1, 2)),
                (Token::OpenBrace, Span::new(2, 3)),
                (Token::CloseBrace, Span::new(3, 4))
            ])
        );
    }

    #[test]
    fn math_ops() {
        let result = lexer().parse("+-*/%").into_result();
        assert_eq!(
            result,
            Ok(vec![
                (Token::Plus, Span::new(0, 1)),
                (Token::Minus, Span::new(1, 2)),
                (Token::Star, Span::new(2, 3)),
                (Token::Slash, Span::new(3, 4)),
                (Token::Percent, Span::new(4, 5))
            ])
        );
    }

    #[test]
    fn logical_ops() {
        let result = lexer().parse("! && ||").into_result();
        assert_eq!(
            result,
            Ok(vec![
                (Token::Exclamation, Span::new(0, 1)),
                (Token::AndAnd, Span::new(2, 4)),
                (Token::PipePipe, Span::new(5, 7))
            ])
        );
    }

    #[test]
    fn comparison_ops() {
        let result = lexer().parse("== != >= <= > <").into_result();
        assert_eq!(
            result,
            Ok(vec![
                (Token::EqualsEquals, Span::new(0, 2)),
                (Token::NotEquals, Span::new(3, 5)),
                (Token::GreaterEquals, Span::new(6, 8)),
                (Token::LesserEquals, Span::new(9, 11)),
                (Token::GreaterThan, Span::new(12, 13)),
                (Token::LesserThan, Span::new(14, 15))
            ])
        );
    }

    #[test]
    fn bitwise_ops() {
        let result = lexer().parse("~ & | ^ << >>").into_result();
        assert_eq!(
            result,
            Ok(vec![
                (Token::Tilde, Span::new(0, 1)),
                (Token::And, Span::new(2, 3)),
                (Token::Pipe, Span::new(4, 5)),
                (Token::Caret, Span::new(6, 7)),
                (Token::LeftShift, Span::new(8, 10)),
                (Token::RightShift, Span::new(11, 13))
            ])
        );
    }

    #[test]
    fn assignment_ops() {
        let result = lexer()
            .parse("= += -= *= /= %= &= |= ^= <<= >>=")
            .into_result();
        assert_eq!(
            result,
            Ok(vec![
                (Token::Equals, Span::new(0, 1)),
                (Token::PlusEquals, Span::new(2, 4)),
                (Token::MinusEquals, Span::new(5, 7)),
                (Token::StarEquals, Span::new(8, 10)),
                (Token::SlashEquals, Span::new(11, 13)),
                (Token::PercentEquals, Span::new(14, 16)),
                (Token::AndEquals, Span::new(17, 19)),
                (Token::PipeEquals, Span::new(20, 22)),
                (Token::CaretEquals, Span::new(23, 25)),
                (Token::LeftShiftEquals, Span::new(26, 29)),
                (Token::RightShiftEquals, Span::new(30, 33))
            ])
        );
    }

    #[test]
    fn conditional_ops() {
        let result = lexer().parse("? :").into_result();
        assert_eq!(
            result,
            Ok(vec![
                (Token::Question, Span::new(0, 1)),
                (Token::Colon, Span::new(2, 3))
            ])
        );
    }

    #[test]
    fn control_ops() {
        let result = lexer().parse(";").into_result();
        assert_eq!(result, Ok(vec![(Token::Semicolon, Span::new(0, 1))]));
    }
}
