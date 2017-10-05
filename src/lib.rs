/// Memcache binary protocol implementation.

#[macro_use] extern crate enum_primitive;
extern crate byteorder;
extern crate bytes;

mod command;
mod request;
mod response;
mod errors;
pub mod extras;

pub use command::Command;
pub use request::Request;
pub use response::{Response, Status};
pub use errors::{ResponseError};

enum_from_primitive! {
    /// Magic byte values for protocol packets.
    ///
    /// Reference: https://github.com/memcached/memcached/wiki/BinaryProtocolRevamped#magic-byte
    #[derive(Debug, PartialEq, Copy, Clone)]
    pub enum Magic {
        Request = 0x80,
        Response = 0x81,
    }
}

enum_from_primitive! {
    /// Reserved for future use.
    ///
    /// https://github.com/memcached/memcached/wiki/BinaryProtocolRevamped#data-types
    #[derive(Debug, PartialEq, Copy, Clone)]
    pub enum DataType {
        RawBytes = 0x00,
    }
}

#[cfg(test)]
mod tests;