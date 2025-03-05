//! Rust bindings for JavaScript objects.
//!
//! This module provides safe wrappers around JavaScriptCore's JSObjectRef type, facilitating
//! interaction with JavaScript objects from Rust code. The implementation encapsulates the
//! complexity of memory management and error handling through RAII principles and type safety,
//! while exposing the full functionality of the underlying C API.

use std::convert::TryFrom;
use std::ffi::{CStr, CString};
use std::marker::PhantomData;
use std::mem;
use std::os::raw::{c_char, c_void, c_int, c_uint};
use std::ptr;
use std::slice;

use crate::javascript_core::context::Context;
use crate::javascript_core::error::{Error, Result};
use crate::javascript_core::ffi;
use crate::javascript_core::string::String;
use crate::javascript_core::value::Value;

/// Attributes that can be assigned to JavaScript object properties.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PropertyAttributes(c_uint);

impl PropertyAttributes {
    /// No special attributes.
    pub const NONE: Self = PropertyAttributes(ffi::kJSPropertyAttributeNone);
    
    /// Property is read-only.
    pub const READ_ONLY: Self = PropertyAttributes(ffi::kJSPropertyAttributeReadOnly);
    
    /// Property will not be enumerated by for-in enumeration.
    pub const DONT_ENUM: Self = PropertyAttributes(ffi::kJSPropertyAttributeDontEnum);
    
    /// Property cannot be deleted.
    pub const DONT_DELETE: Self = PropertyAttributes(ffi::kJSPropertyAttributeDontDelete);
    
    /// Returns the raw value of the property attributes as a c_uint.
    pub fn as_raw(&self) -> c_uint {
        self.0
    }
    
    /// Create a new PropertyAttributes with a combination of attributes.
    pub fn new(read_only: bool, dont_enum: bool, dont_delete: bool) -> Self {
        let mut attrs = ffi::kJSPropertyAttributeNone;
        if read_only { attrs |= ffi::kJSPropertyAttributeReadOnly; }
        if dont_enum { attrs |= ffi::kJSPropertyAttributeDontEnum; }
        if dont_delete { attrs |= ffi::kJSPropertyAttributeDontDelete; }
        PropertyAttributes(attrs)
    }
}

impl std::ops::BitOr for PropertyAttributes {
    type Output = Self;
    
    fn bitor(self, rhs: Self) -> Self::Output {
        PropertyAttributes(self.0 | rhs.0)
    }
}

impl std::ops::BitOrAssign for PropertyAttributes {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

/// Attributes that can be assigned to JavaScript classes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ClassAttributes(c_uint);

impl ClassAttributes {
    /// No special attributes.
    pub const NONE: Self = ClassAttributes(ffi::kJSClassAttributeNone);
    
    /// Class will not automatically generate a shared prototype for its instances.
    pub const NO_AUTOMATIC_PROTOTYPE: Self = ClassAttributes(ffi::kJSClassAttributeNoAutomaticPrototype);
    
    /// Returns the raw value of the class attributes as a c_uint.
    pub fn as_raw(&self) -> c_uint {
        self.0
    }
}

impl std::ops::BitOr for ClassAttributes {
    type Output = Self;
    
    fn bitor(self, rhs: Self) -> Self::Output {
        ClassAttributes(self.0 | rhs.0)
    }
}

impl std::ops::BitOrAssign for ClassAttributes {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

/// A callback when an object is first created.
pub type InitializeCallback = Box<dyn Fn(&Context, &Object)>;

/// A callback when an object is finalized.
pub type FinalizeCallback = Box<dyn Fn(&Object)>;

/// A callback to determine if an object has a property.
pub type HasPropertyCallback = Box<dyn Fn(&Context, &Object, &str) -> bool>;

/// A callback to get a property value.
pub type GetPropertyCallback = Box<dyn Fn(&Context, &Object, &str) -> Result<Value>>;

/// A callback to set a property value.
pub type SetPropertyCallback = Box<dyn Fn(&Context, &Object, &str, Value) -> Result<bool>>;

/// A callback to delete a property.
pub type DeletePropertyCallback = Box<dyn Fn(&Context, &Object, &str) -> Result<bool>>;

/// A callback to collect property names.
pub type GetPropertyNamesCallback = Box<dyn Fn(&Context, &Object, &mut Vec<String>)>;

/// A callback to call an object as a function.
pub type CallAsFunctionCallback = Box<dyn Fn(&Context, &Object, Option<&Object>, &[Value]) -> Result<Value>>;

/// A callback to call an object as a constructor.
pub type CallAsConstructorCallback = Box<dyn Fn(&Context, &Object, &[Value]) -> Result<Object>>;

/// A callback to determine if an object is an instance of a constructor.
pub type HasInstanceCallback = Box<dyn Fn(&Context, &Object, &Value) -> Result<bool>>;

/// A callback to convert an object to a primitive type.
pub type ConvertToTypeCallback = Box<dyn Fn(&Context, &Object, ffi::JSType) -> Result<Value>>;

/// Represents a static value property definition.
pub struct StaticValue {
    /// The name of the property.
    pub name: String,
    
    /// Callback function to get the property value.
    pub getter: Option<GetPropertyCallback>,
    
    /// Callback function to set the property value.
    pub setter: Option<SetPropertyCallback>,
    
    /// Attributes for the property.
    pub attributes: PropertyAttributes,
}

/// Represents a static function property definition.
pub struct StaticFunction {
    /// The name of the function.
    pub name: String,
    
    /// Callback function to invoke when the function is called.
    pub callback: CallAsFunctionCallback,
    
    /// Attributes for the function property.
    pub attributes: PropertyAttributes,
}

/// A definition of a JavaScript class.
pub struct ClassDefinition {
    /// The name of the class.
    pub class_name: String,
    
    /// Attributes for the class.
    pub attributes: ClassAttributes,
    
    /// Parent class, if any.
    pub parent_class: Option<Class>,
    
    /// Static value properties of the class.
    pub static_values: Vec<StaticValue>,
    
    /// Static function properties of the class.
    pub static_functions: Vec<StaticFunction>,
    
    /// Callback when an object is initialized.
    pub initialize: Option<InitializeCallback>,
    
    /// Callback when an object is finalized.
    pub finalize: Option<FinalizeCallback>,
    
    /// Callback to determine if an object has a property.
    pub has_property: Option<HasPropertyCallback>,
    
