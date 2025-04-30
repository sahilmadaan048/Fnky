use std::{collections::HashMap, hash::Hash};
use crate::expr::LiteralValue;

pub struct Environment {
    values: HashMap<String, LiteralValue>,
    enclosing: Box<Environment>,
}

impl Environment{
    pub fn new() -> Self{
        Self {
            values: HashMap::new(),
            enclosing: Box::new_uninit(),
        }
    }

    pub fn define(&mut self, name: String, value: LiteralValue) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Option<&LiteralValue> {
        self.values.get(name)
    }
}