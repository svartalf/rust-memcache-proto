use std::convert::{From, AsRef};

mod get;
mod set;
mod incr;

pub use self::get::{Get, GetK, GetQ, GetKQ};
pub use self::set::{Set, Add, Replace};
pub use self::incr::{Increment, Decrement};

pub trait Extras: Sized + AsRef<[u8]> {}

#[cfg(test)]
mod tests;
