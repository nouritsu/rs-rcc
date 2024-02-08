use super::Span;
use std::collections::HashMap;

const WORD_IN_BYTES: isize = 8;

#[derive(Debug, Clone)]
pub struct Environment<'src> {
    pub sp: isize,
    envs: Vec<HashMap<&'src str, (isize, Span)>>,
}

impl<'src> Environment<'src> {
    pub fn new() -> Self {
        Self {
            sp: 0,
            envs: vec![],
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
        self.get(key).is_some()
    }

    pub fn new_scope(&mut self) {
        self.envs.push(HashMap::new())
    }

    pub fn end_scope(&mut self) -> Option<isize> {
        (!self.envs.is_empty())
            .then_some(
                self.envs
                    .pop()
                    .map(|env| env.len() as isize * WORD_IN_BYTES),
            )
            .flatten()
    }

    fn decrement_sp(&mut self) {
        self.sp -= WORD_IN_BYTES;
    }

    fn _increment_sp(&mut self) {
        self.sp += WORD_IN_BYTES;
    }
}
