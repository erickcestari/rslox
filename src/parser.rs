use std::vec::Vec;

use crate::{expression::Expr, literal::Literal, statment::Stmt, token::Token, token_kind::TokenKind};

#[derive(Debug)]
pub struct ParseError;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, ParseError> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            let token = self.advance();
            statements.push(Stmt::Expression(Expr::Variable(token)));
        }
        Ok(statements)
    }

    fn match_token(&mut self, kinds: &[TokenKind]) -> bool {
        for &token_kind in kinds {
            if self.check(token_kind) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn consume(&mut self, token_kind: TokenKind, message: &str) -> Result<Token, ParseError> {
        if self.check(token_kind) {
            return Ok(self.advance());
        }
        Err(self.error(self.peek(), message))
    }

    fn check(&self, token_kind: TokenKind) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().kind == token_kind
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous().clone()
    }

    fn is_at_end(&self) -> bool {
        self.peek().kind == TokenKind::Eof
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn error(&self, token: &Token, message: &str) -> ParseError {
        eprintln!("[line {}] Error at '{}': {}", token.line, token.lexeme, message);
        ParseError
    }
}