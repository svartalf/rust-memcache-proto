use std::fmt;
use std::default;

use extras;

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
    Get{0x00, request: extras::Get, response: ()},
    Set{0x01, request: (), response: ()},
    Add{0x02, request: (), response: ()},
    Replace{0x03, request: (), response: ()},
    Delete{0x04, request: (), response: ()},
    Increment{0x05, request: (), response: ()},
    Decrement{0x06, request: (), response: ()},
    Quit{0x07, request: (), response: ()},
    Flush{0x08, request: (), response: ()},
}

//enum_from_primitive! {
//
//    /// Available protocol commands.
//    ///
//    /// Reference: <https://github.com/memcached/memcached/wiki/BinaryProtocolRevamped#command-opcodes>
//    #[derive(Debug, Copy, Clone, PartialEq)]
//    pub enum Command {
//        Get = 0x00,
//        Set = 0x01,
//        Add = 0x02,
//        Replace = 0x03,
//        Delete = 0x04,
//        Increment = 0x05,
//        Decrement = 0x06,
//        Quit = 0x07,
//        Flush = 0x08,
//        GetQ = 0x09,
//        Noop = 0x0a,
//        Version = 0x0b,
//        GetK = 0x0c,
//        GetKQ = 0x0d,
//        Append = 0x0e,
//        Prepend = 0x0f,
//        Stat = 0x10,
//        SetQ = 0x11,
//        AddQ = 0x12,
//        ReplaceQ = 0x13,
//        DeleteQ = 0x14,
//        IncrementQ = 0x15,
//        DecrementQ = 0x16,
//        QuitQ = 0x17,
//        FlushQ = 0x18,
//        AppendQ = 0x19,
//        PrependQ = 0x1a,
//        Verbosity = 0x1b,
//        Touch = 0x1c,
//        Gat = 0x1d,
//        GatQ = 0x1e,
//        SaslListMechs = 0x20,
//        SaslAuth = 0x21,
//        SaslStep = 0x22,
//        RGet = 0x30,
//        RSet = 0x31,
//        RSetQ = 0x32,
//        RAppend = 0x33,
//        RAppendQ = 0x34,
//        RPrepend = 0x35,
//        RPrependQ = 0x36,
//        RDelete = 0x37,
//        RDeleteQ = 0x38,
//        RIncr = 0x39,
//        RIncrQ = 0x3a,
//        RDecr = 0x3b,
//        RDecrQ = 0x3c,
//        SetVBucket = 0x3d,
//        GetVBucket = 0x3e,
//        DelVBucket = 0x3f,
//        TapConnect = 0x40,
//        TapMutation = 0x41,
//        TapDelete = 0x42,
//        TapFlush = 0x43,
//        TapOpaque = 0x44,
//        TapVBucketSet = 0x45,
//        TapCheckpointStart = 0x46,
//        TapCheckpointEnd = 0x47,
//    }
//}
