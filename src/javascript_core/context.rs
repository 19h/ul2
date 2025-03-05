//! JavaScriptCore execution context implementation.
//!
//! This module provides a comprehensive abstraction over JSContextRef and JSGlobalContextRef,
//! implementing the requisite memory management strategies and lifetime management to ensure
//! safe interaction with the underlying JavaScript engine. The Context struct represents
//! a JavaScript execution environment with its own global object and execution state,
//! while GlobalContext represents an owning reference to a context.

use std::marker::PhantomData;
use std::ptr;
use std::ffi::{CStr, CString};
use std::convert::TryFrom;

use crate::javascript_core::ffi;
use crate::javascript_core::error::{Error, Result};
use crate::javascript_core::object::Object;
use crate::javascript_core::value::Value;
use crate::javascript_core::string::String;

/// A reference to a JavaScript execution context.
///
/// The Context struct holds a reference to a JSContextRef, representing an execution
/// environment for JavaScript code. Each Context has a global object and maintains
/// the execution state. The Context is non-owning and therefore must not outlive
/// the GlobalContext that created it.
#[derive(Clone)]
pub struct Context<'a> {
    raw: ffi::JSContextRef,
    _phantom: PhantomData<&'a ()>,
}

/// A global JavaScript execution context.
///
/// The GlobalContext struct holds ownership of a JSGlobalContextRef, which is a
/// top-level JavaScript execution environment. It manages the lifetime of the context
/// and ensures proper cleanup when it's dropped. The GlobalContext can provide
/// references to its contained Context for operations that require a context reference.
pub struct GlobalContext {
    raw: ffi::JSGlobalContextRef,
}

impl<'a> Context<'a> {
    /// Creates a Context from a raw JSContextRef.
    ///
    /// # Safety
    ///
    /// The provided JSContextRef must be a valid pointer to a JavaScript context,
    /// and must not be deallocated while this Context is alive.
    pub(crate) unsafe fn from_raw(raw: ffi::JSContextRef) -> Self {
        Context {
            raw,
            _phantom: PhantomData,
        }
    }
    
    /// Creates a dummy Context for use in situations where a real context is not available.
    ///
    /// # Safety
    ///
    /// This should only be used in specific cases like finalize callbacks where
    /// the context is not needed or available. Using methods on this context that
    /// interact with the JavaScript engine may cause undefined behavior.
    pub(crate) unsafe fn dummy() -> Self {
        Context {
            raw: ptr::null(),
            _phantom: PhantomData,
        }
    }
    
    /// Returns the raw JSContextRef pointer.
    pub(crate) fn as_raw(&self) -> ffi::JSContextRef {
        self.raw
    }
    
