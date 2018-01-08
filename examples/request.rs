extern crate memcache_proto;

use memcache_proto::{Request, command, extras};

fn main() {
    let extras = extras::Get::new(0xdeadbeef);
    let request: Request<_, _, ()> = Request::build()
        .command(command::Get)
        .key(Some(Box::new("hello")))
        .finish();

    println!("{:#?}", request);
}
