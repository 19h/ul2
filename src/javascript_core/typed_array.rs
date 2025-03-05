//! JavaScriptCore typed arrays implementation.
//!
//! This module provides a comprehensive abstraction over JavaScript typed arrays,
//! implementing the necessary memory management strategies and binding to the
//! underlying JavaScriptCore C API. The TypedArray struct represents various
//! numeric array types available in JavaScript, with methods for creation,
//! manipulation, and data access.

use std::marker::PhantomData;
use std::ptr;
use std::os::raw::c_void;

use crate::javascript_core::context::Context;
use crate::javascript_core::error::{Error, Result};
use crate::javascript_core::ffi;
use crate::javascript_core::object::Object;
use crate::javascript_core::value::Value;

/// Represents the type of a JavaScript typed array.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TypedArrayType {
    /// An Int8Array - array of 8-bit signed integers
    Int8Array,
    /// An Int16Array - array of 16-bit signed integers
    Int16Array,
    /// An Int32Array - array of 32-bit signed integers
    Int32Array,
    /// A Uint8Array - array of 8-bit unsigned integers
    Uint8Array,
    /// A Uint8ClampedArray - array of 8-bit unsigned integers (clamped)
    Uint8ClampedArray,
    /// A Uint16Array - array of 16-bit unsigned integers
    Uint16Array,
    /// A Uint32Array - array of 32-bit unsigned integers
    Uint32Array,
    /// A Float32Array - array of 32-bit floating point numbers
    Float32Array,
    /// A Float64Array - array of 64-bit floating point numbers
    Float64Array,
    /// A BigInt64Array - array of 64-bit signed integers as BigInts
    BigInt64Array,
    /// A BigUint64Array - array of 64-bit unsigned integers as BigInts
    BigUint64Array,
    /// An ArrayBuffer - raw binary data buffer
    ArrayBuffer,
}

impl TypedArrayType {
    /// Converts the Rust TypedArrayType enum to the FFI JSTypedArrayType enum.
    fn to_ffi(&self) -> ffi::JSTypedArrayType {
        match self {
            TypedArrayType::Int8Array => ffi::JSTypedArrayType::kJSTypedArrayTypeInt8Array,
            TypedArrayType::Int16Array => ffi::JSTypedArrayType::kJSTypedArrayTypeInt16Array,
            TypedArrayType::Int32Array => ffi::JSTypedArrayType::kJSTypedArrayTypeInt32Array,
            TypedArrayType::Uint8Array => ffi::JSTypedArrayType::kJSTypedArrayTypeUint8Array,
            TypedArrayType::Uint8ClampedArray => ffi::JSTypedArrayType::kJSTypedArrayTypeUint8ClampedArray,
            TypedArrayType::Uint16Array => ffi::JSTypedArrayType::kJSTypedArrayTypeUint16Array,
            TypedArrayType::Uint32Array => ffi::JSTypedArrayType::kJSTypedArrayTypeUint32Array,
            TypedArrayType::Float32Array => ffi::JSTypedArrayType::kJSTypedArrayTypeFloat32Array,
            TypedArrayType::Float64Array => ffi::JSTypedArrayType::kJSTypedArrayTypeFloat64Array,
            TypedArrayType::BigInt64Array => ffi::JSTypedArrayType::kJSTypedArrayTypeBigInt64Array,
            TypedArrayType::BigUint64Array => ffi::JSTypedArrayType::kJSTypedArrayTypeBigUint64Array,
            TypedArrayType::ArrayBuffer => ffi::JSTypedArrayType::kJSTypedArrayTypeArrayBuffer,
        }
    }

