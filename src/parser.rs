use crate::common::{Expr, Span, Spanned, Stmt, Token};
use chumsky::prelude::*;

/*
C/C++ Operator Precedence
---------------------------------------------------01---------------------------------------------------
++          Postfix Increment
--          Postfix Decrement
()          Call
[]          Index
.           Member Access
->          Deref Member Access
(T){xs}     Compound Literal
x           Atomic Literals
---------------------------------------------------02---------------------------------------------------
++          Prefix Increment
--          Prefix Decrement
+           Unary Plus
-           Unary Minus
!           Logical Not
~           Bitwise Not
(T)         Cast
*           Dereference
&           Reference
sizeof      Size Of
_Alignof    Align Of
---------------------------------------------------03---------------------------------------------------
*           Multiplication
/           Division
%           Modulo
---------------------------------------------------04---------------------------------------------------
+           Addition
-           Subtraction
---------------------------------------------------05---------------------------------------------------
<<          Bitwise Left Shift
>>          Bitwise Right Shift
---------------------------------------------------06---------------------------------------------------
<           Less Than
>           Greater Than
<=          Lesser Equal
>=          Greater Equal
---------------------------------------------------07---------------------------------------------------
==          Equals
!=          Not Equals
---------------------------------------------------08---------------------------------------------------
&           Bitwise AND
---------------------------------------------------09---------------------------------------------------
^           Bitwise XOR
---------------------------------------------------10---------------------------------------------------
|           Bitwise OR
---------------------------------------------------11---------------------------------------------------
&&          Logical AND
---------------------------------------------------12---------------------------------------------------
||          Logical OR
---------------------------------------------------13---------------------------------------------------
?:          Ternary Conditional Operators
---------------------------------------------------14---------------------------------------------------
=           Assignment
+=          Assignment by Sum
-=          Assignment by Difference
*=          Assignment by Product
/=          Assignment by Quotient
%=          Assignment by Remainder
<<=         Assignment by Bitwise Left Shift
>>=         Assignment by Bitwise Right Shift
&=          Assignment by Bitwise AND
|=          Assignment by Bitwise OR
^=          Assignment by Bitwise XOR
---------------------------------------------------15---------------------------------------------------
,           Comma
--------------------------------------------------------------------------------------------------------
*/

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
        .then(ident.map_with(|ident, e| (ident, e.span())))
        .then(just(Token::Equals).ignore_then(expr()).or_not())
        .then_ignore(just(Token::Semicolon))
        .map_with(|((_ty, ident), expr), e| (Stmt::Declare(ident, expr), e.span()))
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
            choice((stmt_declare, stmt_return, stmt_expr))
                .repeated()
                .collect()
                .delimited_by(just(Token::OpenBrace), just(Token::CloseBrace)),
        )
        .map_with(|((_ty, name), body), e| (Stmt::Function(name, body), e.span()))
        .labelled("function")
        .boxed();

    stmt_fun
}

/* Expressions */
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
    .boxed()
    .labelled("value");

    let variable = select! {
        Token::Identifier(v) => Expr::Variable(v),
    }
    .map_with(|expr, e| (expr, e.span()))
    .boxed()
    .labelled("variable");

    let atom = recursive(|expr| {
        literal
            .or(expr
                .clone()
                .delimited_by(just(Token::OpenParen), just(Token::CloseParen)))
            .or(variable)
    })
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

    let product = unary
        .clone()
        .foldl_with(
            choice((just(Token::Slash), just(Token::Star), just(Token::Percent)))
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

    let shifts = sum
        .clone()
        .foldl_with(
            choice((just(Token::LeftShift), just(Token::RightShift)))
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

    let gtgeltle = shifts
        .clone()
        .foldl_with(
            choice((
                just(Token::GreaterEquals),
                just(Token::LesserEquals),
                just(Token::GreaterThan),
                just(Token::LesserThan),
            ))
            .then(shifts)
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

    let eqne = gtgeltle
        .clone()
        .foldl_with(
            choice((just(Token::NotEquals), just(Token::EqualsEquals)))
                .then(gtgeltle)
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

    let bw_and = eqne
        .clone()
        .foldl_with(
            just(Token::And).then(eqne).repeated(),
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

    let bw_xor = bw_and
        .clone()
        .foldl_with(
            just(Token::Caret).then(bw_and).repeated(),
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

    let bw_or = bw_xor
        .clone()
        .foldl_with(
            just(Token::Pipe).then(bw_xor).repeated(),
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

    let lg_and = bw_or
        .clone()
        .foldl_with(
            just(Token::AndAnd).then(bw_or).repeated(),
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

    let lg_or = lg_and
        .clone()
        .foldl_with(
            just(Token::PipePipe).then(lg_and).repeated(),
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

    let ternary = lg_or
        .clone()
        .foldl_with(
            just(Token::Question)
                .ignore_then(lg_or.clone())
                .then_ignore(just(Token::Colon))
                .then(lg_or)
                .repeated(),
            |cond, (a, b), e| {
                (
                    Expr::Ternary(Box::new(cond), Box::new(a), Box::new(b)),
                    e.span(),
                )
            },
        )
        .boxed();

    let assignment = ternary
        .clone()
        .foldl_with(
            choice((
                just(Token::Equals),
                just(Token::PlusEquals),
                just(Token::MinusEquals),
                just(Token::StarEquals),
                just(Token::SlashEquals),
                just(Token::PercentEquals),
                just(Token::AndEquals),
                just(Token::PipeEquals),
                just(Token::CaretEquals),
                just(Token::LeftShiftEquals),
                just(Token::RightShiftEquals),
            ))
            .then(ternary)
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

    assignment.labelled("expression")
}
