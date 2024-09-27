pub mod operations;

use anyhow::bail;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Value {
    String(String),
    Boolean(bool),
    Number(f64),
    Null,
}

impl Value {
    pub fn as_bool(&self) -> bool {
        match self {
            Value::String(s) => !s.is_empty(),
            Value::Boolean(b) => *b,
            Value::Number(n) => *n != 0.0,
            Value::Null => false,
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Value::String(s) => s.to_owned(),
            Value::Boolean(b) => b.to_string(),
            Value::Number(n) => n.to_string(),
            Value::Null => return write!(f, "null"),
        };
        write!(f, "{}", str)
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

impl<T: Into<Value>> From<Option<T>> for Value {
    fn from(value: Option<T>) -> Self {
        match value {
            Some(t) => t.into(),
            None => Value::Null,
        }
    }
}

impl operations::TryAdd for Value {
    fn try_add(self, rhs: Self) -> anyhow::Result<Self> {
        Ok(match self {
            Value::String(lhs) => match rhs {
                Value::String(rhs) => format!("{lhs}{rhs}").into(),
                Value::Boolean(rhs) => format!("{lhs}{rhs}").into(),
                Value::Number(rhs) => format!("{lhs}{rhs}").into(),
                Value::Null => format!("{lhs}null").into(),
            },
            Value::Boolean(lhs) => match rhs {
                Value::String(rhs) => format!("{lhs}{rhs}").into(),
                Value::Boolean(rhs) => ((lhs as u8 + rhs as u8) as f64).into(),
                Value::Number(rhs) => ((lhs as u8) as f64 + rhs).into(),
                Value::Null => Value::Boolean(lhs),
            },
            Value::Number(lhs) => match rhs {
                Value::String(rhs) => format!("{lhs}{rhs}").into(),
                Value::Boolean(rhs) => (lhs + (rhs as u8) as f64).into(),
                Value::Number(rhs) => (lhs + rhs).into(),
                Value::Null => Value::Number(lhs),
            },
            Value::Null => match rhs {
                Value::String(rhs) => format!("null{rhs}").into(),
                Value::Boolean(rhs) => Value::Boolean(rhs),
                Value::Number(rhs) => Value::Number(rhs),
                Value::Null => Value::Null,
            },
        })
    }
}

impl operations::TrySub for Value {
    fn try_sub(self, rhs: Self) -> anyhow::Result<Self> {
        Ok(match self {
            Value::String(_) => match rhs {
                Value::String(_) => bail!("cannot subtract two strings"),
                Value::Boolean(_) => bail!("cannot subtract a boolean from a string"),
                Value::Number(_) => bail!("cannot subtract a number from a string"),
                Value::Null => bail!("cannot subtract null from a string"),
            },
            Value::Boolean(lhs) => match rhs {
                Value::String(_) => bail!("cannot subtract string from boolean"),
                Value::Boolean(rhs) => (((lhs as u8) as f64) - ((rhs as u8) as f64)).into(),
                Value::Number(rhs) => ((lhs as u8) as f64 - rhs).into(),
                Value::Null => Value::Boolean(lhs),
            },
            Value::Number(lhs) => match rhs {
                Value::String(_) => bail!("cannot subtract string from number"),
                Value::Boolean(rhs) => (lhs - ((rhs as u8) as f64)).into(),
                Value::Number(rhs) => (lhs - rhs).into(),
                Value::Null => Value::Number(lhs),
            },
            Value::Null => match rhs {
                Value::String(_) => bail!("cannot subtract string from null"),
                Value::Boolean(_) => bail!("cannot subtract boolean from null"),
                Value::Number(_) => bail!("cannot subtract number from null"),
                Value::Null => Value::Null,
            }
        })
    }
}

impl operations::TryMul for Value {
    fn try_mul(self, rhs: Self) -> anyhow::Result<Self> {
        Ok(match self {
            Value::String(_) => match rhs {
                Value::String(_) => bail!("cannot multiply two strings"),
                Value::Boolean(_) => bail!("cannot multiply a string and a boolean"),
                Value::Number(_) => bail!("cannot multiply a string and a number"),
                Value::Null => bail!("cannot multiply a string and null"),
            },
            Value::Boolean(lhs) => match rhs {
                Value::String(_) => bail!("cannot multiply a boolean and a string"),
                Value::Boolean(rhs) => (((lhs as u8) as f64) * ((rhs as u8) as f64)).into(),
                Value::Number(rhs) => ((lhs as u8) as f64 * rhs).into(),
                Value::Null => bail!("cannot multiply a boolean and null"),
            },
            Value::Number(lhs) => match rhs {
                Value::String(_) => bail!("cannot multiply a number and a string"),
                Value::Boolean(rhs) => (lhs * ((rhs as u8) as f64)).into(),
                Value::Number(rhs) => (lhs * rhs).into(),
                Value::Null => bail!("cannot multiply a number and null"),
            },
            Value::Null => match rhs {
                Value::String(_) => bail!("cannot multiply a string and null"),
                Value::Boolean(_) => bail!("cannot multiply a boolean and null"),
                Value::Number(_) => bail!("cannot multiply a number and null"),
                Value::Null => Value::Null,
            }
        })
    }
}

impl operations::TryDiv for Value {
    fn try_div(self, rhs: Self) -> anyhow::Result<Self> {
        Ok(match self {
            Value::String(_) => match rhs {
                Value::String(_) => bail!("cannot divide two strings"),
                Value::Boolean(_) => bail!("cannot divide a string and a boolean"),
                Value::Number(_) => bail!("cannot divide a string and a number"),
                Value::Null => bail!("cannot divide a string and null"),
            },
            Value::Boolean(lhs) => match rhs {
                Value::String(_) => bail!("cannot divide a boolean and a string"),
                Value::Boolean(rhs) => (((lhs as u8) as f64) / ((rhs as u8) as f64)).into(),
                Value::Number(rhs) => ((lhs as u8) as f64 / rhs).into(),
                Value::Null => bail!("cannot divide a boolean and null"),
            },
            Value::Number(lhs) => match rhs {
                Value::String(_) => bail!("cannot divide a number and a string"),
                Value::Boolean(rhs) => (lhs / ((rhs as u8) as f64)).into(),
                Value::Number(rhs) => (lhs / rhs).into(),
                Value::Null => bail!("cannot divide a number and null"),
            },
            Value::Null => match rhs {
                Value::String(_) => bail!("cannot divide a string and null"),
                Value::Boolean(_) => bail!("cannot divide a boolean and null"),
                Value::Number(_) => bail!("cannot divide a number and null"),
                Value::Null => Value::Null,
            }
        })
    }
}
