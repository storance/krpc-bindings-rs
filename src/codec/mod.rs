mod decode;
mod encode;

pub use crate::codec::decode::*;
pub use crate::codec::encode::*;

use protobuf::ProtobufError;

#[derive(Debug, Fail)]
pub enum CodecError {
    #[fail(display = "Protobuf Error: {}", _0)]
    ProtobufError(ProtobufError),
    #[fail(display = "Invalid enum value {}", _0)]
    InvalidEnumValue(i64),
    #[fail(display = "Value was unexpectedly null")]
    NullValue,
    #[fail(display = "Expected tuple length {} but was {}", expected, actual)]
    MismatchedTupleLength { actual: usize, expected: usize },
}

impl CodecError {
    pub fn mismatched_tuple_length(actual: usize, expected: usize) -> CodecError {
        CodecError::MismatchedTupleLength { actual, expected }
    }
}

impl From<ProtobufError> for CodecError {
    fn from(err: ProtobufError) -> Self {
        CodecError::ProtobufError(err)
    }
}
