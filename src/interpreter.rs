use std::collections::HashMap;

use crate::{enviroment::Environment, expression::Expr};

pub struct RuntimeError {
    pub message: String,
}

impl RuntimeError {
    pub fn new(message: String) -> Self {
        RuntimeError { message }
    }
}
pub struct Interpreter {
    pub globals: Environment,
    environment: Environment,
    locals: HashMap<Expr, usize>,
}