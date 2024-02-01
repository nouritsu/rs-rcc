use super::Token;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Operator {
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
    LogicalNot,
    LogicalAnd,
    LogicalOr,

    // Bitwise
    BitwiseNot,
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

impl TryFrom<Token<'_>> for Operator {
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
            Token::Exclamation => Self::LogicalNot,
            Token::AndAnd => Self::LogicalAnd,
            Token::PipePipe => Self::LogicalOr,
            Token::Tilde => Self::BitwiseNot,
            Token::And => Self::BitwiseAnd,
            Token::Or => Self::BitwiseOr,
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

impl Operator {
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
