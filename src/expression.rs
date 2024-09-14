use crate::{literal::Literal, token::Token};

#[derive(Debug, Clone)]
pub enum Expr {
    Literal(Literal),
    Variable(Token),
    Assign(Token, Box<Expr>),
    Binary(Box<Expr>, Token, Box<Expr>),
    Logical(Box<Expr>, Token, Box<Expr>),
    Unary(Token, Box<Expr>),
    Grouping(Box<Expr>),
    Call(Box<Expr>, Token, Vec<Expr>),
}