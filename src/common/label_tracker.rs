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
            (LabelKind::TernaryElse, 0),
            (LabelKind::TernaryEnd, 0),
            (LabelKind::IfElse, 0),
            (LabelKind::IfEnd, 0),
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
    TernaryElse,
    TernaryEnd,
    IfElse,
    IfEnd,
}

impl Display for LabelKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LabelKind::Or => write!(f, "or"),
            LabelKind::OrShortCircuit => write!(f, "or_ss"),
            LabelKind::And => write!(f, "and"),
            LabelKind::AndShortCircuit => write!(f, "and_ss"),
            LabelKind::TernaryElse => write!(f, "cond_else"),
            LabelKind::TernaryEnd => write!(f, "cond_end"),
            LabelKind::IfElse => write!(f, "if_else"),
            LabelKind::IfEnd => write!(f, "if_end"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create() {
        let mut tracker = LabelTracker::new();
        let label = tracker.create(LabelKind::Or);
        assert_eq!(label, "or_0");
        let label = tracker.create(LabelKind::OrShortCircuit);
        assert_eq!(label, "or_ss_0");
        let label = tracker.create(LabelKind::And);
        assert_eq!(label, "and_0");
        let label = tracker.create(LabelKind::AndShortCircuit);
        assert_eq!(label, "and_ss_0");
        let label = tracker.create(LabelKind::TernaryElse);
        assert_eq!(label, "cond_else_0");
        let label = tracker.create(LabelKind::TernaryEnd);
        assert_eq!(label, "cond_end_0");
        let label = tracker.create(LabelKind::IfElse);
        assert_eq!(label, "if_else_0");
        let label = tracker.create(LabelKind::IfEnd);
        assert_eq!(label, "if_end_0");
    }

    #[test]
    fn index() {
        let tracker = LabelTracker::new();
        assert_eq!(tracker.index(LabelKind::Or), 0);
        assert_eq!(tracker.index(LabelKind::OrShortCircuit), 0);
        assert_eq!(tracker.index(LabelKind::And), 0);
        assert_eq!(tracker.index(LabelKind::AndShortCircuit), 0);
        assert_eq!(tracker.index(LabelKind::TernaryElse), 0);
        assert_eq!(tracker.index(LabelKind::TernaryEnd), 0);
        assert_eq!(tracker.index(LabelKind::IfElse), 0);
        assert_eq!(tracker.index(LabelKind::IfEnd), 0);
    }

    #[test]
    fn increment() {
        let mut tracker = LabelTracker::new();
        tracker.increment(LabelKind::Or);
        assert_eq!(tracker.index(LabelKind::Or), 1);
        tracker.increment(LabelKind::OrShortCircuit);
        assert_eq!(tracker.index(LabelKind::OrShortCircuit), 1);
        tracker.increment(LabelKind::And);
        assert_eq!(tracker.index(LabelKind::And), 1);
        tracker.increment(LabelKind::AndShortCircuit);
        assert_eq!(tracker.index(LabelKind::AndShortCircuit), 1);
        tracker.increment(LabelKind::TernaryElse);
        assert_eq!(tracker.index(LabelKind::TernaryElse), 1);
        tracker.increment(LabelKind::TernaryEnd);
        assert_eq!(tracker.index(LabelKind::TernaryEnd), 1);
        tracker.increment(LabelKind::IfElse);
        assert_eq!(tracker.index(LabelKind::IfElse), 1);
        tracker.increment(LabelKind::IfEnd);
        assert_eq!(tracker.index(LabelKind::IfEnd), 1);
    }
}
