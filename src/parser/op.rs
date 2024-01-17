use crate::Token;

#[derive(Debug)]
pub enum Operator {
    Minus,
    LogicalNot,
    BitwiseNot,
}

impl TryFrom<Token<'_>> for Operator {
    type Error = ();

    fn try_from(value: Token) -> Result<Self, Self::Error> {
        Ok(match value {
            Token::Minus => Operator::Minus,
            Token::Exclamation => Operator::LogicalNot,
            Token::Tilde => Operator::BitwiseNot,
            _ => return Err(()),
        })
    }
}
