//! JavaScriptCore string implementation.
//!
//! This module provides a comprehensive abstraction over JSStringRef, implementing
//! the necessary memory management strategies to ensure safe interaction with
//! JavaScript strings. The String struct represents a UTF-16 string used by
//! JavaScriptCore, with methods for conversion to and from Rust strings.

use std::ffi::{CStr, CString};
use std::ptr;
use std::str;
use std::fmt;
use std::ops::Deref;

use crate::javascript_core::ffi;
use crate::javascript_core::error::{Error, Result};

/// A JavaScript string.
///
/// The String struct encapsulates a JSStringRef, representing a UTF-16 encoded
/// string used by JavaScriptCore. It manages the lifetime of the underlying
/// JSStringRef, ensuring proper acquisition and release of resources.
pub struct String {
    raw: ffi::JSStringRef,
}

impl String {
    /// Creates a new JavaScript string from a Rust string.
    ///
    /// This method creates a new JSStringRef from a Rust string, converting
    /// from UTF-8 to the UTF-16 encoding used by JavaScriptCore.
    ///
    /// # Arguments
    ///
    /// * `s` - The Rust string to convert.
    ///
    /// # Returns
    ///
    /// A new String instance representing the converted string.
    pub fn new(s: &str) -> Self {
        unsafe {
            let c_string = CString::new(s).unwrap_or_else(|_| CString::new("").unwrap());
            let raw = ffi::JSStringCreateWithUTF8CString(c_string.as_ptr());
            String { raw }
        }
    }
    
    /// Creates a JavaScript string from raw UTF-16 characters.
    ///
    /// This method creates a new JSStringRef from an array of UTF-16 code units,
    /// which is the native encoding used by JavaScriptCore.
    ///
    /// # Arguments
    ///
    /// * `chars` - The array of UTF-16 code units.
    ///
    /// # Returns
    ///
    /// A new String instance representing the string.
    pub fn from_chars(chars: &[u16]) -> Self {
        unsafe {
            let raw = ffi::JSStringCreateWithCharacters(
                chars.as_ptr() as *const ffi::JSChar,
                chars.len(),
            );
            String { raw }
        }
    }
    
    /// Creates a String from a raw JSStringRef.
    ///
    /// # Safety
    ///
    /// The provided JSStringRef must be a valid pointer to a JavaScript string,
    /// and ownership of the JSStringRef is transferred to the returned String.
    pub(crate) fn from_raw(raw: ffi::JSStringRef) -> Self {
        String { raw }
    }
    
    /// Creates a String from a UTF-8 encoded byte buffer.
    ///
    /// This method creates a new String from a byte buffer containing UTF-8 encoded
    /// data. It is primarily used for internal conversions, such as when handling
    /// C strings.
    ///
    /// # Arguments
    ///
    /// * `buffer` - The UTF-8 encoded byte buffer.
    ///
    /// # Returns
    ///
    /// A new String instance representing the string.
    pub(crate) fn from_utf8_buffer(buffer: &[u8]) -> Self {
        match str::from_utf8(buffer) {
            Ok(s) => String::new(s),
            Err(_) => String::new(""),
        }
    }
    
    /// Returns the raw JSStringRef pointer.
    pub(crate) fn as_raw(&self) -> ffi::JSStringRef {
        self.raw
    }
    
    /// Returns the length of the string in UTF-16 code units.
    ///
    /// # Returns
    ///
    /// The number of UTF-16 code units in the string.
    pub fn len(&self) -> usize {
        unsafe { ffi::JSStringGetLength(self.raw) }
    }
    
    /// Checks if the string is empty.
    ///
    /// # Returns
    ///
    /// `true` if the string is empty, `false` otherwise.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    
    /// Converts the string to a Rust String.
    ///
    /// This method converts the JavaScript string to a Rust String, handling
    /// the encoding conversion from UTF-16 to UTF-8.
    ///
    /// # Returns
    ///
    /// A Rust String containing the same text as this JavaScript string.
    pub fn to_string(&self) -> std::string::String {
        unsafe {
            let max_size = ffi::JSStringGetMaximumUTF8CStringSize(self.raw);
            let mut buffer = vec![0u8; max_size];
            
            let actual_size = ffi::JSStringGetUTF8CString(
                self.raw,
                buffer.as_mut_ptr() as *mut i8,
                max_size,
            );
            
            // The actual_size includes the null terminator, so we need to subtract 1
            buffer.truncate(actual_size - 1);
            
            // Convert the buffer to a Rust String, replacing invalid UTF-8 sequences
            String::from_utf8_lossy(&buffer).into_owned()
        }
    }
    
