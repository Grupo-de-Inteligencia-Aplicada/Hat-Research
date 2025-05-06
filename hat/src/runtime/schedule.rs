use super::{parser::expression::Expression, value::time::Time};

#[derive(Debug)]
pub struct Schedule {
    pub name: String,
    pub interval: ScheduleInterval,
    pub conditions: Vec<Expression>,
    pub actions: Vec<Expression>,
}

#[derive(Debug)]
pub enum ScheduleInterval {
    Cron(String),
    Time { weekday: Option<Weekday>, at: Time },
}

#[derive(Debug)]
pub enum Weekday {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl TryFrom<&str> for Weekday {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "monday" => Ok(Self::Monday),
            "tuesday" => Ok(Self::Tuesday),
            "wednesday" => Ok(Self::Wednesday),
            "thursday" => Ok(Self::Thursday),
            "friday" => Ok(Self::Friday),
            "saturday" => Ok(Self::Saturday),
            "sunday" => Ok(Self::Sunday),
            _ => Err(()),
        }
    }
}
