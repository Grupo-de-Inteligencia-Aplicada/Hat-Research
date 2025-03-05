pub mod operations;
pub mod time;

use anyhow::{bail, Context};
use chrono::Timelike;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use time::Time;

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum Value {
    String(String),
    Boolean(bool),
    Number(f64),
    Time(Time),
    Null,
}

impl Value {
    pub fn as_bool(&self) -> bool {
        match self {
            Value::String(s) => !s.is_empty(),
            Value::Boolean(b) => *b,
            Value::Number(n) => *n != 0.0,
            Value::Time(t) => Some(t) != Time::from_hms_opt(0, 0, 0).as_ref(),
            Value::Null => false,
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Value::String(s) => format!("\"{s}\""),
            Value::Boolean(b) => b.to_string(),
            Value::Number(n) => n.to_string(),
            Value::Time(t) => t.to_string(),
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

impl From<Time> for Value {
    fn from(value: Time) -> Self {
        Self::Time(value)
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
                Value::Time(rhs) => format!("{lhs}{rhs}").into(),
                Value::Null => format!("{lhs}null").into(),
            },
            Value::Boolean(lhs) => match rhs {
                Value::String(rhs) => format!("{lhs}{rhs}").into(),
                Value::Boolean(rhs) => ((lhs as u8 + rhs as u8) as f64).into(),
                Value::Number(rhs) => ((lhs as u8) as f64 + rhs).into(),
                Value::Time(_) => bail!("cannot add boolean and time"),
                Value::Null => Value::Boolean(lhs),
            },
            Value::Number(lhs) => match rhs {
                Value::String(rhs) => format!("{lhs}{rhs}").into(),
                Value::Boolean(rhs) => (lhs + (rhs as u8) as f64).into(),
                Value::Number(rhs) => (lhs + rhs).into(),
                Value::Time(_) => bail!("cannot add time and number"),
                Value::Null => Value::Number(lhs),
            },
            Value::Null => match rhs {
                Value::String(rhs) => format!("null{rhs}").into(),
                Value::Boolean(rhs) => Value::Boolean(rhs),
                Value::Number(rhs) => Value::Number(rhs),
                Value::Time(rhs) => Value::Time(rhs),
                Value::Null => Value::Null,
            },
            Value::Time(lhs) => match rhs {
                Value::String(rhs) => format!("{lhs}{rhs}").into(),
                Value::Boolean(_) => bail!("cannot add time and boolean"),
                Value::Number(_) => bail!("cannot add time and number"),
                Value::Time(rhs) => Value::Time(
                    Time::from_hms_opt(
                        lhs.hour() + rhs.hour(),
                        lhs.minute() + rhs.minute(),
                        lhs.second() + rhs.second(),
                    )
                    .context("failed to add times together")?,
                ),
                Value::Null => bail!("cannot add null to a time"),
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
                Value::Time(_) => bail!("cannot subtract time from a string"),
                Value::Null => bail!("cannot subtract null from a string"),
            },
            Value::Boolean(lhs) => match rhs {
                Value::String(_) => bail!("cannot subtract string from boolean"),
                Value::Boolean(rhs) => (((lhs as u8) as f64) - ((rhs as u8) as f64)).into(),
                Value::Number(rhs) => ((lhs as u8) as f64 - rhs).into(),
                Value::Time(_) => bail!("cannot subtract time from a boolean"),
                Value::Null => Value::Boolean(lhs),
            },
            Value::Number(lhs) => match rhs {
                Value::String(_) => bail!("cannot subtract string from number"),
                Value::Boolean(rhs) => (lhs - ((rhs as u8) as f64)).into(),
                Value::Number(rhs) => (lhs - rhs).into(),
                Value::Time(_) => bail!("cannot subtract time from a number"),
                Value::Null => Value::Number(lhs),
            },
            Value::Null => match rhs {
                Value::String(_) => bail!("cannot subtract string from null"),
                Value::Boolean(_) => bail!("cannot subtract boolean from null"),
                Value::Number(_) => bail!("cannot subtract number from null"),
                Value::Time(_) => bail!("cannot subtract time from null"),
                Value::Null => Value::Null,
            },
            Value::Time(lhs) => match rhs {
                Value::String(_) => bail!("cannot subtract a string from time"),
                Value::Boolean(_) => bail!("cannot subtract a boolean from time"),
                Value::Number(_) => bail!("cannot subtract a number from time"),
                Value::Time(rhs) => Value::Time(
                    Time::from_hms_opt(
                        lhs.hour() - rhs.hour(),
                        lhs.minute() - rhs.minute(),
                        lhs.second() - rhs.second(),
                    )
                    .context("failed to add times together")?,
                ),
                Value::Null => bail!("cannot subtract null from a time"),
            },
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
                Value::Time(_) => bail!("cannot multiply a string and a time"),
                Value::Null => bail!("cannot multiply a string and null"),
            },
            Value::Boolean(lhs) => match rhs {
                Value::String(_) => bail!("cannot multiply a boolean and a string"),
                Value::Boolean(rhs) => (((lhs as u8) as f64) * ((rhs as u8) as f64)).into(),
                Value::Number(rhs) => ((lhs as u8) as f64 * rhs).into(),
                Value::Time(_) => bail!("cannot multiply a boolean and a time"),
                Value::Null => bail!("cannot multiply a boolean and null"),
            },
            Value::Number(lhs) => match rhs {
                Value::String(_) => bail!("cannot multiply a number and a string"),
                Value::Boolean(rhs) => (lhs * ((rhs as u8) as f64)).into(),
                Value::Number(rhs) => (lhs * rhs).into(),
                Value::Time(rhs) => Value::Time(
                    Time::from_hms_opt(
                        rhs.hour() * lhs as u32,
                        rhs.minute() * lhs as u32,
                        rhs.second() * lhs as u32,
                    )
                    .context("failed to add times together")?,
                ),
                Value::Null => bail!("cannot multiply a number and null"),
            },
            Value::Null => match rhs {
                Value::String(_) => bail!("cannot multiply a string and null"),
                Value::Boolean(_) => bail!("cannot multiply a boolean and null"),
                Value::Number(_) => bail!("cannot multiply a number and null"),
                Value::Time(_) => bail!("cannot multiply null and a time"),
                Value::Null => Value::Null,
            },
            Value::Time(lhs) => match rhs {
                Value::String(_) => bail!("cannot multiply a time and string"),
                Value::Boolean(_) => bail!("cannot multiply a time and a boolean"),
                Value::Number(rhs) => Value::Time(
                    Time::from_hms_opt(
                        lhs.hour() * rhs as u32,
                        lhs.minute() * rhs as u32,
                        lhs.second() * rhs as u32,
                    )
                    .context("failed to add times together")?,
                ),
                Value::Time(_) => bail!("cannot multiply two times"),
                Value::Null => bail!("cannot subtract null from a time"),
            },
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
                Value::Time(_) => bail!("cannot divide a string by a time"),
                Value::Null => bail!("cannot divide a string and null"),
            },
            Value::Boolean(lhs) => match rhs {
                Value::String(_) => bail!("cannot divide a boolean and a string"),
                Value::Boolean(rhs) => (((lhs as u8) as f64) / ((rhs as u8) as f64)).into(),
                Value::Number(rhs) => ((lhs as u8) as f64 / rhs).into(),
                Value::Time(_) => bail!("cannot divide a boolean by a time"),
                Value::Null => bail!("cannot divide a boolean and null"),
            },
            Value::Number(lhs) => match rhs {
                Value::String(_) => bail!("cannot divide a number and a string"),
                Value::Boolean(rhs) => (lhs / ((rhs as u8) as f64)).into(),
                Value::Number(rhs) => (lhs / rhs).into(),
                Value::Time(_) => bail!("cannot divide a number by a time"),
                Value::Null => bail!("cannot divide a number and null"),
            },
            Value::Null => match rhs {
                Value::String(_) => bail!("cannot divide a string and null"),
                Value::Boolean(_) => bail!("cannot divide a boolean and null"),
                Value::Number(_) => bail!("cannot divide a number and null"),
                Value::Time(_) => bail!("cannot divide null by a time"),
                Value::Null => Value::Null,
            },
            Value::Time(lhs) => match rhs {
                Value::String(_) => bail!("cannot divide a time by a string"),
                Value::Boolean(_) => bail!("cannot divide a time by a boolean"),
                Value::Number(rhs) => Value::Time(
                    Time::from_hms_opt(
                        lhs.hour() / rhs as u32,
                        lhs.minute() / rhs as u32,
                        lhs.second() / rhs as u32,
                    )
                    .context("failed to add times together")?,
                ),
                Value::Time(_) => bail!("cannot divide two times"),
                Value::Null => bail!("cannot divide a time by null"),
            },
        })
    }
}
