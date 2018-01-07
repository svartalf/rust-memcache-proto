use super::Extras;

/// Extras container for `Increment` requests.
///
/// Since `Decrement` requests use the same format, [Decrement type alias](type.Decrement.html)
/// can be used in order to provide consistent interface.
///
/// # Examples
///
/// ```
/// use memcache_proto::extras::Increment;
///
/// let extras = Increment::new(0xdeadbeef);
/// ```
#[derive(Debug)]
pub struct Increment{
    amount: u64,
    initial: u64,
    expiration: u32,
}

pub type IncrementQ = Increment;

/// Extras container for `Decrement` requests.
///
/// It is an alias for [Increment](struct.Increment.html) struct,
/// see [the module documentation](struct.Increment.html) for more.
pub type Decrement = Increment;
pub type DecrementQ = Increment;

impl Increment {
    /// Create new extras container.
    pub fn new(amount: u64, initial: u64, expiration: u32) -> Increment {
        Increment {
            amount,
            initial,
            expiration,
        }
    }

    /// Returns a reference to the associated amount.
    ///
    /// # Examples
    ///
    /// ```
    /// use memcache_proto::extras::Increment;
    ///
    /// let get = Increment::new(42, 0, 0);
    ///
    /// assert_eq!(*get.amount(), 42);
    /// ```
    pub fn amount(&self) -> &u64 {
        &self.amount
    }

}

impl Extras for Increment {
}
