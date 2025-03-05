use crate::ul::ffi::{
    ULSession, ulCreateSession, ulDefaultSession, ulDestroySession, ulSessionGetDiskPath,
    ulSessionGetId, ulSessionGetName, ulSessionIsPersistent,
};
use crate::ul::renderer::Renderer;
use crate::ul::string::String;

/// A safe wrapper around Ultralight's ULSession type.
pub struct Session {
    raw: ULSession,
    owned: bool,
}

impl Session {
    /// Create a new session to store local data.
    pub fn new(renderer: &Renderer, is_persistent: bool, name: &str) -> Self {
        let name_str = String::from_str(name);
        unsafe {
            let raw = ulCreateSession(renderer.raw(), is_persistent, name_str.raw());
            Self { raw, owned: true }
        }
    }

    /// Get the default session.
    pub fn default(renderer: &Renderer) -> Self {
        unsafe {
            let raw = ulDefaultSession(renderer.raw());
            Self { raw, owned: false }
        }
    }

    /// Create a session from a raw ULSession pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid ULSession created by the Ultralight API.
    pub unsafe fn from_raw(raw: ULSession, owned: bool) -> Self {
        Self { raw, owned }
    }

    /// Get a reference to the raw ULSession.
    pub fn raw(&self) -> ULSession {
        self.raw
    }

    /// Check if the session is persistent (backed to disk).
    pub fn is_persistent(&self) -> bool {
        unsafe { ulSessionIsPersistent(self.raw) }
    }

    /// Get the name of the session.
    pub fn name(&self) -> String {
        unsafe {
            let name = ulSessionGetName(self.raw);
            String::from_raw(name)
        }
    }

    /// Get the unique numeric ID of the session.
    pub fn id(&self) -> u64 {
        unsafe { ulSessionGetId(self.raw) }
    }

    /// Get the disk path for persistent sessions.
    pub fn disk_path(&self) -> String {
        unsafe {
            let path = ulSessionGetDiskPath(self.raw);
            String::from_raw(path)
        }
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        if !self.raw.is_null() && self.owned {
            unsafe {
                ulDestroySession(self.raw);
            }
        }
    }
}
