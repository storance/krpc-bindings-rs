use crate::codec::{Decode, Encode};
use crate::krpc::Expression;

use failure::Error;
use protobuf::{CodedInputStream, CodedOutputStream};
use std::net::TcpStream;
use std::sync::Arc;
use std::fmt;

mod error;
mod rpc;
pub mod schema;
mod stream;

pub use self::error::*;
pub use self::schema::{Argument, ProcedureCall, Services, Status};
pub use self::stream::{Event, Stream};

use self::rpc::Rpc;
use self::stream::StreamRaw;

pub const DEFAULT_RPC_PORT: u16 = 50000;
pub const DEFAULT_STREAM_PORT: u16 = 50001;

fn send_msg<T: protobuf::Message>(socket: &mut TcpStream, message: &T) -> KrpcResult<()> {
    let mut cos = CodedOutputStream::new(socket);
    cos.write_message_no_tag(message)?;
    cos.flush()?;
    Ok(())
}

fn recv_msg<T: protobuf::Message>(socket: &mut TcpStream) -> KrpcResult<T> {
    let mut cis = CodedInputStream::new(socket);
    Ok(cis.read_message()?)
}

fn convert_procedure_result(result: &schema::ProcedureResult) -> Result<Vec<u8>, ResponseError> {
    if result.has_error() {
        Err(ResponseError::from(result.get_error()))
    } else {
        Ok(Vec::from(result.get_value()))
    }
}

pub struct Connection {
    name: String,
    rpc: rpc::Rpc,
    stream: stream::StreamManager,
}

impl Connection {
    pub fn connect_localhost(name: &str) -> KrpcResult<Connection> {
        Self::connect_with_ports(name, "localhost", DEFAULT_RPC_PORT, DEFAULT_STREAM_PORT)
    }

    pub fn connect_localhost_with_ports(
        name: &str,
        rpc_port: u16,
        stream_port: u16,
    ) -> KrpcResult<Connection> {
        Self::connect_with_ports(name, "localhost", rpc_port, stream_port)
    }

    pub fn connect(name: &str, host: &str) -> KrpcResult<Connection> {
        Self::connect_with_ports(name, host, DEFAULT_RPC_PORT, DEFAULT_STREAM_PORT)
    }

    pub fn connect_with_ports(
        name: &str,
        host: &str,
        rpc_port: u16,
        stream_port: u16,
    ) -> KrpcResult<Connection> {
        let rpc = rpc::Rpc::connect(name, host, rpc_port)?;
        let stream = stream::StreamManager::connect(rpc.id(), host, stream_port)?;

        Ok(Connection { name: name.to_owned(), rpc, stream })
    }

    pub fn id(&self) -> &[u8] {
        self.rpc.id()
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn invoke(
        &self,
        service: &str,
        procedure: &str,
        args: &Vec<Vec<u8>>,
    ) -> KrpcResult<Vec<u8>> {
        self.rpc.invoke(service, procedure, args)
    }

    pub fn procedure_call(
        &self,
        service: &str,
        procedure: &str,
        args: &[Vec<u8>],
    ) -> ProcedureCall {
        Rpc::create_procedure_call(service, procedure, args)
    }

    pub fn add_event<'a>(&'a self, expr: &Expression) -> KrpcResult<Event<'a>> {
        let args = vec![expr.encode()?];
        let response = self.rpc.invoke("KRPC", "AddEvent", &args)?;

        let event = schema::Event::decode(&response, self)?;
        let id = event.get_stream().get_id();
        let stream_value = Arc::new(StreamRaw::new(id, false));

        self.stream.register(stream_value.clone());

        Ok(Event::new(Stream::new(self, stream_value)))
    }

    pub fn add_stream<'a, T: Decode<'a>>(
        &'a self,
        service: &str,
        procedure: &str,
        args: &[Vec<u8>],
        start: bool,
    ) -> KrpcResult<Stream<'a, T>> {
        let stream_args = vec![
            Rpc::create_procedure_call(service, procedure, args).encode()?,
            start.encode()?,
        ];
        let response = self.rpc.invoke("KRPC", "AddStream", &stream_args)?;

        let stream = schema::Stream::decode(&response, self)?;
        let id = stream.get_id();
        let stream_value = Arc::new(StreamRaw::new(id, start));

        self.stream.register(stream_value.clone());

        Ok(Stream::new(self, stream_value))
    }

    fn deregister_stream(&self, stream_id: u64) {
        self.stream.deregister(stream_id);
    }
}

impl fmt::Debug for Connection {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "Connection{{ name: {}, id: {} }}", self.name, hex::encode(self.rpc.id()))
    }
}

/// Result type for all KRPC services.
pub type KrpcResult<T> = Result<T, Error>;
