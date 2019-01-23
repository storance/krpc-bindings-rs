use crate::codec::*;
use crate::krpc::Expression;

use std::net::TcpStream;
use std::sync::Arc;

use protobuf::{CodedInputStream, CodedOutputStream, ProtobufError};

mod error;
mod rpc;
pub mod schema;
mod stream;

pub use self::error::*;
pub use self::stream::{Event, Stream};

use self::rpc::Rpc;
use self::stream::StreamRaw;

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

fn convert_procedure_result(
    result: &schema::ProcedureResult,
) -> Result<Vec<u8>, error::ResponseError> {
    if result.has_error() {
        Err(error::ResponseError::from(result.get_error()))
    } else {
        Ok(Vec::from(result.get_value()))
    }
}

pub struct Connection {
    rpc: rpc::Rpc,
    stream: stream::StreamManager,
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
        let stream = stream::StreamManager::connect(rpc.id(), host, stream_port)?;

        Ok(Connection { rpc, stream })
    }

    pub fn id(&self) -> &[u8] {
        self.rpc.id()
    }

    pub fn invoke(
        &self,
        service: &str,
        procedure: &str,
        args: &Vec<Vec<u8>>,
    ) -> RpcResult<Vec<u8>> {
        self.rpc.invoke(service, procedure, args)
    }

    pub fn add_event<'a>(&'a self, expr: &Expression) -> StreamResult<Event<'a>> {
        let response = self.rpc.invoke("KRPC", "AddEvent", &vec![expr.encode()?])?;

        let event: schema::Event = decode(&response, self)?;
        let id = event.get_stream().get_id();
        let stream_value = Arc::new(StreamRaw::new(id, false));

        self.stream.register(stream_value.clone());

        Ok(Event::new(Stream::new(self, stream_value)))
    }

    pub fn add_stream<'a, T: Decode<'a>>(
        &'a self,
        service: &str,
        procedure: &str,
        args: &Vec<Vec<u8>>,
        start: bool,
    ) -> StreamResult<Stream<'a, T>> {
        let response = self.rpc.invoke(
            "KRPC",
            "AddStream",
            &vec![
                Rpc::create_procedure_call(service, procedure, args).encode()?,
                start.encode()?,
            ],
        )?;

        let stream: schema::Stream = decode(&response, self)?;
        let id = stream.get_id();
        let stream_value = Arc::new(StreamRaw::new(id, start));

        self.stream.register(stream_value.clone());

        Ok(Stream::new(self, stream_value))
    }

    fn deregister_stream(&self, stream_id: u64) {
        self.stream.deregister(stream_id);
    }
}

/// Result type alias for RPC calls.
pub type RpcResult<T> = Result<T, RpcError>;
/// Result type alias for stream calls.
pub type StreamResult<T> = Result<T, StreamError>;
