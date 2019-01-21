use super::schema::{
    ConnectionRequest, ConnectionRequest_Type, ConnectionResponse, ConnectionResponse_Status,
    StreamUpdate,
};
use super::{
    convert_procedure_result, recv_msg, send_msg, Connection, ConnectionError, ResponseError,
    StreamError,
};
use crate::codec::Decode;

use std::collections::BTreeMap;
use std::io;
use std::marker::PhantomData;
use std::net::TcpStream;
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;

use protobuf::ProtobufError;

#[derive(Debug, Clone)]
pub struct StreamValue<T: Decode> {
    connection: Connection,
    value: Arc<StreamRawValue>,
    phantom: PhantomData<T>,
}

impl<T: Decode + Clone> StreamValue<T> {
    pub fn new(connection: &Connection, value: Arc<StreamRawValue>) -> StreamValue<T> {
        StreamValue {
            connection: connection.clone(),
            value: value,
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
            .value_map(|bytes| Ok(T::decode(bytes, &self.connection)?))
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
    active_streams: Arc<Mutex<BTreeMap<u64, Arc<StreamRawValue>>>>,
    thread_handle: thread::JoinHandle<()>,
    stop_flag: Arc<AtomicBool>,
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
            let active_streams = Arc::new(Mutex::new(BTreeMap::new()));
            let stop_flag = Arc::new(AtomicBool::new(false));
            let thread_handle =
                Self::start_updater(stream_socket, active_streams.clone(), stop_flag.clone());

            Ok(Rc::new(StreamManager {
                thread_handle,
                active_streams,
                stop_flag,
            }))
        } else {
            Err(ConnectionError::ConnectionError(response.message))
        }
    }

    #[allow(unused_must_use)]
    fn start_updater(
        socket: TcpStream,
        active_streams: Arc<Mutex<BTreeMap<u64, Arc<StreamRawValue>>>>,
        stop_flag: Arc<AtomicBool>,
    ) -> thread::JoinHandle<()> {
        let mut stream_socket = socket;

        thread::spawn(move || {
            stream_socket
                .set_read_timeout(Some(Duration::from_secs(1)))
                .expect("Unable to set read timeout");

            while !(*stop_flag).load(Ordering::Relaxed) {
                let msg: Result<StreamUpdate, ProtobufError> = recv_msg(&mut stream_socket);
                if let Some(stream_update) = match msg {
                    Ok(stream_update) => Some(stream_update),
                    Err(ref e) if Self::is_timeout_error(e) => None,
                    Err(ref e) => panic!("Failed to retrieve stream update: {:?}", e),
                } {
                    let streams = active_streams.lock().unwrap();

                    for stream_result in stream_update.results.iter() {
                        if let Some(stream_value) = streams.get(&stream_result.id) {
                            if stream_result.has_result() {
                                stream_value.set_value(convert_procedure_result(
                                    stream_result.get_result(),
                                ));
                            }
                        };
                    }
                }
            }
        })
    }

    fn is_timeout_error(err: &ProtobufError) -> bool {
        match err {
            &ProtobufError::IoError(ref e) => {
                e.kind() == io::ErrorKind::TimedOut || e.kind() == io::ErrorKind::WouldBlock
            }
            _ => false,
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

impl Drop for StreamManager {
    fn drop(&mut self) {
        self.stop_flag.store(true, Ordering::Relaxed);
    }
}
