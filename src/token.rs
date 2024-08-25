use std::any::Any;

use dyn_clone::DynClone;

use crate::token_type::TokenType;
pub trait CloneAny: Any + DynClone {}
dyn_clone::clone_trait_object!(CloneAny);

impl<T> CloneAny for T where T: Any + Clone {}

#[derive(Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
    pub literal: Option<Box<dyn CloneAny>>,
}
