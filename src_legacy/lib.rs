/// Memcache binary protocol implementation.
///
/// This crate does not provide any client functionality,
/// only thin wrappers for memcache protocol packets,
/// which are should be suitable for all client implementations.

#[macro_use] extern crate enum_primitive;
extern crate byteorder;
extern crate bytes;

mod command;
mod request;
mod response;
mod errors;
pub mod extras;

pub use request::{Request, RequestBuilder};
pub use response::{Response, Status};
pub use errors::{ResponseError};



#[cfg(test)]
mod tests;
