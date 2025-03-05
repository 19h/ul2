//! JavaScriptCore value implementation.
//!
//! This module provides a comprehensive abstraction over JSValueRef, implementing
//! the necessary memory management strategies and binding to the underlying
//! JavaScriptCore C API. The Value struct represents any JavaScript value 
//! (primitive or object), with methods for type checking, conversion, and creation.

use std::marker::PhantomData;
use std::ptr;
use std::os::raw::c_double;

use crate::javascript_core::context::Context;
use crate::javascript_core::error::{Error, Result};
use crate::javascript_core::ffi;
use crate::javascript_core::object::Object;
use crate::javascript_core::string::String;

/// Represents the type of a JavaScript value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValueType {
    /// The unique undefined value.
    Undefined,
    /// The unique null value.
    Null,
    /// A primitive boolean value, one of true or false.
    Boolean,
    /// A primitive number value.
    Number,
    /// A primitive string value.
    String,
    /// An object value (meaning that this Value is a JSObjectRef).
    Object,
    /// A primitive symbol value.
    Symbol,
}

impl ValueType {
    /// Converts a JSType to a ValueType.
    fn from_ffi(ty: ffi::JSType) -> Self {
        match ty {
            ffi::JSType::kJSTypeUndefined => ValueType::Undefined,
            ffi::JSType::kJSTypeNull => ValueType::Null,
            ffi::JSType::kJSTypeBoolean => ValueType::Boolean,
            ffi::JSType::kJSTypeNumber => ValueType::Number,
            ffi::JSType::kJSTypeString => ValueType::String,
            ffi::JSType::kJSTypeObject => ValueType::Object,
            ffi::JSType::kJSTypeSymbol => ValueType::Symbol,
        }
    }
}

/// A JavaScript value.
///
/// The Value struct encapsulates a JSValueRef, representing any JavaScript value
/// (primitive or object) within a context. It provides methods for checking the
/// type of a value, converting between different types, and creating new values.
pub struct Value<'a> {
    context: Context<'a>,
    raw: ffi::JSValueRef,
}

impl<'a> Value<'a> {
    /// Creates a Value from a raw JSValueRef.
    ///
    /// # Safety
    ///
    /// The provided JSValueRef must be a valid pointer to a JavaScript value,
    /// and must be associated with the given context. It is the caller's
    /// responsibility to ensure the JSValueRef remains valid for the lifetime
    /// of the Value.
    pub(crate) fn from_raw(context: &Context<'a>, raw: ffi::JSValueRef) -> Self {
        Value {
            context: context.clone(),
            raw,
        }
    }
    
    /// Returns the raw JSValueRef.
    pub(crate) fn as_raw(&self) -> ffi::JSValueRef {
        self.raw
    }
    
