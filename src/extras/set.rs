use std::fmt;
use std::convert::{AsRef, Into};

use byteorder::{ByteOrder, NetworkEndian};

use super::Extras;

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
/// let mut extras = Set::new();
/// extras.set_flags(0xdeadbeef);
/// extra.set_expiration(60);
/// ```
///
/// With builder interface:
///
/// ```rust,ignore
/// let extras = Incr::new()
///     .with_flags(0xdeadbeef)
///     .with_expiration(60);
/// ```
pub struct Set([u8; 8]);

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

    pub fn new() -> Set {
        Set([0; 8])
    }

    pub fn with_flags<T: Into<u32>>(mut self, value: T) -> Self {
        self.set_flags(value);
        self
    }

    pub fn with_expiration<T: Into<u32>>(mut self, value: T) -> Self {
        self.set_expiration(value);
        self
    }

    pub fn set_flags<T: Into<u32>>(&mut self, value: T) {
        NetworkEndian::write_u32(&mut self.0[..4], value.into());
    }

    pub fn flags(&self) -> u32 {
        NetworkEndian::read_u32(&self.0[..4])
    }

    pub fn set_expiration<T: Into<u32>>(&mut self, value: T) {
        NetworkEndian::write_u32(&mut self.0[4..8], value.into());
    }

    pub fn expiration(&self) -> u32 {
        NetworkEndian::read_u32(&self.0[4..8])
    }

}

impl Extras for Set {}

impl AsRef<[u8]> for Set {
    fn as_ref(&self) -> &[u8] {
        &self.0
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