    /// Callback to get a property value.
    pub get_property: Option<GetPropertyCallback>,
    
    /// Callback to set a property value.
    pub set_property: Option<SetPropertyCallback>,
    
    /// Callback to delete a property.
    pub delete_property: Option<DeletePropertyCallback>,
    
    /// Callback to collect property names.
    pub get_property_names: Option<GetPropertyNamesCallback>,
    
    /// Callback to call an object as a function.
    pub call_as_function: Option<CallAsFunctionCallback>,
    
    /// Callback to call an object as a constructor.
    pub call_as_constructor: Option<CallAsConstructorCallback>,
    
    /// Callback to determine if an object is an instance of a constructor.
    pub has_instance: Option<HasInstanceCallback>,
    
    /// Callback to convert an object to a primitive type.
    pub convert_to_type: Option<ConvertToTypeCallback>,
}

impl Default for ClassDefinition {
    fn default() -> Self {
        ClassDefinition {
            class_name: String::new(""),
            attributes: ClassAttributes::NONE,
            parent_class: None,
            static_values: Vec::new(),
            static_functions: Vec::new(),
            initialize: None,
            finalize: None,
            has_property: None,
            get_property: None,
            set_property: None,
            delete_property: None,
            get_property_names: None,
            call_as_function: None,
            call_as_constructor: None,
            has_instance: None,
            convert_to_type: None,
        }
    }
}

// Storage for callback data and destructors
struct ClassCallbackData {
    callbacks: Box<ClassCallbacks>,
}

struct ClassCallbacks {
    initialize: Option<InitializeCallback>,
    finalize: Option<FinalizeCallback>,
    has_property: Option<HasPropertyCallback>,
    get_property: Option<GetPropertyCallback>,
    set_property: Option<SetPropertyCallback>,
    delete_property: Option<DeletePropertyCallback>,
    get_property_names: Option<GetPropertyNamesCallback>,
    call_as_function: Option<CallAsFunctionCallback>,
    call_as_constructor: Option<CallAsConstructorCallback>,
    has_instance: Option<HasInstanceCallback>,
    convert_to_type: Option<ConvertToTypeCallback>,
    static_values: Vec<(CString, Option<GetPropertyCallback>, Option<SetPropertyCallback>)>,
    static_functions: Vec<(CString, CallAsFunctionCallback)>,
}

// C callback implementations
extern "C" fn initialize_callback(ctx: ffi::JSContextRef, object: ffi::JSObjectRef) {
    unsafe {
        let data = ffi::JSObjectGetPrivate(object) as *mut ClassCallbackData;
        if !data.is_null() {
            let data = &*data;
            if let Some(ref callback) = data.callbacks.initialize {
                let context = Context::from_raw(ctx);
                let obj = Object::from_raw(context, object);
                callback(&context, &obj);
            }
        }
    }
}

extern "C" fn finalize_callback(object: ffi::JSObjectRef) {
    unsafe {
        let data = ffi::JSObjectGetPrivate(object) as *mut ClassCallbackData;
        if !data.is_null() {
            // Call the finalize callback if it exists
            let data_ref = &*data;
            if let Some(ref callback) = data_ref.callbacks.finalize {
                // Create a temporary object without a context for the callback
                // Note: This is safe because a finalize callback should not access the context
                let obj = Object::from_raw_no_context(object);
                callback(&obj);
            }
            
            // Free the callback data
            Box::from_raw(data);
        }
    }
}

extern "C" fn has_property_callback(ctx: ffi::JSContextRef, object: ffi::JSObjectRef, property_name: ffi::JSStringRef) -> bool {
    unsafe {
        let data = ffi::JSObjectGetPrivate(object) as *mut ClassCallbackData;
        if !data.is_null() {
            let data = &*data;
            if let Some(ref callback) = data.callbacks.has_property {
                let context = Context::from_raw(ctx);
                let obj = Object::from_raw(context, object);
                let name = String::from_raw(property_name);
                
                return callback(&context, &obj, &name);
            }
        }
        false
    }
}

extern "C" fn get_property_callback(ctx: ffi::JSContextRef, object: ffi::JSObjectRef, property_name: ffi::JSStringRef, exception: *mut ffi::JSValueRef) -> ffi::JSValueRef {
    unsafe {
        let data = ffi::JSObjectGetPrivate(object) as *mut ClassCallbackData;
        if !data.is_null() {
            let data = &*data;
            if let Some(ref callback) = data.callbacks.get_property {
                let context = Context::from_raw(ctx);
                let obj = Object::from_raw(context, object);
                let name = String::from_raw(property_name);
                
                match callback(&context, &obj, &name) {
                    Ok(value) => return value.as_raw(),
                    Err(err) => {
                        if !exception.is_null() {
                            *exception = Value::from_error(&context, &err).as_raw();
                        }
                        return ptr::null();
                    }
                }
            }
        }
        ptr::null()
    }
}

extern "C" fn set_property_callback(ctx: ffi::JSContextRef, object: ffi::JSObjectRef, property_name: ffi::JSStringRef, value: ffi::JSValueRef, exception: *mut ffi::JSValueRef) -> bool {
    unsafe {
        let data = ffi::JSObjectGetPrivate(object) as *mut ClassCallbackData;
        if !data.is_null() {
            let data = &*data;
            if let Some(ref callback) = data.callbacks.set_property {
                let context = Context::from_raw(ctx);
                let obj = Object::from_raw(context, object);
                let name = String::from_raw(property_name);
                let val = Value::from_raw(context, value);
                
                match callback(&context, &obj, &name, val) {
                    Ok(result) => return result,
                    Err(err) => {
                        if !exception.is_null() {
                            *exception = Value::from_error(&context, &err).as_raw();
                        }
                        return false;
                    }
                }
            }
        }
        false
    }
}

extern "C" fn delete_property_callback(ctx: ffi::JSContextRef, object: ffi::JSObjectRef, property_name: ffi::JSStringRef, exception: *mut ffi::JSValueRef) -> bool {
    unsafe {
        let data = ffi::JSObjectGetPrivate(object) as *mut ClassCallbackData;
        if !data.is_null() {
            let data = &*data;
            if let Some(ref callback) = data.callbacks.delete_property {
                let context = Context::from_raw(ctx);
                let obj = Object::from_raw(context, object);
                let name = String::from_raw(property_name);
                
                match callback(&context, &obj, &name) {
                    Ok(result) => return result,
                    Err(err) => {
                        if !exception.is_null() {
                            *exception = Value::from_error(&context, &err).as_raw();
                        }
                        return false;
                    }
                }
            }
        }
        false
    }
}

extern "C" fn get_property_names_callback(ctx: ffi::JSContextRef, object: ffi::JSObjectRef, property_names: ffi::JSPropertyNameAccumulatorRef) {
    unsafe {
        let data = ffi::JSObjectGetPrivate(object) as *mut ClassCallbackData;
        if !data.is_null() {
            let data = &*data;
            if let Some(ref callback) = data.callbacks.get_property_names {
                let context = Context::from_raw(ctx);
                let obj = Object::from_raw(context, object);
                let mut names = Vec::new();
                
                callback(&context, &obj, &mut names);
                
                for name in names {
                    ffi::JSPropertyNameAccumulatorAddName(property_names, name.as_raw());
                }
            }
        }
    }
}

extern "C" fn call_as_function_callback(ctx: ffi::JSContextRef, function: ffi::JSObjectRef, this_object: ffi::JSObjectRef, argument_count: usize, arguments: *const ffi::JSValueRef, exception: *mut ffi::JSValueRef) -> ffi::JSValueRef {
    unsafe {
        let data = ffi::JSObjectGetPrivate(function) as *mut ClassCallbackData;
        if !data.is_null() {
            let data = &*data;
            if let Some(ref callback) = data.callbacks.call_as_function {
                let context = Context::from_raw(ctx);
                let func = Object::from_raw(context, function);
                let this = if this_object.is_null() {
                    None
                } else {
                    Some(Object::from_raw(context, this_object))
                };
                
                let args = if argument_count == 0 || arguments.is_null() {
                    Vec::new()
                } else {
                    let args_slice = std::slice::from_raw_parts(arguments, argument_count);
                    args_slice.iter()
                        .map(|&arg| Value::from_raw(context, arg))
                        .collect()
                };
                
                match callback(&context, &func, this.as_ref(), &args) {
                    Ok(result) => return result.as_raw(),
                    Err(err) => {
                        if !exception.is_null() {
                            *exception = Value::from_error(&context, &err).as_raw();
                        }
                        return ptr::null();
                    }
                }
            }
        }
        ptr::null()
    }
}

extern "C" fn call_as_constructor_callback(ctx: ffi::JSContextRef, constructor: ffi::JSObjectRef, argument_count: usize, arguments: *const ffi::JSValueRef, exception: *mut ffi::JSValueRef) -> ffi::JSObjectRef {
    unsafe {
        let data = ffi::JSObjectGetPrivate(constructor) as *mut ClassCallbackData;
        if !data.is_null() {
            let data = &*data;
            if let Some(ref callback) = data.callbacks.call_as_constructor {
                let context = Context::from_raw(ctx);
                let ctor = Object::from_raw(context, constructor);
                
                let args = if argument_count == 0 || arguments.is_null() {
                    Vec::new()
                } else {
                    let args_slice = std::slice::from_raw_parts(arguments, argument_count);
                    args_slice.iter()
                        .map(|&arg| Value::from_raw(context, arg))
                        .collect()
                };
                
                match callback(&context, &ctor, &args) {
                    Ok(result) => return result.as_raw(),
                    Err(err) => {
                        if !exception.is_null() {
                            *exception = Value::from_error(&context, &err).as_raw();
                        }
                        return ptr::null_mut();
                    }
                }
            }
        }
        ptr::null_mut()
    }
}

extern "C" fn has_instance_callback(ctx: ffi::JSContextRef, constructor: ffi::JSObjectRef, possible_instance: ffi::JSValueRef, exception: *mut ffi::JSValueRef) -> bool {
    unsafe {
        let data = ffi::JSObjectGetPrivate(constructor) as *mut ClassCallbackData;
        if !data.is_null() {
            let data = &*data;
            if let Some(ref callback) = data.callbacks.has_instance {
                let context = Context::from_raw(ctx);
                let ctor = Object::from_raw(context, constructor);
                let instance = Value::from_raw(context, possible_instance);
                
                match callback(&context, &ctor, &instance) {
                    Ok(result) => return result,
                    Err(err) => {
                        if !exception.is_null() {
                            *exception = Value::from_error(&context, &err).as_raw();
                        }
                        return false;
                    }
                }
            }
        }
        false
    }
}

extern "C" fn convert_to_type_callback(ctx: ffi::JSContextRef, object: ffi::JSObjectRef, type_: ffi::JSType, exception: *mut ffi::JSValueRef) -> ffi::JSValueRef {
    unsafe {
        let data = ffi::JSObjectGetPrivate(object) as *mut ClassCallbackData;
        if !data.is_null() {
            let data = &*data;
            if let Some(ref callback) = data.callbacks.convert_to_type {
                let context = Context::from_raw(ctx);
                let obj = Object::from_raw(context, object);
                
                match callback(&context, &obj, type_) {
                    Ok(result) => return result.as_raw(),
                    Err(err) => {
                        if !exception.is_null() {
                            *exception = Value::from_error(&context, &err).as_raw();
                        }
                        return ptr::null();
                    }
                }
            }
        }
        ptr::null()
    }
}

extern "C" fn static_value_getter(ctx: ffi::JSContextRef, object: ffi::JSObjectRef, property_name: ffi::JSStringRef, exception: *mut ffi::JSValueRef) -> ffi::JSValueRef {
    unsafe {
        let data = ffi::JSObjectGetPrivate(object) as *mut ClassCallbackData;
        if !data.is_null() {
            let data = &*data;
            let name = String::from_raw(property_name);
            
            for (stored_name, getter, _) in &data.callbacks.static_values {
                let stored_name_str = String::from_utf8_buffer(CStr::from_ptr(stored_name.as_ptr()).to_bytes());
                if name == stored_name_str {
                    if let Some(ref getter_fn) = getter {
                        let context = Context::from_raw(ctx);
                        let obj = Object::from_raw(context, object);
                        
                        match getter_fn(&context, &obj, &name) {
                            Ok(value) => return value.as_raw(),
                            Err(err) => {
                                if !exception.is_null() {
                                    *exception = Value::from_error(&context, &err).as_raw();
                                }
                                return ptr::null();
                            }
                        }
                    }
                    break;
                }
            }
        }
        ptr::null()
    }
}

extern "C" fn static_value_setter(ctx: ffi::JSContextRef, object: ffi::JSObjectRef, property_name: ffi::JSStringRef, value: ffi::JSValueRef, exception: *mut ffi::JSValueRef) -> bool {
    unsafe {
        let data = ffi::JSObjectGetPrivate(object) as *mut ClassCallbackData;
        if !data.is_null() {
            let data = &*data;
            let name = String::from_raw(property_name);
            
            for (stored_name, _, setter) in &data.callbacks.static_values {
                let stored_name_str = String::from_utf8_buffer(CStr::from_ptr(stored_name.as_ptr()).to_bytes());
                if name == stored_name_str {
                    if let Some(ref setter_fn) = setter {
                        let context = Context::from_raw(ctx);
                        let obj = Object::from_raw(context, object);
                        let val = Value::from_raw(context, value);
                        
                        match setter_fn(&context, &obj, &name, val) {
                            Ok(result) => return result,
                            Err(err) => {
                                if !exception.is_null() {
                                    *exception = Value::from_error(&context, &err).as_raw();
                                }
                                return false;
                            }
                        }
                    }
                    break;
                }
            }
        }
        false
    }
}

extern "C" fn static_function_callback(ctx: ffi::JSContextRef, function: ffi::JSObjectRef, this_object: ffi::JSObjectRef, argument_count: usize, arguments: *const ffi::JSValueRef, exception: *mut ffi::JSValueRef) -> ffi::JSValueRef {
    unsafe {
        // Get the function name from the function object
        let function_name_prop = String::new("name");
        let mut exc = ptr::null();
        let name_value = ffi::JSObjectGetProperty(ctx, function, function_name_prop.as_raw(), &mut exc);
        
        if exc.is_null() && !name_value.is_null() {
            let mut str_exc = ptr::null();
            let name_str = ffi::JSValueToStringCopy(ctx, name_value, &mut str_exc);
            
            if str_exc.is_null() && !name_str.is_null() {
                let name = String::from_raw(name_str);
                
                // Get the class data from the this object
                let data = ffi::JSObjectGetPrivate(this_object) as *mut ClassCallbackData;
                if !data.is_null() {
                    let data = &*data;
                    
                    // Find the function by name
                    for (stored_name, callback) in &data.callbacks.static_functions {
                        let stored_name_str = String::from_utf8_buffer(CStr::from_ptr(stored_name.as_ptr()).to_bytes());
                        if name == stored_name_str {
                            let context = Context::from_raw(ctx);
                            let func = Object::from_raw(context, function);
                            let this = Object::from_raw(context, this_object);
                            
                            let args = if argument_count == 0 || arguments.is_null() {
                                Vec::new()
                            } else {
                                let args_slice = std::slice::from_raw_parts(arguments, argument_count);
                                args_slice.iter()
                                    .map(|&arg| Value::from_raw(context, arg))
                                    .collect()
                            };
                            
                            match callback(&context, &func, Some(&this), &args) {
                                Ok(result) => return result.as_raw(),
                                Err(err) => {
                                    if !exception.is_null() {
                                        *exception = Value::from_error(&context, &err).as_raw();
                                    }
                                    return ptr::null();
                                }
                            }
                        }
                    }
                }
            }
        }
        
        // Function not found or error
        ptr::null()
    }
}

/// A JavaScript class.
pub struct Class {
    raw: ffi::JSClassRef,
}

impl Class {
    /// Create a new JavaScript class definition.
    pub fn new(definition: ClassDefinition) -> Result<Self> {
        // Create callback data
        let callbacks = Box::new(ClassCallbacks {
            initialize: definition.initialize,
            finalize: definition.finalize,
            has_property: definition.has_property,
            get_property: definition.get_property,
            set_property: definition.set_property,
            delete_property: definition.delete_property,
            get_property_names: definition.get_property_names,
            call_as_function: definition.call_as_function,
            call_as_constructor: definition.call_as_constructor,
            has_instance: definition.has_instance,
            convert_to_type: definition.convert_to_type,
            static_values: definition.static_values.into_iter().map(|v| {
                (
                    CString::new(v.name.to_string()).unwrap(),
                    v.getter,
                    v.setter,
                )
            }).collect(),
            static_functions: definition.static_functions.into_iter().map(|f| {
                (
                    CString::new(f.name.to_string()).unwrap(),
                    f.callback,
                )
            }).collect(),
        });
        let callback_data = Box::new(ClassCallbackData {
            callbacks,
        });
        
        // Create the static values and functions arrays for the JSClassDefinition
        let mut static_values: Vec<ffi::JSStaticValue> = Vec::new();
        for (name, has_getter, has_setter) in &callback_data.callbacks.static_values {
            static_values.push(ffi::JSStaticValue {
                name: name.as_ptr(),
                getProperty: if has_getter.is_some() { Some(static_value_getter) } else { None },
                setProperty: if has_setter.is_some() { Some(static_value_setter) } else { None },
                attributes: if has_setter.is_none() { ffi::kJSPropertyAttributeReadOnly } else { ffi::kJSPropertyAttributeNone },
            });
        }
        static_values.push(ffi::JSStaticValue {
            name: ptr::null(),
            getProperty: None,
            setProperty: None,
            attributes: 0,
        });
        
        let mut static_functions: Vec<ffi::JSStaticFunction> = Vec::new();
        for (name, _) in &callback_data.callbacks.static_functions {
            static_functions.push(ffi::JSStaticFunction {
                name: name.as_ptr(),
                callAsFunction: Some(static_function_callback),
                attributes: ffi::kJSPropertyAttributeNone,
            });
        }
        static_functions.push(ffi::JSStaticFunction {
            name: ptr::null(),
            callAsFunction: None,
            attributes: 0,
        });
        
        // Convert the class name to a CString
        let class_name = CString::new(definition.class_name.to_string())
            .map_err(|_| Error::InvalidParameter("Class name contains null bytes"))?;
        
        // Create the JSClassDefinition
        let mut def = ffi::JSClassDefinition {
            version: 0,
            attributes: definition.attributes.as_raw(),
            className: class_name.as_ptr(),
            parentClass: definition.parent_class.map_or(ptr::null_mut(), |c| c.raw),
            staticValues: if static_values.len() > 1 { static_values.as_ptr() } else { ptr::null() },
            staticFunctions: if static_functions.len() > 1 { static_functions.as_ptr() } else { ptr::null() },
            initialize: if definition.initialize.is_some() { Some(initialize_callback) } else { None },
            finalize: if definition.finalize.is_some() { Some(finalize_callback) } else { None },
            hasProperty: if definition.has_property.is_some() { Some(has_property_callback) } else { None },
            getProperty: if definition.get_property.is_some() { Some(get_property_callback) } else { None },
            setProperty: if definition.set_property.is_some() { Some(set_property_callback) } else { None },
            deleteProperty: if definition.delete_property.is_some() { Some(delete_property_callback) } else { None },
            getPropertyNames: if definition.get_property_names.is_some() { Some(get_property_names_callback) } else { None },
            callAsFunction: if definition.call_as_function.is_some() { Some(call_as_function_callback) } else { None },
            callAsConstructor: if definition.call_as_constructor.is_some() { Some(call_as_constructor_callback) } else { None },
            hasInstance: if definition.has_instance.is_some() { Some(has_instance_callback) } else { None },
            convertToType: if definition.convert_to_type.is_some() { Some(convert_to_type_callback) } else { None },
        };
        
        // Create the JS class
        let raw = unsafe { ffi::JSClassCreate(&def) };
        
        if raw.is_null() {
            return Err(Error::JSError("Failed to create JavaScript class".to_string()));
        }
        
        // Store the callback data in a Box that will be leaked and later freed in the finalize callback
        let leaked_data = Box::into_raw(callback_data);
        
        // We need to store the callback data somewhere associated with the class
        // In a real implementation, we would maintain a global registry of class data
        
        Ok(Class { raw })
    }
    