    /// Converts the FFI JSTypedArrayType enum to the Rust TypedArrayType enum.
    fn from_ffi(ty: ffi::JSTypedArrayType) -> Option<Self> {
        match ty {
            ffi::JSTypedArrayType::kJSTypedArrayTypeInt8Array => Some(TypedArrayType::Int8Array),
            ffi::JSTypedArrayType::kJSTypedArrayTypeInt16Array => Some(TypedArrayType::Int16Array),
            ffi::JSTypedArrayType::kJSTypedArrayTypeInt32Array => Some(TypedArrayType::Int32Array),
            ffi::JSTypedArrayType::kJSTypedArrayTypeUint8Array => Some(TypedArrayType::Uint8Array),
            ffi::JSTypedArrayType::kJSTypedArrayTypeUint8ClampedArray => Some(TypedArrayType::Uint8ClampedArray),
            ffi::JSTypedArrayType::kJSTypedArrayTypeUint16Array => Some(TypedArrayType::Uint16Array),
            ffi::JSTypedArrayType::kJSTypedArrayTypeUint32Array => Some(TypedArrayType::Uint32Array),
            ffi::JSTypedArrayType::kJSTypedArrayTypeFloat32Array => Some(TypedArrayType::Float32Array),
            ffi::JSTypedArrayType::kJSTypedArrayTypeFloat64Array => Some(TypedArrayType::Float64Array),
            ffi::JSTypedArrayType::kJSTypedArrayTypeBigInt64Array => Some(TypedArrayType::BigInt64Array),
            ffi::JSTypedArrayType::kJSTypedArrayTypeBigUint64Array => Some(TypedArrayType::BigUint64Array),
            ffi::JSTypedArrayType::kJSTypedArrayTypeArrayBuffer => Some(TypedArrayType::ArrayBuffer),
            ffi::JSTypedArrayType::kJSTypedArrayTypeNone => None,
        }
    }

    /// Returns the element size in bytes for this typed array type.
    pub fn element_size(&self) -> usize {
        match self {
            TypedArrayType::Int8Array | TypedArrayType::Uint8Array | TypedArrayType::Uint8ClampedArray => 1,
            TypedArrayType::Int16Array | TypedArrayType::Uint16Array => 2,
            TypedArrayType::Int32Array | TypedArrayType::Uint32Array | TypedArrayType::Float32Array => 4,
            TypedArrayType::Float64Array | TypedArrayType::BigInt64Array | TypedArrayType::BigUint64Array => 8,
            TypedArrayType::ArrayBuffer => 1, // ArrayBuffer has byte-level access
        }
    }
}

/// A safe wrapper around a JavaScript typed array.
///
/// The TypedArray struct encapsulates a JSObjectRef representing a JavaScript
/// typed array, providing type-safe access to its data and properties.
pub struct TypedArray<'a> {
    /// The underlying JavaScript object.
    object: Object<'a>,
    /// The type of the typed array.
    ty: TypedArrayType,
}

impl<'a> TypedArray<'a> {
    /// Creates a new typed array with the specified element count.
    ///
    /// # Arguments
    ///
    /// * `context` - The context in which to create the typed array.
    /// * `ty` - The type of typed array to create.
    /// * `length` - The number of elements in the typed array.
    ///
    /// # Returns
    ///
    /// A Result containing the new typed array or an error.
    pub fn new(context: &Context<'a>, ty: TypedArrayType, length: usize) -> Result<Self> {
        let jsc_ty = ty.to_ffi();
        
        unsafe {
            let mut exception = ptr::null();
            let raw = ffi::JSObjectMakeTypedArray(
                context.as_raw(),
                jsc_ty,
                length,
                &mut exception
            );
            
            if !exception.is_null() {
                return Err(Error::from_js_exception(context.as_raw(), exception));
            }
            
            if raw.is_null() {
                return Err(Error::JSError(format!("Failed to create typed array of type {:?}", ty)));
            }
            
            Ok(TypedArray {
                object: Object::from_raw(context.clone(), raw),
                ty,
            })
        }
    }
    
