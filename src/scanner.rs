use std::collections;
use std::collections::HashMap;
use std::fmt::{self, format};
use std::path::Ancestors;
use std::string::String;

fn is_digit(ch: char) -> bool {
    return ch as u8 >= '0' as u8 && ch as u8 <= '9' as u8;
}

fn is_alpha(ch: char) -> bool {
    let uch = ch as u8;
    return (uch >= 'a' as u8 && uch <= 'z' as u8)
        || (uch >= 'A' as u8 && uch <= 'Z' as u8)
        || (ch == '_');
}

fn is_alpha_numeric(ch: char) -> bool {
    is_alpha(ch) || is_digit(ch)
}

fn get_keywords_hashmap() -> HashMap<&'static str, TokenType> {
    HashMap::from([
        ("and", AND),
        ("class", CLASS),
        ("else", ELSE),
        ("false", FALSE),
        ("for", FOR),
        ("fun", FUN),
        ("if", IF),
        ("nil", NIL),
        ("or", OR),
        ("print", PRINT),
        ("return", RETURN),
        ("super", SUPER),
        ("this", THIS),
        ("true", TRUE),
        ("var", VAR),
        ("while", WHILE),
    ])
}

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: u64,

    keywords: HashMap<&'static str, TokenType>,
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.to_string(),
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
            keywords: get_keywords_hashmap(),
        }
    }

    pub fn get_keywords_hashmap() {}

    pub fn scan_tokens(self: &mut Self) -> Result<Vec<Token>, String> {
        let mut errors = vec![];
        while !self.is_at_end() {
            self.start = self.current;
            match self.scan_token() {
                Ok(_) => (),
                Err(msg) => errors.push(msg),
            }
        }

        self.tokens.push(Token {
            token_type: EoF,
            lexeme: "".to_string(),
            literal: None,
            line_number: self.line,
        });

        if errors.len() > 0 {
            let mut joined = "".to_string();
            for error in errors {
                joined.push_str(&error);
                joined.push_str("\n");
            }
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

            '"' => self.string()?,
            c => {
                if is_digit(c) {
                    self.number()?;
                } else if is_alpha(c) {
                    self.identifier()?;
                } else {
                    return Err(format!("unrecognized char: {} at line {}", c, self.line));
                }
            }
        }
        Ok(())
        // todo!()
    }

    fn identifier(&mut self) -> Result<(), String> {
        while is_alpha_numeric(self.peek()) {
            self.advance();
        }

        let substring = &self.source[self.start..self.current];
        if let Some(&t_type) = self.keywords.get(substring) {
            self.add_token(t_type);
        } else {
            self.add_token(IDENTIFIER);
        }
        Ok(())
    }

    fn number(self: &mut Self) -> Result<(), String> {
        while is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && is_digit(self.peek_next()) {
            self.advance();

            while is_digit(self.peek()) {
                self.advance();
            }
        }

        let substring = &self.source[self.start..self.current];
        let value = substring.parse::<f64>();

        match value {
            Ok(value) => self.add_token_lit(NUMBER, Some(FValue(value))),
            Err(_) => return Err("Failed to parse number".to_string()),
        }

        Ok(())
    }

    fn peek_next(self: &mut Self) -> char {
        if self.current >= self.source.len() {
            return '\0';
        }

        self.source.chars().nth(self.current + 1).unwrap()
    }

    fn string(self: &mut Self) -> Result<(), String> {
        // "ajajnka"
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }
        if self.is_at_end() {
            return Err("Undeterminated string".to_string());
        }
        self.advance();

        let value = &self.source[self.start + 1..self.current - 1]; // Remove quotes
        self.add_token_lit(
            TokenType::STRING,
            Some(LiteralValue::StringValue(value.to_string())),
        );
        Ok(())
    }

    fn peek(self: &Self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source.chars().nth(self.current).unwrap()
    }

    fn char_match(self: &mut Self, ch: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.chars().nth(self.current).unwrap() != ch {
            return false;
        } else {
            self.current += 1;
            return true;
        }
    }

    fn advance(&mut self) -> char {
        let c = self.source.chars().nth(self.current).unwrap_or('\0');
        self.current += 1;

        c
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
#[derive(Debug, Copy, Clone, PartialEq)]
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
use LiteralValue::*;
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
        assert_eq!(scanner.tokens[2].token_type, RIGHT_PAREN);
        assert_eq!(scanner.tokens[3].token_type, RIGHT_PAREN);
        assert_eq!(scanner.tokens[4].token_type, EoF);
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
        assert_eq!(scanner.tokens[2].token_type, EQUAL_EQUAL);
        assert_eq!(scanner.tokens[3].token_type, GREATER_EQUAL);
        assert_eq!(scanner.tokens[4].token_type, EoF);
    }

    #[test]
    fn handle_string_lit() {
        let source = r#""ABC""#; // Corrected input
        let mut scanner = Scanner::new(source);
        scanner.scan_tokens().expect("Failed to scan tokens");
        assert_eq!(scanner.tokens.len(), 2); // EOF token included
        assert_eq!(scanner.tokens[0].token_type, STRING);

        match scanner.tokens[0].literal.as_ref().unwrap() {
            StringValue(val) => assert_eq!(val, "ABC"),
            _ => panic!("Incorrect literal type"),
        }
    }

    #[test]
    fn handle_string_lit_unterminated() {
        let source = r#""ABC"#; // Corrected input
        let mut scanner = Scanner::new(source);
        let result = scanner.scan_tokens();
        match result {
            Err(_) => (),
            _ => panic!("should have failed"),
        }
    }

    #[test]
    fn handle_string_lit_multiline() {
        let source = "\"ABC\ndef\""; // Corrected input
        let mut scanner = Scanner::new(source);
        scanner.scan_tokens().expect("Failed to scan tokens");
        assert_eq!(scanner.tokens.len(), 2); // EOF token included
        assert_eq!(scanner.tokens[0].token_type, STRING);

        match scanner.tokens[0].literal.as_ref().unwrap() {
            StringValue(val) => assert_eq!(val, "ABC\ndef"),
            _ => panic!("Incorrect literal type"),
        }
    }

    #[test]
    fn number_literals() {
        let source = "123.123\n121.0\n5"; // Corrected input
        let mut scanner = Scanner::new(source);
        scanner.scan_tokens().unwrap();
        assert_eq!(scanner.tokens.len(), 4); // EOF token included
        for i in 0..3 {
            assert_eq!(scanner.tokens[i].token_type, NUMBER);
        }
        match scanner.tokens[0].literal.as_ref().unwrap() {
            FValue(val) => assert_eq!(*val, 123.123),
            _ => panic!("Incorrect literal type"),
        }
        match scanner.tokens[1].literal.as_ref().unwrap() {
            FValue(val) => assert_eq!(*val, 121.0),
            _ => panic!("Incorrect literal type"),
        }
        match scanner.tokens[2].literal.as_ref().unwrap() {
            FValue(val) => assert_eq!(*val, 5.0),
            _ => panic!("Incorrect literal type"),
        }
    }

    #[test]
    fn get_identifier() {
        let source = "this_is_a_var = 12;";
        let mut scanner = Scanner::new(source);
        scanner.scan_tokens().unwrap();
        assert_eq!(scanner.tokens.len(), 5);
        assert_eq!(scanner.tokens[0].token_type, IDENTIFIER);
        assert_eq!(scanner.tokens[1].token_type, EQUAL);
        assert_eq!(scanner.tokens[2].token_type, NUMBER);
        assert_eq!(scanner.tokens[3].token_type, SEMICOLON);
        assert_eq!(scanner.tokens[4].token_type, EoF);
    }

    #[test]
    fn get_keyword() {
        let source = r#"var this_is_a_var = 12; while true { print 3 };"#;
        let mut scanner = Scanner::new(source);
        scanner.scan_tokens().unwrap();

        assert_eq!(scanner.tokens.len(), 13);

        assert_eq!(scanner.tokens[0].token_type, VAR);
        assert_eq!(scanner.tokens[1].token_type, IDENTIFIER);
        assert_eq!(scanner.tokens[2].token_type, EQUAL);
        assert_eq!(scanner.tokens[3].token_type, NUMBER);
        assert_eq!(scanner.tokens[4].token_type, SEMICOLON);
        assert_eq!(scanner.tokens[5].token_type, WHILE);
        assert_eq!(scanner.tokens[6].token_type, TRUE);
        assert_eq!(scanner.tokens[7].token_type, LEFT_BRACE);
        assert_eq!(scanner.tokens[8].token_type, PRINT);
        assert_eq!(scanner.tokens[9].token_type, NUMBER);
        assert_eq!(scanner.tokens[10].token_type, RIGHT_BRACE);
        assert_eq!(scanner.tokens[11].token_type, SEMICOLON);
        assert_eq!(scanner.tokens[12].token_type, EoF);
    }
}
