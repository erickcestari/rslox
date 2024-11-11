use core::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
}

impl Literal {
    pub fn to_string(&self) -> String {
        match self {
            Literal::Number(n) => n.to_string(),
            Literal::String(s) => s.to_string(),
            Literal::Boolean(b) => b.to_string(),
            Literal::Nil => "nil".into(),
        }
    }

    pub fn is_truthy(&self) -> bool {
        match self {
            Literal::Boolean(b) => *b,
            Literal::Nil => false,
            _ => true,
        }
    }

    pub fn is_number(&self) -> bool {
        match self {
            Literal::Number(_) => true,
            _ => false,
        }
    }
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