    /// Creates a new typed array from an existing buffer without copying.
    ///
    /// # Arguments
    ///
    /// * `context` - The context in which to create the typed array.
    /// * `ty` - The type of typed array to create.
    /// * `bytes` - Pointer to the byte buffer to use.
    /// * `byte_length` - The length of the buffer in bytes.
    /// * `deallocator` - Optional callback to free the buffer when the typed array is garbage collected.
    /// * `deallocator_context` - Optional context passed to the deallocator.
    ///
    /// # Returns
    ///
    /// A Result containing the new typed array or an error.
    pub fn from_bytes_no_copy(
        context: &Context<'a>,
        ty: TypedArrayType,
        bytes: *mut c_void,
        byte_length: usize,
        deallocator: Option<ffi::JSTypedArrayBytesDeallocator>,
        deallocator_context: Option<*mut c_void>
    ) -> Result<Self> {
        let jsc_ty = ty.to_ffi();
        
        unsafe {
            let mut exception = ptr::null();
            let raw = ffi::JSObjectMakeTypedArrayWithBytesNoCopy(
                context.as_raw(),
                jsc_ty,
                bytes,
                byte_length,
                deallocator,
                deallocator_context.unwrap_or(ptr::null_mut()),
                &mut exception
            );
            
            if !exception.is_null() {
                return Err(Error::from_js_exception(context.as_raw(), exception));
            }
            
            if raw.is_null() {
                return Err(Error::JSError(format!("Failed to create typed array from bytes")));
            }
            
            Ok(TypedArray {
                object: Object::from_raw(context.clone(), raw),
                ty,
            })
        }
    }
    
    /// Creates a new typed array using an existing ArrayBuffer as its backing store.
    ///
    /// # Arguments
    ///
    /// * `context` - The context in which to create the typed array.
    /// * `ty` - The type of typed array to create.
    /// * `buffer` - The ArrayBuffer to use as backing store.
    ///
    /// # Returns
    ///
    /// A Result containing the new typed array or an error.
    pub fn from_array_buffer(
        context: &Context<'a>,
        ty: TypedArrayType,
        buffer: &Object<'a>
    ) -> Result<Self> {
        if ty == TypedArrayType::ArrayBuffer {
            return Err(Error::InvalidType("Cannot create ArrayBuffer from ArrayBuffer".to_string()));
        }
        
        let jsc_ty = ty.to_ffi();
        
        unsafe {
            let mut exception = ptr::null();
            let raw = ffi::JSObjectMakeTypedArrayWithArrayBuffer(
                context.as_raw(),
                jsc_ty,
                buffer.as_raw(),
                &mut exception
            );
            
            if !exception.is_null() {
                return Err(Error::from_js_exception(context.as_raw(), exception));
            }
            
            if raw.is_null() {
                return Err(Error::JSError(format!("Failed to create typed array from array buffer")));
            }
            
            Ok(TypedArray {
                object: Object::from_raw(context.clone(), raw),
                ty,
            })
        }
    }
    
    /// Creates a typed array using a subset of an existing ArrayBuffer.
    ///
    /// # Arguments
    ///
    /// * `context` - The context in which to create the typed array.
    /// * `ty` - The type of typed array to create.
    /// * `buffer` - The ArrayBuffer to use as backing store.
    /// * `byte_offset` - The offset in bytes from the start of the ArrayBuffer.
    /// * `length` - The number of elements (not bytes) in the typed array.
    ///
    /// # Returns
    ///
    /// A Result containing the new typed array or an error.
    pub fn from_array_buffer_with_offset(
        context: &Context<'a>,
        ty: TypedArrayType,
        buffer: &Object<'a>,
        byte_offset: usize,
        length: usize
    ) -> Result<Self> {
        if ty == TypedArrayType::ArrayBuffer {
            return Err(Error::InvalidType("Cannot create ArrayBuffer from ArrayBuffer with offset".to_string()));
        }
        
        let jsc_ty = ty.to_ffi();
        
        unsafe {
            let mut exception = ptr::null();
            let raw = ffi::JSObjectMakeTypedArrayWithArrayBufferAndOffset(
                context.as_raw(),
                jsc_ty,
                buffer.as_raw(),
                byte_offset,
                length,
                &mut exception
            );
            
            if !exception.is_null() {
                return Err(Error::from_js_exception(context.as_raw(), exception));
            }
            
            if raw.is_null() {
                return Err(Error::JSError(format!("Failed to create typed array from array buffer with offset")));
            }
            
            Ok(TypedArray {
                object: Object::from_raw(context.clone(), raw),
                ty,
            })
        }
    }
    
