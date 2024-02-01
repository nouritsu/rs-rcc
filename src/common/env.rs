use super::Span;
use std::collections::HashMap;

const WORD_IN_BYTES: isize = 8;

#[derive(Debug, Clone)]
pub struct Environment<'src> {
    sp: isize,
    envs: Vec<HashMap<&'src str, (isize, Span)>>,
}

impl<'src> Environment<'src> {
    pub fn new() -> Self {
        Self {
            sp: 0,
            envs: vec![HashMap::new()],
        }
    }

    pub fn put(&mut self, key: &'src str, span: Span) -> bool {
        self.decrement_sp();
        (!self.contains(key))
            .then(|| {
                self.envs
                    .iter_mut()
                    .last()
                    .map(|env| env.insert(key, (self.sp, span)))
                    .flatten()
            })
            .flatten()
            .is_none()
    }

    pub fn get(&self, key: &str) -> Option<(isize, Span)> {
        self.envs.iter().rev().find_map(|env| env.get(key).copied())
    }

    pub fn contains(&self, key: &str) -> bool {
        self.envs
            .iter()
            .last()
            .map(|env| env.get(key))
            .flatten()
            .is_some()
    }

    pub fn new_scope(&mut self) {
        self.envs.push(HashMap::new())
    }

    pub fn end_scope(&mut self) -> bool {
        self.envs.len() != 1 && self.envs.pop().is_some()
    }

    fn decrement_sp(&mut self) {
        self.sp -= WORD_IN_BYTES;
    }
}