    /// Create a new class from a raw JSClassRef.
    pub(crate) unsafe fn from_raw(raw: ffi::JSClassRef) -> Self {
        Class { raw }
    }
    
    /// Get a reference to the raw JSClassRef.
    pub(crate) fn as_raw(&self) -> ffi::JSClassRef {
        self.raw
    }
}

impl Drop for Class {
    fn drop(&mut self) {
        unsafe {
            ffi::JSClassRelease(self.raw);
        }
    }
}

impl Clone for Class {
    fn clone(&self) -> Self {
        unsafe {
            let raw = ffi::JSClassRetain(self.raw);
            Class { raw }
        }
    }
}

/// A JavaScript object.
pub struct Object<'a> {
    pub(crate) context: Context<'a>,
    raw: ffi::JSObjectRef,
}

impl<'a> Object<'a> {
    /// Create a new empty JavaScript object.
    pub fn new(context: &Context<'a>) -> Self {
        unsafe {
            let raw = ffi::JSObjectMake(context.as_raw(), ptr::null_mut(), ptr::null_mut());
            Object {
                context: context.clone(),
                raw,
            }
        }
    }
    
    /// Create a new JavaScript object with a specific class.
    pub fn with_class(context: &Context<'a>, class: &Class, private_data: Option<*mut c_void>) -> Self {
        unsafe {
            let raw = ffi::JSObjectMake(
                context.as_raw(),
                class.as_raw(),
                private_data.unwrap_or(ptr::null_mut()),
            );
            Object {
                context: context.clone(),
                raw,
            }
        }
    }
    
