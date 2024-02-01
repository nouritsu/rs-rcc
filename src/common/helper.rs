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
    }

    #[test]
    fn index() {
        let tracker = LabelTracker::new();
        assert_eq!(tracker.index(LabelKind::Or), 0);
        assert_eq!(tracker.index(LabelKind::OrShortCircuit), 0);
        assert_eq!(tracker.index(LabelKind::And), 0);
        assert_eq!(tracker.index(LabelKind::AndShortCircuit), 0);
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
    }
}
