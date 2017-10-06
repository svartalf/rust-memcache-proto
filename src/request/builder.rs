use std::default::Default;

use super::{Command, Request};

pub struct RequestBuilder(Request);

impl RequestBuilder {
    pub fn new(command: Command) -> Self {
        RequestBuilder(Request{
            opcode: command,
            ..Request::default()
        })
    }

    pub fn vbucket_id(mut self, vbucket_id: u16) -> Self {
        self.0.set_vbucket_id(vbucket_id);
        self
    }

    pub fn opaque(mut self, opaque: u32) -> Self {
        self.0.set_opaque(opaque);
        self
    }

    pub fn cas(mut self, cas: u64) -> Self {
        self.0.set_cas(cas);
        self
    }

    pub fn extras<T: AsRef<[u8]>>(mut self, extras: Option<T>) -> Self {
        self.0.set_extras(extras);
        self
    }

    pub fn key<T: AsRef<[u8]>>(mut self, key: Option<T>) -> Self {
        self.0.set_key(key);
        self
    }

    pub fn value<T: AsRef<[u8]>>(mut self, value: Option<T>) -> Self {
        self.0.set_value(value);
        self
    }

    pub fn finish(self) -> Request {
        self.0
    }

}
