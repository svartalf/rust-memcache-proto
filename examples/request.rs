extern crate memcache_proto;

use memcache_proto::{Request, Command, extras};

fn main() {
    let extras = extras::Set::new()
        .with_flags(0xdeadbeef_u32)
        .with_expiration(7200_u32);
    let mut request = Request::new(Command::Set);
    request.set_key(Some(b"Hello"));
    request.set_value(Some(b"World"));
    request.set_extras(Some(extras));

    println!("{:#?}", request);
}
