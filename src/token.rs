use crate::{literal::Literal, token_kind::TokenKind};

#[derive(Clone)]
pub struct Token {
    pub token_type: TokenKind,
    pub lexeme: String,
    pub line: usize,
    pub literal: Option<Literal>,
}
