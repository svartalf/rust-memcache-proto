extern crate memcache_proto;

use memcache_proto::{Request, Command};

fn main() {
    let request = Request::build(Command::Set)
        .key(Some("hello"))
        .value(Some("world"))
        .cas(1024)
        .finish();

    println!("{:#?}", request);
}