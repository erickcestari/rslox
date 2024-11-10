use crate::{
    environment::Environment, interpreter::RuntimeError, literal::Literal, token::Token,
    token_kind::TokenKind,
};

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

pub trait Evaluate {
    fn evaluate(&self, environment: &mut Environment) -> Result<Literal, RuntimeError>;
}
impl Evaluate for Expr {
    fn evaluate(&self, environment: &mut Environment) -> Result<Literal, RuntimeError> {
        match self {
            Expr::Literal(literal) => Ok(literal.clone()),
            Expr::Variable(token) => environment.get(token.clone()),
            Expr::Assign(token, expr) => {
                let value = expr.evaluate(environment)?;
                environment.assign(token.clone(), value.clone())?;
                Ok(value)
            }
            Expr::Binary(left, token, right) => match token.kind {
                TokenKind::Plus => {
                    let left = left.evaluate(environment)?;
                    let right = right.evaluate(environment)?;
                    match (left, right) {
                        (Literal::Number(left), Literal::Number(right)) => {
                            return Ok(Literal::Number(left + right))
                        }
                        (Literal::String(left), Literal::String(right)) => {
                            return Ok(Literal::String(format!("{}{}", left, right)))
                        }
                        (Literal::String(left), Literal::Number(right)) => {
                            return Ok(Literal::String(format!("{}{}", left, right)))
                        }
                        (Literal::Number(left), Literal::String(right)) => {
                            return Ok(Literal::String(format!("{}{}", left, right)))
                        }
                        _ => {
                            return Err(RuntimeError::new(
                                "Operands must be numbers or strings".to_string(),
                                Some(token.clone()),
                            ))
                        }
                    }
                }
                TokenKind::Minus => {
                    let left = left.evaluate(environment)?;
                    let right = right.evaluate(environment)?;
                    match (left, right) {
                        (Literal::Number(left), Literal::Number(right)) => {
                            return Ok(Literal::Number(left - right))
                        }
                        _ => {
                            return Err(RuntimeError::new(
                                "Operands must be two numbers".to_string(),
                                Some(token.clone()),
                            ))
                        }
                    }
                }
                TokenKind::Slash => {
                    let left = left.evaluate(environment)?;
                    let right = right.evaluate(environment)?;
                    match (left, right) {
                        (Literal::Number(left), Literal::Number(right)) => {
                            return Ok(Literal::Number(left / right))
                        }
                        _ => {
                            return Err(RuntimeError::new(
                                "Operands must be two numbers".to_string(),
                                Some(token.clone()),
                            ))
                        }
                    }
                }
                TokenKind::Star => {
                    let left = left.evaluate(environment)?;
                    let right = right.evaluate(environment)?;
                    match (left, right) {
                        (Literal::Number(left), Literal::Number(right)) => {
                            return Ok(Literal::Number(left * right))
                        }
                        _ => {
                            return Err(RuntimeError::new(
                                "Operands must be two numbers".to_string(),
                                Some(token.clone()),
                            ))
                        }
                    }
                }
                TokenKind::Greater => {
                    let left = left.evaluate(environment)?;
                    let right = right.evaluate(environment)?;
                    match (left, right) {
                        (Literal::Number(left), Literal::Number(right)) => {
                            return Ok(Literal::Boolean(left > right))
                        }
                        _ => {
                            return Err(RuntimeError::new(
                                "Operands must be two numbers".to_string(),
                                Some(token.clone()),
                            ))
                        }
                    }
                }
                TokenKind::GreaterEqual => {
                    let left = left.evaluate(environment)?;
                    let right = right.evaluate(environment)?;
                    match (left, right) {
                        (Literal::Number(left), Literal::Number(right)) => {
                            return Ok(Literal::Boolean(left >= right))
                        }
                        _ => {
                            return Err(RuntimeError::new(
                                "Operands must be two numbers".to_string(),
                                Some(token.clone()),
                            ))
                        }
                    }
                }
                TokenKind::Less => {
                    let left = left.evaluate(environment)?;
                    let right = right.evaluate(environment)?;
                    match (left, right) {
                        (Literal::Number(left), Literal::Number(right)) => {
                            return Ok(Literal::Boolean(left < right))
                        }
                        _ => {
                            return Err(RuntimeError::new(
                                "Operands must be two numbers".to_string(),
                                Some(token.clone()),
                            ))
                        }
                    }
                }
                TokenKind::LessEqual => {
                    let left = left.evaluate(environment)?;
                    let right = right.evaluate(environment)?;
                    match (left, right) {
                        (Literal::Number(left), Literal::Number(right)) => {
                            return Ok(Literal::Boolean(left <= right))
                        }
                        _ => {
                            return Err(RuntimeError::new(
                                "Operands must be two numbers".to_string(),
                                Some(token.clone()),
                            ))
                        }
                    }
                }
                TokenKind::EqualEqual => {
                    let left = left.evaluate(environment)?;
                    let right = right.evaluate(environment)?;
                    return Ok(Literal::Boolean(left == right));
                }
                TokenKind::BangEqual => {
                    let left = left.evaluate(environment)?;
                    let right = right.evaluate(environment)?;
                    return Ok(Literal::Boolean(left != right));
                }
                _ => panic!("Invalid binary operator"),
            },
            Expr::Logical(left, token, right) => {
                // Implement logical operation evaluation logic here
                unimplemented!("Logical operation evaluation is not implemented yet")
            }
            Expr::Unary(token, expr) => {
                // Implement unary operation evaluation logic here
                unimplemented!("Unary operation evaluation is not implemented yet")
            }
            Expr::Grouping(expr) => expr.evaluate(environment),
            Expr::Call(callee, token, arguments) => {
                // Implement function call evaluation logic here
                unimplemented!("Function call evaluation is not implemented yet")
            }
        }
    }
}
