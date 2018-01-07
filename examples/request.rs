extern crate memcache_proto;

use memcache_proto::{Request, command};

fn main() {
    let request: Request<_, _, ()> = Request::build()
        .command(command::Get)
        .key(Some(Box::new("hello")))
        .finish();

    println!("{:#?}", request);
}
