//! JavaScriptCore bindings for Rust
//! 
//! This module provides safe, idiomatic Rust bindings to the JavaScriptCore C API.

// Re-export the main components for a clean public API
pub use context::{Context, GlobalContext};
pub use value::{Value, ValueType};
pub use object::{Object, Class, ClassDefinition, PropertyAttributes, ClassAttributes};
pub use string::String;
pub use typed_array::{TypedArray, TypedArrayType};
pub use exception::Exception;

pub mod ffi;
mod context;
mod value;
mod object;
mod string;
mod typed_array;
mod exception;
