extern crate memcache_proto;

use memcache_proto::{Request, Command};
use memcache_proto::extras::Decrement;

fn main() {
    let get = Decrement::new().with_amount(100);
    let mut request = Request::new(Command::Decrement);
    request.set_extras(&get);
    println!("{:#?}", request);
}