    /// Create a JavaScript array.
    pub fn array(context: &Context<'a>, values: &[Value]) -> Result<Self> {
        unsafe {
            let mut exception = ptr::null();
            let args: Vec<ffi::JSValueRef> = values.iter().map(|v| v.as_raw()).collect();
            
            let raw = ffi::JSObjectMakeArray(
                context.as_raw(),
                args.len(),
                args.as_ptr(),
                &mut exception,
            );
            
            if !exception.is_null() {
                return Err(Error::from_js_exception(context.as_raw(), exception));
            }
            
            Ok(Object {
                context: context.clone(),
                raw,
            })
        }
    }
    
    /// Create a JavaScript date.
    pub fn date(context: &Context<'a>, timestamp: f64) -> Result<Self> {
        unsafe {
            let mut exception = ptr::null();
            let timestamp_val = Value::number(context, timestamp);
            let args = [timestamp_val.as_raw()];
            
            let raw = ffi::JSObjectMakeDate(
                context.as_raw(),
                1,
                args.as_ptr(),
                &mut exception,
            );
            
            if !exception.is_null() {
                return Err(Error::from_js_exception(context.as_raw(), exception));
            }
            
            Ok(Object {
                context: context.clone(),
                raw,
            })
        }
    }
    
    /// Create a JavaScript error.
    pub fn error(context: &Context<'a>, message: &str) -> Result<Self> {
        unsafe {
            let mut exception = ptr::null();
            let message_val = Value::string(context, message);
            let args = [message_val.as_raw()];
            
            let raw = ffi::JSObjectMakeError(
                context.as_raw(),
                1,
                args.as_ptr(),
                &mut exception,
            );
            
            if !exception.is_null() {
                return Err(Error::from_js_exception(context.as_raw(), exception));
            }
            
            Ok(Object {
                context: context.clone(),
                raw,
            })
        }
    }
    
