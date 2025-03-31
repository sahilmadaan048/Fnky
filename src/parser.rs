use crate::expr::{Expr, Expr::*};
use crate::scanner::{Token, TokenType, TokenType::*};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

macro_rules! match_tokens {
    ($self:expr, $($token:ident),+) => {
        {
            let mut result = false;
            $(
                result |= $self.match_token($token);
            )+
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
        todo!()
    }

    fn match_token(&mut self, typ: TokenType) -> bool {
        if self.is_at_end() {
            false
        } else if self.peek().token_type == typ {
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
