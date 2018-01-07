#[macro_use] extern crate enum_primitive;
extern crate byteorder;
extern crate bytes;

mod magic;
pub mod command;
mod data_type;
mod request;
pub mod extras;

pub use magic::Magic;
pub use command::*;
pub use data_type::DataType;
pub use request::{Request};

// TODO: https://play.rust-lang.org/?gist=2d75f12791dcc1c8264b5d2ddbb0d428&version=nightly
