use super::Extras;

#[derive(Debug)]
pub struct Flush {
    expiration: u32,
}

impl Extras for Flush {}

pub type FlushQ = Flush;
