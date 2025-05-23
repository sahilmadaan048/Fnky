use std::fmt::{self, format};

use crate::expr::Expr;
use crate::scanner::Token;

pub enum Stmt {
    Expression {
        expression: Expr,
    },
    Print {
        expression: Expr,
    },
    Var {
        expression: Expr,
        name: Token,
        initialiser: Expr,
    },  
    Block {
        statements: Vec<Stmt>,
    },
}


impl Stmt {
    #[allow(dead_code)]
    pub fn to_string(&self) -> String {
        use Stmt::*;
        match self {
            Expression { expression } => expression.to_string(),
            Print { expression } => format!("(print {})", expression.to_string()),
            Var {
                expression,
                name,
                initialiser,
            } => format!("(var {})", name.lexeme),
            Block {
                statements
            } => format!("(block {})", statements.iter().map(|stmt| stmt.to_string()).collect::<Vec<String>>().join(",")),            
        }
    }
}