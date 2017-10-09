#![feature(test)]
extern crate test;
extern crate memcache_proto;

use test::Bencher;

use memcache_proto::{Request, Command};

#[bench]
fn bench_request_get_serialization(b: &mut Bencher) {
    let mut request = Request::new(Command::Get);
    request.set_key(Some(b"Hello"));

    let mut result: Vec<u8> = Vec::with_capacity(32);
    b.iter(|| {
        request.write(&mut result).unwrap();
        result.clear();
    });
}
