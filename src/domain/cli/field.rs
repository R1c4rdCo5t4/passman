use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Field {
    Username,
    Password,
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Field::Username => write!(f, "username"),
            Field::Password => write!(f, "password"),
        }
    }
}

