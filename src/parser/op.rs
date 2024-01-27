use crate::Token;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
    Eq,
    Ne,
    Gt,
    Lt,
    Ge,
    Le,
    LogicalNot,
    LogicalAnd,
    LogicalOr,
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
            Token::EqualsEquals => Operator::Eq,
            Token::NotEquals => Operator::Ne,
            Token::GreaterThan => Operator::Gt,
            Token::LesserThan => Operator::Lt,
            Token::GreaterEqual => Operator::Ge,
            Token::LesserEqual => Operator::Le,
            Token::Exclamation => Operator::LogicalNot,
            Token::AndAnd => Operator::LogicalAnd,
            Token::OrOr => Operator::LogicalOr,
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
        assert_eq!(Token::EqualsEquals.try_into(), Ok(Operator::Eq));
        assert_eq!(Token::NotEquals.try_into(), Ok(Operator::Ne));
        assert_eq!(Token::GreaterThan.try_into(), Ok(Operator::Gt));
        assert_eq!(Token::LesserThan.try_into(), Ok(Operator::Lt));
        assert_eq!(Token::GreaterEqual.try_into(), Ok(Operator::Ge));
        assert_eq!(Token::LesserEqual.try_into(), Ok(Operator::Le));
        assert_eq!(Token::AndAnd.try_into(), Ok(Operator::LogicalAnd));
        assert_eq!(Token::OrOr.try_into(), Ok(Operator::LogicalOr));
        assert_eq!(Token::Exclamation.try_into(), Ok(Operator::LogicalNot));
        assert_eq!(Token::Tilde.try_into(), Ok(Operator::BitwiseNot));
    }
}
