use super::Extras;

/// Extras container for `Increment` requests.
///
/// Since `IncrementQ`, `Decrement` and `DecrementQ` requests are using the same format,
/// associated type alias can be used in order to provide consistent interface.
///
/// See [IncrementQ](type.IncrementQ.html), [Decrement](type.Decrement.html)
/// and [DecrementQ](type.DecrementQ.html) type aliases for more.
///
/// # Examples
///
/// ```
/// use memcache_proto::extras::Increment;
///
/// let extras = Increment::new(10, 0, 3600);
/// ```
#[derive(Debug)]
pub struct Increment {
    amount: u64,
    initial: u64,
    expiration: u32,
}

/// Extras container for `IncrementQ` requests.
///
/// It is an alias for [Increment](struct.Increment.html) struct,
/// see [the module documentation](struct.Increment.html) for more.
pub type IncrementQ = Increment;

/// Extras container for `Decrement` requests.
///
/// It is an alias for [Increment](struct.Increment.html) struct,
/// see [the module documentation](struct.Increment.html) for more.
pub type Decrement = Increment;

/// Extras container for `DecrementQ` requests.
///
/// It is an alias for [Increment](struct.Increment.html) struct,
/// see [the module documentation](struct.Increment.html) for more.
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

    /// Returns a mutable reference to the associated amount.
    ///
    /// # Examples
    ///
    /// ```
    /// use memcache_proto::extras::Increment;
    ///
    /// let mut get = Increment::new(0, 0, 0);
    /// *get.amount_mut() = 42;
    ///
    /// assert_eq!(*get.amount(), 42);
    /// ```
    pub fn amount_mut(&mut self) -> &mut u64 {
        &mut self.amount
    }

    /// Returns a reference to the associated initial value.
    ///
    /// # Examples
    ///
    /// ```
    /// use memcache_proto::extras::Increment;
    ///
    /// let get = Increment::new(0, 42, 0);
    ///
    /// assert_eq!(*get.initial(), 42);
    /// ```
    pub fn initial(&self) -> &u64 {
        &self.initial
    }

    /// Returns a mutable reference to the associated initial value.
    ///
    /// # Examples
    ///
    /// ```
    /// use memcache_proto::extras::Increment;
    ///
    /// let mut get = Increment::new(0, 0, 0);
    /// *get.initial_mut() = 42;
    ///
    /// assert_eq!(*get.initial(), 42);
    /// ```
    pub fn initial_mut(&mut self) -> &mut u64 {
        &mut self.initial
    }

    /// Returns a reference to the associated expiration value.
    ///
    /// # Examples
    ///
    /// ```
    /// use memcache_proto::extras::Increment;
    ///
    /// let get = Increment::new(0, 0, 3600);
    ///
    /// assert_eq!(*get.expiration(), 3600);
    /// ```
    pub fn expiration(&self) -> &u32 {
        &self.expiration
    }

    /// Returns a mutable reference to the associated expiration value.
    ///
    /// # Examples
    ///
    /// ```
    /// use memcache_proto::extras::Increment;
    ///
    /// let mut get = Increment::new(0, 0, 0);
    /// *get.expiration_mut() = 3600;
    ///
    /// assert_eq!(*get.expiration(), 3600);
    /// ```
    pub fn expiration_mut(&mut self) -> &mut u32 {
        &mut self.expiration
    }
}

impl Extras for Increment {
}
