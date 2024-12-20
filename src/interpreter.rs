use std::collections::HashMap;

use crate::{
    environment::Environment,
    expression::{Evaluate, Expr},
    statement::Stmt,
    token::Token,
};

pub struct RuntimeError {
    pub message: String,
    pub token: Option<Token>,
}

impl RuntimeError {
    pub fn new(message: String, token: Option<Token>) -> Self {
        RuntimeError { message, token }
    }

    pub fn print(&self) {
        if self.token.is_none() {
            eprintln!("{}", self.message);
            return;
        }
        eprintln!(
            "{} \n[Line {} ]",
            self.message,
            self.token.clone().unwrap().line
        );
    }
}
pub struct Interpreter {
    pub globals: Environment,
    environment: Environment,
    locals: HashMap<Expr, usize>,
}

impl Interpreter {
    pub fn new() -> Self {
        let globals = Environment::new(None);
        let environment = globals.clone();
        let locals = HashMap::new();
        Interpreter {
            globals,
            environment,
            locals,
        }
    }

    pub fn interpret(&mut self, statements: Vec<Stmt>) -> Result<(), RuntimeError> {
        for statement in statements {
            if let Err(err) = self.execute(&statement) {
                return Err(err);
            }
        }
        Ok(())
    }

    fn execute(&mut self, statement: &Stmt) -> Result<(), RuntimeError> {
        match statement {
            Stmt::Expression(expr) => {
                expr.evaluate(&mut self.environment)?;
            }
            Stmt::Print(expr) => {
                let value = expr.evaluate(&mut self.environment)?;
                println!("{}", value);
            }
            Stmt::Var(name, expr) => match expr {
                Some(expr) => {
                    let value = expr.evaluate(&mut self.environment)?;
                    self.environment.define(name.lexeme.clone(), value);
                }
                None => {
                    self.environment
                        .define(name.lexeme.clone(), crate::literal::Literal::Nil);
                }
            },
            Stmt::Block(statements) => {
                self.environment = Environment::new(Some(Box::new(self.environment.clone())));
                self.interpret(statements.to_vec())?;
                self.environment = *self.environment.enclosing.clone().unwrap();
            }
            Stmt::If(condition, then_branch, else_branch) => {
                if condition.evaluate(&mut self.environment)?.is_truthy() {
                    self.execute(then_branch)?;
                } else if let Some(else_branch) = else_branch {
                    self.execute(else_branch)?;
                }
            }
            Stmt::While(condition, body) => {
                while condition.evaluate(&mut self.environment)?.is_truthy() {
                    self.execute(body)?;
                }
            }
            // Stmt::Function(name, params, body) => {
            //     let function = crate::literal::Literal::Function(
            //         name.lexeme.clone(),
            //         params.clone(),
            //         body.clone(),
            //         self.environment.clone(),
            //     );
            //     self.environment.define(name.lexeme.clone(), function);
            // }
            // Stmt::Return(keyword, value) => {
            //     let value = if let Some(value) = value {
            //         self.evaluate(value)?
            //     } else {
            //         crate::literal::Literal::Nil
            //     };
            //     return Err(RuntimeError::new(
            //         "Return".to_string(),
            //         Some(keyword.clone()),
            //     ));
            // }
            _ => unimplemented!(),
        }
        Ok(())
    }
}
