use super::Extras;

/// Extras container for `Flush` requests.
///
/// Since `FlushQ` requests are using the same format,
/// associated type alias can be used in order to provide consistent interface.
///
/// See [FlushQ](type.FlushQ.html) type alias for more.
///
/// # Examples
///
/// ```
/// use memcache_proto::extras::Flush;
///
/// let extras = Flush::new(3600);
/// ```
#[derive(Debug)]
pub struct Flush {
    expiration: u32,
}

impl Flush {
    pub fn new(expiration: u32) -> Flush {
        Flush {
            expiration,
        }
    }

    pub fn expiration(&self) -> &u32 {
        &self.expiration
    }

    pub fn expiration_mut(&mut self) -> &mut u32 {
        &mut self.expiration
    }
}

impl Extras for Flush {}

/// Extras container for `FlushQ` requests.
///
/// It is an alias for [Flush](struct.Flush.html) struct,
/// see [the module documentation](struct.Flush.html) for more.
pub type FlushQ = Flush;
