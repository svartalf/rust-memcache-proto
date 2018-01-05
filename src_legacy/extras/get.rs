use std::io;
use std::default::Default;
use std::fmt;

use bytes::{Buf, BufMut};
use byteorder::NetworkEndian;

use super::Extras;

/// Extras container for `Get` responses.
///
/// Since `GetK`/`GetQ`/`GetKQ` responses are using the same format,
/// associated type alias can be used in order to provide consistent interface.
///
/// See [GetK](type.GetK.html), [GetQ](type.GetQ.html) and [GetKQ](type.GetKQ.html) type aliases
/// for more.
///
/// # Examples
///
/// ```rust,ignore
/// use memcache_proto::extras::Get;
///
/// let mut extras = Get::new();
/// extras.set_flags(0xdeadbeef_u32);
/// ```
///
/// With builder interface:
///
/// ```rust,ignore
/// use memcache_proto::extras::Get;
///
/// let extras = Get::new()
///     .with_flags(0xdeadbeef_u32);
/// ```
pub struct Get{
    flags: u32,
}

/// Extras container for `GetQ` requests.
///
/// It is an alias for [Get](struct.Get.html) struct,
/// see [the module documentation](struct.Get.html) for more.
pub type GetQ = Get;

/// Extras container for `GetK` requests.
///
/// It is an alias for [Get](struct.Get.html) struct,
/// see [the module documentation](struct.Get.html) for more.
pub type GetK = Get;

/// Extras container for `GetKQ` requests.
///
/// It is an alias for [Get](struct.Get.html) struct,
/// see [the module documentation](struct.Get.html) for more.
pub type GetKQ = Get;

impl Get {
    /// Create new extras container.
    pub fn new(flags: u32) -> Get {
        Get {
            flags: flags,
        }
    }

    pub fn build() -> GetBuilder {
        GetBuilder(Get::default())
    }

    pub fn set_flags(&mut self, flags: u32) {
        self.flags = flags;
    }

    pub fn flags(&self) -> u32 {
        self.flags
    }
}

pub struct GetBuilder(Get);

impl GetBuilder {
    pub fn flags(mut self, flags: u32) -> Self {
        self.0.set_flags(flags);
        self
    }

    pub fn finish(self) -> Get {
        self.0
    }
}

impl Extras for Get {
    fn read<T: Buf>(buf: &mut T) -> io::Result<Self> {
        Ok(Self {
            flags: buf.get_u32::<NetworkEndian>(),
        })
    }

    fn write<T: BufMut>(&self, buf: &mut T) -> io::Result<()> {
        buf.put_u32::<NetworkEndian>(self.flags);
        Ok(())
    }
}

impl Default for Get {
    fn default() -> Self {
        Self {
            flags: 0,
        }
    }
}

impl fmt::Debug for Get {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Get")
            .field("flags", &self.flags)
            .finish()
    }
}
