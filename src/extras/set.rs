use std::io;
use std::fmt;
use std::default::Default;
use std::convert::Into;

use bytes::{Buf, BufMut};
use byteorder::NetworkEndian;

use super::{Extras, Expiration};

/// Extras container for `Set` requests.
///
/// All fields are zeroed by default.
///
/// Since `Add` and `Replace` requests use the same format,
/// [Add](type.Add.html) and [Replace](type.Replace.html) type aliases can be used
/// fin order to provide consistent interface.
///
/// # Examples
///
/// ```rust,ignore
/// use memcache_proto::extras::Set;
///
/// let mut extras = Set::new();
/// extras.set_flags(0xdeadbeef_u32);
/// extras.set_expiration(60_u32);
/// ```
///
/// With builder interface:
///
/// ```rust,ignore
/// use memcache_proto::extras::Set;
///
/// let extras = Set::new()
///     .with_flags(0xdeadbeef_u32)
///     .with_expiration(60_u32);
/// ```
pub struct Set{
    flags: u32,
    expiration: u32,
}

/// Extras container for `Add` requests.
///
/// It is an alias for [Set](struct.Set.html) struct,
/// see [the module documentation](struct.Set.html) for more.
pub type Add = Set;

/// Extras container for `Replace` requests.
///
/// It is an alias for [Set](struct.Set.html) struct,
/// see [the module documentation](struct.Set.html) for more.
pub type Replace = Set;

impl Set {

    pub fn new<T: Expiration>(flags: u32, expiration: T) -> Set {
        Self {
            flags: flags,
            expiration: expiration.into_expiration(),
        }
    }

    pub fn build() -> SetBuilder {
        SetBuilder(Set::default())
    }

    pub fn set_flags<T: Into<u32>>(&mut self, value: T) {
        self.flags = value.into();
    }

    pub fn flags(&self) -> u32 {
        self.flags
    }

    pub fn set_expiration<T: Expiration>(&mut self, value: T) {
        self.expiration = value.into_expiration();
    }

    pub fn expiration(&self) -> u32 {
        self.expiration
    }
}

pub struct SetBuilder(Set);

impl SetBuilder {

    pub fn flags(mut self, flags: u32) -> Self {
        self.0.set_flags(flags);
        self
    }

    pub fn expiration<T: Expiration>(mut self, expiration: T) -> Self {
        self.0.set_expiration(expiration);
        self
    }

    pub fn finish(self) -> Set {
        self.0
    }

}

impl Extras for Set {
    fn read<T: Buf>(buf: &mut T) -> io::Result<Self> {
        Ok(Self{
            flags: buf.get_u32::<NetworkEndian>(),
            expiration: buf.get_u32::<NetworkEndian>(),
        })
    }

    fn write<T: BufMut>(&self, buf: &mut T) -> io::Result<()> {
        buf.put_u32::<NetworkEndian>(self.flags);
        buf.put_u32::<NetworkEndian>(self.expiration);
        Ok(())
    }

}

impl Default for Set {
    fn default() -> Self {
        Self {
            flags: 0,
            expiration: 0,
        }
    }
}

impl fmt::Debug for Set {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Set")
            .field("flags", &self.flags())
            .field("expiration", &self.expiration())
            .finish()
    }
}