    /// Returns the global object for this context.
    ///
    /// The global object is the top-level object in the JavaScript environment,
    /// equivalent to the 'window' object in a browser context or 'global' in Node.js.
    pub fn global_object(&self) -> Object<'a> {
        unsafe {
            let obj = ffi::JSContextGetGlobalObject(self.raw);
            Object::from_raw(self.clone(), obj)
        }
    }
    
    /// Returns the context group that this context belongs to.
    ///
    /// A context group associates JavaScript contexts with one another. Contexts in the
    /// same group may share and exchange JavaScript objects.
    pub fn group(&self) -> ffi::JSContextGroupRef {
        unsafe {
            ffi::JSContextGetGroup(self.raw)
        }
    }
    
    /// Returns the global context that this context belongs to.
    ///
    /// A global context is the root context that owns the JavaScript environment.
    /// This method retrieves the owning global context for any context reference.
    pub fn global_context(&self) -> ffi::JSGlobalContextRef {
        unsafe {
            ffi::JSContextGetGlobalContext(self.raw)
        }
    }
    
    /// Evaluates JavaScript code in this context.
    ///
    /// This method executes the provided JavaScript code within this context and
    /// returns the result of the evaluation. If the code throws a JavaScript exception,
    /// that exception will be caught and returned as an `Err` variant.
    ///
    /// # Arguments
    ///
    /// * `script` - The JavaScript code to evaluate.
    /// * `this_object` - Optional object to use as 'this' during execution.
    /// * `source_url` - Optional URL for the script's source, used for debugging.
    /// * `starting_line` - The line number to report as the start of the script.
    ///
    /// # Returns
    ///
    /// A `Result` containing either the JavaScript evaluation result or an error
    /// if an exception occurred during evaluation.
    pub fn evaluate_script(
        &self,
        script: &str,
        this_object: Option<&Object<'a>>,
        source_url: Option<&str>,
        starting_line: i32,
    ) -> Result<Value<'a>> {
        let script_str = String::new(script);
        let source_url_str = source_url.map(String::new);
        let this_obj = this_object.map_or(ptr::null_mut(), |o| o.as_raw());
        
        unsafe {
            let mut exception = ptr::null();
            let result = ffi::JSEvaluateScript(
                self.raw,
                script_str.as_raw(),
                this_obj,
                source_url_str.as_ref().map_or(ptr::null_mut(), |s| s.as_raw()),
                starting_line,
                &mut exception,
            );
            
            if !exception.is_null() {
                return Err(Error::from_js_exception(self.raw, exception));
            }
            
            if result.is_null() {
                return Err(Error::JSError("Evaluation returned null".to_string()));
            }
            
            Ok(Value::from_raw(self, result))
        }
    }
    
    /// Checks if JavaScript code has valid syntax without executing it.
    ///
    /// This method parses the provided JavaScript code to determine if it has valid
    /// syntax, without evaluating it. This is useful for validating user input
    /// before execution.
    ///
    /// # Arguments
    ///
    /// * `script` - The JavaScript code to check.
    /// * `source_url` - Optional URL for the script's source, used for debugging.
    /// * `starting_line` - The line number to report as the start of the script.
    ///
    /// # Returns
    ///
    /// A `Result` containing a boolean indicating whether the syntax is valid, or
    /// an error if an exception occurred during syntax checking.
    pub fn check_script_syntax(
        &self,
        script: &str,
        source_url: Option<&str>,
        starting_line: i32,
    ) -> Result<bool> {
        let script_str = String::new(script);
        let source_url_str = source_url.map(String::new);
        
        unsafe {
            let mut exception = ptr::null();
            let result = ffi::JSCheckScriptSyntax(
                self.raw,
                script_str.as_raw(),
                source_url_str.as_ref().map_or(ptr::null_mut(), |s| s.as_raw()),
                starting_line,
                &mut exception,
            );
            
            if !exception.is_null() {
                return Err(Error::from_js_exception(self.raw, exception));
            }
            
            Ok(result)
        }
    }
    
    /// Performs a JavaScript garbage collection cycle.
    ///
    /// This method explicitly triggers the JavaScript garbage collector. While the
    /// JavaScript engine performs garbage collection automatically, this method can
    /// be used to reclaim memory at specific points in your application's lifecycle.
    ///
    /// Note: Values created within a context group are automatically destroyed when
    /// the last reference to the context group is released.
    pub fn garbage_collect(&self) {
        unsafe {
            ffi::JSGarbageCollect(self.raw);
        }
    }
}

impl GlobalContext {
    /// Creates a new global JavaScript context with default settings.
    ///
    /// This creates a global JavaScript execution environment with the default
    /// global object class. The global object is populated with standard JavaScript
    /// built-in objects and functions.
    ///
    /// # Returns
    ///
    /// A new GlobalContext instance.
    pub fn new() -> Self {
        unsafe {
            let raw = ffi::JSGlobalContextCreate(ptr::null_mut());
            GlobalContext { raw }
        }
    }
    
    /// Creates a new global JavaScript context with a custom global object class.
    ///
    /// This creates a global JavaScript execution environment with a specified class
    /// for the global object. This allows customization of the global object's behavior
    /// through the class's callback functions.
    ///
    /// # Arguments
    ///
    /// * `global_class` - The class to use for the global object.
    ///
    /// # Returns
    ///
    /// A new GlobalContext instance.
    pub fn with_class(global_class: ffi::JSClassRef) -> Self {
        unsafe {
            let raw = ffi::JSGlobalContextCreate(global_class);
            GlobalContext { raw }
        }
    }
    
    /// Creates a new global JavaScript context in a specific context group.
    ///
    /// This creates a global JavaScript execution environment that belongs to the
    /// specified context group. Contexts in the same group may share JavaScript objects.
    ///
    /// # Arguments
    ///
    /// * `group` - The context group to use, or None to create a new context group.
    /// * `global_class` - The class to use for the global object, or None to use the default.
    ///
    /// # Returns
    ///
    /// A new GlobalContext instance.
    pub fn with_group(group: Option<ffi::JSContextGroupRef>, global_class: Option<ffi::JSClassRef>) -> Self {
        unsafe {
            let raw = ffi::JSGlobalContextCreateInGroup(
                group.unwrap_or(ptr::null()),
                global_class.unwrap_or(ptr::null_mut()),
            );
            GlobalContext { raw }
        }
    }
    
    /// Returns the raw JSGlobalContextRef pointer.
    pub(crate) fn as_raw(&self) -> ffi::JSGlobalContextRef {
        self.raw
    }
    
