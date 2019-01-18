use super::schema;
use crate::codec::CodecError;

use protobuf::ProtobufError;
use std::io;

#[derive(Debug, Fail)]
pub enum ConnectionError {
    #[fail(display = "IO Error: {}", _0)]
    IoError(#[fail(cause)] io::Error),
    #[fail(display = "Protobuf Error: {}", _0)]
    ProtobufError(#[fail(cause)] ProtobufError),
    #[fail(display = "Connection Error: {}", _0)]
    ConnectionError(String),
}

impl From<io::Error> for ConnectionError {
    fn from(err: io::Error) -> Self {
        ConnectionError::IoError(err)
    }
}

impl From<ProtobufError> for ConnectionError {
    fn from(err: ProtobufError) -> Self {
        ConnectionError::ProtobufError(err)
    }
}

#[derive(Debug, Fail)]
pub enum StreamError {
    #[fail(display = "RPC Error: {}", _0)]
    RpcError(#[fail(cause)] RpcError),
    #[fail(display = "The stream has not been started.")]
    NotStarted,
    #[fail(display = "The stream could not be found.")]
    NotFound,
}

impl From<RpcError> for StreamError {
    fn from(err: RpcError) -> Self {
        StreamError::RpcError(err)
    }
}

impl From<ResponseError> for StreamError {
    fn from(err: ResponseError) -> Self {
        StreamError::RpcError(RpcError::from(err))
    }
}

impl From<CodecError> for StreamError {
    fn from(err: CodecError) -> Self {
        StreamError::RpcError(RpcError::from(err))
    }
}

#[derive(Debug, Clone, Fail)]
pub enum ResponseError {
    #[fail(display = "{}", description)]
    ResponseError {
        service: String,
        name: String,
        description: String,
        stack_trace: String,
    },
    #[fail(display = "No result returned for the rpc call.")]
    MissingResult,
}

impl From<schema::Error> for ResponseError {
    fn from(err: schema::Error) -> Self {
        ResponseError::ResponseError {
            service: err.get_service().to_owned(),
            name: err.get_name().to_owned(),
            description: err.get_description().to_owned(),
            stack_trace: err.get_description().to_owned(),
        }
    }
}
impl From<&schema::Error> for ResponseError {
    fn from(err: &schema::Error) -> Self {
        ResponseError::ResponseError {
            service: err.get_service().to_owned(),
            name: err.get_name().to_owned(),
            description: err.get_description().to_owned(),
            stack_trace: err.get_description().to_owned(),
        }
    }
}

#[derive(Debug, Fail)]
pub enum RpcError {
    #[fail(display = "Protobuf Error: {}", _0)]
    ProtobufError(#[fail(cause)] ProtobufError),
    #[fail(display = "Codec Error: {}", _0)]
    CodecError(#[fail(cause)] CodecError),
    #[fail(display = "ResponseError: {}", _0)]
    ResponseError(#[fail(cause)] ResponseError),
}

impl From<ProtobufError> for RpcError {
    fn from(err: ProtobufError) -> Self {
        RpcError::ProtobufError(err)
    }
}

impl From<CodecError> for RpcError {
    fn from(err: CodecError) -> Self {
        RpcError::CodecError(err)
    }
}

impl From<ResponseError> for RpcError {
    fn from(err: ResponseError) -> Self {
        RpcError::ResponseError(err)
    }
}
