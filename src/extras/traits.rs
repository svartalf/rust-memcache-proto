use std::convert::Into;
use std::time::{SystemTime, Duration, UNIX_EPOCH};

// Expiration times are specified in unsigned integer seconds.
// They can be set from 0, meaning "never expire", to 30 days (60*60*24*30).
// Any time higher than 30 days is interpreted as a unix timestamp date.
// If you want to expire an object on january 1st of next year, this is how you do that.

pub const MAX_SECONDS: u32 = 60 * 60 * 24 * 30;

/// Expiration timeout
///
/// Reference: https://github.com/memcached/memcached/wiki/Programming#expiration
pub trait Expiration {
    /// Convert object into a memcached expiration timeout.
    ///
    /// Implementors are required to call [convert_expiration()](#method.convert_expiration)
    /// in order to convert value properly.
    fn into_expiration(self) -> u32;

    /// Ensure that expiration timeout is not higher than "30 days" (60*60*24*30 seconds),
    /// and if it is - converts it into a UNIX timestamp date.
    fn convert_expiration(value: u32) -> u32 {
        if value > MAX_SECONDS {
            let now = SystemTime::now()
                // TODO: Handle `.unwrap()` error
                .duration_since(UNIX_EPOCH).unwrap();
            println!("code ts: {:?}", now);
            now.checked_add(Duration::new(value as u64, 0))
                // TODO: Possible value truncation
                .unwrap().as_secs() as u32
        } else {
            value
        }
    }
}


impl Expiration for u8 {
    fn into_expiration(self) -> u32 {
        self as u32
    }
}

impl Expiration for u16 {
    fn into_expiration(self) -> u32 {
        self as u32
    }
}

impl Expiration for u32 {
    fn into_expiration(self) -> u32 {
        Self::convert_expiration(self)
    }
}

impl Expiration for Duration {
    fn into_expiration(self) -> u32 {
        // TODO: Possible value truncation
        Self::convert_expiration(self.as_secs() as u32)
    }
}