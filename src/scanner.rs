use crate::{
    literal::Literal, token::Token, token_kind::TokenKind
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

        self.tokens.push(Token {
            lexeme: "".to_string(),
            line: self.line,
            token_type: TokenKind::Eof,
            literal: None,
        });
        self.tokens.clone()
    }

    pub fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token_no_literal(TokenKind::LeftParen),
            ')' => self.add_token_no_literal(TokenKind::RightParen),
            '{' => self.add_token_no_literal(TokenKind::LeftBrace),
            '}' => self.add_token_no_literal(TokenKind::RightBrace),
            ',' => self.add_token_no_literal(TokenKind::Comma),
            '.' => self.add_token_no_literal(TokenKind::Dot),
            '-' => self.add_token_no_literal(TokenKind::Minus),
            '+' => self.add_token_no_literal(TokenKind::Plus),
            ';' => self.add_token_no_literal(TokenKind::Semicolon),
            '*' => self.add_token_no_literal(TokenKind::Star),
            '!' => {
                let token_type = if self.match_char('=') {
                    TokenKind::BangEqual
                } else {
                    TokenKind::Bang
                };
                self.add_token_no_literal(token_type);
            }
            '=' => {
                let token_type = if self.match_char('=') {
                    TokenKind::EqualEqual
                } else {
                    TokenKind::Equal
                };
                self.add_token_no_literal(token_type);
            }
            '<' => {
                let token_type = if self.match_char('=') {
                    TokenKind::LessEqual
                } else {
                    TokenKind::Less
                };
                self.add_token_no_literal(token_type);
            }
            '>' => {
                let token_type = if self.match_char('=') {
                    TokenKind::GreaterEqual
                } else {
                    TokenKind::Greater
                };
                self.add_token_no_literal(token_type);
            }
            '/' => {
                if self.match_char('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token_no_literal(TokenKind::Slash);
                }
            }
            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,
            '"' => self.string(),
            '0'..='9' => self.number(),
            'a'..='z' | 'A'..='Z' | '_' => self.identifier(),
            _ => println!("Unexpected character"),
        }
    }

    pub fn advance(&mut self) -> char {
        self.current += 1;
        self.source.chars().nth(self.current - 1).unwrap()
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.chars().nth(self.current).unwrap() != expected {
            return false;
        }

        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source.chars().nth(self.current).unwrap()
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn add_token_no_literal(&mut self, token_type: TokenKind) {
        self.add_token(token_type, None)
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            println!("Unterminated string");
            return;
        }

        self.advance();

        let value = self.source[self.start + 1..self.current - 1].to_string();
        self.add_token(TokenKind::String, Some(Literal::String(value)));
    }

    fn number(&mut self) {
        while self.peek().is_digit(10) {
            self.advance();
        }

        if self.peek() == '.' && self.peek().is_digit(10) {
            self.advance();
            while self.peek().is_digit(10) {
                self.advance();
            }
        }

        let value = self.source[self.start..self.current]
            .parse::<f64>()
            .unwrap();
        self.add_token(TokenKind::Number, Some(Literal::Number(value)));
    }

    fn identifier(&mut self) {
        while self.peek().is_alphanumeric() {
            self.advance();
        }

        let text = &self.source[self.start..self.current];
        let token_type = match text {
            "and" => TokenKind::And,
            "class" => TokenKind::Class,
            "else" => TokenKind::Else,
            "false" => TokenKind::False,
            "for" => TokenKind::For,
            "fun" => TokenKind::Fun,
            "if" => TokenKind::If,
            "nil" => TokenKind::Nil,
            "or" => TokenKind::Or,
            "print" => TokenKind::Print,
            "return" => TokenKind::Return,
            "super" => TokenKind::Super,
            "this" => TokenKind::This,
            "true" => TokenKind::True,
            "var" => TokenKind::Var,
            "while" => TokenKind::While,
            _ => TokenKind::Identifier,
        };

        self.add_token(token_type, None);
    }

    fn add_token(&mut self, token_type: TokenKind, literal: Option<Literal>) {
        let token = self.source[self.start..self.current].to_string();
        self.tokens.push(Token {
            lexeme: token,
            line: self.line,
            token_type,
            literal,
        })
    }
}
