mod encode;
mod decode;

pub use crate::codec::encode::*;
pub use crate::codec::decode::*;

use protobuf::{ProtobufError};

use std::error::{Error};
use std::fmt;

#[derive(Debug)]
pub enum CodecError {
    ProtobufError(ProtobufError),
    InvalidEnumValue(i64),
    NullValue,
    MismatchedTupleLength {
        actual: usize,
        expected: usize
    }
}

impl CodecError {
    pub fn mismatched_tuple_length(actual: usize, expected: usize) -> CodecError {
        CodecError::MismatchedTupleLength {actual, expected}
    }
}

impl fmt::Display for CodecError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl Error for CodecError {
    fn description(&self) -> &str {
        match self {
            &CodecError::ProtobufError(ref e) => e.description(),
            &CodecError::InvalidEnumValue(..) => "Invalid enum value",
            &CodecError::NullValue => "Value was unexpectedly null",
            &CodecError::MismatchedTupleLength{..} => "Actual tuple length does not match the expected tuple length",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match self {
            &CodecError::ProtobufError(ref e) => Some(e),
            _ => None
        }
    }
}

impl From<ProtobufError> for CodecError {
    fn from(err: ProtobufError) -> Self {
        CodecError::ProtobufError(err)
    }
}