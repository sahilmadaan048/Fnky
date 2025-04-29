use crate::expr::{Expr, LiteralValue};
use crate::stmt::Stmt;
use crate::environment::Environment;

pub struct Interpreter {
    //Global state
    environment: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Environment::new()
        }
    }

    pub fn interpret_expr(&mut self, expr: Expr) -> Result<LiteralValue, String> {
        expr.evaluate(&self.environment)
    }

    pub fn interpret(&mut self, stmts: Vec<Stmt>) -> Result<(), String> {
        for stmt in stmts {
            match stmt {
                Stmt::Expression { expression } => {
                    expression.evaluate(&self.environment)?;
                }
                Stmt::Print { expression } => {
                    let value = expression.evaluate(&self.environment)?;
                    println!("{value:?}");
                }
                Stmt::Var {
                    expression,
                    name,
                    initialiser,
                } => {
                    let value = initialiser.evaluate(&self.environment)?;
                    
                    self.environment.define(name.lexeme, value);
                }
            };
        }

        Ok(())
    }
}
