use crate::{literal::Literal, token_kind::TokenKind};

#[derive(Clone, Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub line: usize,
    pub literal: Option<Literal>,
}
