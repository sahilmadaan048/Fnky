use crate::expr::{Expr, LiteralValue};

pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Self {
        Self {}
    }
    //doing this for fun
    pub fn interpret(&mut self, expr: Expr) -> Result<LiteralValue, String> {
        expr.evaluate()
    }
}
