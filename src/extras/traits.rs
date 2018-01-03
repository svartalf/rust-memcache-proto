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
    /// Convert object into a seconds timeout.
    ///
    /// Implementations should not bother about wrapping around magic "30 days" limit,
    /// just return seconds value.
    fn as_expiration(&self) -> u32;

    fn get_timeout(&self) -> u32 {
        let value = self.as_expiration();
        if value > MAX_SECONDS {
            let now = SystemTime::now()
                // TODO: Handle `.unwrap()` error
                .duration_since(UNIX_EPOCH).unwrap();
            now.checked_add(Duration::new(value as u64, 0))
                // TODO: Possible value truncation
                .unwrap().as_secs() as u32
        } else {
            value
        }
    }
}


impl Expiration for u8 {
    fn as_expiration(&self) -> u32 {
        *self as u32
    }
}

impl Expiration for u16 {
    fn as_expiration(&self) -> u32 {
        *self as u32
    }
}

impl Expiration for u32 {
    fn as_expiration(&self) -> u32 {
        *self as u32
    }
}

impl Expiration for Duration {
    fn as_expiration(&self) -> u32 {
        // TODO: Possible value truncation
        self.as_secs() as u32
    }
}
