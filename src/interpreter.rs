use crate::expr::{Expr, LiteralValue};
use crate::stmt::Stmt;

pub struct Interpreter {
    //Global state
}

impl Interpreter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn interpret_expr(&mut self, expr: Expr) -> Result<LiteralValue, String> {
        expr.evaluate()
    }

    pub fn interpret(&mut self, stmts: Vec<Stmt>) -> Result<(), String> {
        for stmt in stmts {
            match stmt {
                Stmt::Expression { expression } => {
                    expression.evaluate()?;
                }
                Stmt::Print { expression } => {
                    let value = expression.evaluate()?;
                    println!("{value:?}");
                }
                Stmt::Var {
                    expression,
                    name,
                    initialiser,
                } => {
                    // Stub: Evaluate the expression and store it in a variable environment
                    let _value = expression.evaluate()?;
                    // TODO: Store `_value` into some environment map with variable name
                }
            };
        }

        Ok(())
    }
}
