use std::convert::Into;
use std::time::{SystemTime, Duration, UNIX_EPOCH};

// Expiration times are specified in unsigned integer seconds.
// They can be set from 0, meaning "never expire", to 30 days (60*60*24*30).
// Any time higher than 30 days is interpreted as a unix timestamp date.
// If you want to expire an object on january 1st of next year, this is how you do that.

const MAX_SECONDS: u32 = 60 * 60 * 24 * 30;

/// Reference: https://github.com/memcached/memcached/wiki/Programming#expiration
pub trait Expiration {
    fn into_expiration(self) -> u32;

    fn convert(value: u32) -> u32 {
        println!("max: {}, val: {}", MAX_SECONDS, value);
        if value > MAX_SECONDS {
            let start = SystemTime::now();
            // TODO: Handle `.unwrap()` error
            let ts = start.duration_since(UNIX_EPOCH).unwrap();
            println!("now: {:?} - {:?}", start, ts);
            ts.checked_add(Duration::new(value as u64, 0));
            // TODO: Possible value truncation
            ts.as_secs() as u32
        } else {
            value
        }
    }
}

impl<T: Into<u32>> Expiration for T {
    fn into_expiration(self) -> u32 {
        Self::convert(self.into())
    }
}