    /// Returns a reference to the context.
    ///
    /// This provides a reference to the underlying context that can be used
    /// for JavaScript operations. The returned Context is borrowed from this
    /// GlobalContext and must not outlive it.
    pub fn context<'a>(&'a self) -> Context<'a> {
        unsafe {
            Context::from_raw(self.raw)
        }
    }
    
    /// Returns the global object for this context.
    ///
    /// This is a convenience method that retrieves the global object from the
    /// underlying context.
    pub fn global_object<'a>(&'a self) -> Object<'a> {
        self.context().global_object()
    }
    
    /// Evaluates JavaScript code in this context.
    ///
    /// This is a convenience method that delegates to the underlying context's
    /// evaluate_script method.
    ///
    /// # Arguments
    ///
    /// * `script` - The JavaScript code to evaluate.
    /// * `source_url` - Optional URL for the script's source, used for debugging.
    /// * `starting_line` - The line number to report as the start of the script.
    ///
    /// # Returns
    ///
    /// A `Result` containing either the JavaScript evaluation result or an error
    /// if an exception occurred during evaluation.
    pub fn evaluate_script<'a>(&'a self, script: &str, source_url: Option<&str>, starting_line: i32) -> Result<Value<'a>> {
        self.context().evaluate_script(script, None, source_url, starting_line)
    }
    
    /// Gets the name of this global context.
    ///
    /// The name is used for debugging purposes and is visible when inspecting the context.
    ///
    /// # Returns
    ///
    /// The name of the context, or an empty string if no name has been set.
    pub fn name(&self) -> String {
        unsafe {
            let name = ffi::JSGlobalContextCopyName(self.raw);
            if name.is_null() {
                String::new("")
            } else {
                String::from_raw(name)
            }
        }
    }
    
    /// Sets the name of this global context.
    ///
    /// The name is used for debugging purposes and is visible when inspecting the context.
    ///
    /// # Arguments
    ///
    /// * `name` - The name to set for the context.
    pub fn set_name(&self, name: &str) {
        let name_str = String::new(name);
        unsafe {
            ffi::JSGlobalContextSetName(self.raw, name_str.as_raw());
        }
    }
    
    /// Checks if this global context is inspectable.
    ///
    /// If a context is inspectable, it can be connected to by a JavaScript debugger
    /// for debugging purposes.
    ///
    /// # Returns
    ///
    /// `true` if the context is inspectable, `false` otherwise.
    pub fn is_inspectable(&self) -> bool {
        unsafe {
            ffi::JSGlobalContextIsInspectable(self.raw)
        }
    }
    
    /// Sets whether this global context is inspectable.
    ///
    /// If a context is inspectable, it can be connected to by a JavaScript debugger
    /// for debugging purposes.
    ///
    /// # Arguments
    ///
    /// * `inspectable` - Whether the context should be inspectable.
    pub fn set_inspectable(&self, inspectable: bool) {
        unsafe {
            ffi::JSGlobalContextSetInspectable(self.raw, inspectable);
        }
    }
    
    /// Performs a JavaScript garbage collection cycle.
    ///
    /// This is a convenience method that delegates to the underlying context's
    /// garbage_collect method.
    pub fn garbage_collect(&self) {
        self.context().garbage_collect();
    }
}

impl Default for GlobalContext {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for GlobalContext {
    fn drop(&mut self) {
        unsafe {
            ffi::JSGlobalContextRelease(self.raw);
        }
    }
}

/// A wrapper for context groups.
///
/// A context group associates JavaScript contexts with one another. Contexts in the
/// same group may share and exchange JavaScript objects.
pub struct ContextGroup {
    raw: ffi::JSContextGroupRef,
}

impl ContextGroup {
    /// Creates a new context group.
    ///
    /// # Returns
    ///
    /// A new ContextGroup instance.
    pub fn new() -> Self {
        unsafe {
            let raw = ffi::JSContextGroupCreate();
            ContextGroup { raw }
        }
    }
    
    /// Returns the raw JSContextGroupRef pointer.
    pub fn as_raw(&self) -> ffi::JSContextGroupRef {
        self.raw
    }
    
    /// Creates a new global context in this context group.
    ///
    /// # Arguments
    ///
    /// * `global_class` - The class to use for the global object, or None to use the default.
    ///
    /// # Returns
    ///
    /// A new GlobalContext instance that belongs to this context group.
    pub fn create_global_context(&self, global_class: Option<ffi::JSClassRef>) -> GlobalContext {
        GlobalContext::with_group(Some(self.raw), global_class)
    }
}

impl Default for ContextGroup {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for ContextGroup {
    fn drop(&mut self) {
        unsafe {
            ffi::JSContextGroupRelease(self.raw);
        }
    }
}

impl Clone for ContextGroup {
    fn clone(&self) -> Self {
        unsafe {
            let raw = ffi::JSContextGroupRetain(self.raw);
            ContextGroup { raw }
        }
    }
}