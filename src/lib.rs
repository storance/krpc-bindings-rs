extern crate krpc;
extern crate protobuf;
extern crate uom;

pub mod spacecenter;
pub mod units;
#[macro_use]
mod macros;
mod codec;

use krpc::rpc::{Rpc};
use krpc::stream::{Stream};
use krpc::{TransceiverError};

use protobuf::{ProtobufError};

use std::rc::{Rc};
use std::cell::{RefCell};
use std::error::{Error};
use std::fmt;
use std::io;


use crate::codec::{CodecError};

pub trait RemoteObject {
    fn new(client: Rc<RefCell<KrpcClient>>, id: u64) -> Self where Self: Sized;

    fn id(&self) -> u64;
}

pub trait RemoteEnum {
    fn from_value(value: i64) -> Option<Self> where Self: Sized;

    fn value(&self) -> i64;
}

pub struct KrpcClient {
    pub rpc : Rpc,
    pub stream: Stream
}

#[derive(Debug)]
pub enum KrpcError {
    ProtobufError(ProtobufError),
    SocketError(io::Error),
    ResponseHasError(String),
    CodecError(CodecError),
    NullResponseValue
}

impl fmt::Display for KrpcError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl Error for KrpcError {
    fn description(&self) -> &str {
        match self {
            &KrpcError::ProtobufError(ref e) => e.description(),
            &KrpcError::SocketError(ref e) => e.description(),
            &KrpcError::ResponseHasError(..) => "RPC call responded with an error",
            &KrpcError::CodecError(ref e) => e.description(),
            &KrpcError::NullResponseValue => "Response value was unexpectedly null"
        }
    }

    fn cause(&self) -> Option<&Error> {
        match self {
            &KrpcError::ProtobufError(ref e) => Some(e),
            &KrpcError::SocketError(ref e) => Some(e),
            &KrpcError::ResponseHasError(..) => None,
            &KrpcError::CodecError(ref e) => Some(e),
            &KrpcError::NullResponseValue => None
        }
    }
}

impl From<TransceiverError> for KrpcError {
    fn from(err: TransceiverError) -> Self {
        match err {
            TransceiverError::Protobuf(e) => KrpcError::ProtobufError(e),
            TransceiverError::Socket(e) => KrpcError::SocketError(e),
            TransceiverError::ResponseHasError(str) => KrpcError::ResponseHasError(str),
        }
    }
}

impl From<CodecError> for KrpcError {
    fn from(err: CodecError) -> Self {
        KrpcError::CodecError(err)
    }
}

pub type KrpcResult<T> = Result<T, KrpcError>;