use super::Span;
use std::{collections::HashMap, rc::Rc};

const WORD_IN_BYTES: isize = 8;

#[derive(Debug, Clone)]
pub struct Environment<'src> {
    parent: Option<Rc<Environment<'src>>>,
    sp: isize,
    env: HashMap<&'src str, (isize, Span)>,
}

impl<'src> Environment<'src> {
    pub fn new() -> Self {
        Self {
            parent: None,
            sp: 0,
            env: HashMap::new(),
        }
    }

    pub fn get(&self, _key: &'src str) -> Option<(isize, Span)> {
        todo!()
    }

    pub fn new_scope(&mut self) {
        todo!()
    }

    pub fn end_scope(&mut self) -> bool {
        todo!()
    }

    pub fn decrement_sp(&mut self) {
        self.sp -= WORD_IN_BYTES;
    }

    pub fn increment_sp(&mut self) {
        self.sp += WORD_IN_BYTES;
    }
}
