use std::io;
use std::fmt;

use enum_primitive::FromPrimitive;
use byteorder::{ByteOrder, NetworkEndian, ReadBytesExt};

use super::{Magic, Command, DataType, ResponseError};

const HEADER_SIZE: usize = 24;

enum_from_primitive! {
    /// Response status variants
    #[derive(Debug, PartialEq)]
    pub enum Status {
        Ok = 0x0000,
        KeyNotFound = 0x0001,
        KeyExists = 0x0002,
        ValueTooLarge = 0x0003,
        InvalidArguments = 0x0004,
        ItemNotStored = 0x0005,
        IncrDecrOnNonNumericValue = 0x0006,
        VBucketBelongsToAnotherServer = 0x0007,
        AuthenticationError = 0x0008,
        AuthenticationContinue = 0x0009,
        UnknownCommand = 0x0081,
        OutOfMemory = 0x0082,
        NotSupported = 0x00083,
        InternalError = 0x0084,
        Busy = 0x0085,
        TemporaryFailure = 0x0086,
    }
}

/// Parsed `memcached` response.
pub struct Response {
    magic: Magic,
    opcode: Command,
    key_length: u16,
    extras_length: u8,
    data_type: DataType,
    status: Status,
    body_length: u32,
    opaque: u32,
    cas: u64,
    body: Vec<u8>,
}

impl Response {
    pub fn magic(&self) -> &Magic {
        &self.magic
    }

    pub fn command(&self) -> &Command {
        &self.opcode
    }

    pub fn status(&self) -> &Status {
        &self.status
    }

    pub fn data_type(&self) -> &DataType {
        &self.data_type
    }

    pub fn opaque(&self) -> &u32 {
        &self.opaque
    }

    pub fn cas(&self) -> &u64 {
        &self.cas
    }

    pub fn extras(&self) -> Option<&[u8]> {
        if self.extras_length > 0 {
            let end = self.extras_length as usize;
            return Some(&self.body[..end]);
        }
        None
    }

    pub fn key(&self) -> Option<&[u8]> {
        if self.key_length > 0 {
            let start = self.extras_length as usize;
            let end = start + self.key_length as usize;

            return Some(&self.body[start..end]);
        }

        None
    }

    pub fn value(&self) -> Option<&[u8]> {
        let start: usize = self.extras_length as usize + self.key_length as usize;
        if self.body_length as usize > start {
            return Some(&self.body[start..]);
        }

        None
    }

    /// Attempt to read a response from the `raw`.
    ///
    /// Available return values:
    ///
    ///   * `Ok(None)` - there is not enough data to read full response
    ///   * `Ok(Some(Response))` - read was successful
    ///   * `Err(..)` - invalid response was read
    ///
    /// It might be required to remove read bytes from the `raw`
    /// based on the implementation specific.
    /// `Response::len()` method should help in that.
    pub fn read(src: &[u8]) -> Result<Option<Response>, ResponseError> {
        let length = src.len();
        if length < HEADER_SIZE {
            return Ok(None);
        }

        let body_length = NetworkEndian::read_u32(&src[8..12]) as usize;
        let packet_length = HEADER_SIZE + body_length;
        if packet_length < length {  // Body is not received yet
            return Ok(None);
        }

        let mut cursor = io::Cursor::new(src);

        let response = Response {
            magic: Magic::from_u8(cursor.read_u8()?)
                .ok_or_else(|| ResponseError::UnknownMagic)?,
            opcode: Command::from_u8(cursor.read_u8()?)
                .ok_or_else(|| ResponseError::UnknownCommand)?,
            key_length: cursor.read_u16::<NetworkEndian>()?,
            extras_length: cursor.read_u8()?,
            data_type: DataType::from_u8(cursor.read_u8()?)
                .ok_or_else(|| ResponseError::UnknownDataType)?,
            status: Status::from_u16(cursor.read_u16::<NetworkEndian>()?)
                .ok_or_else(|| ResponseError::UnknownStatus)?,
            body_length: cursor.read_u32::<NetworkEndian>()?,
            opaque: cursor.read_u32::<NetworkEndian>()?,
            cas: cursor.read_u64::<NetworkEndian>()?,
            body: Vec::from(&src[(cursor.position() as usize)..(cursor.position() as usize + body_length)]),
        };

        Ok(Some(response))
    }

    pub fn len(&self) -> usize {
        HEADER_SIZE + self.body_length as usize
    }

    pub fn is_ok(&self) -> bool {
        self.status == Status::Ok
    }

    pub fn is_err(&self) -> bool {
        !self.is_ok()
    }
}

impl fmt::Debug for Response {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut resp = f.debug_struct("Response");

        resp
            .field("command", &self.opcode)
            .field("status", &self.status);

        if let Some(key) = self.key() {
            resp.field("key", &key);
        }

        if let Some(value) = self.value() {
            resp.field("value", &value);
        }

        if let Some(extras) = self.extras() {
            resp.field("extras", &extras);
        }

        resp.finish()
    }
}
