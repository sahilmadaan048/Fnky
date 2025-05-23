use std::char::EscapeUnicode;

use crate::expr::{Expr, Expr::*};
use crate::expr::{LiteralValue, LiteralValue::*};
use crate::scanner::*;
use crate::scanner::{Token, TokenType, TokenType::*};
use crate::stmt::Stmt;
use crate::stmt::Stmt::*;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

//the modification donen and reverted backk from sublime text
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
        let mut stmts: Vec<Stmt> = Vec::new();
        let mut errs: Vec<String> = Vec::new();

        while !self.is_at_end() {
            let stmt = self.declaration();
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

    fn declaration(&mut self) -> Result<Stmt, String> {
        if self.match_token(&VAR) {
            match self.var_declaration() {
                Ok(stmt) => Ok(stmt),
                Err(msg) => {
                    self.synchronize();
                    Err(msg)
                }
            }
        } else {
            self.statement()
        }
    }

    fn var_declaration(&mut self) -> Result<Stmt, String> {
        let token = self.consume(IDENTIFIER, "Expected bvariable name")?;

        let initialser;

        if self.match_token(&EQUAL) {
            initialser = self.expression()?;
        } else {
            initialser = Literal {value: LiteralValue::Nil };
        }

        self.consume(SEMICOLON, "Expected ':' after variable decalaration")?;

        Ok(Stmt:: Var {
            expression: self.expression()?,
            name: token,
            initialiser: initialser,
        })
    }

    fn statement(&mut self) -> Result<Stmt, String> {
        if self.match_token(&PRINT) {
            self.print_statement()
        } else if self.match_token(&LEFT_BRACE)  {
            self.block_statement()
        } else {
            self.expression_statement()
        }
    }

    fn block_statement(&mut self) -> Result<Stmt, String> {
        let mut statements = vec![];

        while !self.check(RIGHT_BRACE) && !self.is_at_end() {
            let decl = self.declaration()?;
            statements.push(decl);
        }
        self.consume(RIGHT_BRACE, "eXPECTED: '} AFTER A BLOCK");
        Ok( Stmt::Block { statements} )
    }

    fn print_statement(&mut self) -> Result<Stmt, String> {
        let value = self.expression()?;
        self.consume(SEMICOLON, "Expected ';' after '.'")?;
        Ok(Stmt::Print { expression: value })
    }

    fn expression_statement(&mut self) -> Result<Stmt, String> {
        let expr = self.expression()?;
        self.consume(SEMICOLON, "Expected ';' after expression")?;
        Ok(Stmt::Expression { expression: expr })
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

    //here we are checking for the parenthesis
    fn primary(&mut self) -> Result<Expr, String> {
        let token = self.peek().clone(); //returns the token where i am currently at

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
            Var => {
                self.advance();
                result = Ok(Variable { name: self.previous() });
            }
            _ => return Err("Expected expression".to_string()),
        }
        result
    }

    fn consume(&mut self, token_type: TokenType, msg: &str) -> Result<Token, String> {
        if self.peek().token_type == token_type {
            self.advance();
            let token = self.previous();
            Ok(token)
        } else {
            Err(msg.to_string())
        }
    }

    fn check(&mut self, typ: TokenType) -> bool {
        self.peek().token_type == typ
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

    /*
    Subtracts 1 from self.current, but if self.current is 0, it won’t panic or go negative — instead, it stays at 0.
    
    Purpose:
This method returns the token just before the current one in a parser or tokenizer, handling the start-of-list edge case gracefull
     */
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
        let string_expr = parsed_expr.to_string();
        // println!("{:?}", string_expr);
        assert_eq!(string_expr, "(+ 1 2)");
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