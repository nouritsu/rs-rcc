use crate::Token;

#[derive(Debug, PartialEq, Eq)]
pub enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
    LogicalNot,
    BitwiseNot,
}

impl TryFrom<Token<'_>> for Operator {
    type Error = ();

    fn try_from(value: Token) -> Result<Self, Self::Error> {
        Ok(match value {
            Token::Plus => Operator::Plus,
            Token::Star => Operator::Multiply,
            Token::Slash => Operator::Divide,
            Token::Minus => Operator::Minus,
            Token::Exclamation => Operator::LogicalNot,
            Token::Tilde => Operator::BitwiseNot,
            _ => return Err(()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_from() {
        assert_eq!(Token::Plus.try_into(), Ok(Operator::Plus));
        assert_eq!(Token::Star.try_into(), Ok(Operator::Multiply));
        assert_eq!(Token::Slash.try_into(), Ok(Operator::Divide));
        assert_eq!(Token::Minus.try_into(), Ok(Operator::Minus));
        assert_eq!(Token::Exclamation.try_into(), Ok(Operator::LogicalNot));
        assert_eq!(Token::Tilde.try_into(), Ok(Operator::BitwiseNot));
    }
}
