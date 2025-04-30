use std::clone;
use std::fmt::format;

use crate::scanner;
use crate::scanner::{Token, TokenType};
use crate::environment::Environment;

#[derive(Clone, Debug, PartialEq)]
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

#[derive(Debug)]
pub enum Expr {
    Assign {
        name: Token, 
        value: Box<Expr>,
    },
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
    Variable {
        name: Token,
    },
}

impl LiteralValue {
    pub fn to_string(&self) -> String {
        match self {
            Number(x) => x.to_string(),
            StringValue(s) => s.clone(),
            True => "true".to_string(),
            False => "false".to_string(),
            Nil => "nil".to_string(),
        }
    }

    pub fn to_type(&self) -> &str {
        match self {
            Number(_) => "Number",
            StringValue(_) => "String",
            True | False => "Boolean",
            Nil => "nil",
        }
    }

    pub fn from_token(token: Token) -> Self {
        match token.token_type {
            TokenType::NUMBER => Self::Number(unwrap_as_f32(token.literal)),
            TokenType::STRING => Self::StringValue(unwrap_as_string(token.literal)),
            TokenType::FALSE => Self::False,
            TokenType::TRUE => Self::True,
            TokenType::NIL => Self::Nil,
            _ => panic!(
                "Could not create LiteralValue from token type {:?}",
                token.token_type
            ),
        }
    }

    pub fn from_bool(b: bool) -> Self {
        if b { True } else { False }
    }

    pub fn is_falsy(&self) -> bool {
        match self {
            Number(x) => *x == 0.0,
            StringValue(s) => s.is_empty(),
            True => false,
            False => true,
            Nil => true,
        }
    }
}

impl Expr {
    pub fn to_string(&self) -> String {
        match self {
            Expr::Assign { name, value } => format!("({name:?} : {})", value.to_string()),
            Expr::Binary    {
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
            Expr::Variable { name } => format!("(var {})",name.lexeme),
            _ => todo!()
        }
    }

    pub fn evaluate(&self, environment: &mut Environment) -> Result<LiteralValue, String> {
        match self {
            Expr::Assign { name, value } => {
                // First, evaluate the right‐hand side
                let new_value = value.evaluate(environment)?;
                
                let assign_success = environment.assign(&name.lexeme, new_value.clone());
                
                if(assign_success) {
                    Ok(new_value)

                } else {
                    Err(format!("Variable {:#?} has not been declared", name.lexeme))
                }
            }
            Expr::Variable { name } => {
                match environment.get(&name.lexeme.clone()) {
                    Some(value) => Ok(value.clone()),
                    None => Err(format!("Variable '{:#?}' has not declared", name.lexeme)),
                }
            }
            Expr::Literal { value } => Ok(value.clone()),

            Expr::Grouping { expression } => Ok(expression.evaluate(environment)?),

            Expr::Unary { operator, right } => {
                let right = right.evaluate(environment)?;

                match (right.clone(), operator.token_type) {
                    (Number(x), TokenType::MINUS) => Ok(Number(-x)),
                    (_, TokenType::MINUS) => {
                        Err(format!("Minus not implemented for {:?}", right.to_type()))
                    }
                    (any, TokenType::BANG) => Ok(LiteralValue::from_bool(any.is_falsy())),
                    (_, ttype) => Err(format!("{} is not a valid unary operator", ttype)),
                }
            }

            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let left = left.evaluate(environment)?;
                let right = right.evaluate(environment)?;

                match (&left, operator.token_type, &right) {
                    (Number(x), TokenType::PLUS, Number(y)) => Ok(Number(x + y)),
                    (Number(x), TokenType::MINUS, Number(y)) => Ok(Number(x - y)),
                    (Number(x), TokenType::STAR, Number(y)) => Ok(Number(x * y)),
                    (Number(x), TokenType::SLASH, Number(y)) => Ok(Number(x / y)),

                    (Number(x), TokenType::GREATER, Number(y)) => {
                        Ok(LiteralValue::from_bool(x > y))
                    }
                    (Number(x), TokenType::GREATER_EQUAL, Number(y)) => {
                        Ok(LiteralValue::from_bool(x >= y))
                    }
                    (Number(x), TokenType::LESS, Number(y)) => Ok(LiteralValue::from_bool(x < y)),
                    (Number(x), TokenType::LESS_EQUAL, Number(y)) => {
                        Ok(LiteralValue::from_bool(x <= y))
                    }

                    (StringValue(_), op, Number(_)) | (Number(_), op, StringValue(_)) => {
                        Err(format!("{} is not defined for string and number", op))
                    }

                    (StringValue(s1), TokenType::PLUS, StringValue(s2)) => {
                        Ok(StringValue(format!("{}{}", s1, s2)))
                    }

                    (x, TokenType::BANG_EQUAL, y) => Ok(LiteralValue::from_bool(x != y)),
                    (x, TokenType::EQUAL_EQUAL, y) => Ok(LiteralValue::from_bool(x == y)),

                    (StringValue(s1), TokenType::GREATER, StringValue(s2)) => {
                        Ok(LiteralValue::from_bool(s1 > s2))
                    }
                    (StringValue(s1), TokenType::GREATER_EQUAL, StringValue(s2)) => {
                        Ok(LiteralValue::from_bool(s1 >= s2))
                    }
                    (StringValue(s1), TokenType::LESS, StringValue(s2)) => {
                        Ok(LiteralValue::from_bool(s1 < s2))
                    }
                    (StringValue(s1), TokenType::LESS_EQUAL, StringValue(s2)) => {
                        Ok(LiteralValue::from_bool(s1 <= s2))
                    }

                    (x, ttype, y) => Err(format!(
                        "{} is not implemented for operands {:?} and {:?}",
                        ttype, x, y
                    )),
                }
            }
           // Expr::Variable { name } => Ok(expression.evaluate(environment)?),
            Expr::Variable { name } => todo!(),
            Expr::Assign { name, value } => todo!(),
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
        // commmiting after code review and revision
        ast.print();
        let result = ast.to_string();
        assert_eq!(result, "(* (- 123) (group 45.67))");
    }
}