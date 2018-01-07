use std::io;
use std::any::Any;
use std::fmt::Debug;

use bytes::{Buf, BufMut};

mod get;
mod set;

pub use self::get::{Get, GetK, GetQ, GetKQ};
pub use self::set::{Set, Add, Replace};

pub trait Extras: Sized + Debug {
    fn read<T: Buf>(buf: &mut T) -> io::Result<Self>;
    fn write<T: BufMut>(&self, buf: &mut T) -> io::Result<()>;
}

impl Extras for () {
    fn read<T: Buf>(buf: &mut T) -> io::Result<Self> where Self: Sized {
        unimplemented!()
    }

    fn write<T: BufMut>(&self, buf: &mut T) -> io::Result<()> where Self: Sized {
        unimplemented!()
    }
}
