use {Request, command};


pub struct Builder<C, K, V>(Request<C, K, V>)
    where C: command::Command;

impl<C, K, V> Builder<C, K, V> where C: command::Command {

    pub fn new() -> Builder<C, K, V> {
        Builder(Request::new())
    }

    pub fn command(mut self, value: C) -> Self {
        *self.0.command_mut() = value;
        self
    }

    pub fn vbucket_id(mut self, value: u16) -> Self {
        *self.0.vbucket_id_mut() = value;
        self
    }

    pub fn opaque(mut self, value: u32) -> Self {
        *self.0.opaque_mut() = value;
        self
    }

    pub fn cas(mut self, value: u64) -> Self {
        *self.0.cas_mut() = value;
        self
    }

    pub fn extras(mut self, value: Option<C::RequestExtras>) -> Self {
        *self.0.extras_mut() = value;
        self
    }

    pub fn key(mut self, value: Option<K>) -> Self {
        *self.0.key_mut() = value;
        self
    }

    pub fn value(mut self, value: Option<V>) -> Self {
        *self.0.value_mut() = value;
        self
    }

    pub fn finish(self) -> Request<C, K, V> {
        self.0
    }
}
