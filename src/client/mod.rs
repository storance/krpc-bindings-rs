use std::net::TcpStream;
use std::sync::Arc;

use failure::Error;
use protobuf::{CodedInputStream, CodedOutputStream};

mod error;
mod rpc;
mod schema;
mod stream;

pub use self::error::*;
pub use self::schema::*;

pub const DEFAULT_RPC_PORT: u16 = 50000;
pub const DEFAULT_STREAM_PORT: u16 = 50001;

fn send_msg<T: protobuf::Message>(socket: &mut TcpStream, message: &T) -> Result<(), Error> {
    let mut cos = CodedOutputStream::new(socket);
    Ok(cos.write_message_no_tag(message)?)
}

fn recv_msg<T: protobuf::Message>(socket: &mut TcpStream) -> Result<T, Error> {
    let mut cis = CodedInputStream::new(socket);
    Ok(cis.read_message()?)
}

pub struct Connection {
    rpc: Arc<rpc::Rpc>,
    stream: Arc<stream::StreamManager>,
}

impl Connection {
    pub fn connect(name: &str, host: &str) -> Result<Connection, Error> {
        Self::connect_with_ports(name, host, DEFAULT_RPC_PORT, DEFAULT_STREAM_PORT)
    }

    pub fn connect_with_ports(
        name: &str,
        host: &str,
        rpc_port: u16,
        stream_port: u16,
    ) -> Result<Connection, Error> {
        let rpc = rpc::Rpc::connect(name, host, rpc_port)?;
        let stream = stream::StreamManager::connect(rpc.clone(), host, stream_port)?;

        Ok(Connection { rpc, stream })
    }
}

impl Connection {
    pub fn id(&self) -> Vec<u8> {
        self.rpc.id().clone()
    }

    pub fn invoke(
        &self,
        service: &str,
        procedure: &str,
        args: &Vec<Vec<u8>>,
    ) -> Result<Vec<u8>, Error> {
        self.rpc.invoke(service, procedure, args)
    }
}

/// Result type alias for Krpc calls.
pub type RpcResult<T> = Result<T, Error>;
