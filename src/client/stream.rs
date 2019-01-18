use super::schema::{
    ConnectionRequest, ConnectionRequest_Type, ConnectionResponse, ConnectionResponse_Status,
};
use super::{recv_msg, send_msg, Connection, ConnectionError, ResponseError, StreamError};
use crate::codec::Decode;

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::marker::PhantomData;
use std::net::TcpStream;
use std::rc::Rc;
use std::sync::{Arc, Condvar, Mutex};

#[derive(Debug)]
pub struct StreamValue<T: Decode> {
    connection: Rc<Connection>,
    value: Arc<StreamRawValue>,
    phantom: PhantomData<T>,
}

impl<T: Decode + Clone> StreamValue<T> {
    pub fn new(connection: Rc<Connection>, value: Arc<StreamRawValue>) -> StreamValue<T> {
        StreamValue {
            connection,
            value,
            phantom: PhantomData,
        }
    }

    pub fn id(&self) -> u64 {
        self.value.id()
    }

    pub fn value(&self) -> Result<T, StreamError> {
        if !self.value.started()? {
            self.start()?;
            self.wait()?;
        }

        self.value
            .value_map(|bytes| Ok(T::decode(bytes, self.connection.clone())?))
    }

    pub fn started(&self) -> Result<bool, StreamError> {
        self.value.started()
    }

    pub fn start(&self) -> Result<(), StreamError> {
        self.connection.start_stream(&self.value)
    }

    pub fn rate(&self) -> Result<f32, StreamError> {
        self.value.rate()
    }

    pub fn set_rate(&self, rate: f32) -> Result<(), StreamError> {
        self.connection.set_stream_rate(&self.value, rate)
    }

    pub fn remove(&self) -> Result<(), StreamError> {
        self.connection.remove_stream(&self.value)
    }

    pub fn wait(&self) -> Result<(), StreamError> {
        self.value.wait()
    }
}

#[derive(Debug)]
pub struct StreamRawValue {
    id: u64,
    state: Mutex<StreamState>,
    update_cvar: Condvar,
}

impl StreamRawValue {
    pub(super) fn new(id: u64, started: bool) -> StreamRawValue {
        StreamRawValue {
            id,
            state: Mutex::new(StreamState::new(started)),
            update_cvar: Condvar::new(),
        }
    }

    pub fn wait(&self) -> Result<(), StreamError> {
        let mut state = self.state.lock().unwrap();

        if !state.started {
            return Err(StreamError::NotStarted);
        }

        let initial_version = state.version;
        // keep waiting until the state version has been incremented
        while state.version <= initial_version {
            state = self.update_cvar.wait(state).unwrap();
        }

        Ok(())
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub(super) fn version(&self) -> Result<u64, StreamError> {
        let state = self.state.lock().unwrap();

        Ok(state.version)
    }

    pub fn value(&self) -> Result<Vec<u8>, StreamError> {
        let state = self.state.lock().unwrap();

        if !state.started {
            return Err(StreamError::NotStarted);
        }

        Ok(state.value.clone()?)
    }

    pub fn value_map<F, R>(&self, map: F) -> Result<R, StreamError>
    where
        F: FnOnce(&Vec<u8>) -> Result<R, StreamError>,
    {
        let state = self.state.lock().unwrap();

        if !state.started {
            return Err(StreamError::NotStarted);
        }

        let bytes = state.value.as_ref().map_err(Clone::clone)?;
        map(bytes)
    }

    pub fn started(&self) -> Result<bool, StreamError> {
        let state = self.state.lock().unwrap();

        Ok(state.started)
    }

    pub fn rate(&self) -> Result<f32, StreamError> {
        let state = self.state.lock().unwrap();

        Ok(state.rate)
    }

    pub(super) fn set_value(
        &self,
        value: Result<Vec<u8>, ResponseError>,
    ) -> Result<(), StreamError> {
        let mut state = self.state.lock().unwrap();

        state.version += 1;
        state.value = value;

        self.update_cvar.notify_all();
        Ok(())
    }

    pub(super) fn set_rate(&self, rate: f32) -> Result<(), StreamError> {
        let mut state = self.state.lock().unwrap();

        state.rate = rate;
        Ok(())
    }

    pub(super) fn set_started(&self) -> Result<(), StreamError> {
        let mut state = self.state.lock().unwrap();

        state.started = true;
        Ok(())
    }
}

#[derive(Debug)]
struct StreamState {
    started: bool,
    version: u64,
    rate: f32,
    value: Result<Vec<u8>, ResponseError>,
}

impl StreamState {
    fn new(started: bool) -> Self {
        StreamState {
            started,
            version: 0,
            rate: 0.0,
            value: Err(ResponseError::MissingResult),
        }
    }
}

#[derive(Debug)]
pub struct StreamManager {
    socket: RefCell<TcpStream>,
    active_streams: Mutex<BTreeMap<u64, Arc<StreamRawValue>>>,
}

impl StreamManager {
    pub(super) fn connect(
        client_id: Vec<u8>,
        host: &str,
        port: u16,
    ) -> Result<Rc<StreamManager>, ConnectionError> {
        let mut stream_socket = TcpStream::connect((host, port))?;

        let mut request = ConnectionRequest::new();
        request.set_field_type(ConnectionRequest_Type::STREAM);
        request.set_client_identifier(client_id);

        send_msg(&mut stream_socket, &request)?;
        let response: ConnectionResponse = recv_msg(&mut stream_socket)?;

        if response.status == ConnectionResponse_Status::OK {
            Ok(Rc::new(StreamManager {
                socket: RefCell::new(stream_socket),
                active_streams: Mutex::new(BTreeMap::new()),
            }))
        } else {
            Err(ConnectionError::ConnectionError(response.message))
        }
    }

    pub(super) fn register(&self, stream: Arc<StreamRawValue>) {
        let mut active_streams = self.active_streams.lock().unwrap();
        (*active_streams).insert(stream.id, stream);
    }

    pub(super) fn deregister(&self, stream_id: u64) {
        let mut active_streams = self.active_streams.lock().unwrap();
        (*active_streams).remove(&stream_id);
    }
}
