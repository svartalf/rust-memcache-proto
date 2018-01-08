use std::fmt;
use std::default;

use {Magic, Command, DataType};
use self::builder::Builder;

mod builder;

/// Represents an Memcached request.
///
/// An Memcached request have optional key, value and extras fields.
/// Key and value fields are generic, enabling arbitrary types to represent them.
/// For example, the key could be `Vec<u8>`.
/// (**TODO**: Create more examples, ensure that they are working and add them here.)
///
/// # Examples
///
/// Creating a `Request` to send
///
/// ```rust
///
/// use memcache_proto::{Request, command};
///
/// let mut request: Request<command::Get, &[u8], ()> = Request::new();
/// *request.key_mut() = Some(Box::new(b"some-cached-value"));
/// ```
pub struct Request<C, K, V> where C: Command {
    magic: Magic,
    opcode: C,
    data_type: DataType,
    vbucket_id: u16,
    opaque: u32,
    cas: u64,

    extras: Option<C::RequestExtras>,
    key: Option<Box<K>>,
    value: Option<Box<V>>,
}

impl<C, K, V> Request<C, K, V> where C: Command {

    /// Create a new blank `Request`.
    ///
    /// All fields will set to their defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// use memcache_proto::{Request, command};
    ///
    /// let request: Request<command::Get, Vec<u8>, ()> = Request::new();
    /// ```
    pub fn new() -> Request<C, K, V> {
        Request {
            opcode: C::default(),
            ..Self::default()
        }
    }

    /// Creates a new builder-style object to manufacture a `Request`.
    ///
    /// This method returns an instance of `Builder` which can be used to create a `Request`.
    pub fn build() -> Builder<C, K, V> {
        Builder::new()
    }

    /// Returns a reference to the associated `Command`.
    ///
    /// # Examples
    ///
    /// ```
    /// use memcache_proto::{Request, command};
    ///
    /// let mut request: Request<command::Set, Vec<u8>, Vec<u8>> = Request::new();
    ///
    /// assert_eq!(*request.command(), command::Set);
    /// ```
    pub fn command(&self) -> &C {
        &self.opcode
    }

    /// Returns a mutable reference to the associated `Command`.
    ///
    /// Useful only at request creation, since `Request` struct is parametrized over `Command`,
    /// so it is impossible to replace command for an already instatiated `Request`.
    ///
    /// # Examples
    ///
    /// ```
    /// use memcache_proto::{Request, command};
    ///
    /// let mut request: Request<_, (), ()> = Request::new();
    /// *request.command_mut() = command::Get;
    /// ```
    pub fn command_mut(&mut self) -> &mut C {
        &mut self.opcode
    }

    /// Returns a reference to the associated Virtual Bucket ID.
    ///
    /// # Examples
    ///
    /// ```
    /// use memcache_proto::{Request, command};
    ///
    /// let mut request: Request<command::Set, Vec<u8>, Vec<u8>> = Request::new();
    ///
    /// assert_eq!(*request.vbucket_id(), 0);
    /// ```
    pub fn vbucket_id(&self) -> &u16 {
        &self.vbucket_id
    }

    /// Returns a mutable reference to the associated Virtual Bucket ID.
    ///
    /// # Examples
    ///
    /// ```
    /// use memcache_proto::{Request, command};
    ///
    /// let mut request: Request<command::Set, Vec<u8>, Vec<u8>> = Request::new();
    /// *request.vbucket_id_mut() = 5;
    ///
    /// assert_eq!(*request.vbucket_id(), 5);
    /// ```
    pub fn vbucket_id_mut(&mut self) -> &mut u16 {
        &mut self.vbucket_id
    }

    /// Returns a reference to the associated Opaque value.
    ///
    /// # Examples
    ///
    /// ```
    /// use memcache_proto::{Request, command};
    ///
    /// let mut request: Request<command::Set, Vec<u8>, Vec<u8>> = Request::new();
    ///
    /// assert_eq!(*request.opaque(), 0);
    /// ```
    pub fn opaque(&self) -> &u32 {
        &self.opaque
    }

