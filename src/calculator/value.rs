// src/calculator/value.rs
#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    Coroutine(Vec<String>),
}

impl Value {
    pub fn as_number(&self) -> Option<f64> {
        match self {
            Value::Number(n) => Some(*n),
            _ => None,
        }
    }
    
    pub fn as_coroutine(&self) -> Option<&Vec<String>> {
        match self {
            Value::Coroutine(c) => Some(c),
            _ => None,
        }
    }
}