    /// Creates a TypedArray from an existing JavaScript object.
    ///
    /// # Arguments
    ///
    /// * `context` - The context in which the object exists.
    /// * `object` - The JavaScript object to convert.
    ///
    /// # Returns
    ///
    /// A Result containing the typed array or an error if the object is not a valid typed array.
    pub fn from_object(context: &Context<'a>, object: Object<'a>) -> Result<Self> {
        unsafe {
            let mut exception = ptr::null();
            let js_type = ffi::JSValueGetTypedArrayType(
                context.as_raw(),
                object.as_raw() as ffi::JSValueRef,
                &mut exception
            );
            
            if !exception.is_null() {
                return Err(Error::from_js_exception(context.as_raw(), exception));
            }
            
            if js_type == ffi::JSTypedArrayType::kJSTypedArrayTypeNone {
                return Err(Error::InvalidType("Object is not a typed array".to_string()));
            }
            
            let ty = TypedArrayType::from_ffi(js_type)
                .ok_or_else(|| Error::InvalidType("Unknown typed array type".to_string()))?;
            
            Ok(TypedArray { object, ty })
        }
    }
    
    /// Gets the underlying JavaScript object.
    ///
    /// # Returns
    ///
    /// A reference to the underlying JavaScript object.
    pub fn as_object(&self) -> &Object<'a> {
        &self.object
    }
    
    /// Gets the type of this typed array.
    ///
    /// # Returns
    ///
    /// The type of this typed array.
    pub fn array_type(&self) -> TypedArrayType {
        self.ty
    }
    
    /// Gets the number of elements in this typed array.
    ///
    /// # Returns
    ///
    /// A Result containing the length or an error.
    pub fn length(&self) -> Result<usize> {
        let context = self.object.context();
        
        unsafe {
            let mut exception = ptr::null();
            let length = ffi::JSObjectGetTypedArrayLength(
                context.as_raw(),
                self.object.as_raw(),
                &mut exception
            );
            
            if !exception.is_null() {
                return Err(Error::from_js_exception(context.as_raw(), exception));
            }
            
            Ok(length)
        }
    }
    
    /// Gets the byte length of this typed array.
    ///
    /// # Returns
    ///
    /// A Result containing the byte length or an error.
    pub fn byte_length(&self) -> Result<usize> {
        let context = self.object.context();
        
        unsafe {
            let mut exception = ptr::null();
            let length = ffi::JSObjectGetTypedArrayByteLength(
                context.as_raw(),
                self.object.as_raw(),
                &mut exception
            );
            
            if !exception.is_null() {
                return Err(Error::from_js_exception(context.as_raw(), exception));
            }
            
            Ok(length)
        }
    }
    
    /// Gets the byte offset of this typed array if it was created from an ArrayBuffer.
    ///
    /// # Returns
    ///
    /// A Result containing the byte offset or an error.
    pub fn byte_offset(&self) -> Result<usize> {
        let context = self.object.context();
        
        unsafe {
            let mut exception = ptr::null();
            let offset = ffi::JSObjectGetTypedArrayByteOffset(
                context.as_raw(),
                self.object.as_raw(),
                &mut exception
            );
            
            if !exception.is_null() {
                return Err(Error::from_js_exception(context.as_raw(), exception));
            }
            
            Ok(offset)
        }
    }
    
    /// Gets a pointer to the typed array's data buffer.
    ///
    /// # Safety
    ///
    /// The returned pointer is only valid until the next time JavaScript code runs.
    /// The caller must ensure the pointer is not used after that.
    ///
    /// # Returns
    ///
    /// A Result containing a pointer to the data buffer or an error.
    pub unsafe fn bytes_ptr(&self) -> Result<*mut u8> {
        let context = self.object.context();
        
        let mut exception = ptr::null();
        let ptr = ffi::JSObjectGetTypedArrayBytesPtr(
            context.as_raw(),
            self.object.as_raw(),
            &mut exception
        ) as *mut u8;
        
        if !exception.is_null() {
            return Err(Error::from_js_exception(context.as_raw(), exception));
        }
        
        if ptr.is_null() {
            return Err(Error::JSError("Failed to get typed array bytes".to_string()));
        }
        
        Ok(ptr)
    }
    
    /// Gets a slice to the typed array's data buffer.
    ///
    /// # Safety
    ///
    /// The returned slice is only valid until the next time JavaScript code runs.
    /// The caller must ensure the slice is not used after that.
    ///
    /// # Returns
    ///
    /// A Result containing a slice of the data buffer or an error.
    pub unsafe fn as_slice<T>(&self) -> Result<&[T]> {
        let ptr = self.bytes_ptr()? as *const T;
        let len = self.length()?;
        
        Ok(std::slice::from_raw_parts(ptr, len))
    }
    
    /// Gets a mutable slice to the typed array's data buffer.
    ///
    /// # Safety
    ///
    /// The returned slice is only valid until the next time JavaScript code runs.
    /// The caller must ensure the slice is not used after that.
    ///
    /// # Returns
    ///
    /// A Result containing a mutable slice of the data buffer or an error.
    pub unsafe fn as_slice_mut<T>(&self) -> Result<&mut [T]> {
        let ptr = self.bytes_ptr()? as *mut T;
        let len = self.length()?;
        
        Ok(std::slice::from_raw_parts_mut(ptr, len))
    }
    
    /// Gets the underlying ArrayBuffer for this typed array.
    ///
    /// # Returns
    ///
    /// A Result containing the ArrayBuffer or an error.
    pub fn buffer(&self) -> Result<Object<'a>> {
        let context = self.object.context();
        
        unsafe {
            let mut exception = ptr::null();
            let raw = ffi::JSObjectGetTypedArrayBuffer(
                context.as_raw(),
                self.object.as_raw(),
                &mut exception
            );
            
            if !exception.is_null() {
                return Err(Error::from_js_exception(context.as_raw(), exception));
            }
            
            if raw.is_null() {
                return Err(Error::JSError("Failed to get array buffer".to_string()));
            }
            
            Ok(Object::from_raw(context.clone(), raw))
        }
    }
    
    /// Converts this typed array to a JavaScript value.
    ///
    /// # Returns
    ///
    /// A JavaScript value representing this typed array.
    pub fn to_value(&self) -> Value<'a> {
        self.object.to_value()
    }
}

