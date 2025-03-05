//! Raw FFI bindings to the JavaScriptCore C API.
//!
//! This module provides direct, unsafe bindings to the JavaScriptCore C API functions
//! and types. These bindings strictly adhere to the memory management and calling
//! conventions of the underlying C library while providing type-safe declarations
//! suitable for consumption by the higher-level safe Rust abstractions.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use std::os::raw::{c_char, c_double, c_int, c_uint, c_void, c_ulong, c_uchar, c_ushort};
use std::ptr;

// Opaque types
pub enum OpaqueJSContextGroup {}
pub enum OpaqueJSContext {}
pub enum OpaqueJSString {}
pub enum OpaqueJSClass {}
pub enum OpaqueJSPropertyNameArray {}
pub enum OpaqueJSPropertyNameAccumulator {}
pub enum OpaqueJSValue {}

// Type definitions
pub type JSContextGroupRef = *const OpaqueJSContextGroup;
pub type JSContextRef = *const OpaqueJSContext;
pub type JSGlobalContextRef = *mut OpaqueJSContext;
pub type JSStringRef = *mut OpaqueJSString;
pub type JSClassRef = *mut OpaqueJSClass;
pub type JSPropertyNameArrayRef = *mut OpaqueJSPropertyNameArray;
pub type JSPropertyNameAccumulatorRef = *mut OpaqueJSPropertyNameAccumulator;
pub type JSValueRef = *const OpaqueJSValue;
pub type JSObjectRef = *mut OpaqueJSValue;
pub type JSChar = c_ushort;

// Callback types
pub type JSTypedArrayBytesDeallocator = Option<unsafe extern "C" fn(bytes: *mut c_void, deallocatorContext: *mut c_void)>;
pub type JSObjectInitializeCallback = Option<unsafe extern "C" fn(ctx: JSContextRef, object: JSObjectRef)>;
pub type JSObjectFinalizeCallback = Option<unsafe extern "C" fn(object: JSObjectRef)>;
pub type JSObjectHasPropertyCallback = Option<unsafe extern "C" fn(ctx: JSContextRef, object: JSObjectRef, propertyName: JSStringRef) -> bool>;
pub type JSObjectGetPropertyCallback = Option<unsafe extern "C" fn(ctx: JSContextRef, object: JSObjectRef, propertyName: JSStringRef, exception: *mut JSValueRef) -> JSValueRef>;
pub type JSObjectSetPropertyCallback = Option<unsafe extern "C" fn(ctx: JSContextRef, object: JSObjectRef, propertyName: JSStringRef, value: JSValueRef, exception: *mut JSValueRef) -> bool>;
pub type JSObjectDeletePropertyCallback = Option<unsafe extern "C" fn(ctx: JSContextRef, object: JSObjectRef, propertyName: JSStringRef, exception: *mut JSValueRef) -> bool>;
pub type JSObjectGetPropertyNamesCallback = Option<unsafe extern "C" fn(ctx: JSContextRef, object: JSObjectRef, propertyNames: JSPropertyNameAccumulatorRef)>;
pub type JSObjectCallAsFunctionCallback = Option<unsafe extern "C" fn(ctx: JSContextRef, function: JSObjectRef, thisObject: JSObjectRef, argumentCount: usize, arguments: *const JSValueRef, exception: *mut JSValueRef) -> JSValueRef>;
pub type JSObjectCallAsConstructorCallback = Option<unsafe extern "C" fn(ctx: JSContextRef, constructor: JSObjectRef, argumentCount: usize, arguments: *const JSValueRef, exception: *mut JSValueRef) -> JSObjectRef>;
pub type JSObjectHasInstanceCallback = Option<unsafe extern "C" fn(ctx: JSContextRef, constructor: JSObjectRef, possibleInstance: JSValueRef, exception: *mut JSValueRef) -> bool>;
pub type JSObjectConvertToTypeCallback = Option<unsafe extern "C" fn(ctx: JSContextRef, object: JSObjectRef, type_: JSType, exception: *mut JSValueRef) -> JSValueRef>;

