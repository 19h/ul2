use crate::ul::error::Error;
use crate::ul::ffi::{
    ULChar16, ULString, ulCreateString, ulCreateStringFromCopy, ulCreateStringUTF8,
    ulCreateStringUTF16, ulDestroyString, ulStringAssignCString, ulStringAssignString,
    ulStringGetData, ulStringGetLength, ulStringIsEmpty,
};
use std::ffi::{CStr, CString};
use std::fmt;
use std::ops::Deref;
use std::os::raw::c_char;

/// A safe wrapper around Ultralight's ULString type.
pub struct String {
    raw: ULString,
    owned: bool,
}

impl String {
    /// Create a new string from a raw ULString pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid ULString created by the Ultralight API.
    /// If `owned` is true, the Drop implementation will call ulDestroyString.
    /// The caller must ensure that:
    /// 1. The pointer is valid and properly aligned
    /// 2. The ownership semantics are correctly specified
    /// 3. If `owned` is false, the string outlives all references to this wrapper
    pub unsafe fn from_raw(raw: ULString, owned: bool) -> Self {
        Self { raw, owned }
    }

    /// Create a new empty string.
    ///
    /// This function creates a new empty string. The resulting string will be owned
    /// by this wrapper and automatically destroyed when dropped.
    pub fn empty() -> Self {
        Self::from_str("")
    }

    /// Create a new string from a Rust string slice.
    ///
    /// This function creates a new string from a Rust string slice. The resulting
    /// string will be owned by this wrapper and automatically destroyed when dropped.
    ///
    /// # Panics
    ///
    /// Panics if the input string contains null bytes.
    pub fn from_str(s: &str) -> Self {
        let c_str = CString::new(s).unwrap();
        unsafe {
            let raw = ulCreateString(c_str.as_ptr());
            Self { raw, owned: true }
        }
    }

    /// Create a new string from UTF-8 data.
    ///
    /// This function creates a new string from UTF-8 data. The resulting string
    /// will be owned by this wrapper and automatically destroyed when dropped.
    pub fn from_utf8(s: &[u8]) -> Self {
        unsafe {
            let raw = ulCreateStringUTF8(s.as_ptr() as *const c_char, s.len());
            Self { raw, owned: true }
        }
    }

    /// Create a new string from UTF-16 data.
    ///
    /// This function creates a new string from UTF-16 data. The resulting string
    /// will be owned by this wrapper and automatically destroyed when dropped.
    pub fn from_utf16(s: &[u16]) -> Self {
        unsafe {
            let raw = ulCreateStringUTF16(s.as_ptr() as *mut ULChar16, s.len());
            Self { raw, owned: true }
        }
    }

    /// Create a copy of another string.
    ///
    /// This function creates a deep copy of another string. The resulting string
    /// will be owned by this wrapper and automatically destroyed when dropped.
    pub fn from_copy(other: &Self) -> Self {
        unsafe {
            let raw = ulCreateStringFromCopy(other.raw);
            Self { raw, owned: true }
        }
    }

    /// Get a reference to the raw ULString.
    ///
    /// This function returns the raw ULString pointer. This is primarily intended
    /// for internal use and for passing to Ultralight API functions.
    pub fn raw(&self) -> ULString {
        self.raw
    }

    /// Get the UTF-8 data as a string slice.
    ///
    /// This function returns a reference to the UTF-8 data as a string slice.
    /// If the string is null or contains invalid UTF-8, an error is returned.
    ///
    /// # Returns
    ///
    /// - `Ok(&str)` - If the string is valid UTF-8
    /// - `Err(Error::NullReference)` - If the string or its data is null
    /// - `Err(Error::InvalidOperation)` - If the string contains invalid UTF-8
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
            c_str
                .to_str()
                .map_err(|_| Error::InvalidOperation("Invalid UTF-8 in string"))
        }
    }

    /// Get the length of the string in bytes.
    ///
    /// This function returns the length of the string in bytes, not including the null terminator.
    /// If the string is null, 0 is returned.
    pub fn len(&self) -> usize {
        if self.raw.is_null() {
            return 0;
        }

        unsafe { ulStringGetLength(self.raw) }
    }

    /// Check if the string is empty.
    ///
    /// This function returns true if the string is empty or null.
    pub fn is_empty(&self) -> bool {
        if self.raw.is_null() {
            return true;
        }

        unsafe { ulStringIsEmpty(self.raw) }
    }

    /// Assign the content of another string to this one.
    ///
    /// This function assigns the content of another string to this one.
    /// If either string is null, an error is returned.
    ///
    /// # Returns
    ///
    /// - `Ok(())` - If the assignment was successful
    /// - `Err(Error::NullReference)` - If either string is null
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
    ///
    /// This function assigns a Rust string slice to this string.
    /// If the string is null, an error is returned.
    ///
    /// # Returns
    ///
    /// - `Ok(())` - If the assignment was successful
    /// - `Err(Error::NullReference)` - If the string is null
    ///
    /// # Panics
    ///
    /// Panics if the input string contains null bytes.
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

    /// Determines if this string wrapper owns the underlying ULString.
    ///
    /// If this returns true, the Drop implementation will call ulDestroyString.
    /// If this returns false, the Drop implementation will not call ulDestroyString.
    pub fn is_owned(&self) -> bool {
        self.owned
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