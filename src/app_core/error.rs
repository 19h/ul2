use std::error::Error as StdError;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    NullReference(&'static str),
    InvalidOperation(&'static str),
    CreationFailed(&'static str),
    InvalidArgument(&'static str),
    ResourceNotFound(&'static str),
    ResourceAllocationFailed(&'static str),
    CallbackRegistrationFailed(&'static str),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::NullReference(desc) => write!(f, "Null reference: {}", desc),
            Error::InvalidOperation(desc) => write!(f, "Invalid operation: {}", desc),
            Error::CreationFailed(desc) => write!(f, "Resource creation failed: {}", desc),
            Error::InvalidArgument(desc) => write!(f, "Invalid argument: {}", desc),
            Error::ResourceNotFound(desc) => write!(f, "Resource not found: {}", desc),
            Error::ResourceAllocationFailed(desc) => write!(f, "Resource allocation failed: {}", desc),
            Error::CallbackRegistrationFailed(desc) => write!(f, "Callback registration failed: {}", desc),
        }
    }
}

impl StdError for Error {}