use std::error::Error as StdError;
use std::fmt;
use std::io;
use std::str::Utf8Error;

/// Errors that can occur in the Ultralight Rust bindings.
#[derive(Debug)]
pub enum Error {
    /// A null reference was encountered where a valid reference was expected.
    NullReference(&'static str),
    
    /// An invalid operation was attempted.
    InvalidOperation(&'static str),
    
    /// A JavaScript error occurred during evaluation.
    JavaScriptError(String),
    
    /// Invalid UTF-8 was encountered in a string.
    InvalidUtf8(Utf8Error),
    
    /// An I/O error occurred.
    IoError(io::Error),
    
    /// A function argument was invalid.
    InvalidArgument(&'static str),
    
    /// Attempted to perform an operation on a resource that has been destroyed.
    ResourceDestroyed(&'static str),
    
    /// An error occurred in the Ultralight API.
    UltralightError(&'static str),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::NullReference(desc) => write!(f, "Null reference: {}", desc),
            Error::InvalidOperation(desc) => write!(f, "Invalid operation: {}", desc),
            Error::JavaScriptError(desc) => write!(f, "JavaScript error: {}", desc),
            Error::InvalidUtf8(err) => write!(f, "Invalid UTF-8: {}", err),
            Error::IoError(err) => write!(f, "I/O error: {}", err),
            Error::InvalidArgument(desc) => write!(f, "Invalid argument: {}", desc),
            Error::ResourceDestroyed(desc) => write!(f, "Resource destroyed: {}", desc),
            Error::UltralightError(desc) => write!(f, "Ultralight error: {}", desc),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Error::InvalidUtf8(err) => Some(err),
            Error::IoError(err) => Some(err),
            _ => None,
        }
    }
}

impl From<Utf8Error> for Error {
    fn from(err: Utf8Error) -> Self {
        Error::InvalidUtf8(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IoError(err)
    }
}