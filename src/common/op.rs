use super::Token;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum UnaryOperator {
    Minus,
    LogicalNot,
    BitwiseNot,
}

impl TryFrom<Token<'_>> for UnaryOperator {
    type Error = ();

    fn try_from(value: Token<'_>) -> Result<Self, Self::Error> {
        Ok(match value {
            Token::Minus => Self::Minus,
            Token::Exclamation => Self::LogicalNot,
            Token::Tilde => Self::BitwiseNot,
            _ => return Err(()),
        })
    }
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BinaryOperator {
    // Math
    Plus,
    Minus,
    Multiply,
    Divide,
    Mod,

    // Comparison
    EqEq,
    Ne,
    Gt,
    Lt,
    Ge,
    Le,

    // Logical
    LogicalAnd,
    LogicalOr,

    // Bitwise
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    LeftShift,
    RightShift,

    // Assignment
    Eq,
    PlusEquals,
    MinusEquals,
    MultiplyEquals,
    DivideEquals,
    ModEquals,
    AndEquals,
    OrEquals,
    XorEquals,
    LeftShiftEquals,
    RightShiftEquals,
}

impl TryFrom<Token<'_>> for BinaryOperator {
    type Error = ();

    fn try_from(value: Token) -> Result<Self, Self::Error> {
        Ok(match value {
            Token::Plus => Self::Plus,
            Token::Star => Self::Multiply,
            Token::Slash => Self::Divide,
            Token::Minus => Self::Minus,
            Token::Percent => Self::Mod,
            Token::EqualsEquals => Self::EqEq,
            Token::NotEquals => Self::Ne,
            Token::GreaterThan => Self::Gt,
            Token::LesserThan => Self::Lt,
            Token::GreaterEquals => Self::Ge,
            Token::LesserEquals => Self::Le,
            Token::AndAnd => Self::LogicalAnd,
            Token::PipePipe => Self::LogicalOr,
            Token::And => Self::BitwiseAnd,
            Token::Pipe => Self::BitwiseOr,
            Token::LeftShift => Self::LeftShift,
            Token::RightShift => Self::RightShift,
            Token::Equals => Self::Eq,
            Token::PlusEquals => Self::PlusEquals,
            Token::MinusEquals => Self::MinusEquals,
            Token::StarEquals => Self::MultiplyEquals,
            Token::SlashEquals => Self::DivideEquals,
            Token::PercentEquals => Self::ModEquals,
            Token::AndEquals => Self::AndEquals,
            Token::PipeEquals => Self::OrEquals,
            Token::CaretEquals => Self::XorEquals,
            Token::LeftShiftEquals => Self::LeftShiftEquals,
            Token::RightShiftEquals => Self::RightShiftEquals,
            _ => return Err(()),
        })
    }
}

impl BinaryOperator {
    pub fn is_compound_assignment(&self) -> bool {
        matches!(
            self,
            Self::PlusEquals
                | Self::MinusEquals
                | Self::MultiplyEquals
                | Self::DivideEquals
                | Self::ModEquals
                | Self::AndEquals
                | Self::OrEquals
                | Self::XorEquals
                | Self::LeftShiftEquals
                | Self::RightShiftEquals
        )
    }

    pub fn compound_to_operator(self) -> Option<Self> {
        Some(match self {
            Self::PlusEquals => Self::Plus,
            Self::MinusEquals => Self::Minus,
            Self::MultiplyEquals => Self::Multiply,
            Self::DivideEquals => Self::Divide,
            Self::ModEquals => Self::Mod,
            Self::AndEquals => Self::BitwiseAnd,
            Self::OrEquals => Self::BitwiseOr,
            Self::XorEquals => Self::BitwiseXor,
            Self::LeftShiftEquals => Self::LeftShift,
            Self::RightShiftEquals => Self::RightShift,
            _ => return None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_compound_assignment() {
        assert!(BinaryOperator::PlusEquals.is_compound_assignment());
        assert!(BinaryOperator::MinusEquals.is_compound_assignment());
        assert!(BinaryOperator::MultiplyEquals.is_compound_assignment());
        assert!(BinaryOperator::DivideEquals.is_compound_assignment());
        assert!(BinaryOperator::ModEquals.is_compound_assignment());
        assert!(BinaryOperator::AndEquals.is_compound_assignment());
        assert!(BinaryOperator::OrEquals.is_compound_assignment());
        assert!(BinaryOperator::XorEquals.is_compound_assignment());
        assert!(BinaryOperator::LeftShiftEquals.is_compound_assignment());
        assert!(BinaryOperator::RightShiftEquals.is_compound_assignment());

        assert!(!BinaryOperator::Plus.is_compound_assignment());
        assert!(!BinaryOperator::Minus.is_compound_assignment());
        assert!(!BinaryOperator::Multiply.is_compound_assignment());
        assert!(!BinaryOperator::Divide.is_compound_assignment());
        assert!(!BinaryOperator::Mod.is_compound_assignment());
        assert!(!BinaryOperator::BitwiseAnd.is_compound_assignment());
        assert!(!BinaryOperator::BitwiseOr.is_compound_assignment());
        assert!(!BinaryOperator::BitwiseXor.is_compound_assignment());
        assert!(!BinaryOperator::LeftShift.is_compound_assignment());
        assert!(!BinaryOperator::RightShift.is_compound_assignment());
    }

    #[test]
    fn compound_to_operator() {
        assert_eq!(
            BinaryOperator::PlusEquals.compound_to_operator(),
            Some(BinaryOperator::Plus)
        );
        assert_eq!(
            BinaryOperator::MinusEquals.compound_to_operator(),
            Some(BinaryOperator::Minus)
        );
        assert_eq!(
            BinaryOperator::MultiplyEquals.compound_to_operator(),
            Some(BinaryOperator::Multiply)
        );
        assert_eq!(
            BinaryOperator::DivideEquals.compound_to_operator(),
            Some(BinaryOperator::Divide)
        );
        assert_eq!(
            BinaryOperator::ModEquals.compound_to_operator(),
            Some(BinaryOperator::Mod)
        );
        assert_eq!(
            BinaryOperator::AndEquals.compound_to_operator(),
            Some(BinaryOperator::BitwiseAnd)
        );
        assert_eq!(
            BinaryOperator::OrEquals.compound_to_operator(),
            Some(BinaryOperator::BitwiseOr)
        );
        assert_eq!(
            BinaryOperator::XorEquals.compound_to_operator(),
            Some(BinaryOperator::BitwiseXor)
        );
        assert_eq!(
            BinaryOperator::LeftShiftEquals.compound_to_operator(),
            Some(BinaryOperator::LeftShift)
        );
        assert_eq!(
            BinaryOperator::RightShiftEquals.compound_to_operator(),
            Some(BinaryOperator::RightShift)
        );
        assert_eq!(BinaryOperator::Plus.compound_to_operator(), None);
        assert_eq!(BinaryOperator::Minus.compound_to_operator(), None);
        assert_eq!(BinaryOperator::Multiply.compound_to_operator(), None);
        assert_eq!(BinaryOperator::Divide.compound_to_operator(), None);
        assert_eq!(BinaryOperator::Mod.compound_to_operator(), None);
        assert_eq!(BinaryOperator::BitwiseAnd.compound_to_operator(), None);
        assert_eq!(BinaryOperator::BitwiseOr.compound_to_operator(), None);
        assert_eq!(BinaryOperator::BitwiseXor.compound_to_operator(), None);
        assert_eq!(BinaryOperator::LeftShift.compound_to_operator(), None);
        assert_eq!(BinaryOperator::RightShift.compound_to_operator(), None);
    }
}
