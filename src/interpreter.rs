use crate::expr::{Expr, LiteralValue};
use crate::stmt::Stmt;
use crate::environment::Environment;
use std::f32::consts::E;
use std::rc::Rc;

pub struct Interpreter{
    //Global state
    environment: Rc<Environment>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Rc::new(Environment::new()),
        }
    }

    // pub fn interpret_expr(&mut self, expr: Expr) -> Result<(), String> {
    //     expr.evaluate(Rc::get_mut(&mut self.environment).expect("could not get mutable refernce to environment"))?;
    // }

    pub fn interpret(&mut self, stmts: Vec<Stmt>) -> Result<(), String> {
        for stmt in stmts {
            match stmt {
                Stmt::Expression { expression } => {
                    expression.evaluate(Rc::get_mut(&mut self.environment).expect("could not get mutable refernce to environment"))?;
                }
                Stmt::Print { expression } => {
                    let value = expression.evaluate(Rc::get_mut(&mut self.environment).expect("could not get mutable refernce to environment"))?;
                    println!("{value:?}");
                }
                Stmt::Var {
                    expression,
                    name,
                    initialiser,
                } => {
                    let value = initialiser.evaluate(Rc::get_mut(&mut self.environment).expect("could not get mutable refernce to environment"))?;

                    Rc::get_mut(&mut self.environment).expect("could not get mutable ref to env").define(name.lexeme, value);
                }
                Stmt::Block {
                    statements
                } => {
                    let mut new_environment = Environment::new();
                    new_environment.enclosing = Some(self.environment.clone());                    
                    let old_environment = self.environment.clone();
                    self.environment = Rc::new(new_environment);
                    let block_result = self.interpret(statements);
                    self.environment = old_environment;
                    
                    block_result?;
                }
            };
        }
        Ok(())
    }
}