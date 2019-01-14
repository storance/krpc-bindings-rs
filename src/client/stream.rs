use super::rpc::Rpc;
use super::{recv_msg, send_msg, ConnectionNegotiateError};
use super::{
    ConnectionRequest, ConnectionRequest_Type, ConnectionResponse, ConnectionResponse_Status,
};

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::net::TcpStream;
use std::sync::{Arc, Condvar, Mutex};

use failure::Error;

pub struct StreamValue {
    id: u64,
    value: Mutex<StreamState>,
    update_cvar: Condvar,
}

struct StreamState {
    started: bool,
    updated: bool,
    rate: f32,
    value: Result<Vec<u8>, Error>,
}

pub struct StreamManager {
    rpc: Arc<Rpc>,
    socket: RefCell<TcpStream>,
    active_streams: Mutex<BTreeMap<u64, Arc<StreamValue>>>,
}

impl StreamManager {
    pub(super) fn connect(
        rpc: Arc<Rpc>,
        host: &str,
        port: u16,
    ) -> Result<Arc<StreamManager>, Error> {
        let mut stream_socket = TcpStream::connect((host, port))?;

        let mut request = ConnectionRequest::new();
        request.set_field_type(ConnectionRequest_Type::STREAM);
        request.set_client_identifier(rpc.id().clone());

        send_msg(&mut stream_socket, &request)?;
        let response: ConnectionResponse = recv_msg(&mut stream_socket)?;

        match response.status {
            ConnectionResponse_Status::OK => Ok(Arc::new(StreamManager {
                rpc,
                socket: RefCell::new(stream_socket),
                active_streams: Mutex::new(BTreeMap::new()),
            })),
            ConnectionResponse_Status::MALFORMED_MESSAGE => {
                Err(ConnectionNegotiateError::MalformedMessage(response.message))?
            }
            ConnectionResponse_Status::TIMEOUT => {
                Err(ConnectionNegotiateError::Timeout(response.message))?
            }
            ConnectionResponse_Status::WRONG_TYPE => {
                Err(ConnectionNegotiateError::WrongType(response.message))?
            }
        }
    }
}
