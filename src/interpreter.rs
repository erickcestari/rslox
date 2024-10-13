use std::collections::HashMap;

use crate::{enviroment::Environment, expression::Expr};

pub struct Interpreter {
    pub globals: Environment,
    environment: Environment,
    locals: HashMap<Expr, usize>,
}