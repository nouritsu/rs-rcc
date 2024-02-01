use std::{collections::HashMap, fmt::Display};
use strum::EnumCount;

pub struct LabelTracker(HashMap<LabelKind, usize>);

impl LabelTracker {
    pub fn new() -> Self {
        let hm = HashMap::from([
            (LabelKind::Or, 0),
            (LabelKind::OrShortCircuit, 0),
            (LabelKind::And, 0),
            (LabelKind::AndShortCircuit, 0),
        ]);
        assert_eq!(hm.len(), LabelKind::COUNT);
        Self(hm)
    }

    pub fn create(&mut self, kind: LabelKind) -> String {
        let s = format!("{}_{}", kind, self.index(kind));
        self.increment(kind);
        return s;
    }

    fn index(&self, kind: LabelKind) -> usize {
        self.0.get(&kind).copied().expect("infallible")
    }

    fn increment(&mut self, kind: LabelKind) {
        let i = self.0.get_mut(&kind).expect("infallible");
        *i += 1;
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, EnumCount)]
pub enum LabelKind {
    Or,
    OrShortCircuit,
    And,
    AndShortCircuit,
}

impl Display for LabelKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LabelKind::Or => write!(f, "or"),
            LabelKind::OrShortCircuit => write!(f, "or_ss"),
            LabelKind::And => write!(f, "and"),
            LabelKind::AndShortCircuit => write!(f, "and_ss"),
        }
    }
}
