use std::io;

use bytes::BufMut;
use serde;

use Command;
use super::Request;

impl<C, K, V> Request<C, K, V>
    where C: Command, K: serde::Serialize, V: serde::Serialize {

    pub fn to_writer<S, W>(&self, serializer: &mut S) -> io::Result<()>
            where S: serde::Serializer, W: BufMut {


        unimplemented!()
    }

}
