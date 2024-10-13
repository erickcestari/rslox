use std::{collections::HashMap, fmt::Error};

use crate::{literal::Literal, token::Token};

#[derive(Debug, Clone, PartialEq)]
pub struct Environment {
    enclosing: Option<Box<Environment>>,
    values: HashMap<String, Literal>,
}

impl Environment {
    pub fn new(enclosing: Option<Box<Environment>>) -> Self {
        Environment {
            enclosing,
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: Literal) {
        self.values.insert(name, value);
    }

    pub fn ancestor(&self, distance: usize) -> Option<Box<Environment>> {
      if distance == 0 {
          return Some(Box::new(self.clone()));
      }
      let mut environment = self;
      for _ in 0..distance {
          match &environment.enclosing {
              Some(enclosing) => environment = enclosing,
              None => return None,
          }
      }
      Some(Box::new(environment.clone()))
  }

  pub fn get_at(self, distance: usize, name: String) -> Option<Literal> {
    match self.ancestor(distance){
        Some(env) => env.values.get(&name).cloned(),
        None =>  None,
    }
  }

  pub fn assign_at(&mut self, distance: usize, name: Token, value: Literal) {
    match self.ancestor(distance){
      Some(mut env) => env.values.insert(name.lexeme, value),
      None =>  None,
    };
  }

  // pub fn get(&self, name: Token) -> Result<Option<Literal>, Error> {
  //   if self.values.contains_key(&name.lexeme) {
  //     return Ok(self.values.get(&name.lexeme))
  //   }

  //   if(self.enclosing.is_none()) {
  //     return self.enclosing.
  //   }
  // }
}