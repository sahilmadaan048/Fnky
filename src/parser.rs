use crate::expr::{Expr, Expr::*};
use crate::expr::{LiteralValue, LiteralValue::*};
use crate::scanner::*;
use crate::scanner::{Token, TokenType, TokenType::*};
use crate::stmt::Stmt;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

macro_rules! match_tokens {
    ($self:expr, $($token:ident),+) => {
        {
            let mut result = false;
            $( result |= $self.match_token(&$token); )+
            result
        }
    };
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, String> {
        let mut stmts = Vec![];
        let mut errs = Vec![];

        while !self.is_at_end() {
            let stmt = self.statement();
            match stmt {
                Ok(s) => stmts.push(s),
                Err(msg) => errs.push(msg),
            }
        }

        if errs.len() == 0 {
            Ok(stmts)
        } else {
            Err(errs.join("\n"))
        }
    }

    fn statement(&mut self) -> Result<Stmt, String> {
        todo!()
    }

    pub fn expression(&mut self) -> Result<Expr, String> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, String> {
        let mut expr = self.comparison()?;
        while match_tokens!(self, BANG_EQUAL, EQUAL_EQUAL) {
            let operator = self.previous();
            let rhs = self.comparison()?;
            expr = Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(rhs),
            };
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, String> {
        let mut expr = self.term()?;
        while match_tokens!(self, GREATER, GREATER_EQUAL, LESS, LESS_EQUAL) {
            let op = self.previous();
            let rhs = self.term()?;
            expr = Binary {
                left: Box::new(expr),
                operator: op,
                right: Box::new(rhs),
            };
        }
        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, String> {
        let mut expr = self.factor()?;
        while match_tokens!(self, MINUS, PLUS) {
            let op = self.previous();
            let rhs = self.factor()?;
            expr = Binary {
                left: Box::new(expr),
                operator: op,
                right: Box::new(rhs),
            };
        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, String> {
        let mut expr = self.unary()?;
        while match_tokens!(self, SLASH, STAR) {
            let op = self.previous();
            let rhs = self.unary()?;
            expr = Binary {
                left: Box::new(expr),
                operator: op,
                right: Box::new(rhs),
            };
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, String> {
        if match_tokens!(self, BANG, MINUS) {
            let op = self.previous();
            let rhs = self.unary()?;
            Ok(Unary {
                operator: op,
                right: Box::new(rhs),
            })
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Result<Expr, String> {
        let token = self.peek().clone();

        let result;
        match token.token_type {
            LEFT_PAREN => {
                self.advance();
                let expr = self.expression()?;
                self.consume(RIGHT_PAREN, "Expected: ')'")?;
                result = Ok(Grouping {
                    expression: Box::from(expr),
                });
            }
            FALSE | TRUE | NIL | NUMBER | STRING => {
                self.advance();
                result = Ok(Literal {
                    value: LiteralValue::from_token(token.clone()),
                })
            }
            _ => return Err("Expected expression".to_string()),
        }
        result
    }

    fn consume(&mut self, token_type: TokenType, msg: &str) -> Result<(), String> {
        if self.peek().token_type == token_type {
            self.advance();
            Ok(())
        } else {
            Err(msg.to_string())
        }
    }

    fn match_token(&mut self, typ: &TokenType) -> bool {
        if self.is_at_end() {
            false
        } else if &self.peek().token_type == typ {
            self.advance();
            true
        } else {
            false
        }
    }

    fn advance(&mut self) {
        if !self.is_at_end() {
            self.current += 1;
        }
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> Token {
        self.tokens[self.current.saturating_sub(1)].clone()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == EoF
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == SEMICOLON {
                return;
            }

            match self.peek().token_type {
                CLASS | FUN | VAR | FOR | IF | WHILE | PRINT | RETURN => return,
                _ => (),
            }

            self.advance();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scanner::LiteralValue::*;

    #[test]
    fn test_addition() {
        let tokens = vec![
            Token {
                token_type: NUMBER,
                lexeme: "1".to_string(),
                literal: Some(IntValue(1)),
                line_number: 0,
            },
            Token {
                token_type: PLUS,
                lexeme: "+".to_string(),
                literal: None,
                line_number: 0,
            },
            Token {
                token_type: NUMBER,
                lexeme: "2".to_string(),
                literal: Some(IntValue(2)),
                line_number: 0,
            },
            Token {
                token_type: SEMICOLON,
                lexeme: ";".to_string(),
                literal: None,
                line_number: 0,
            },
        ];
        let mut parser = Parser::new(tokens);
        let parsed_expr = parser.expression().unwrap();
        assert_eq!(parsed_expr.to_string(), "(+ 1 2)");
    }

    #[test]
    fn test_comparison() {
        let tokens = vec![
            Token {
                token_type: NUMBER,
                lexeme: "1".to_string(),
                literal: Some(IntValue(1)),
                line_number: 0,
            },
            Token {
                token_type: GREATER,
                lexeme: ">".to_string(),
                literal: None,
                line_number: 0,
            },
            Token {
                token_type: NUMBER,
                lexeme: "2".to_string(),
                literal: Some(IntValue(2)),
                line_number: 0,
            },
            Token {
                token_type: SEMICOLON,
                lexeme: ";".to_string(),
                literal: None,
                line_number: 0,
            },
        ];
        let mut parser = Parser::new(tokens);
        let parsed_expr = parser.expression().unwrap();
        assert_eq!(parsed_expr.to_string(), "(> 1 2)");
    }

    #[test]
    fn test_comparison2() {
        let source = "1 + 2 == 5 + 7";
        let mut scanner = Scanner::new(source);

        let tokens = scanner.scan_tokens().unwrap();
        let mut parser = Parser::new(tokens);

        let mut parsed_expr = parser.expression().unwrap();
        let string_expr = parsed_expr.to_string();

        println!("{}", string_expr);

        assert_eq!(string_expr, "(== (+ 1 2) (+ 5 7))");
    }

    #[test]
    fn test_eq_with_param() {
        let source = "1 == (2 + 2)";
        let mut scanner = Scanner::new(source);

        let tokens = scanner.scan_tokens().unwrap();
        let mut parser = Parser::new(tokens);

        let mut parsed_expr = parser.expression().unwrap();
        let string_expr = parsed_expr.to_string();

        println!("{}", string_expr);

        assert_eq!(string_expr, "(== 1 (group (+ 2 2)))");
    }
}

