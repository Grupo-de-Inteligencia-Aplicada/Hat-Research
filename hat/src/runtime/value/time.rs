use std::{
    fmt::Display,
    ops::{Deref, DerefMut},
};

use anyhow::bail;
use chrono::{DateTime, Local, NaiveTime, SubsecRound, Utc};
use serde::{Deserialize, Serialize};

use crate::runtime::parser::parse_time;

use super::Value;

#[derive(Debug, Clone, PartialOrd, Serialize, Deserialize)]
pub struct Time(NaiveTime);

/// Represents a time of the day *IN LOCAL TIMEZONE*
impl Time {
    pub fn now() -> Self {
        Self(Local::now().time())
    }
    pub fn from_hms_opt(hour: u32, min: u32, sec: u32) -> Option<Self> {
        let inner = NaiveTime::from_hms_opt(hour, min, sec)?;
        Some(Self(inner))
    }
}

/// Time comparisons in Hat will allways round values up to seconds
impl PartialEq for Time {
    fn eq(&self, other: &Self) -> bool {
        self.0.round_subsecs(0) == other.0.round_subsecs(0)
    }
}

impl From<DateTime<Local>> for Time {
    fn from(value: DateTime<Local>) -> Self {
        Self(value.time())
    }
}

impl From<DateTime<Utc>> for Time {
    fn from(value: DateTime<Utc>) -> Self {
        let local: DateTime<Local> = value.into();
        Self(local.time())
    }
}

impl Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Default for Time {
    fn default() -> Self {
        Self::now()
    }
}

impl DerefMut for Time {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for Time {
    type Target = NaiveTime;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn coerce_to_time(arg: Option<&Value>) -> anyhow::Result<Time> {
    match arg {
        Some(arg) => {
            if let Value::String(s) = arg {
                parse_time(&s)
            } else {
                bail!("time function only accepts strings");
            }
        }
        None => Ok(Time::now()),
    }
}
