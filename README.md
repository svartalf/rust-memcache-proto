# memcache-proto

Memcached binary protocol implementation for Rust.

This is not an actual client, but only the wrappers
around protocol packets with encoding/decoding ability.

Main idea is to provide common structures for other crates
which are will provide client functionality (ex. via tokio or system threads).