    /// Create a JavaScript regular expression.
    pub fn regexp(context: &Context<'a>, pattern: &str, flags: &str) -> Result<Self> {
        unsafe {
            let mut exception = ptr::null();
            let pattern_val = Value::string(context, pattern);
            let flags_val = Value::string(context, flags);
            let args = [pattern_val.as_raw(), flags_val.as_raw()];
            
            let raw = ffi::JSObjectMakeRegExp(
                context.as_raw(),
                2,
                args.as_ptr(),
                &mut exception,
            );
            
            if !exception.is_null() {
                return Err(Error::from_js_exception(context.as_raw(), exception));
            }
            
            Ok(Object {
                context: context.clone(),
                raw,
            })
        }
    }
    
    /// Create a JavaScript function.
    pub fn function(context: &Context<'a>, name: Option<&str>, parameters: &[&str], body: &str, source_url: Option<&str>, starting_line: i32) -> Result<Self> {
        unsafe {
            let mut exception = ptr::null();
            
            let name_string = name.map(|n| String::new(n));
            let body_string = String::new(body);
            let source_url_string = source_url.map(|url| String::new(url));
            
            let param_strings: Vec<String> = parameters.iter().map(|&p| String::new(p)).collect();
            let param_ptrs: Vec<ffi::JSStringRef> = param_strings.iter().map(|s| s.as_raw()).collect();
            
            let raw = ffi::JSObjectMakeFunction(
                context.as_raw(),
                name_string.as_ref().map_or(ptr::null_mut(), |s| s.as_raw()),
                param_strings.len() as u32,
                if param_strings.is_empty() { ptr::null() } else { param_ptrs.as_ptr() },
                body_string.as_raw(),
                source_url_string.as_ref().map_or(ptr::null_mut(), |s| s.as_raw()),
                starting_line,
                &mut exception,
            );
            
            if !exception.is_null() {
                return Err(Error::from_js_exception(context.as_raw(), exception));
            }
            
            if raw.is_null() {
                return Err(Error::JSError("Failed to create function".to_string()));
            }
            
            Ok(Object {
                context: context.clone(),
                raw,
            })
        }
    }
    
