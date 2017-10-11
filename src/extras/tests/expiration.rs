use std::time::Duration;

use extras::Expiration;

#[test]
fn test_simple_timeout() {
    let timeout: u32 = 3600;

    assert_eq!(3600u32, timeout.into_expiration());
}

#[test]
fn test_more_that_30_days_timeout() {
    let timeout: u32 = 60 * 60 * 24 * 30 + 1024;

    println!("{:?}", timeout);
    assert_eq!(0, timeout.into_expiration());
}

#[test]
fn test_small_duration() {
    let timeout = Duration::new(3600, 0);

    assert_eq!(3600u32, timeout.into_expiration());
}
