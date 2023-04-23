use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum Error {
    InvalidCharacterError(char),
    InvalidCommandError(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Error::InvalidCharacterError(d) => write!(f, "invalid character `{d}`"),
            Error::InvalidCommandError(d) => write!(f, "invalid command `{d}`"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Error::InvalidCharacterError(_) => None,
            Error::InvalidCommandError(_) => None,
        }
    }
}

impl Error {
    pub fn kind(&self) -> String {
        String::from("syntax error")
    }
}
