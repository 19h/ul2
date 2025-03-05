//! JavaScriptCore exception and error handling.
//!
//! This module provides a systematic approach to handling JavaScript exceptions
//! and errors within the Rust bindings. It defines a dedicated error type hierarchy
//! and methods for converting JavaScriptCore exceptions into Rust errors, ensuring
//! proper propagation of error information throughout the binding interface.

use std::error::Error as StdError;
use std::fmt;
use std::ptr;

use crate::javascript_core::ffi;
use crate::javascript_core::context::Context;
use crate::javascript_core::string::String;
use crate::javascript_core::value::Value;

/// Result type alias for operations that may produce a JavaScript exception.
///
/// This type alias simplifies the return type signatures throughout the codebase
/// for functions that may result in JavaScript exceptions.
pub type Result<T> = std::result::Result<T, Error>;

/// Comprehensive error type for JavaScriptCore operations.
///
/// This enum encompasses all potential error conditions that may arise during
/// interaction with the JavaScriptCore API, including JavaScript exceptions,
/// invalid parameters, and operational failures.
#[derive(Debug)]
pub enum Error {
    /// A JavaScript exception was thrown during execution.
    JSException {
        /// The message describing the exception.
        message: String,
        /// The source URL where the exception occurred, if available.
        source_url: Option<String>,
        /// The line number where the exception occurred, if available.
        line: Option<u32>,
        /// The column number where the exception occurred, if available.
        column: Option<u32>,
        /// The stack trace for the exception, if available.
        stack_trace: Option<String>,
    },
    
    /// A general JavaScript error that doesn't have specific exception information.
    JSError(String),
    
    /// An error indicating that a parameter was invalid.
    InvalidParameter(&'static str),
    
    /// An error indicating that an incorrect type was used.
    InvalidType(String),
    
    /// An error during conversion between Rust and JavaScript types.
    ConversionError(String),
    
    /// An error due to attempting to access null or undefined values.
    NullAccess(&'static str),
    
    /// An error due to an operation not being supported.
    UnsupportedOperation(&'static str),
}

impl Error {
    /// Creates an Error from a JavaScript exception value.
    ///
    /// This method extracts information from a JavaScript exception value to
    /// create a detailed Error::JSException. It attempts to extract as much
    /// diagnostic information as possible, including the message, source location,
    /// and stack trace.
    ///
    /// # Arguments
    ///
    /// * `ctx` - The JavaScript context in which the exception occurred.
    /// * `exception` - The raw JSValueRef representing the exception.
    ///
    /// # Returns
    ///
    /// An Error representing the JavaScript exception.
    pub(crate) fn from_js_exception(ctx: ffi::JSContextRef, exception: ffi::JSValueRef) -> Self {
        unsafe {
            // Extract the exception message
            let context = Context::from_raw(ctx);
            let exception_value = Value::from_raw(&context, exception);
            
            // Try to get the exception message
            let message = match exception_value.to_string() {
                Ok(msg) => msg,
                Err(_) => String::new("Unknown JavaScript exception"),
            };
            
            // Try to extract more information from the exception object
            let mut source_url = None;
            let mut line = None;
            let mut column = None;
            let mut stack_trace = None;
            
            if exception_value.is_object() {
                if let Ok(exception_obj) = exception_value.to_object() {
                    // Try to get source URL
                    if let Ok(url_value) = exception_obj.get_property("sourceURL") {
                        if let Ok(url) = url_value.to_string() {
                            source_url = Some(url);
                        }
                    }
                    
                    // Try to get line number
                    if let Ok(line_value) = exception_obj.get_property("line") {
                        if let Ok(line_num) = line_value.to_number() {
                            line = Some(line_num as u32);
                        }
                    }
                    
                    // Try to get column number
                    if let Ok(column_value) = exception_obj.get_property("column") {
                        if let Ok(column_num) = column_value.to_number() {
                            column = Some(column_num as u32);
                        }
                    }
                    
                    // Try to get stack trace
                    if let Ok(stack_value) = exception_obj.get_property("stack") {
                        if let Ok(stack) = stack_value.to_string() {
                            stack_trace = Some(stack);
                        }
                    }
                }
            }
            
            Error::JSException {
                message,
                source_url,
                line,
                column,
                stack_trace,
            }
        }
    }
    
    /// Creates a Value representation of this error.
    ///
    /// This method converts the Error into a JavaScript Error object that can
    /// be returned to JavaScript code.
    ///
    /// # Arguments
    ///
    /// * `context` - The JavaScript context in which to create the error.
    ///
    /// # Returns
    ///
    /// A JavaScript Error object representing this error.
    pub(crate) fn to_js_error<'a>(&self, context: &Context<'a>) -> Value<'a> {
        match self {
            Error::JSException { message, .. } => {
                // Create a new Error object with the message
                let error_constructor = context.global_object().get_property("Error")
                    .ok()
                    .and_then(|v| v.to_object().ok());
                
                if let Some(constructor) = error_constructor {
                    let args = [Value::string(context, &message)];
                    constructor.call_as_constructor(&args)
                        .map(|obj| obj.to_value())
                        .unwrap_or_else(|_| Value::string(context, &message))
                } else {
                    Value::string(context, &message)
                }
            },
            Error::JSError(message) => {
                // Create a new Error object with the message
                let error_constructor = context.global_object().get_property("Error")
                    .ok()
                    .and_then(|v| v.to_object().ok());
                
                if let Some(constructor) = error_constructor {
                    let args = [Value::string(context, message)];
                    constructor.call_as_constructor(&args)
                        .map(|obj| obj.to_value())
                        .unwrap_or_else(|_| Value::string(context, message))
                } else {
                    Value::string(context, message)
                }
            },
            Error::InvalidParameter(message) => Value::string(context, &format!("Invalid parameter: {}", message)),
            Error::InvalidType(message) => Value::string(context, &format!("Invalid type: {}", message)),
            Error::ConversionError(message) => Value::string(context, &format!("Conversion error: {}", message)),
            Error::NullAccess(message) => Value::string(context, &format!("Null access: {}", message)),
            Error::UnsupportedOperation(message) => Value::string(context, &format!("Unsupported operation: {}", message)),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::JSException { message, source_url, line, column, stack_trace } => {
                write!(f, "JavaScript exception: {}", message)?;
                
                if let Some(url) = source_url {
                    write!(f, " at {}", url)?;
                    
                    if let Some(line_num) = line {
                        write!(f, ":{}", line_num)?;
                        
                        if let Some(column_num) = column {
                            write!(f, ":{}", column_num)?;
                        }
                    }
                }
                
                if let Some(stack) = stack_trace {
                    write!(f, "\nStack trace:\n{}", stack)?;
                }
                
                Ok(())
            },
            Error::JSError(message) => write!(f, "JavaScript error: {}", message),
            Error::InvalidParameter(message) => write!(f, "Invalid parameter: {}", message),
            Error::InvalidType(message) => write!(f, "Invalid type: {}", message),
            Error::ConversionError(message) => write!(f, "Conversion error: {}", message),
            Error::NullAccess(message) => write!(f, "Null access: {}", message),
            Error::UnsupportedOperation(message) => write!(f, "Unsupported operation: {}", message),
        }
    }
}

impl StdError for Error {}