    /// Returns a mutable reference to the associated Opaque value.
    ///
    /// # Examples
    ///
    /// ```
    /// use memcache_proto::{Request, command};
    ///
    /// let mut request: Request<command::Set, Vec<u8>, Vec<u8>> = Request::new();
    /// *request.opaque_mut() = 5;
    ///
    /// assert_eq!(*request.opaque(), 5);
    /// ```
    pub fn opaque_mut(&mut self) -> &mut u32 {
        &mut self.opaque
    }

    /// Returns a reference to the associated Compare-and-swap value.
    ///
    /// # Examples
    ///
    /// ```
    /// use memcache_proto::{Request, command};
    ///
    /// let mut request: Request<command::Set, Vec<u8>, Vec<u8>> = Request::new();
    ///
    /// assert_eq!(*request.cas(), 0);
    /// ```
    pub fn cas(&self) -> &u64 {
        &self.cas
    }

    /// Returns a mutable reference to the associated Compare-and-swap value.
    ///
    /// # Examples
    ///
    /// ```
    /// use memcache_proto::{Request, command};
    ///
    /// let mut request: Request<command::Set, Vec<u8>, Vec<u8>> = Request::new();
    /// *request.cas_mut() = 42;
    ///
    /// assert_eq!(*request.cas(), 42);
    /// ```
    pub fn cas_mut(&mut self) -> &mut u64 {
        &mut self.cas
    }

    /// Returns a reference to the associated extras.
    ///
    /// # Examples
    ///
    /// ```
    /// use memcache_proto::{Request, command};
    ///
    /// let mut request: Request<command::Set, Vec<u8>, Vec<u8>> = Request::new();
    ///
    /// assert!(request.extras().is_none());
    /// ```
    pub fn extras(&self) -> &Option<C::RequestExtras> {
        &self.extras
    }

    /// Returns a mutable reference to the associated extras.
    ///
    /// Extras type is defined by Request' `Command`.
    ///
    /// # Examples
    ///
    /// ```
    /// use memcache_proto::{Request, command, extras};
    ///
    /// let mut request: Request<command::Set, Vec<u8>, Vec<u8>> = Request::new();
    /// let extras = extras::Set::new(0xdeadbeef, 3600);
    /// *request.extras_mut() = Some(extras);
    ///
    /// assert!(request.extras().is_some());
    ///
    /// let my_extras = request.extras().as_ref().unwrap();
    /// assert_eq!(*my_extras.flags(), 0xdeadbeef);
    /// assert_eq!(*my_extras.expiration(), 3600);
    /// ```

    pub fn extras_mut(&mut self) -> &mut Option<C::RequestExtras> {
        &mut self.extras
    }

    pub fn key(&self) -> &Option<Box<K>> {
        &self.key
    }

    pub fn key_mut(&mut self) -> &mut Option<Box<K>> {
        &mut self.key
    }

    pub fn value(&self) -> &Option<Box<V>> {
        &self.value
    }

    pub fn value_mut(&mut self) -> &mut Option<Box<V>> {
        &mut self.value
    }

}


impl<C, K, V> fmt::Debug for Request<C, K, V>
        where C: Command, C::RequestExtras: fmt::Debug, K: fmt::Debug, V: fmt::Debug {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Request")
            .field("command", &self.opcode)
            .field("vbucket_id", &self.vbucket_id)
            .field("opaque", &self.opaque)
            .field("cas", &self.cas)
            .field("key", &self.key)
            .field("value", &self.value)
            .field("extras", &self.extras)
            .finish()
    }
}


impl<C, K, V> default::Default for Request<C, K, V>
        where C: Command + default::Default {
    fn default() -> Self {
        Request {
            magic: Magic::Request,
            opcode: C::default(),
            data_type: DataType::RawBytes,
            vbucket_id: 0,
            opaque: 0,
            cas: 0,
            extras: None,
            key: None,
            value: None,
        }
    }
}

#[cfg(test)]
mod tests;