/// A safe wrapper around a JavaScript ArrayBuffer.
pub struct ArrayBuffer<'a> {
    /// The underlying typed array.
    typed_array: TypedArray<'a>,
}

impl<'a> ArrayBuffer<'a> {
    /// Creates a new ArrayBuffer with the specified byte length.
    ///
    /// # Arguments
    ///
    /// * `context` - The context in which to create the ArrayBuffer.
    /// * `byte_length` - The size of the ArrayBuffer in bytes.
    ///
    /// # Returns
    ///
    /// A Result containing the new ArrayBuffer or an error.
    pub fn new(context: &Context<'a>, byte_length: usize) -> Result<Self> {
        let typed_array = TypedArray::new(context, TypedArrayType::ArrayBuffer, byte_length)?;
        Ok(ArrayBuffer { typed_array })
    }
    
    /// Creates a new ArrayBuffer from an existing buffer without copying.
    ///
    /// # Arguments
    ///
    /// * `context` - The context in which to create the ArrayBuffer.
    /// * `bytes` - Pointer to the byte buffer to use.
    /// * `byte_length` - The length of the buffer in bytes.
    /// * `deallocator` - Optional callback to free the buffer when the ArrayBuffer is garbage collected.
    /// * `deallocator_context` - Optional context passed to the deallocator.
    ///
    /// # Returns
    ///
    /// A Result containing the new ArrayBuffer or an error.
    pub fn from_bytes_no_copy(
        context: &Context<'a>,
        bytes: *mut c_void,
        byte_length: usize,
        deallocator: Option<ffi::JSTypedArrayBytesDeallocator>,
        deallocator_context: Option<*mut c_void>
    ) -> Result<Self> {
        unsafe {
            let mut exception = ptr::null();
            let raw = ffi::JSObjectMakeArrayBufferWithBytesNoCopy(
                context.as_raw(),
                bytes,
                byte_length,
                deallocator,
                deallocator_context.unwrap_or(ptr::null_mut()),
                &mut exception
            );
            
            if !exception.is_null() {
                return Err(Error::from_js_exception(context.as_raw(), exception));
            }
            
            if raw.is_null() {
                return Err(Error::JSError("Failed to create array buffer from bytes".to_string()));
            }
            
            let object = Object::from_raw(context.clone(), raw);
            let typed_array = TypedArray { object, ty: TypedArrayType::ArrayBuffer };
            
            Ok(ArrayBuffer { typed_array })
        }
    }
    
