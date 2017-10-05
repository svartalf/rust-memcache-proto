use std::convert::AsRef;
use std::fmt;

use byteorder::{ByteOrder, NetworkEndian};

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
/// let mut extras = Get::new();
/// extras.set_flags(0xdeadbeef);
/// ```
///
/// With builder interface:
///
/// ```rust,ignore
/// let extras = Get::new()
///     .with_flags(0xdeadbeef);
/// ```
pub struct Get([u8; 4]);

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
    /// All fields are zeroed by default.
    pub fn new() -> Get {
        Get([0; 4])
    }

    pub fn with_flags(mut self, flags: u32) -> Get {
        self.set_flags(flags);
        self
    }

    pub fn set_flags(&mut self, value: u32) {
        NetworkEndian::write_u32(&mut self.0[..4], value);
    }

    pub fn flags(&self) -> u32 {
        NetworkEndian::read_u32(&self.0[..4])
    }
}

impl Extras for Get {}

impl AsRef<[u8]> for Get {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl fmt::Debug for Get {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Get")
            .field("flags", &self.flags())
            .finish()
    }
}
