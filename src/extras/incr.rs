use std::fmt;
use std::convert::AsRef;

use byteorder::{ByteOrder, NetworkEndian};

use super::Extras;

/// Extras container for `Increment` requests.
///
/// All fields are zeroed by default.
///
/// Since `Decrement` requests use the same format, [Decrement type alias](type.Decrement.html)
/// can be used in order to provide consistent interface.
///
/// # Examples
///
/// ```rust
/// use memcache_proto::extras::Increment;
///
/// let mut extras = Increment::new();
/// extras.set_amount(1);
/// extras.set_initial(0);
/// extras.set_expiration(60);
/// ```
///
/// With builder interface:
///
/// ```rust
/// use memcache_proto::extras::Increment;
///
/// let extras = Increment::new()
///     .with_amount(1)
///     .with_initial(0)
///     .with_expiration(60);
/// ```
pub struct Increment([u8; 40]);

/// Extras container for `Decrement` requests.
///
/// It is an alias for [Increment](struct.Increment.html) struct,
/// see [the module documentation](struct.Increment.html) for more.
pub type Decrement = Increment;

impl Increment {

    pub fn new() -> Self {
        Increment([0; 40])
    }

    pub fn with_amount(mut self, value: u64) -> Self {
        self.set_amount(value);
        self
    }

    pub fn with_initial(mut self, value: u64) -> Self {
        self.set_initial(value);
        self
    }

    pub fn with_expiration(mut self, value: u32) -> Self {
        self.set_expiration(value);
        self
    }

    pub fn set_amount(&mut self, value: u64) {
        NetworkEndian::write_u64(&mut self.0[..8], value);
    }

    pub fn amount(&self) -> u64 {
        NetworkEndian::read_u64(&self.0[..8])
    }

    pub fn set_initial(&mut self, value: u64) {
        NetworkEndian::write_u64(&mut self.0[8..16], value);
    }

    pub fn initial(&self) -> u64 {
        NetworkEndian::read_u64(&self.0[8..16])
    }

    pub fn set_expiration(&mut self, value: u32) {
        NetworkEndian::write_u32(&mut self.0[16..20], value);
    }

    pub fn expiration(&self) -> u32 {
        NetworkEndian::read_u32(&self.0[16..20])
    }

}

impl Extras for Increment {}

impl AsRef<[u8]> for Increment {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}


impl fmt::Debug for Increment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Increment")
            .field("amount", &self.amount())
            .field("initial", &self.initial())
            .field("expiration", &self.expiration())
            .finish()
    }
}