    /// Create a JavaScript function with a callback.
    pub fn function_with_callback<F>(context: &Context<'a>, name: Option<&str>, callback: F) -> Self
    where
        F: Fn(&Context, &Object, Option<&Object>, &[Value]) -> Result<Value> + 'static,
    {
        unsafe {
            let callback_box: Box<dyn Fn(&Context, &Object, Option<&Object>, &[Value]) -> Result<Value>> = Box::new(callback);
            let callback_ptr = Box::into_raw(Box::new(callback_box));
            
            extern "C" fn trampoline(
                ctx: ffi::JSContextRef,
                function: ffi::JSObjectRef,
                this_object: ffi::JSObjectRef,
                argument_count: usize,
                arguments: *const ffi::JSValueRef,
                exception: *mut ffi::JSValueRef,
            ) -> ffi::JSValueRef {
                unsafe {
                    let context = Context::from_raw(ctx);
                    let func = Object::from_raw(context, function);
                    let this = if this_object.is_null() {
                        None
                    } else {
                        Some(Object::from_raw(context, this_object))
                    };
                    
                    let callback_ptr = ffi::JSObjectGetPrivate(function) as *mut Box<dyn Fn(&Context, &Object, Option<&Object>, &[Value]) -> Result<Value>>;
                    let callback = &**callback_ptr;
                    
                    let args = if argument_count == 0 || arguments.is_null() {
                        Vec::new()
                    } else {
                        let args_slice = std::slice::from_raw_parts(arguments, argument_count);
                        args_slice.iter()
                            .map(|&arg| Value::from_raw(context, arg))
                            .collect::<Vec<_>>()
                    };
                    
                    match callback(&context, &func, this.as_ref(), &args) {
                        Ok(result) => result.as_raw(),
                        Err(err) => {
                            if !exception.is_null() {
                                let err_val = Value::from_error(&context, &err);
                                *exception = err_val.as_raw();
                            }
                            ptr::null()
                        }
                    }
                }
            }
            
            extern "C" fn finalize(object: ffi::JSObjectRef) {
                unsafe {
                    let callback_ptr = ffi::JSObjectGetPrivate(object) as *mut Box<dyn Fn(&Context, &Object, Option<&Object>, &[Value]) -> Result<Value>>;
                    if !callback_ptr.is_null() {
                        drop(Box::from_raw(callback_ptr));
                    }
                }
            }
            
            // Create a class for the function object
            let class_definition = ffi::JSClassDefinition {
                version: 0,
                attributes: 0,
                className: b"RustFunctionCallback\0".as_ptr() as *const c_char,
                parentClass: ptr::null_mut(),
                staticValues: ptr::null(),
                staticFunctions: ptr::null(),
                initialize: None,
                finalize: Some(finalize),
                hasProperty: None,
                getProperty: None,
                setProperty: None,
                deleteProperty: None,
                getPropertyNames: None,
                callAsFunction: Some(trampoline),
                callAsConstructor: None,
                hasInstance: None,
                convertToType: None,
            };
            
            let class = ffi::JSClassCreate(&class_definition);
            
            let name_string = name.map(|n| String::new(n));
            
            let raw = ffi::JSObjectMakeFunctionWithCallback(
                context.as_raw(),
                name_string.as_ref().map_or(ptr::null_mut(), |s| s.as_raw()),
                Some(trampoline),
            );
            
            // Set the callback as private data on the function object
            ffi::JSObjectSetPrivate(raw, callback_ptr as *mut c_void);
            
            // Release the class since we don't need it anymore
            ffi::JSClassRelease(class);
            
            Object {
                context: context.clone(),
                raw,
            }
        }
    }
    
    /// Create a Promise object.
    pub fn promise(context: &Context<'a>) -> Result<(Self, Self, Self)> {
        unsafe {
            let mut resolve = ptr::null_mut();
            let mut reject = ptr::null_mut();
            let mut exception = ptr::null();
            
            let promise_raw = ffi::JSObjectMakeDeferredPromise(
                context.as_raw(),
                &mut resolve,
                &mut reject,
                &mut exception,
            );
            
            if !exception.is_null() {
                return Err(Error::from_js_exception(context.as_raw(), exception));
            }
            
            if promise_raw.is_null() || resolve.is_null() || reject.is_null() {
                return Err(Error::JSError("Failed to create Promise".to_string()));
            }
            
            Ok((
                Object { context: context.clone(), raw: promise_raw },
                Object { context: context.clone(), raw: resolve },
                Object { context: context.clone(), raw: reject },
            ))
        }
    }
    
    /// Create an Object from a raw JSObjectRef.
    pub(crate) fn from_raw(context: Context<'a>, raw: ffi::JSObjectRef) -> Self {
        Object { context, raw }
    }
    
    /// Create an Object from a raw JSObjectRef without a context.
    /// This should only be used in finalize callbacks.
    unsafe fn from_raw_no_context(raw: ffi::JSObjectRef) -> Self {
        Object {
            context: Context::dummy(),
            raw,
        }
    }
    
    /// Convert a Value to an Object if possible.
    pub fn from_value(value: Value<'a>) -> Result<Self> {
        if !value.is_object() {
            return Err(Error::InvalidType("Value is not an object".to_string()));
        }
        
        unsafe {
            let mut exception = ptr::null();
            let obj = ffi::JSValueToObject(value.context().as_raw(), value.as_raw(), &mut exception);
            
            if !exception.is_null() {
                return Err(Error::from_js_exception(value.context().as_raw(), exception));
            }
            
            if obj.is_null() {
                return Err(Error::JSError("Failed to convert value to object".to_string()));
            }
            
            Ok(Object {
                context: value.context().clone(),
                raw: obj,
            })
        }
    }
    
    /// Get the raw JSObjectRef pointer.
    pub(crate) fn as_raw(&self) -> ffi::JSObjectRef {
        self.raw
    }
    
    /// Get the associated Context.
    pub fn context(&self) -> &Context<'a> {
        &self.context
    }
    
