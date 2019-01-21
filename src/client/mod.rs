use std::net::TcpStream;
use std::rc::Rc;
use std::sync::Arc;

use crate::codec::{decode_message, Encode};
use protobuf::{CodedInputStream, CodedOutputStream, ProtobufError};

mod error;
mod rpc;
pub mod schema;
mod stream;

pub use self::error::*;
pub use self::stream::{StreamRawValue, StreamValue};

use self::rpc::Rpc;
use self::schema::ProcedureResult;
use self::schema::Stream;

pub const DEFAULT_RPC_PORT: u16 = 50000;
pub const DEFAULT_STREAM_PORT: u16 = 50001;

fn send_msg<T: protobuf::Message>(
    socket: &mut TcpStream,
    message: &T,
) -> Result<(), ProtobufError> {
    let mut cos = CodedOutputStream::new(socket);
    cos.write_message_no_tag(message)?;
    cos.flush()
}

fn recv_msg<T: protobuf::Message>(socket: &mut TcpStream) -> Result<T, ProtobufError> {
    let mut cis = CodedInputStream::new(socket);
    cis.read_message()
}

fn convert_procedure_result(result: &ProcedureResult) -> Result<Vec<u8>, error::ResponseError> {
    if result.has_error() {
        Err(error::ResponseError::from(result.get_error()))
    } else {
        Ok(Vec::from(result.get_value()))
    }
}

#[derive(Debug, Clone)]
pub struct Connection {
    rpc: Rc<rpc::Rpc>,
    stream: Rc<stream::StreamManager>,
}

impl Connection {
    pub fn connect(name: &str, host: &str) -> Result<Connection, ConnectionError> {
        Self::connect_with_ports(name, host, DEFAULT_RPC_PORT, DEFAULT_STREAM_PORT)
    }

    pub fn connect_with_ports(
        name: &str,
        host: &str,
        rpc_port: u16,
        stream_port: u16,
    ) -> Result<Connection, ConnectionError> {
        let rpc = rpc::Rpc::connect(name, host, rpc_port)?;
        let stream = stream::StreamManager::connect(rpc.id().clone(), host, stream_port)?;

        Ok(Connection { rpc, stream })
    }

    pub fn id(&self) -> Vec<u8> {
        self.rpc.id().clone()
    }

    pub fn invoke(
        &self,
        service: &str,
        procedure: &str,
        args: &Vec<Vec<u8>>,
    ) -> RpcResult<Vec<u8>> {
        self.rpc.invoke(service, procedure, args)
    }

    pub fn add_stream(
        &self,
        service: &str,
        procedure: &str,
        args: &Vec<Vec<u8>>,
        start: bool,
    ) -> StreamResult<Arc<StreamRawValue>> {
        let response = self.rpc.invoke(
            "KRPC",
            "AddStream",
            &vec![
                Rpc::create_procedure_call(service, procedure, args).encode()?,
                start.encode()?,
            ],
        )?;
        let stream: Stream = decode_message(&response)?;
        let id = stream.get_id();
        let stream_value = Arc::new(StreamRawValue::new(id, start));

        self.stream.register(stream_value.clone());

        Ok(stream_value)
    }

    pub fn start_stream(&self, stream: &StreamRawValue) -> StreamResult<()> {
        if stream.started()? {
            return Ok(());
        }

        self.rpc
            .invoke("KRPC", "StartStream", &vec![stream.id().encode()?])?;

        stream.set_started()?;
        Ok(())
    }

    pub fn remove_stream(&self, stream: &StreamRawValue) -> StreamResult<()> {
        self.rpc
            .invoke("KRPC", "RemoveStream", &vec![stream.id().encode()?])?;

        self.stream.deregister(stream.id());

        Ok(())
    }

    pub fn set_stream_rate(&self, stream: &StreamRawValue, rate: f32) -> StreamResult<()> {
        self.rpc.invoke(
            "KRPC",
            "SetStreamRate",
            &vec![stream.id().encode()?, rate.encode()?],
        )?;

        stream.set_rate(rate)
    }
}

/// Result type alias for RPC calls.
pub type RpcResult<T> = Result<T, RpcError>;
/// Result type alias for stream calls.
pub type StreamResult<T> = Result<T, StreamError>;
