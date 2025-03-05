use std::error::Error as StdError;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    NullReference(&'static str),
    InvalidOperation(&'static str),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::NullReference(desc) => write!(f, "Null reference: {}", desc),
            Error::InvalidOperation(desc) => write!(f, "Invalid operation: {}", desc),
        }
    }
}

impl StdError for Error {}
