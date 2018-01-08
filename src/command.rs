//! Available protocol commands.
//!
//! Reference: <https://github.com/memcached/memcached/wiki/BinaryProtocolRevamped#command-opcodes>

use std::fmt;
use std::default;

use extras;

// It might seems too verbose, but we can validate `Extras` object at compile time,
// which is better that storing `Box<Extras>` and downcasting each time in a hope that
// everything is okay.
pub trait Command: fmt::Debug + default::Default {
    type RequestExtras: extras::Extras;
    type ResponseExtras: extras::Extras;

    #[inline]
    fn value(&self) -> u8;
}

macro_rules! commands {
    (
        $(
            $name:ident{$value:expr, request: $request:ty, response: $response:ty},
        )*
    ) => {
        $(
            #[derive(Debug, Copy, Clone, PartialEq)]
            pub struct $name;

            impl Command for $name {
                type RequestExtras = $request;
                type ResponseExtras = $response;

                #[inline]
                fn value(&self) -> u8 {
                    $value
                }
            }

            impl default::Default for $name {
                fn default() -> Self {
                    Self{}
                }
            }
        )*
    }
}

commands! {
    Get{0x00, request: (), response: extras::Get},
    Set{0x01, request: extras::Set, response: ()},
    Add{0x02, request: extras::Add, response: ()},
    Replace{0x03, request: extras::Replace, response: ()},
    Delete{0x04, request: (), response: ()},
    Increment{0x05, request: extras::Increment, response: ()},
    Decrement{0x06, request: extras::Decrement, response: ()},
    Quit{0x07, request: (), response: ()},
    Flush{0x08, request: extras::Flush, response: ()},
    GetQ{0x09, request: (), response: extras::GetQ},
    Noop{0x0a, request: (), response: ()},
    Version{0x0b, request: (), response: ()},
    GetK{0x0c, request: (), response: extras::GetK},
    GetKQ{0x0d, request: (), response: extras::GetKQ},
    Append{0x0e, request: (), response: ()},
    Prepend{0xf, request: (), response: ()},
    Stat{0x10, request: (), response: ()},
    SetQ{0x11, request: extras::SetQ, response: ()},
    AddQ{0x12, request: extras::AddQ, response: ()},
    ReplaceQ{0x13, request: extras::ReplaceQ, response: ()},
    DeleteQ{0x14, request: (), response: ()},
    IncrementQ{0x15, request: extras::IncrementQ, response: ()},
    DecrementQ{0x16, request: extras::DecrementQ, response: ()},
    QuitQ{0x17, request: (), response: ()},
    FlushQ{0x18, request: extras::FlushQ, response: ()},
    AppendQ{0x19, request: (), response: ()},
    PrependQ{0x1a, request: (), response: ()},
    Verbosity{0x1b, request: extras::Verbosity, response: ()},
    Touch{0x1c, request: (), response: ()},  // TODO: Missing extras
    Gat{0x1d, request: (), response: ()},  // TODO: Missing extras
    GatQ{0x1e, request: (), response: ()},  // TODO: Missing extras

    // TODO: All following commands are probably missing extras too
    SaslListMechs{0x20, request: (), response: ()},
    SaslAuth{0x21, request: (), response: ()},
    SaslStep{0x22, request: (), response: ()},
    RGet{0x30, request: (), response: ()},
    RSet{0x31, request: (), response: ()},
    RSetQ{0x32, request: (), response: ()},
    RAppend{0x33, request: (), response: ()},
    RAppendQ{0x34, request: (), response: ()},
    RPrepend{0x35, request: (), response: ()},
    RPrependQ{0x36, request: (), response: ()},
    RDelete{0x37, request: (), response: ()},
    RDeleteQ{0x38, request: (), response: ()},
    RIncr{0x39, request: (), response: ()},
    RIncrQ{0x3a, request: (), response: ()},
    RDecr{0x3b, request: (), response: ()},
    RDecrQ{0x3c, request: (), response: ()},
    SetVBucket{0x3d, request: (), response: ()},
    GetVBucket{0x3e, request: (), response: ()},
    DelVBucket{0x3f, request: (), response: ()},
    TapConnect{0x40, request: (), response: ()},
    TapMutation{0x41, request: (), response: ()},
    TapDelete{0x42, request: (), response: ()},
    TapFlush{0x43, request: (), response: ()},
    TapOpaque{0x44, request: (), response: ()},
    TapVBucketSet{0x45, request: (), response: ()},
    TapCheckpointStart{0x46, request: (), response: ()},
    TapCheckpointEnd{0x47, request: (), response: ()},
}
