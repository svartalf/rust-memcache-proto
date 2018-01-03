use std::io;
use std::fmt;
use std::default::Default;

use bytes::{Buf, BufMut};
use byteorder::NetworkEndian;

use super::{Extras, Expiration};

/// Extras container for `Increment` requests.
///
/// Since `Decrement` requests use the same format, [Decrement type alias](type.Decrement.html)
/// can be used in order to provide consistent interface.
///
/// # Examples
///
/// ```rust,ignore
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
/// ```rust,ignore
/// use memcache_proto::extras::Increment;
///
/// let extras = Increment::new()
///     .with_amount(1)
///     .with_initial(0)
///     .with_expiration(60);
/// ```
pub struct Increment{
    amount: u64,
    initial: u64,
    expiration: u32,
}

/// Extras container for `Decrement` requests.
///
/// It is an alias for [Increment](struct.Increment.html) struct,
/// see [the module documentation](struct.Increment.html) for more.
pub type Decrement = Increment;

impl Increment {

    pub fn new<T: Expiration>(amount: u64, initial: u64, expiration: T) -> Self {
        Self {
            amount: amount,
            initial: initial,
            expiration: expiration.get_timeout(),
        }
    }

    pub fn build() -> IncrementBuilder {
        IncrementBuilder(Increment::default())
    }

    pub fn set_amount(&mut self, value: u64) {
        self.amount = value;
    }

    pub fn amount(&self) -> u64 {
        self.amount
    }

    pub fn set_initial(&mut self, value: u64) {
        self.initial = value;
    }

    pub fn initial(&self) -> u64 {
        self.initial
    }

    pub fn set_expiration<T: Expiration>(&mut self, value: T) {
        self.expiration = value.get_timeout();
    }

    pub fn expiration(&self) -> u32 {
        self.expiration
    }

}

pub struct IncrementBuilder(Increment);

impl IncrementBuilder {
    pub fn amount(mut self, amount: u64) -> Self {
        self.0.set_amount(amount);
        self
    }

    pub fn initial(mut self, initial: u64) -> Self {
        self.0.set_initial(initial);
        self
    }

    pub fn expiration<T: Expiration>(mut self, expiration: T) -> Self {
        self.0.set_expiration(expiration);
        self
    }

    pub fn finish(self) -> Increment {
        self.0
    }
}

impl Extras for Increment {
    fn read<T: Buf>(buf: &mut T) -> io::Result<Self> {
        Ok(Self{
            amount: buf.get_u64::<NetworkEndian>(),
            initial: buf.get_u64::<NetworkEndian>(),
            expiration: buf.get_u32::<NetworkEndian>(),
        })
    }

    fn write<T: BufMut>(&self, buf: &mut T) -> io::Result<()> {
        buf.put_u64::<NetworkEndian>(self.amount);
        buf.put_u64::<NetworkEndian>(self.initial);
        buf.put_u32::<NetworkEndian>(self.expiration);
        Ok(())
    }

}

impl Default for Increment {
    fn default() -> Self {
        Self {
            amount: 0,
            initial: 0,
            expiration: 0,
        }
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