    /// Creates an ArrayBuffer from an existing JavaScript object.
    ///
    /// # Arguments
    ///
    /// * `context` - The context in which the object exists.
    /// * `object` - The JavaScript object to convert.
    ///
    /// # Returns
    ///
    /// A Result containing the ArrayBuffer or an error if the object is not a valid ArrayBuffer.
    pub fn from_object(context: &Context<'a>, object: Object<'a>) -> Result<Self> {
        let typed_array = TypedArray::from_object(context, object)?;
        
        if typed_array.ty != TypedArrayType::ArrayBuffer {
            return Err(Error::InvalidType("Object is not an ArrayBuffer".to_string()));
        }
        
        Ok(ArrayBuffer { typed_array })
    }
    
    /// Gets the underlying TypedArray.
    ///
    /// # Returns
    ///
    /// A reference to the underlying TypedArray.
    pub fn as_typed_array(&self) -> &TypedArray<'a> {
        &self.typed_array
    }
    
    /// Gets the underlying JavaScript object.
    ///
    /// # Returns
    ///
    /// A reference to the underlying JavaScript object.
    pub fn as_object(&self) -> &Object<'a> {
        self.typed_array.as_object()
    }
    
    /// Gets the byte length of this ArrayBuffer.
    ///
    /// # Returns
    ///
    /// A Result containing the byte length or an error.
    pub fn byte_length(&self) -> Result<usize> {
        let context = self.typed_array.object.context();
        
        unsafe {
            let mut exception = ptr::null();
            let length = ffi::JSObjectGetArrayBufferByteLength(
                context.as_raw(),
                self.typed_array.object.as_raw(),
                &mut exception
            );
            
            if !exception.is_null() {
                return Err(Error::from_js_exception(context.as_raw(), exception));
            }
            
            Ok(length)
        }
    }
    
    /// Gets a pointer to the ArrayBuffer's data.
    ///
    /// # Safety
    ///
    /// The returned pointer is only valid until the next time JavaScript code runs.
    /// The caller must ensure the pointer is not used after that.
    ///
    /// # Returns
    ///
    /// A Result containing a pointer to the data or an error.
    pub unsafe fn bytes_ptr(&self) -> Result<*mut u8> {
        let context = self.typed_array.object.context();
        
        let mut exception = ptr::null();
        let ptr = ffi::JSObjectGetArrayBufferBytesPtr(
            context.as_raw(),
            self.typed_array.object.as_raw(),
            &mut exception
        ) as *mut u8;
        
        if !exception.is_null() {
            return Err(Error::from_js_exception(context.as_raw(), exception));
        }
        
        if ptr.is_null() {
            return Err(Error::JSError("Failed to get array buffer bytes".to_string()));
        }
        
        Ok(ptr)
    }
    
    /// Gets a slice to the ArrayBuffer's data.
    ///
    /// # Safety
    ///
    /// The returned slice is only valid until the next time JavaScript code runs.
    /// The caller must ensure the slice is not used after that.
    ///
    /// # Returns
    ///
    /// A Result containing a slice of the data or an error.
    pub unsafe fn as_slice(&self) -> Result<&[u8]> {
        let ptr = self.bytes_ptr()?;
        let len = self.byte_length()?;
        
        Ok(std::slice::from_raw_parts(ptr, len))
    }
    
    /// Gets a mutable slice to the ArrayBuffer's data.
    ///
    /// # Safety
    ///
    /// The returned slice is only valid until the next time JavaScript code runs.
    /// The caller must ensure the slice is not used after that.
    ///
    /// # Returns
    ///
    /// A Result containing a mutable slice of the data or an error.
    pub unsafe fn as_slice_mut(&self) -> Result<&mut [u8]> {
        let ptr = self.bytes_ptr()?;
        let len = self.byte_length()?;
        
        Ok(std::slice::from_raw_parts_mut(ptr, len))
    }
    
    /// Converts this ArrayBuffer to a JavaScript value.
    ///
    /// # Returns
    ///
    /// A JavaScript value representing this ArrayBuffer.
    pub fn to_value(&self) -> Value<'a> {
        self.typed_array.to_value()
    }
}