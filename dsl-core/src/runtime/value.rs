use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Value {
    String(String),
    Boolean(bool),
    Number(f64),
}

impl Value {
    pub fn as_bool(&self) -> bool {
        match self {
            Value::String(s) => !s.is_empty(),
            Value::Boolean(b) => *b,
            Value::Number(n) => *n != 0.0,
        }
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}
