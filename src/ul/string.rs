use std::ffi::{CStr, CString};
use std::fmt;
use std::ops::Deref;
use std::os::raw::{c_char, c_ushort};
use crate::ul::ffi::{ULString, ULChar16, ulCreateString, ulCreateStringUTF8, ulCreateStringUTF16, 
                ulCreateStringFromCopy, ulDestroyString, ulStringGetData, ulStringGetLength, 
                ulStringIsEmpty, ulStringAssignString, ulStringAssignCString};
use crate::ul::error::Error;

/// A safe wrapper around Ultralight's ULString type.
pub struct String {
    pub raw: ULString,
    pub owned: bool,
}

impl String {
    /// Create a new string from a raw ULString pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid ULString created by the Ultralight API.
    pub unsafe fn from_raw(raw: ULString, owned: bool) -> Self {
        Self { raw, owned }
    }
    
    /// Create a new empty string.
    pub fn empty() -> Self {
        Self::from_str("")
    }
    
    /// Create a new string from a Rust string slice.
    pub fn from_str(s: &str) -> Self {
        let c_str = CString::new(s).unwrap();
        unsafe {
            let raw = ulCreateString(c_str.as_ptr());
            Self { raw, owned: true }
        }
    }
    
    /// Create a new string from UTF-8 data.
    pub fn from_utf8(s: &[u8]) -> Self {
        unsafe {
            let raw = ulCreateStringUTF8(s.as_ptr() as *const c_char, s.len());
            Self { raw, owned: true }
        }
    }
    
    /// Create a new string from UTF-16 data.
    pub fn from_utf16(s: &[u16]) -> Self {
        unsafe {
            let raw = ulCreateStringUTF16(s.as_ptr() as *mut ULChar16, s.len());
            Self { raw, owned: true }
        }
    }
    
    /// Create a copy of another string.
    pub fn from_copy(other: &Self) -> Self {
        unsafe {
            let raw = ulCreateStringFromCopy(other.raw);
            Self { raw, owned: true }
        }
    }
    
    /// Get a reference to the raw ULString.
    pub fn raw(&self) -> ULString {
        self.raw
    }
    
    /// Get the UTF-8 data as a string slice.
    pub fn as_str(&self) -> Result<&str, Error> {
        if self.raw.is_null() {
            return Err(Error::NullReference("String is null"));
        }
        
        unsafe {
            let data = ulStringGetData(self.raw);
            if data.is_null() {
                return Err(Error::NullReference("String data is null"));
            }
            
            let c_str = CStr::from_ptr(data);
            c_str.to_str().map_err(|_| Error::InvalidOperation("Invalid UTF-8 in string"))
        }
    }
    
    /// Get the length of the string in bytes.
    pub fn len(&self) -> usize {
        if self.raw.is_null() {
            return 0;
        }
        
        unsafe { ulStringGetLength(self.raw) }
    }
    
    /// Check if the string is empty.
    pub fn is_empty(&self) -> bool {
        if self.raw.is_null() {
            return true;
        }
        
        unsafe { ulStringIsEmpty(self.raw) }
    }
    
    /// Assign the content of another string to this one.
    pub fn assign(&mut self, other: &Self) -> Result<(), Error> {
        if self.raw.is_null() || other.raw.is_null() {
            return Err(Error::NullReference("String is null"));
        }
        
        unsafe {
            ulStringAssignString(self.raw, other.raw);
        }
        
        Ok(())
    }
    
    /// Assign a Rust string slice to this string.
    pub fn assign_str(&mut self, s: &str) -> Result<(), Error> {
        if self.raw.is_null() {
            return Err(Error::NullReference("String is null"));
        }
        
        let c_str = CString::new(s).unwrap();
        unsafe {
            ulStringAssignCString(self.raw, c_str.as_ptr());
        }
        
        Ok(())
    }
}

impl Clone for String {
    fn clone(&self) -> Self {
        Self::from_copy(self)
    }
}

impl Drop for String {
    fn drop(&mut self) {
        if !self.raw.is_null() && self.owned {
            unsafe {
                ulDestroyString(self.raw);
            }
        }
    }
}

impl fmt::Debug for String {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.as_str() {
            Ok(s) => write!(f, "String({:?})", s),
            Err(_) => write!(f, "String(<invalid>)"),
        }
    }
}

impl fmt::Display for String {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.as_str() {
            Ok(s) => write!(f, "{}", s),
            Err(_) => write!(f, "<invalid string>"),
        }
    }
}

impl From<&str> for String {
    fn from(s: &str) -> Self {
        Self::from_str(s)
    }
}

impl From<std::string::String> for String {
    fn from(s: std::string::String) -> Self {
        Self::from_str(&s)
    }
}

impl Deref for String {
    type Target = str;
    
    fn deref(&self) -> &Self::Target {
        self.as_str().unwrap_or("")
    }
}