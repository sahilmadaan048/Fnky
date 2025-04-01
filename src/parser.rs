use crate::expr::{Expr, Expr::*};
use crate::expr::{LiteralValue, LiteralValue::*};
use crate::scanner::*;
use crate::scanner::{Token, TokenType, TokenType::*};

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

    fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();

        while match_tokens!(self, BANG_EQUAL, EQUAL_EQUAL) {
            let operator = self.previous();
            let rhs = self.comparison();
            expr = Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(rhs),
            };
        }
        expr
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();

        while match_tokens!(self, GREATER, GREATER_EQUAL, LESS, LESS_EQUAL) {
            let op = self.previous();
            let rhs = self.term();
            expr = Binary {
                left: Box::new(expr),
                operator: op,
                right: Box::new(rhs),
            };
        }
        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();

        while match_tokens!(self, MINUS, PLUS) {
            let op = self.previous();
            let rhs = self.factor();
            expr = Binary {
                left: Box::new(expr),
                operator: op,
                right: Box::new(rhs),
            };
        }
        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();
        while match_tokens!(self, SLASH, STAR) {
            let op = self.previous();
            let rhs = self.unary();
            expr = Binary {
                left: Box::new(expr),
                operator: op,
                right: Box::new(rhs),
            };
        }
        expr
    }

    fn unary(&mut self) -> Expr {
        if match_tokens!(self, BANG, MINUS) {
            let op = self.previous();
            let rhs = self.unary();
            Unary {
                operator: op,
                right: Box::new(rhs),
            }
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Expr {
        if self.match_token(&LEFT_PAREN) {
            let expr = self.expression();
            self.consume(RIGHT_PAREN, "Expected ')' ");
            Grouping {
                expression: Box::new(expr),
            }
        } else {
            let token = self.previous();
            Literal {
                value: LiteralValue::from_token(token),
            }
        }
    }

    fn consume(&mut self, token_type: TokenType, msg: &str) {
        if self.peek().token_type == token_type {
            self.advance();
        } else {
            panic!("{}", msg);
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

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == EoF
    }
}
