use std::io;
use std::any::Any;
use std::fmt::Debug;

use bytes::{Buf, BufMut};

mod get;
mod set;

pub use self::get::{Get, GetK, GetQ, GetKQ};
pub use self::set::{Set, Add, Replace};


pub trait Extras: Debug {
    // TODO: Can I get rid of these `where Self: Sized` below? `Box<Extras>` failing without them.
    fn read<T: Buf>(buf: &mut T) -> io::Result<Self> where Self: Sized;
    fn write<T: BufMut>(&self, buf: &mut T) -> io::Result<()> where Self: Sized;
}
