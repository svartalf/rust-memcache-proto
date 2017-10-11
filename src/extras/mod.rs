use std::io;
use bytes::{Buf, BufMut};

mod get;
mod set;
mod incr;
mod traits;

pub use self::get::{Get, GetK, GetQ, GetKQ};
pub use self::set::{Set, Add, Replace};
pub use self::incr::{Increment, Decrement};
pub use self::traits::Expiration;

pub trait Extras: Sized {
    fn read<T: Buf>(buf: &mut T) -> io::Result<Self>;
    fn write<T: BufMut>(&self, buf: &mut T) -> io::Result<()>;
}

#[cfg(test)]
mod tests;
