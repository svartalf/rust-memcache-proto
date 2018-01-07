use std::fmt;
use std::default::Default;

use super::Extras;

/// Extras container for `Set` requests.
///
/// All fields are zeroed by default.
///
/// Since `Add` and `Replace` requests use the same format,
/// [Add](type.Add.html) and [Replace](type.Replace.html) type aliases can be used
/// fin order to provide consistent interface.
pub struct Set{
    flags: u32,
    expiration: u32,
}

pub type SetQ = Set;

/// Extras container for `Add` requests.
///
/// It is an alias for [Set](struct.Set.html) struct,
/// see [the module documentation](struct.Set.html) for more.
pub type Add = Set;
pub type AddQ = Set;

/// Extras container for `Replace` requests.
///
/// It is an alias for [Set](struct.Set.html) struct,
/// see [the module documentation](struct.Set.html) for more.
pub type Replace = Set;
pub type ReplaceQ = Set;

impl Set {

    pub fn new(flags: u32, expiration: u32) -> Set {
        Self {
            flags,
            expiration,
        }
    }

    pub fn flags(&self) -> u32 {
        self.flags
    }

    pub fn expiration(&self) -> u32 {
        self.expiration
    }
}


impl Extras for Set {
}

impl Default for Set {
    fn default() -> Self {
        Self {
            flags: 0,
            expiration: 0,
        }
    }
}

impl fmt::Debug for Set {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Set")
            .field("flags", &self.flags)
            .field("expiration", &self.expiration)
            .finish()
    }
}