    /// Returns the context of this value.
    pub fn context(&self) -> &Context<'a> {
        &self.context
    }
    
    /// Gets the type of this JavaScript value.
    ///
    /// # Returns
    ///
    /// A ValueType that identifies the type of this value.
    pub fn get_type(&self) -> ValueType {
        let raw_type = unsafe { ffi::JSValueGetType(self.context.as_raw(), self.raw) };
        ValueType::from_ffi(raw_type)
    }
    
    /// Creates an undefined value in the given context.
    ///
    /// # Arguments
    ///
    /// * `context` - The context in which to create the value.
    ///
    /// # Returns
    ///
    /// A new undefined value.
    pub fn undefined(context: &Context<'a>) -> Self {
        let raw = unsafe { ffi::JSValueMakeUndefined(context.as_raw()) };
        Value::from_raw(context, raw)
    }
    
    /// Creates a null value in the given context.
    ///
    /// # Arguments
    ///
    /// * `context` - The context in which to create the value.
    ///
    /// # Returns
    ///
    /// A new null value.
    pub fn null(context: &Context<'a>) -> Self {
        let raw = unsafe { ffi::JSValueMakeNull(context.as_raw()) };
        Value::from_raw(context, raw)
    }
    
    /// Creates a boolean value in the given context.
    ///
    /// # Arguments
    ///
    /// * `context` - The context in which to create the value.
    /// * `value` - The boolean value to represent.
    ///
    /// # Returns
    ///
    /// A new boolean value.
    pub fn boolean(context: &Context<'a>, value: bool) -> Self {
        let raw = unsafe { ffi::JSValueMakeBoolean(context.as_raw(), value) };
        Value::from_raw(context, raw)
    }
    
    /// Creates a number value in the given context.
    ///
    /// # Arguments
    ///
    /// * `context` - The context in which to create the value.
    /// * `value` - The numeric value to represent.
    ///
    /// # Returns
    ///
    /// A new number value.
    pub fn number(context: &Context<'a>, value: f64) -> Self {
        let raw = unsafe { ffi::JSValueMakeNumber(context.as_raw(), value) };
        Value::from_raw(context, raw)
    }
    
    /// Creates a string value in the given context.
    ///
    /// # Arguments
    ///
    /// * `context` - The context in which to create the value.
    /// * `value` - The string value to represent.
    ///
    /// # Returns
    ///
    /// A new string value.
    pub fn string(context: &Context<'a>, value: &str) -> Self {
        let js_string = String::new(value);
        let raw = unsafe { ffi::JSValueMakeString(context.as_raw(), js_string.as_raw()) };
        Value::from_raw(context, raw)
    }
    
    /// Creates a string value from a JSString in the given context.
    ///
    /// # Arguments
    ///
    /// * `context` - The context in which to create the value.
    /// * `string` - The JSString to use.
    ///
    /// # Returns
    ///
    /// A new string value representing the given JSString.
    pub fn from_jsstring(context: &Context<'a>, string: &String) -> Self {
        let raw = unsafe { ffi::JSValueMakeString(context.as_raw(), string.as_raw()) };
        Value::from_raw(context, raw)
    }
    
    /// Creates a symbol value in the given context.
    ///
    /// # Arguments
    ///
    /// * `context` - The context in which to create the value.
    /// * `description` - Optional description for the symbol.
    ///
    /// # Returns
    ///
    /// A new symbol value.
    pub fn symbol(context: &Context<'a>, description: Option<&str>) -> Self {
        let desc_string = description.map(String::new);
        let raw = unsafe {
            ffi::JSValueMakeSymbol(
                context.as_raw(),
                desc_string.as_ref().map_or(ptr::null_mut(), |s| s.as_raw())
            )
        };
        Value::from_raw(context, raw)
    }
    
    /// Creates a value from a JavaScript exception.
    ///
    /// # Arguments
    ///
    /// * `context` - The context in which to create the value.
    /// * `error` - The error to convert to a JavaScript value.
    ///
    /// # Returns
    ///
    /// A new value representing the error.
    pub(crate) fn from_error(context: &Context<'a>, error: &Error) -> Self {
        error.to_js_error(context)
    }
    
    /// Checks if this value is undefined.
    ///
    /// # Returns
    ///
    /// `true` if this value is undefined, otherwise `false`.
    pub fn is_undefined(&self) -> bool {
        unsafe { ffi::JSValueIsUndefined(self.context.as_raw(), self.raw) }
    }
    
    /// Checks if this value is null.
    ///
    /// # Returns
    ///
    /// `true` if this value is null, otherwise `false`.
    pub fn is_null(&self) -> bool {
        unsafe { ffi::JSValueIsNull(self.context.as_raw(), self.raw) }
    }
    
    /// Checks if this value is a boolean.
    ///
    /// # Returns
    ///
    /// `true` if this value is a boolean, otherwise `false`.
    pub fn is_boolean(&self) -> bool {
        unsafe { ffi::JSValueIsBoolean(self.context.as_raw(), self.raw) }
    }
    
    /// Checks if this value is a number.
    ///
    /// # Returns
    ///
    /// `true` if this value is a number, otherwise `false`.
    pub fn is_number(&self) -> bool {
        unsafe { ffi::JSValueIsNumber(self.context.as_raw(), self.raw) }
    }
    
    /// Checks if this value is a string.
    ///
    /// # Returns
    ///
    /// `true` if this value is a string, otherwise `false`.
    pub fn is_string(&self) -> bool {
        unsafe { ffi::JSValueIsString(self.context.as_raw(), self.raw) }
    }
    
    /// Checks if this value is an object.
    ///
    /// # Returns
    ///
    /// `true` if this value is an object, otherwise `false`.
    pub fn is_object(&self) -> bool {
        unsafe { ffi::JSValueIsObject(self.context.as_raw(), self.raw) }
    }
    
    /// Checks if this value is a symbol.
    ///
    /// # Returns
    ///
    /// `true` if this value is a symbol, otherwise `false`.
    pub fn is_symbol(&self) -> bool {
        unsafe { ffi::JSValueIsSymbol(self.context.as_raw(), self.raw) }
    }
    
    /// Checks if this value is an array.
    ///
    /// # Returns
    ///
    /// `true` if this value is an array, otherwise `false`.
    pub fn is_array(&self) -> bool {
        unsafe { ffi::JSValueIsArray(self.context.as_raw(), self.raw) }
    }
    
    /// Checks if this value is a date.
    ///
    /// # Returns
    ///
    /// `true` if this value is a date, otherwise `false`.
    pub fn is_date(&self) -> bool {
        unsafe { ffi::JSValueIsDate(self.context.as_raw(), self.raw) }
    }
    
    /// Converts this value to a boolean.
    ///
    /// # Returns
    ///
    /// The boolean result of conversion.
    pub fn to_boolean(&self) -> bool {
        unsafe { ffi::JSValueToBoolean(self.context.as_raw(), self.raw) }
    }
    
    /// Converts this value to a number.
    ///
    /// # Returns
    ///
    /// A Result containing the numeric result of conversion, or an error if conversion fails.
    pub fn to_number(&self) -> Result<f64> {
        unsafe {
            let mut exception = ptr::null();
            let result = ffi::JSValueToNumber(self.context.as_raw(), self.raw, &mut exception);
            
            if !exception.is_null() {
                return Err(Error::from_js_exception(self.context.as_raw(), exception));
            }
            
            Ok(result)
        }
    }
    
    /// Converts this value to a string.
    ///
    /// # Returns
    ///
    /// A Result containing the string result of conversion, or an error if conversion fails.
    pub fn to_string(&self) -> Result<String> {
        unsafe {
            let mut exception = ptr::null();
            let result = ffi::JSValueToStringCopy(self.context.as_raw(), self.raw, &mut exception);
            
            if !exception.is_null() {
                return Err(Error::from_js_exception(self.context.as_raw(), exception));
            }
            
            if result.is_null() {
                return Err(Error::JSError("Failed to convert value to string".to_string()));
            }
            
            Ok(String::from_raw(result))
        }
    }
    
    /// Converts this value to an object.
    ///
    /// # Returns
    ///
    /// A Result containing the object result of conversion, or an error if conversion fails.
    pub fn to_object(&self) -> Result<Object<'a>> {
        unsafe {
            let mut exception = ptr::null();
            let result = ffi::JSValueToObject(self.context.as_raw(), self.raw, &mut exception);
            
            if !exception.is_null() {
                return Err(Error::from_js_exception(self.context.as_raw(), exception));
            }
            
            if result.is_null() {
                return Err(Error::JSError("Failed to convert value to object".to_string()));
            }
            
            Ok(Object::from_raw(self.context.clone(), result))
        }
    }
    
    /// Creates a JavaScript value from a JSON string.
    ///
    /// # Arguments
    ///
    /// * `context` - The context in which to create the value.
    /// * `json` - A string containing valid JSON.
    ///
    /// # Returns
    ///
    /// A Result containing the parsed value, or an error if parsing fails.
    pub fn from_json(context: &Context<'a>, json: &str) -> Result<Self> {
        let js_string = String::new(json);
        let raw = unsafe {
            ffi::JSValueMakeFromJSONString(context.as_raw(), js_string.as_raw())
        };
        
        if raw.is_null() {
            return Err(Error::JSError("Invalid JSON".to_string()));
        }
        
        Ok(Value::from_raw(context, raw))
    }
    
    /// Converts this value to a JSON string.
    ///
    /// # Arguments
    ///
    /// * `indent` - The number of spaces to indent when nesting. If 0, the resulting JSON will not contain newlines.
    ///
    /// # Returns
    ///
    /// A Result containing the JSON string, or an error if conversion fails.
    pub fn to_json(&self, indent: u32) -> Result<String> {
        unsafe {
            let mut exception = ptr::null();
            let result = ffi::JSValueCreateJSONString(
                self.context.as_raw(),
                self.raw,
                indent,
                &mut exception
            );
            
            if !exception.is_null() {
                return Err(Error::from_js_exception(self.context.as_raw(), exception));
            }
            
            if result.is_null() {
                return Err(Error::JSError("Failed to convert value to JSON".to_string()));
            }
            
            Ok(String::from_raw(result))
        }
    }
    
    /// Compares this value with another for equality using the JavaScript == operator.
    ///
    /// # Arguments
    ///
    /// * `other` - The value to compare with.
    ///
    /// # Returns
    ///
    /// A Result containing `true` if the values are equal, `false` otherwise, or an error if comparison fails.
    pub fn equals(&self, other: &Value<'a>) -> Result<bool> {
        unsafe {
            let mut exception = ptr::null();
            let result = ffi::JSValueIsEqual(
                self.context.as_raw(),
                self.raw,
                other.raw,
                &mut exception
            );
            
            if !exception.is_null() {
                return Err(Error::from_js_exception(self.context.as_raw(), exception));
            }
            
            Ok(result)
        }
    }
    
    /// Compares this value with another for strict equality using the JavaScript === operator.
    ///
    /// # Arguments
    ///
    /// * `other` - The value to compare with.
    ///
    /// # Returns
    ///
    /// `true` if the values are strictly equal, `false` otherwise.
    pub fn strict_equals(&self, other: &Value<'a>) -> bool {
        unsafe {
            ffi::JSValueIsStrictEqual(
                self.context.as_raw(),
                self.raw,
                other.raw
            )
        }
    }
    
    /// Checks if this value is an instance of a constructor using the JavaScript instanceof operator.
    ///
    /// # Arguments
    ///
    /// * `constructor` - The constructor to check against.
    ///
    /// # Returns
    ///
    /// A Result containing `true` if this value is an instance of the constructor, `false` otherwise, or an error if the check fails.
    pub fn is_instance_of(&self, constructor: &Object<'a>) -> Result<bool> {
        unsafe {
            let mut exception = ptr::null();
            let result = ffi::JSValueIsInstanceOfConstructor(
                self.context.as_raw(),
                self.raw,
                constructor.as_raw(),
                &mut exception
            );
            
            if !exception.is_null() {
                return Err(Error::from_js_exception(self.context.as_raw(), exception));
            }
            
            Ok(result)
        }
    }
    
    /// Protects this value from garbage collection.
    ///
    /// A value may be protected multiple times and must be unprotected an equal number of times
    /// before becoming eligible for garbage collection.
    pub fn protect(&self) {
        unsafe {
            ffi::JSValueProtect(self.context.as_raw(), self.raw);
        }
    }
    
    /// Unprotects this value from garbage collection.
    ///
    /// A value may be protected multiple times and must be unprotected an equal number of times
    /// before becoming eligible for garbage collection.
    pub fn unprotect(&self) {
        unsafe {
            ffi::JSValueUnprotect(self.context.as_raw(), self.raw);
        }
    }
    
    /// Determines if this value is of a specific object class.
    ///
    /// # Arguments
    ///
    /// * `class` - The JSClassRef to check against.
    ///
    /// # Returns
    ///
    /// `true` if this value is an object of the specified class, `false` otherwise.
    pub fn is_of_class(&self, class: ffi::JSClassRef) -> bool {
        unsafe {
            ffi::JSValueIsObjectOfClass(self.context.as_raw(), self.raw, class)
        }
    }
    
    /// Gets the typed array type of this value, if it is a typed array.
    ///
    /// # Returns
    ///
    /// A Result containing the typed array type, or an error if this value is not a typed array.
    pub fn get_typed_array_type(&self) -> Result<Option<ffi::JSTypedArrayType>> {
        unsafe {
            let mut exception = ptr::null();
            let result = ffi::JSValueGetTypedArrayType(
                self.context.as_raw(),
                self.raw,
                &mut exception
            );
            
            if !exception.is_null() {
                return Err(Error::from_js_exception(self.context.as_raw(), exception));
            }
            
            match result {
                ffi::JSTypedArrayType::kJSTypedArrayTypeNone => Ok(None),
                _ => Ok(Some(result)),
            }
        }
    }
}

impl<'a> Clone for Value<'a> {
    fn clone(&self) -> Self {
        Value {
            context: self.context.clone(),
            raw: self.raw,
        }
    }
}

impl<'a> PartialEq for Value<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.strict_equals(other)
    }
}

impl<'a> From<Object<'a>> for Value<'a> {
    fn from(obj: Object<'a>) -> Self {
        obj.to_value()
    }
}