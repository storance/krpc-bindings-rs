mod decode;
mod encode;

pub use crate::codec::decode::*;
pub use crate::codec::encode::*;

pub use failure::Error;

#[derive(Debug, Fail)]
pub enum CodecError {
    #[fail(display = "Invalid enum value {}", _0)]
    InvalidEnumValue(i64),
    #[fail(display = "Value was unexpectedly null")]
    NullValue,
    #[fail(display = "Expected tuple length {} but was {}", expected, actual)]
    MismatchedTupleLength { actual: usize, expected: usize },
}

pub type CodecResult<T> = Result<T, Error>;
