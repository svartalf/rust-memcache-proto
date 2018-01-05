use std::error;
use std::error::Error;
use std::io;
use std::fmt;
use std::convert;

pub enum ResponseError {
    UnknownMagic,
    UnknownCommand,
    UnknownDataType,
    UnknownStatus,
    InvalidData(io::Error),
}

impl error::Error for ResponseError {
    fn description(&self) -> &str {
        unimplemented!()
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            ResponseError::InvalidData(ref error) => Some(error),
            _ => None,
        }
    }
}

impl convert::From<io::Error> for ResponseError {
    fn from(error: io::Error) -> Self {
        ResponseError::InvalidData(error)
    }
}

impl fmt::Debug for ResponseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("ResponseError")
            .field("kind", &self)
            .finish()
    }
}

impl fmt::Display for ResponseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.description())
    }
}
