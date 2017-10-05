# memcache-proto

Memcached binary protocol implementation for Rust.

This is not an actual client, but only the wrappers
around protocol packets with encoding/decoding ability.

Main idea is to provide common structures for other crates
which are will provide client functionality (ex. via tokio or system threads).

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