// Enum definitions
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum JSType {
    kJSTypeUndefined,
    kJSTypeNull,
    kJSTypeBoolean,
    kJSTypeNumber,
    kJSTypeString,
    kJSTypeObject,
    kJSTypeSymbol,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum JSTypedArrayType {
    kJSTypedArrayTypeInt8Array,
    kJSTypedArrayTypeInt16Array,
    kJSTypedArrayTypeInt32Array,
    kJSTypedArrayTypeUint8Array,
    kJSTypedArrayTypeUint8ClampedArray,
    kJSTypedArrayTypeUint16Array,
    kJSTypedArrayTypeUint32Array,
    kJSTypedArrayTypeFloat32Array,
    kJSTypedArrayTypeFloat64Array,
    kJSTypedArrayTypeArrayBuffer,
    kJSTypedArrayTypeNone,
    kJSTypedArrayTypeBigInt64Array,
    kJSTypedArrayTypeBigUint64Array,
}

// Property and class attributes
pub const kJSPropertyAttributeNone: c_uint = 0;
pub const kJSPropertyAttributeReadOnly: c_uint = 1 << 1;
pub const kJSPropertyAttributeDontEnum: c_uint = 1 << 2;
pub const kJSPropertyAttributeDontDelete: c_uint = 1 << 3;

pub const kJSClassAttributeNone: c_uint = 0;
pub const kJSClassAttributeNoAutomaticPrototype: c_uint = 1 << 1;

// Struct definitions
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JSStaticValue {
    pub name: *const c_char,
    pub getProperty: JSObjectGetPropertyCallback,
    pub setProperty: JSObjectSetPropertyCallback,
    pub attributes: c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JSStaticFunction {
    pub name: *const c_char,
    pub callAsFunction: JSObjectCallAsFunctionCallback,
    pub attributes: c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JSClassDefinition {
    pub version: c_int,
    pub attributes: c_uint,
    pub className: *const c_char,
    pub parentClass: JSClassRef,
    pub staticValues: *const JSStaticValue,
    pub staticFunctions: *const JSStaticFunction,
    pub initialize: JSObjectInitializeCallback,
    pub finalize: JSObjectFinalizeCallback,
    pub hasProperty: JSObjectHasPropertyCallback,
    pub getProperty: JSObjectGetPropertyCallback,
    pub setProperty: JSObjectSetPropertyCallback,
    pub deleteProperty: JSObjectDeletePropertyCallback,
    pub getPropertyNames: JSObjectGetPropertyNamesCallback,
    pub callAsFunction: JSObjectCallAsFunctionCallback,
    pub callAsConstructor: JSObjectCallAsConstructorCallback,
    pub hasInstance: JSObjectHasInstanceCallback,
    pub convertToType: JSObjectConvertToTypeCallback,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JSClassDefinitionEmpty {
    pub version: c_int,
    pub attributes: c_uint,
    pub className: *const c_char,
    pub parentClass: *mut c_void,
    pub staticValues: *const c_void,
    pub staticFunctions: *const c_void,
    pub initialize: *const c_void,
    pub finalize: *const c_void,
    pub hasProperty: *const c_void,
    pub getProperty: *const c_void,
    pub setProperty: *const c_void,
    pub deleteProperty: *const c_void,
    pub getPropertyNames: *const c_void,
    pub callAsFunction: *const c_void,
    pub callAsConstructor: *const c_void,
    pub hasInstance: *const c_void,
    pub convertToType: *const c_void,
}

extern "C" {
    pub static kJSClassDefinitionEmpty: JSClassDefinitionEmpty;
}

// Function declarations for Context API
extern "C" {
    // Context Group Functions
    pub fn JSContextGroupCreate() -> JSContextGroupRef;
    pub fn JSContextGroupRetain(group: JSContextGroupRef) -> JSContextGroupRef;
    pub fn JSContextGroupRelease(group: JSContextGroupRef);

    // Context Functions
    pub fn JSGlobalContextCreate(globalObjectClass: JSClassRef) -> JSGlobalContextRef;
    pub fn JSGlobalContextCreateInGroup(group: JSContextGroupRef, globalObjectClass: JSClassRef) -> JSGlobalContextRef;
    pub fn JSGlobalContextRetain(ctx: JSGlobalContextRef) -> JSGlobalContextRef;
    pub fn JSGlobalContextRelease(ctx: JSGlobalContextRef);
    pub fn JSContextGetGlobalObject(ctx: JSContextRef) -> JSObjectRef;
    pub fn JSContextGetGroup(ctx: JSContextRef) -> JSContextGroupRef;
    pub fn JSContextGetGlobalContext(ctx: JSContextRef) -> JSGlobalContextRef;
    pub fn JSGlobalContextCopyName(ctx: JSGlobalContextRef) -> JSStringRef;
    pub fn JSGlobalContextSetName(ctx: JSGlobalContextRef, name: JSStringRef);
    pub fn JSGlobalContextIsInspectable(ctx: JSGlobalContextRef) -> bool;
    pub fn JSGlobalContextSetInspectable(ctx: JSGlobalContextRef, inspectable: bool);
}

// Function declarations for String API
extern "C" {
    pub fn JSStringCreateWithCharacters(chars: *const JSChar, numChars: usize) -> JSStringRef;
    pub fn JSStringCreateWithUTF8CString(string: *const c_char) -> JSStringRef;
    pub fn JSStringRetain(string: JSStringRef) -> JSStringRef;
    pub fn JSStringRelease(string: JSStringRef);
    pub fn JSStringGetLength(string: JSStringRef) -> usize;
    pub fn JSStringGetCharactersPtr(string: JSStringRef) -> *const JSChar;
    pub fn JSStringGetMaximumUTF8CStringSize(string: JSStringRef) -> usize;
    pub fn JSStringGetUTF8CString(string: JSStringRef, buffer: *mut c_char, bufferSize: usize) -> usize;
    pub fn JSStringIsEqual(a: JSStringRef, b: JSStringRef) -> bool;
    pub fn JSStringIsEqualToUTF8CString(a: JSStringRef, b: *const c_char) -> bool;
}

// Function declarations for Object API
extern "C" {
    pub fn JSClassCreate(definition: *const JSClassDefinition) -> JSClassRef;
    pub fn JSClassRetain(jsClass: JSClassRef) -> JSClassRef;
    pub fn JSClassRelease(jsClass: JSClassRef);
    pub fn JSObjectMake(ctx: JSContextRef, jsClass: JSClassRef, data: *mut c_void) -> JSObjectRef;
    pub fn JSObjectMakeFunctionWithCallback(ctx: JSContextRef, name: JSStringRef, callback: JSObjectCallAsFunctionCallback) -> JSObjectRef;
    pub fn JSObjectMakeConstructor(ctx: JSContextRef, jsClass: JSClassRef, callAsConstructor: JSObjectCallAsConstructorCallback) -> JSObjectRef;
    pub fn JSObjectMakeArray(ctx: JSContextRef, argumentCount: usize, arguments: *const JSValueRef, exception: *mut JSValueRef) -> JSObjectRef;
    pub fn JSObjectMakeDate(ctx: JSContextRef, argumentCount: usize, arguments: *const JSValueRef, exception: *mut JSValueRef) -> JSObjectRef;
    pub fn JSObjectMakeError(ctx: JSContextRef, argumentCount: usize, arguments: *const JSValueRef, exception: *mut JSValueRef) -> JSObjectRef;
    pub fn JSObjectMakeRegExp(ctx: JSContextRef, argumentCount: usize, arguments: *const JSValueRef, exception: *mut JSValueRef) -> JSObjectRef;
    pub fn JSObjectMakeDeferredPromise(ctx: JSContextRef, resolve: *mut JSObjectRef, reject: *mut JSObjectRef, exception: *mut JSValueRef) -> JSObjectRef;
    pub fn JSObjectMakeFunction(ctx: JSContextRef, name: JSStringRef, parameterCount: c_uint, parameterNames: *const JSStringRef, body: JSStringRef, sourceURL: JSStringRef, startingLineNumber: c_int, exception: *mut JSValueRef) -> JSObjectRef;
    pub fn JSObjectGetPrototype(ctx: JSContextRef, object: JSObjectRef) -> JSValueRef;
    pub fn JSObjectSetPrototype(ctx: JSContextRef, object: JSObjectRef, value: JSValueRef);
    pub fn JSObjectHasProperty(ctx: JSContextRef, object: JSObjectRef, propertyName: JSStringRef) -> bool;
    pub fn JSObjectGetProperty(ctx: JSContextRef, object: JSObjectRef, propertyName: JSStringRef, exception: *mut JSValueRef) -> JSValueRef;
    pub fn JSObjectSetProperty(ctx: JSContextRef, object: JSObjectRef, propertyName: JSStringRef, value: JSValueRef, attributes: c_uint, exception: *mut JSValueRef);
    pub fn JSObjectDeleteProperty(ctx: JSContextRef, object: JSObjectRef, propertyName: JSStringRef, exception: *mut JSValueRef) -> bool;
    pub fn JSObjectHasPropertyForKey(ctx: JSContextRef, object: JSObjectRef, propertyKey: JSValueRef, exception: *mut JSValueRef) -> bool;
    pub fn JSObjectGetPropertyForKey(ctx: JSContextRef, object: JSObjectRef, propertyKey: JSValueRef, exception: *mut JSValueRef) -> JSValueRef;
    pub fn JSObjectSetPropertyForKey(ctx: JSContextRef, object: JSObjectRef, propertyKey: JSValueRef, value: JSValueRef, attributes: c_uint, exception: *mut JSValueRef);
    pub fn JSObjectDeletePropertyForKey(ctx: JSContextRef, object: JSObjectRef, propertyKey: JSValueRef, exception: *mut JSValueRef) -> bool;
    pub fn JSObjectGetPropertyAtIndex(ctx: JSContextRef, object: JSObjectRef, propertyIndex: c_uint, exception: *mut JSValueRef) -> JSValueRef;
    pub fn JSObjectSetPropertyAtIndex(ctx: JSContextRef, object: JSObjectRef, propertyIndex: c_uint, value: JSValueRef, exception: *mut JSValueRef);
    pub fn JSObjectGetPrivate(object: JSObjectRef) -> *mut c_void;
    pub fn JSObjectSetPrivate(object: JSObjectRef, data: *mut c_void) -> bool;
    pub fn JSObjectIsFunction(ctx: JSContextRef, object: JSObjectRef) -> bool;
    pub fn JSObjectCallAsFunction(ctx: JSContextRef, object: JSObjectRef, thisObject: JSObjectRef, argumentCount: usize, arguments: *const JSValueRef, exception: *mut JSValueRef) -> JSValueRef;
    pub fn JSObjectIsConstructor(ctx: JSContextRef, object: JSObjectRef) -> bool;
    pub fn JSObjectCallAsConstructor(ctx: JSContextRef, object: JSObjectRef, argumentCount: usize, arguments: *const JSValueRef, exception: *mut JSValueRef) -> JSObjectRef;
    pub fn JSObjectCopyPropertyNames(ctx: JSContextRef, object: JSObjectRef) -> JSPropertyNameArrayRef;
    pub fn JSPropertyNameArrayRetain(array: JSPropertyNameArrayRef) -> JSPropertyNameArrayRef;
    pub fn JSPropertyNameArrayRelease(array: JSPropertyNameArrayRef);
    pub fn JSPropertyNameArrayGetCount(array: JSPropertyNameArrayRef) -> usize;
    pub fn JSPropertyNameArrayGetNameAtIndex(array: JSPropertyNameArrayRef, index: usize) -> JSStringRef;
    pub fn JSPropertyNameAccumulatorAddName(accumulator: JSPropertyNameAccumulatorRef, propertyName: JSStringRef);
    
    // ObjectRef Private API
    pub fn JSObjectSetPrivateProperty(ctx: JSContextRef, object: JSObjectRef, propertyName: JSStringRef, value: JSValueRef) -> bool;
    pub fn JSObjectGetPrivateProperty(ctx: JSContextRef, object: JSObjectRef, propertyName: JSStringRef) -> JSValueRef;
    pub fn JSObjectDeletePrivateProperty(ctx: JSContextRef, object: JSObjectRef, propertyName: JSStringRef) -> bool;
    pub fn JSObjectGetProxyTarget(object: JSObjectRef) -> JSObjectRef;
    pub fn JSObjectGetGlobalContext(object: JSObjectRef) -> JSGlobalContextRef;
}

// Function declarations for Value API
extern "C" {
    pub fn JSValueGetType(ctx: JSContextRef, value: JSValueRef) -> JSType;
    pub fn JSValueIsUndefined(ctx: JSContextRef, value: JSValueRef) -> bool;
    pub fn JSValueIsNull(ctx: JSContextRef, value: JSValueRef) -> bool;
    pub fn JSValueIsBoolean(ctx: JSContextRef, value: JSValueRef) -> bool;
    pub fn JSValueIsNumber(ctx: JSContextRef, value: JSValueRef) -> bool;
    pub fn JSValueIsString(ctx: JSContextRef, value: JSValueRef) -> bool;
    pub fn JSValueIsSymbol(ctx: JSContextRef, value: JSValueRef) -> bool;
    pub fn JSValueIsObject(ctx: JSContextRef, value: JSValueRef) -> bool;
    pub fn JSValueIsObjectOfClass(ctx: JSContextRef, value: JSValueRef, jsClass: JSClassRef) -> bool;
    pub fn JSValueIsArray(ctx: JSContextRef, value: JSValueRef) -> bool;
    pub fn JSValueIsDate(ctx: JSContextRef, value: JSValueRef) -> bool;
    pub fn JSValueGetTypedArrayType(ctx: JSContextRef, value: JSValueRef, exception: *mut JSValueRef) -> JSTypedArrayType;
    pub fn JSValueIsEqual(ctx: JSContextRef, a: JSValueRef, b: JSValueRef, exception: *mut JSValueRef) -> bool;
    pub fn JSValueIsStrictEqual(ctx: JSContextRef, a: JSValueRef, b: JSValueRef) -> bool;
    pub fn JSValueIsInstanceOfConstructor(ctx: JSContextRef, value: JSValueRef, constructor: JSObjectRef, exception: *mut JSValueRef) -> bool;
    pub fn JSValueMakeUndefined(ctx: JSContextRef) -> JSValueRef;
    pub fn JSValueMakeNull(ctx: JSContextRef) -> JSValueRef;
    pub fn JSValueMakeBoolean(ctx: JSContextRef, boolean: bool) -> JSValueRef;
    pub fn JSValueMakeNumber(ctx: JSContextRef, number: c_double) -> JSValueRef;
    pub fn JSValueMakeString(ctx: JSContextRef, string: JSStringRef) -> JSValueRef;
    pub fn JSValueMakeSymbol(ctx: JSContextRef, description: JSStringRef) -> JSValueRef;
    pub fn JSValueMakeFromJSONString(ctx: JSContextRef, string: JSStringRef) -> JSValueRef;
    pub fn JSValueCreateJSONString(ctx: JSContextRef, value: JSValueRef, indent: c_uint, exception: *mut JSValueRef) -> JSStringRef;
    pub fn JSValueToBoolean(ctx: JSContextRef, value: JSValueRef) -> bool;
    pub fn JSValueToNumber(ctx: JSContextRef, value: JSValueRef, exception: *mut JSValueRef) -> c_double;
    pub fn JSValueToStringCopy(ctx: JSContextRef, value: JSValueRef, exception: *mut JSValueRef) -> JSStringRef;
    pub fn JSValueToObject(ctx: JSContextRef, value: JSValueRef, exception: *mut JSValueRef) -> JSObjectRef;
    pub fn JSValueProtect(ctx: JSContextRef, value: JSValueRef);
    pub fn JSValueUnprotect(ctx: JSContextRef, value: JSValueRef);
}

// Function declarations for Typed Array API
extern "C" {
    pub fn JSObjectMakeTypedArray(ctx: JSContextRef, arrayType: JSTypedArrayType, length: usize, exception: *mut JSValueRef) -> JSObjectRef;
    pub fn JSObjectMakeTypedArrayWithBytesNoCopy(ctx: JSContextRef, arrayType: JSTypedArrayType, bytes: *mut c_void, byteLength: usize, bytesDeallocator: JSTypedArrayBytesDeallocator, deallocatorContext: *mut c_void, exception: *mut JSValueRef) -> JSObjectRef;
    pub fn JSObjectMakeTypedArrayWithArrayBuffer(ctx: JSContextRef, arrayType: JSTypedArrayType, buffer: JSObjectRef, exception: *mut JSValueRef) -> JSObjectRef;
    pub fn JSObjectMakeTypedArrayWithArrayBufferAndOffset(ctx: JSContextRef, arrayType: JSTypedArrayType, buffer: JSObjectRef, byteOffset: usize, length: usize, exception: *mut JSValueRef) -> JSObjectRef;
    pub fn JSObjectGetTypedArrayBytesPtr(ctx: JSContextRef, object: JSObjectRef, exception: *mut JSValueRef) -> *mut c_void;
    pub fn JSObjectGetTypedArrayLength(ctx: JSContextRef, object: JSObjectRef, exception: *mut JSValueRef) -> usize;
    pub fn JSObjectGetTypedArrayByteLength(ctx: JSContextRef, object: JSObjectRef, exception: *mut JSValueRef) -> usize;
    pub fn JSObjectGetTypedArrayByteOffset(ctx: JSContextRef, object: JSObjectRef, exception: *mut JSValueRef) -> usize;
    pub fn JSObjectGetTypedArrayBuffer(ctx: JSContextRef, object: JSObjectRef, exception: *mut JSValueRef) -> JSObjectRef;
    pub fn JSObjectMakeArrayBufferWithBytesNoCopy(ctx: JSContextRef, bytes: *mut c_void, byteLength: usize, bytesDeallocator: JSTypedArrayBytesDeallocator, deallocatorContext: *mut c_void, exception: *mut JSValueRef) -> JSObjectRef;
    pub fn JSObjectGetArrayBufferBytesPtr(ctx: JSContextRef, object: JSObjectRef, exception: *mut JSValueRef) -> *mut c_void;
    pub fn JSObjectGetArrayBufferByteLength(ctx: JSContextRef, object: JSObjectRef, exception: *mut JSValueRef) -> usize;
}

// Function declarations for Script Evaluation
extern "C" {
    pub fn JSEvaluateScript(ctx: JSContextRef, script: JSStringRef, thisObject: JSObjectRef, sourceURL: JSStringRef, startingLineNumber: c_int, exception: *mut JSValueRef) -> JSValueRef;
    pub fn JSCheckScriptSyntax(ctx: JSContextRef, script: JSStringRef, sourceURL: JSStringRef, startingLineNumber: c_int, exception: *mut JSValueRef) -> bool;
    pub fn JSGarbageCollect(ctx: JSContextRef);
}