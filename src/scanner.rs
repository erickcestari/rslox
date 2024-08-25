use crate::{
    token::{CloneAny, Token},
    token_type::TokenType,
};

pub struct Scanner {
    pub start: usize,
    pub current: usize,
    pub line: usize,
    pub source: String,
    tokens: Vec<Token>,
}

impl Scanner {
    pub fn new(source: &String) -> Self {
        Self {
            line: 1,
            current: 0,
            start: 0,
            source: source.clone(),
            tokens: Vec::new(),
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.clone()
    }

    pub fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token_no_literal(TokenType::LeftParen),
            ')' => self.add_token_no_literal(TokenType::RightParen),
            '{' => self.add_token_no_literal(TokenType::LeftBrace),
            '}' => self.add_token_no_literal(TokenType::RightBrace),
            ',' => self.add_token_no_literal(TokenType::Comma),
            '.' => self.add_token_no_literal(TokenType::Dot),
            '-' => self.add_token_no_literal(TokenType::Minus),
            '+' => self.add_token_no_literal(TokenType::Plus),
            ';' => self.add_token_no_literal(TokenType::Semicolon),
            '*' => self.add_token_no_literal(TokenType::Star),
            _ => println!("Unexpected character"),
        }
    }

    pub fn advance(&mut self) -> char {
        self.current += 1;
        self.source.chars().nth(self.current - 1).unwrap()
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn add_token_no_literal(&mut self, token_type: TokenType) {
        self.add_token(token_type, None)
    }

    fn add_token(&mut self, token_type: TokenType, literal: Option<Box<dyn CloneAny>>) {
        let token = self.source[self.start..self.current].to_string();
        self.tokens.push(Token {
            lexeme: token,
            line: self.line,
            token_type,
            literal,
        })
    }
}
