use crate::scanner;
use crate::scanner::{Token, TokenType}; // For using scanner::LiteralValue

#[derive(Clone, Debug)]
pub enum LiteralValue {
    Number(f32),
    StringValue(String),
    True,
    False,
    Nil,
}
use LiteralValue::*;

fn unwrap_as_f32(literal: Option<scanner::LiteralValue>) -> f32 {
    match literal {
        Some(scanner::LiteralValue::IntValue(x)) => x as f32,
        Some(scanner::LiteralValue::FValue(x)) => x as f32,
        _ => panic!("Could not unwrap as f32"),
    }
}

fn unwrap_as_string(literal: Option<scanner::LiteralValue>) -> String {
    match literal {
        Some(scanner::LiteralValue::StringValue(s)) => s.clone(),
        Some(scanner::LiteralValue::IdentifierVal(s)) => s.clone(),
        _ => panic!("Could not unwrap as string"),
    }
}

pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Literal {
        value: LiteralValue,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
}

impl LiteralValue {
    pub fn to_string(&self) -> String {
        match self {
            LiteralValue::Number(x) => x.to_string(),
            LiteralValue::StringValue(s) => s.clone(),
            LiteralValue::True => "true".to_string(),
            LiteralValue::False => "false".to_string(),
            LiteralValue::Nil => "nil".to_string(),
        }
    }

    pub fn from_token(token: Token) -> Self {
        match token.token_type {
            TokenType::NUMBER => Self::Number(unwrap_as_f32(token.literal)),
            TokenType::STRING => Self::StringValue(unwrap_as_string(token.literal)),
            TokenType::FALSE => Self::False,
            TokenType::TRUE => Self::True,
            TokenType::NIL => Self::Nil,
            _ => panic!("Could not create LiteralValue from {:?}", token),
        }
    }

    pub fn is_falsy(&self) -> LiteralValue {
        match self {
            Number(x) => {
                if *x == 0.0 {
                    True
                } else {
                    False
                }
            }
            StringValue(s) => {
                if s.len() == 0 {
                    True
                } else {
                    False
                }
            }
            True => False,
            False => True,
            Nil => True,
        }
    }
}

impl Expr {
    pub fn to_string(&self) -> String {
        match self {
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                format!(
                    "({} {} {})",
                    operator.lexeme,
                    left.to_string(),
                    right.to_string()
                )
            }
            Expr::Grouping { expression } => format!("(group {})", expression.to_string()),
            Expr::Literal { value } => value.to_string(),
            Expr::Unary { operator, right } => {
                format!("({} {})", operator.lexeme, right.to_string())
            }
        }
    }

    pub fn evaluate(&self) -> Result<LiteralValue, String> {
        match self {
            Expr::Literal { value } => Ok(value.clone()),

            Expr::Grouping { expression } => expression.evaluate(),

            Expr::Unary { operator, right } => {
                let right = right.evaluate()?;

                match (right.clone(), operator.token_type) {
                    (LiteralValue::Number(x), TokenType::MINUS) => Ok(Number(-x)),
                    (_, TokenType::MINUS) => Err(format!("Minus not implemented for {:?}", right)),
                    (any, TokenType::BANG) => Ok(any.is_falsy()),
                    (LiteralValue::True, TokenType::BANG) => Ok(False),
                    (LiteralValue::False, TokenType::BANG) => Ok(True),
                    (_, TokenType::BANG) => Err(format!("Bang operator not implemented for {:?}", right)),
                    _ => Err(format!(
                        "Unsupported unary operation: {:?}",
                        operator.token_type
                    )),
                }
            }
            _ => todo!()
        }
    }

    pub fn print(&self) {
        println!("{}", self.to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::{Expr, LiteralValue};
    use crate::scanner::{Token, TokenType};

    #[test]
    fn pretty_print_ast() {
        let minus_token = Token {
            token_type: TokenType::MINUS,
            lexeme: "-".to_string(),
            literal: None,
            line_number: 0,
        };

        let onetwothree = Expr::Literal {
            value: LiteralValue::Number(123.0),
        };

        let group = Expr::Grouping {
            expression: Box::new(Expr::Literal {
                value: LiteralValue::Number(45.67),
            }),
        };

        let multi = Token {
            token_type: TokenType::STAR,
            lexeme: "*".to_string(),
            literal: None,
            line_number: 0,
        };

        let ast = Expr::Binary {
            left: Box::new(Expr::Unary {
                operator: minus_token,
                right: Box::new(onetwothree),
            }),
            operator: multi,
            right: Box::new(group),
        };

        ast.print();
        let result = ast.to_string();
        assert_eq!(result, "(* (- 123) (group 45.67))");
    }
}