    /// Convert this Object to a Value.
    pub fn to_value(&self) -> Value<'a> {
        Value::from_raw(&self.context, self.raw as ffi::JSValueRef)
    }
    
    /// Get the prototype of this object.
    pub fn get_prototype(&self) -> Value<'a> {
        unsafe {
            let proto = ffi::JSObjectGetPrototype(self.context.as_raw(), self.raw);
            Value::from_raw(&self.context, proto)
        }
    }
    
    /// Set the prototype of this object.
    pub fn set_prototype(&self, prototype: Value<'a>) {
        unsafe {
            ffi::JSObjectSetPrototype(self.context.as_raw(), self.raw, prototype.as_raw());
        }
    }
    
    /// Check if this object has a property with the given name.
    pub fn has_property(&self, name: &str) -> bool {
        let name_string = String::new(name);
        unsafe {
            ffi::JSObjectHasProperty(self.context.as_raw(), self.raw, name_string.as_raw())
        }
    }
    
    /// Get a property value by name.
    pub fn get_property(&self, name: &str) -> Result<Value<'a>> {
        let name_string = String::new(name);
        unsafe {
            let mut exception = ptr::null();
            let result = ffi::JSObjectGetProperty(
                self.context.as_raw(),
                self.raw,
                name_string.as_raw(),
                &mut exception,
            );
            
            if !exception.is_null() {
                return Err(Error::from_js_exception(self.context.as_raw(), exception));
            }
            
            Ok(Value::from_raw(&self.context, result))
        }
    }
    
    /// Set a property value by name.
    pub fn set_property(&self, name: &str, value: Value<'a>, attributes: PropertyAttributes) -> Result<()> {
        let name_string = String::new(name);
        unsafe {
            let mut exception = ptr::null();
            ffi::JSObjectSetProperty(
                self.context.as_raw(),
                self.raw,
                name_string.as_raw(),
                value.as_raw(),
                attributes.as_raw(),
                &mut exception,
            );
            
            if !exception.is_null() {
                return Err(Error::from_js_exception(self.context.as_raw(), exception));
            }
            
            Ok(())
        }
    }
    
    /// Delete a property by name.
    pub fn delete_property(&self, name: &str) -> Result<bool> {
        let name_string = String::new(name);
        unsafe {
            let mut exception = ptr::null();
            let result = ffi::JSObjectDeleteProperty(
                self.context.as_raw(),
                self.raw,
                name_string.as_raw(),
                &mut exception,
            );
            
            if !exception.is_null() {
                return Err(Error::from_js_exception(self.context.as_raw(), exception));
            }
            
            Ok(result)
        }
    }
    
    /// Get a property value by numeric index.
    pub fn get_property_at_index(&self, index: u32) -> Result<Value<'a>> {
        unsafe {
            let mut exception = ptr::null();
            let result = ffi::JSObjectGetPropertyAtIndex(
                self.context.as_raw(),
                self.raw,
                index,
                &mut exception,
            );
            
            if !exception.is_null() {
                return Err(Error::from_js_exception(self.context.as_raw(), exception));
            }
            
            Ok(Value::from_raw(&self.context, result))
        }
    }
    
    /// Set a property value by numeric index.
    pub fn set_property_at_index(&self, index: u32, value: Value<'a>) -> Result<()> {
        unsafe {
            let mut exception = ptr::null();
            ffi::JSObjectSetPropertyAtIndex(
                self.context.as_raw(),
                self.raw,
                index,
                value.as_raw(),
                &mut exception,
            );
            
            if !exception.is_null() {
                return Err(Error::from_js_exception(self.context.as_raw(), exception));
            }
            
            Ok(())
        }
    }
    
    /// Get a property value by key.
    pub fn get_property_for_key(&self, key: Value<'a>) -> Result<Value<'a>> {
        unsafe {
            let mut exception = ptr::null();
            let result = ffi::JSObjectGetPropertyForKey(
                self.context.as_raw(),
                self.raw,
                key.as_raw(),
                &mut exception,
            );
            
            if !exception.is_null() {
                return Err(Error::from_js_exception(self.context.as_raw(), exception));
            }
            
            Ok(Value::from_raw(&self.context, result))
        }
    }
    
    /// Set a property value by key.
    pub fn set_property_for_key(&self, key: Value<'a>, value: Value<'a>, attributes: PropertyAttributes) -> Result<()> {
        unsafe {
            let mut exception = ptr::null();
            ffi::JSObjectSetPropertyForKey(
                self.context.as_raw(),
                self.raw,
                key.as_raw(),
                value.as_raw(),
                attributes.as_raw(),
                &mut exception,
            );
            
            if !exception.is_null() {
                return Err(Error::from_js_exception(self.context.as_raw(), exception));
            }
            
            Ok(())
        }
    }
    
    /// Delete a property by key.
    pub fn delete_property_for_key(&self, key: Value<'a>) -> Result<bool> {
        unsafe {
            let mut exception = ptr::null();
            let result = ffi::JSObjectDeletePropertyForKey(
                self.context.as_raw(),
                self.raw,
                key.as_raw(),
                &mut exception,
            );
            
            if !exception.is_null() {
                return Err(Error::from_js_exception(self.context.as_raw(), exception));
            }
            
            Ok(result)
        }
    }
    
    /// Get the private data associated with this object.
    pub fn get_private(&self) -> *mut c_void {
        unsafe {
            ffi::JSObjectGetPrivate(self.raw)
        }
    }
    
    /// Set private data on this object.
    pub fn set_private(&self, data: *mut c_void) -> bool {
        unsafe {
            ffi::JSObjectSetPrivate(self.raw, data)
        }
    }
    
    /// Set a private property on this object.
    /// Private properties cannot be accessed from JavaScript.
    pub fn set_private_property(&self, name: &str, value: Option<Value<'a>>) -> Result<bool> {
        let name_string = String::new(name);
        unsafe {
            let result = ffi::JSObjectSetPrivateProperty(
                self.context.as_raw(),
                self.raw,
                name_string.as_raw(),
                value.map_or(ptr::null(), |v| v.as_raw()),
            );
            
            Ok(result)
        }
    }
    
    /// Get a private property from this object.
    pub fn get_private_property(&self, name: &str) -> Result<Option<Value<'a>>> {
        let name_string = String::new(name);
        unsafe {
            let result = ffi::JSObjectGetPrivateProperty(
                self.context.as_raw(),
                self.raw,
                name_string.as_raw(),
            );
            
            if result.is_null() {
                Ok(None)
            } else {
                Ok(Some(Value::from_raw(&self.context, result)))
            }
        }
    }
    
    /// Delete a private property from this object.
    pub fn delete_private_property(&self, name: &str) -> Result<bool> {
        let name_string = String::new(name);
        unsafe {
            let result = ffi::JSObjectDeletePrivateProperty(
                self.context.as_raw(),
                self.raw,
                name_string.as_raw(),
            );
            
            Ok(result)
        }
    }
    
    /// Get all property names of this object.
    pub fn get_property_names(&self) -> Result<Vec<String>> {
        unsafe {
            let names_array = ffi::JSObjectCopyPropertyNames(self.context.as_raw(), self.raw);
            if names_array.is_null() {
                return Err(Error::JSError("Failed to get property names".to_string()));
            }
            
            let count = ffi::JSPropertyNameArrayGetCount(names_array);
            let mut result = Vec::with_capacity(count);
            
            for i in 0..count {
                let name = ffi::JSPropertyNameArrayGetNameAtIndex(names_array, i);
                result.push(String::from_raw(name));
            }
            
            ffi::JSPropertyNameArrayRelease(names_array);
            
            Ok(result)
        }
    }
    
    /// Check if this object is a function.
    pub fn is_function(&self) -> bool {
        unsafe {
            ffi::JSObjectIsFunction(self.context.as_raw(), self.raw)
        }
    }
    
    /// Call this object as a function.
    pub fn call(&self, this_object: Option<&Object<'a>>, arguments: &[Value<'a>]) -> Result<Value<'a>> {
        if !self.is_function() {
            return Err(Error::InvalidType("Object is not a function".to_string()));
        }
        
        unsafe {
            let mut exception = ptr::null();
            let this_obj = this_object.map_or(ptr::null_mut(), |o| o.raw);
            
            let args: Vec<ffi::JSValueRef> = arguments.iter().map(|v| v.as_raw()).collect();
            
            let result = ffi::JSObjectCallAsFunction(
                self.context.as_raw(),
                self.raw,
                this_obj,
                args.len(),
                if args.is_empty() { ptr::null() } else { args.as_ptr() },
                &mut exception,
            );
            
            if !exception.is_null() {
                return Err(Error::from_js_exception(self.context.as_raw(), exception));
            }
            
            if result.is_null() {
                return Err(Error::JSError("Function call failed".to_string()));
            }
            
            Ok(Value::from_raw(&self.context, result))
        }
    }
    
    /// Check if this object is a constructor.
    pub fn is_constructor(&self) -> bool {
        unsafe {
            ffi::JSObjectIsConstructor(self.context.as_raw(), self.raw)
        }
    }
    
    /// Call this object as a constructor.
    pub fn construct(&self, arguments: &[Value<'a>]) -> Result<Object<'a>> {
        if !self.is_constructor() {
            return Err(Error::InvalidType("Object is not a constructor".to_string()));
        }
        
        unsafe {
            let mut exception = ptr::null();
            
            let args: Vec<ffi::JSValueRef> = arguments.iter().map(|v| v.as_raw()).collect();
            
            let result = ffi::JSObjectCallAsConstructor(
                self.context.as_raw(),
                self.raw,
                args.len(),
                if args.is_empty() { ptr::null() } else { args.as_ptr() },
                &mut exception,
            );
            
            if !exception.is_null() {
                return Err(Error::from_js_exception(self.context.as_raw(), exception));
            }
            
            if result.is_null() {
                return Err(Error::JSError("Constructor call failed".to_string()));
            }
            
            Ok(Object {
                context: self.context.clone(),
                raw: result,
            })
        }
    }
    
    /// Check if a value is an instance of this constructor.
    pub fn is_instance_of(&self, value: &Value<'a>) -> Result<bool> {
        unsafe {
            let mut exception = ptr::null();
            
            let result = ffi::JSValueIsInstanceOfConstructor(
                self.context.as_raw(),
                value.as_raw(),
                self.raw,
                &mut exception,
            );
            
            if !exception.is_null() {
                return Err(Error::from_js_exception(self.context.as_raw(), exception));
            }
            
            Ok(result)
        }
    }
    
    /// Get the prototype object of this class instance.
    pub fn get_constructor_prototype(&self) -> Result<Object<'a>> {
        let constructor = self.get_property("constructor")?;
        let constructor_obj = Object::from_value(constructor)?;
        let prototype = constructor_obj.get_property("prototype")?;
        Object::from_value(prototype)
    }
    
    /// If this object is a Proxy, get its target.
    pub fn get_proxy_target(&self) -> Option<Object<'a>> {
        unsafe {
            let target = ffi::JSObjectGetProxyTarget(self.raw);
            if target.is_null() {
                None
            } else {
                Some(Object {
                    context: self.context.clone(),
                    raw: target,
                })
            }
        }
    }
    
    /// Get the Global context this object belongs to.
    pub fn get_global_context(&self) -> Option<Context<'a>> {
        unsafe {
            let ctx = ffi::JSObjectGetGlobalContext(self.raw);
            if ctx.is_null() {
                None
            } else {
                Some(Context::from_raw(ctx))
            }
        }
    }
}

impl<'a> From<Object<'a>> for Value<'a> {
    fn from(obj: Object<'a>) -> Self {
        obj.to_value()
    }
}

impl<'a> TryFrom<Value<'a>> for Object<'a> {
    type Error = Error;
    
    fn try_from(value: Value<'a>) -> Result<Self> {
        Object::from_value(value)
    }
}