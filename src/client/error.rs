use super::schema;
use crate::codec::CodecError;

use std::io;

#[derive(Debug, Clone, Fail)]
pub enum ConnectionError {
    #[fail(display = "Timeout: {}", _0)]
    Timeout(String),
    #[fail(display = "Malformed Message: {}", _0)]
    MalformedMessage(String),
    #[fail(display = "Wrong Type: {}", _0)]
    WrongType(String),
}

#[derive(Debug, Clone, Fail)]
pub enum StreamError {
    #[fail(display = "The stream has not been started.")]
    NotStarted,
    #[fail(display = "The stream has been removed.")]
    Removed,
}

#[derive(Debug, Clone, Fail)]
pub enum ResponseError {
    #[fail(display = "{}.{}: {}", service, name, description)]
    Other {
        service: String,
        name: String,
        description: String,
        stack_trace: String,
    },
    #[fail(display = "Invalid Operation: {}", _0)]
    InvalidOperation(String),
    #[fail(display = "Invalid Argument: {}", _0)]
    InvalidArgument(String),
    #[fail(display = "Null Argument: {}", _0)]
    NullArgument(String),
    #[fail(display = "Argument Out of Range: {}", _0)]
    ArgumentOutOfRange(String),
    #[fail(display = "No result returned for the rpc call.")]
    MissingResult,
}

impl From<schema::Error> for ResponseError {
    fn from(err: schema::Error) -> Self {
        Self::from(&err)
    }
}
impl From<&schema::Error> for ResponseError {
    fn from(err: &schema::Error) -> Self {
        if err.get_service().eq("KRPC") && err.get_name().eq("InvalidOperationException") {
            ResponseError::InvalidOperation(err.get_description().to_owned())
        } else if err.get_service().eq("KRPC") && err.get_name().eq("ArgumentException") {
            ResponseError::InvalidArgument(err.get_description().to_owned())
        } else if err.get_service().eq("KRPC") && err.get_name().eq("ArgumentNullException") {
            ResponseError::NullArgument(err.get_description().to_owned())
        } else if err.get_service().eq("KRPC") && err.get_name().eq("ArgumentOutOfRangeException") {
            ResponseError::ArgumentOutOfRange(err.get_description().to_owned())
        } else {
            ResponseError::Other {
                service: err.get_service().to_owned(),
                name: err.get_name().to_owned(),
                description: err.get_description().to_owned(),
                stack_trace: err.get_stack_trace().to_owned(),
            }
        }
    }
}

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "IO Error: {}", _0)]
    IoError(#[fail(cause)] io::Error),
    #[fail(display = "Codec Error: {}", _0)]
    CodecError(#[fail(cause)] CodecError),
    #[fail(display = "Response Error: {}", _0)]
    ResponseError(#[fail(cause)] ResponseError),
    #[fail(display = "Connection Error: {}", _0)]
    ConnectionError(#[fail(cause)] ConnectionError),
    #[fail(display = "Stream Error: {}", _0)]
    StreamError(#[fail(cause)] StreamError),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IoError(err)
    }
}

impl From<CodecError> for Error {
    fn from(err: CodecError) -> Self {
        Error::CodecError(err)
    }
}

impl From<ResponseError> for Error {
    fn from(err: ResponseError) -> Self {
        Error::ResponseError(err)
    }
}

impl From<ConnectionError> for Error {
    fn from(err: ConnectionError) -> Self {
        Error::ConnectionError(err)
    }
}

impl From<StreamError> for Error {
    fn from(err: StreamError) -> Self {
        Error::StreamError(err)
    }
}
