use crate::expr::{Expr, LiteralValue};

pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Self {
        Self {}
    }
    //interpreter working fine with the expr.rs thingy
    pub fn interpret(&mut self, expr: Expr) -> Result<LiteralValue, String> {
        expr.evaluate()
    }
}
