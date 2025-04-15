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
}
