use std::fmt;
use std::default::Default;

use super::{Command, Request};

/// Builder, which can be used in order to create a new [Request](type.Request.html)
/// via chain calls.
///
/// # Examples
///
/// ```rust,ignore
/// let request = Request::build(Command::Set)
///     .key(Some("hello"))
///     .value(Some("world"))
///     .cas(1024)
///     .finish();
/// ```
pub struct RequestBuilder(Request);

impl RequestBuilder {
    pub fn new(command: Command) -> Self {
        RequestBuilder(Request{
            opcode: command,
            ..Request::default()
        })
    }

    /// Provide Virtual bucket ID
    pub fn vbucket_id(mut self, vbucket_id: u16) -> Self {
        self.0.set_vbucket_id(vbucket_id);
        self
    }

    /// Provide opaque
    pub fn opaque(mut self, opaque: u32) -> Self {
        self.0.set_opaque(opaque);
        self
    }

    /// Provide CAS
    pub fn cas(mut self, cas: u64) -> Self {
        self.0.set_cas(cas);
        self
    }

    /// Provide extras
    pub fn extras<T: AsRef<[u8]>>(mut self, extras: Option<T>) -> Self {
        self.0.set_extras(extras);
        self
    }

    /// Provide key
    pub fn key<T: AsRef<[u8]>>(mut self, key: Option<T>) -> Self {
        self.0.set_key(key);
        self
    }

    /// Provide value
    pub fn value<T: AsRef<[u8]>>(mut self, value: Option<T>) -> Self {
        self.0.set_value(value);
        self
    }

    /// Consume builder and return complete [Request](struct.Request.html)
    pub fn finish(self) -> Request {
        self.0
    }

}

impl fmt::Debug for RequestBuilder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("RequestBuilder")
            .field("request", &self.0)
            .finish()
    }
}
