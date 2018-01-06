use std::fmt;
use std::default;

use {Magic, Command, DataType};
use extras::Extras;

pub struct Request {
    magic: Magic,
    opcode: Command,
    data_type: DataType,
    vbucket_id: u16,
    opaque: u32,
    cas: u64,

    extras: Option<Box<Extras>>,
    // TODO: key field
    // TODO: value field
}

impl Request {

    /// Create a new blank `Request` with the `Command`.
    ///
    /// All fields will set to their defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// use memcache_proto::{Request, Command};
    ///
    /// let request = Request::new(Command::Get);
    /// assert_eq!(*request.command(), Command::Get);
    /// ```
    pub fn new(command: Command) -> Request {
        Request {
            opcode: command,
            ..Self::default()
        }
    }

    /// Returns a reference to the associated `Command`.
    ///
    /// # Examples
    ///
    /// ```
    /// use memcache_proto::{Request, Command};
    ///
    /// let mut request = Request::new(Command::Set);
    ///
    /// assert_eq!(*request.command(), Command::Set);
    /// ```
    pub fn command(&self) -> &Command {
        &self.opcode
    }

    /// Returns a mutable reference to the associated `Command`.
    ///
    /// # Examples
    ///
    /// ```
    /// use memcache_proto::{Request, Command};
    ///
    /// let mut request = Request::new(Command::Get);
    /// *request.command_mut() = Command::Increment;
    ///
    /// assert_eq!(*request.command(), Command::Increment);
    /// ```
    pub fn command_mut(&mut self) -> &mut Command {
        &mut self.opcode
    }

    /// Returns a reference to the associated Virtual Bucket ID.
    ///
    /// # Examples
    ///
    /// ```
    /// use memcache_proto::{Request, Command};
    ///
    /// let mut request = Request::new(Command::Set);
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
    /// use memcache_proto::{Request, Command};
    ///
    /// let mut request = Request::new(Command::Set);
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
    /// use memcache_proto::{Request, Command};
    ///
    /// let mut request = Request::new(Command::Set);
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
    /// use memcache_proto::{Request, Command};
    ///
    /// let mut request = Request::new(Command::Set);
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
    /// use memcache_proto::{Request, Command};
    ///
    /// let mut request = Request::new(Command::Set);
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
    /// use memcache_proto::{Request, Command};
    ///
    /// let mut request = Request::new(Command::Set);
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
    /// use memcache_proto::{Request, Command};
    ///
    /// let mut request = Request::new(Command::Get);
    ///
    /// assert!(request.extras().is_none());
    /// ```
    pub fn extras(&self) -> &Option<Box<Extras>> {
        // TODO: It is really painful to cast `Box<Extras>` into a struct object
        // Also, library users will need to determine what struct (Get/Set/..)
        // should be used here and cast them manually.
        // Can we return an `Option<StructType>` here dynamically somehow?
        &self.extras
    }

    /// Returns a mutable reference to the associated extras.
    ///
    /// # Examples
    ///
    /// ```
    /// use memcache_proto::{Request, Command, extras};
    ///
    /// let mut request = Request::new(Command::Set);
    /// let extras = extras::Set::new(0xdeadbeef, 3600);
    /// *request.extras_mut() = Some(Box::new(extras));
    ///
    /// assert!(request.extras().is_some());
    ///
    /// let my_extras: extras::Set = **request.extras().as_ref().unwrap();
    /// assert_eq!(my_extras.flags(), 0xdeadbeef);
    /// assert_eq!(my_extras.expiration(), 3600);
    /// ```

    pub fn extras_mut(&mut self) -> &mut Option<Box<Extras>> {
        &mut self.extras
    }

}


impl fmt::Debug for Request {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Request")
            .field("command", &self.opcode)
            .field("vbucket_id", &self.vbucket_id)
            .field("opaque", &self.opaque)
            .field("cas", &self.cas)
            // .field("key", &self.key)
            // .field("value", &self.value)
            .field("extras", &self.extras)
            .finish()
    }
}


impl default::Default for Request {
    fn default() -> Self {
        Request {
            magic: Magic::Request,
            opcode: Command::Get,
            data_type: DataType::RawBytes,
            vbucket_id: 0,
            opaque: 0,
            cas: 0,
            extras: None,
//            key: None,
//            value: None,
        }
    }
}
