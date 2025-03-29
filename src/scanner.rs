use std::fmt;
use std::string::String;

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: u64,
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.to_string(),
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, String> {
        let mut errors = vec![];
        while !self.is_at_end() {
            self.start = self.current;
            if let Err(msg) = self.scan_token() {
                errors.push(msg);
            }
        }
        self.tokens.push(Token {
            token_type: TokenType::EoF,
            lexeme: "".to_string(),
            literal: None,
            line_number: self.line,
        });

        if !errors.is_empty() {
            let joined = errors.join("\n");
            return Err(joined);
        }
        Ok(self.tokens.clone())
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) -> Result<(), String> {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LEFT_PAREN),
            ')' => self.add_token(TokenType::RIGHT_PAREN),
            '{' => self.add_token(TokenType::LEFT_BRACE),
            '}' => self.add_token(TokenType::RIGHT_BRACE),
            ',' => self.add_token(TokenType::COMMA),
            '.' => self.add_token(TokenType::DOT),
            '+' => self.add_token(TokenType::PLUS),
            '-' => self.add_token(TokenType::MINUS),
            ';' => self.add_token(TokenType::SEMICOLON),
            '*' => self.add_token(TokenType::STAR),
            '!' => {
                let token = if self.char_match('=') {
                    BANG_EQUAL
                } else {
                    BANG
                };
                self.add_token(token);
            }
            '=' => {
                let token = if self.char_match('=') {
                    EQUAL_EQUAL
                } else {
                    EQUAL
                };
                self.add_token(token);
            }
            '<' => {
                let token = if self.char_match('=') {
                    LESS_EQUAL
                } else {
                    LESS
                };
                self.add_token(token);
            }
            '>' => {
                let token = if self.char_match('=') {
                    GREATER_EQUAL
                } else {
                    GREATER
                };
                self.add_token(token);
            }
            '/' => {
                if self.char_match('/') {
                    loop {
                        if self.peek() == '\n' || self.is_at_end() {
                            break;
                        }
                        self.advance();
                    }
                } else {
                    self.add_token(SLASH);
                }
            }
            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,
            _ => return Err(format!("unrecognized char: {} at line {}", c, self.line)),
        }
        Ok(())
        // todo!()
    }

    fn peek(self: &Self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source.as_bytes()[self.current] as char
    }

    fn char_match(self: &mut Self, ch: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.as_bytes()[self.current] as char != ch {
            return false;
        } else {
            self.current += 1;
            return true;
        }
    }

    fn advance(&mut self) -> char {
        let c = self.source.chars().nth(self.current).unwrap_or('\0');
        self.current += 1;

        c as char
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_lit(token_type, None);
    }

    fn add_token_lit(&mut self, token_type: TokenType, literal: Option<LiteralValue>) {
        let text = self.source[self.start..self.current].to_string();
        self.tokens.push(Token {
            token_type,
            lexeme: text,
            literal,
            line_number: self.line,
        });
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,
    BANG,
    BANG_EQUAL,
    EQUAL,
    EQUAL_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,
    IDENTIFIER,
    STRING,
    NUMBER,
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,
    EoF,
}

use TokenType::*;

// Implement Display for TokenType
impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone)]
pub enum LiteralValue {
    IntValue(i64),
    FValue(f64),
    StringValue(String),
    IdentifierVal(String),
}

#[derive(Clone, Debug)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Option<LiteralValue>,
    line_number: u64,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        literal: Option<LiteralValue>,
        line_number: u64,
    ) -> Self {
        Self {
            token_type,
            lexeme,
            literal,
            line_number,
        }
    }

    pub fn to_string(&self) -> String {
        format!("{} {} {:?}", self.token_type, self.lexeme, self.literal)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handle_one_char_tokens() {
        let source = "((  ))";
        let mut scanner = Scanner::new(source);
        scanner.scan_tokens();
        println!("{:?}", scanner.tokens);
        assert_eq!(scanner.tokens.len(), 5);
        assert_eq!(scanner.tokens[0].token_type, LEFT_PAREN);
        assert_eq!(scanner.tokens[1].token_type, LEFT_PAREN);
        assert_eq!(scanner.tokens[2].token_type,  RIGHT_PAREN);
        assert_eq!(scanner.tokens[3].token_type,  RIGHT_PAREN);
        assert_eq!(scanner.tokens[4].token_type,  EoF);
    }

    #[test]
    fn handle_two_char_tokens() {
        let source = "! != == >=";
        let mut scanner = Scanner::new(source);
        scanner.scan_tokens();
        println!("{:?}", scanner.tokens);
        assert_eq!(scanner.tokens.len(), 5);
        assert_eq!(scanner.tokens[0].token_type, BANG);
        assert_eq!(scanner.tokens[1].token_type, BANG_EQUAL);
        assert_eq!(scanner.tokens[2].token_type,  EQUAL_EQUAL);
        assert_eq!(scanner.tokens[3].token_type,  GREATER_EQUAL);
        assert_eq!(scanner.tokens[4].token_type,  EoF);
    }
}