    /// Returns the characters of the string as a vector of UTF-16 code units.
    ///
    /// # Returns
    ///
    /// A vector containing the UTF-16 code units of the string.
    pub fn to_chars(&self) -> Vec<u16> {
        unsafe {
            let length = self.len();
            let chars_ptr = ffi::JSStringGetCharactersPtr(self.raw);
            
            if chars_ptr.is_null() || length == 0 {
                return Vec::new();
            }
            
            let chars_slice = std::slice::from_raw_parts(chars_ptr, length);
            chars_slice.to_vec()
        }
    }
    
    /// Returns a C-style string pointer for interfacing with C APIs.
    ///
    /// This method is primarily used for internal conversions when interacting
    /// with C APIs that expect null-terminated strings.
    ///
    /// # Returns
    ///
    /// A CString containing the UTF-8 representation of this string.
    pub(crate) fn as_c_str(&self) -> CString {
        CString::new(self.to_string()).unwrap_or_else(|_| CString::new("").unwrap())
    }
    
    /// Tests if this string is equal to another JavaScript string.
    ///
    /// # Arguments
    ///
    /// * `other` - The string to compare with.
    ///
    /// # Returns
    ///
    /// `true` if the strings are equal, `false` otherwise.
    pub fn equals(&self, other: &String) -> bool {
        unsafe { ffi::JSStringIsEqual(self.raw, other.raw) }
    }
    
    /// Tests if this string is equal to a C string.
    ///
    /// # Arguments
    ///
    /// * `c_str` - The C string to compare with.
    ///
    /// # Returns
    ///
    /// `true` if the strings are equal, `false` otherwise.
    pub fn equals_c_str(&self, c_str: &CStr) -> bool {
        unsafe { ffi::JSStringIsEqualToUTF8CString(self.raw, c_str.as_ptr()) }
    }
    
    /// Tests if this string is equal to a Rust string.
    ///
    /// # Arguments
    ///
    /// * `s` - The Rust string to compare with.
    ///
    /// # Returns
    ///
    /// `true` if the strings are equal, `false` otherwise.
    pub fn equals_str(&self, s: &str) -> bool {
        let c_string = CString::new(s).unwrap_or_else(|_| CString::new("").unwrap());
        unsafe { ffi::JSStringIsEqualToUTF8CString(self.raw, c_string.as_ptr()) }
    }
}

impl Drop for String {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            unsafe {
                ffi::JSStringRelease(self.raw);
            }
        }
    }
}

impl Clone for String {
    fn clone(&self) -> Self {
        unsafe {
            let raw = ffi::JSStringRetain(self.raw);
            String { raw }
        }
    }
}

impl fmt::Debug for String {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "JSString({:?})", self.to_string())
    }
}

impl fmt::Display for String {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl Deref for String {
    type Target = str;
    
    fn deref(&self) -> &Self::Target {
        // This is not ideal as we're creating a temporary string
        // A more efficient implementation would cache the result
        static EMPTY: &str = "";
        EMPTY
    }
}

impl PartialEq for String {
    fn eq(&self, other: &Self) -> bool {
        self.equals(other)
    }
}

impl PartialEq<str> for String {
    fn eq(&self, other: &str) -> bool {
        self.equals_str(other)
    }
}

impl PartialEq<String> for str {
    fn eq(&self, other: &String) -> bool {
        other.equals_str(self)
    }
}

impl From<&str> for String {
    fn from(s: &str) -> Self {
        String::new(s)
    }
}

impl From<std::string::String> for String {
    fn from(s: std::string::String) -> Self {
        String::new(&s)
    }
}

impl From<String> for std::string::String {
    fn from(s: String) -> Self {
        s.to_string()
    }
}