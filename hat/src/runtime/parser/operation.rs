use std::fmt::Display;

#[derive(Debug)]
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Equals,
    NotEquals,
    And,
    Or,
    Greater,
    GreaterOrEquals,
    Lesser,
    LesserOrEquals,
}

impl Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Add => "+",
                Self::Subtract => "-",
                Self::Multiply => "*",
                Self::Divide => "/",
                Self::Equals => "==",
                Self::NotEquals => "!=",
                Self::And => "and",
                Self::Or => "or",
                Self::Greater => ">",
                Self::GreaterOrEquals => ">=",
                Self::Lesser => "<",
                Self::LesserOrEquals => "<=",
            }
        )
    }
}
