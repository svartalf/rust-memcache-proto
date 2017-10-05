use std::io;
use std::fmt;
use std::convert::Into;

use bytes::BufMut;
use byteorder::{NetworkEndian};

use super::extras::Extras;
use super::{Magic, Command, DataType};

const HEADER_SIZE: usize = 24;

/// Memcached request instance.
pub struct Request {
    magic: Magic,
    opcode: Command,
    data_type: DataType,
    vbucket_id: u16,
    opaque: u32,
    cas: u64,

    // body
    extras: Option<Vec<u8>>,
    key: Option<Vec<u8>>,
    value: Option<Vec<u8>>,
}

impl Request {

    /// Create new Request with all fields blank.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let mut request = Request::new(Command::Get);
    /// ```
    pub fn new(command: Command) -> Request {
        Request {
            magic: Magic::Request,
            opcode: command,
            data_type: DataType::RawBytes,
            vbucket_id: 0x00,
            opaque: 0,
            cas: 0,
            extras: None,
            key: None,
            value: None,
        }
    }

    /// Provide key field.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let mut request = Request::new(Command::Get);
    /// request.set_key(b"Hello");
    /// ```
    pub fn set_key(&mut self, key: &[u8]) {
        self.key = Some(Vec::from(key));
    }

    /// Provide value field.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let mut request = Request::new(Command::Set);
    /// request.set_key(b"Hello");
    /// request.set_value(b"World");
    /// ```
    pub fn set_value(&mut self, value: &[u8]) {
        self.value = Some(Vec::from(value));
    }

    pub fn set_extras<T: Extras>(&mut self, extras: &T) {
        self.extras = Some(Vec::from(extras.as_ref()));
    }

    /// Provide virtual bucket ID field.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let mut request = Request::new(Command::Set);
    /// request.set_vbucket_id(5u16);
    /// ```
    pub fn set_vbucket_id<T: Into<u16>>(&mut self, value: T) {
        self.vbucket_id = value.into();
    }

    /// Provide CAS field.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let mut request = Request::new(Command::Set);
    /// request.set_cas(10u64);
    /// ```
    pub fn set_cas<T: Into<u64>>(&mut self, value: T) {
        self.cas = value.into();
    }

    /// Write serialized request as a bytes into `T`
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`][Error] if write had failed somehow.
    ///
    /// [Error]: ../../std/io/struct.Error.html
    pub fn write<T: BufMut>(&self, out: &mut T) -> io::Result<()> {
        out.put_u8(self.magic as u8);
        out.put_u8(self.opcode as u8);
        let key_length = match self.key {
            // TODO: Possible len truncation
            Some(ref key) => key.len() as u16,
            None => 0,
        };
        out.put_u16::<NetworkEndian>(key_length);
        let extras_length = match self.extras {
            // TODO: Possible len truncation
            Some(ref extras) => extras.len() as u8,
            None => 0,
        };
        out.put_u8(extras_length);
        out.put_u8(self.data_type as u8);
        out.put_u16::<NetworkEndian>(self.vbucket_id);
        let body_length: u32 = match self.value {
            Some(ref value) => value.len() as u32,
            None => 0,
        } + key_length as u32 + extras_length as u32;
        out.put_u32::<NetworkEndian>(body_length);
        out.put_u32::<NetworkEndian>(self.opaque);
        out.put_u64::<NetworkEndian>(self.cas);

        if let Some(ref extras) = self.extras {
            out.put_slice(extras);
        }

        if let Some(ref key) = self.key {
            out.put_slice(key);
        }

        if let Some(ref value) = self.value {
            out.put_slice(value);
        }

        Ok(())
    }

    pub fn len(&self) -> usize {
        let mut result: usize = HEADER_SIZE;
        if let Some(ref extras) = self.extras {
            result += extras.len();
        }

        if let Some(ref key) = self.key {
            result += key.len();
        }

        if let Some(ref value) = self.value {
            result += value.len();
        }

        result
    }
}


impl fmt::Debug for Request {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut resp = f.debug_struct("Request");

        resp
            .field("command", &self.opcode)
            .field("key", &self.key)
            .field("value", &self.value)
            .finish()
    }
}
