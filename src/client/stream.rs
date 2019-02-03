use super::schema::{
    ConnectionRequest, ConnectionRequest_Type, ConnectionResponse, ConnectionResponse_Status,
    StreamUpdate,
};
use super::{
    convert_procedure_result, recv_msg, send_msg, Connection, ConnectionError, KrpcResult,
    ResponseError, StreamError,
};
use crate::codec::{Decode, Encode};

use std::io;
use std::thread;

use failure::Error;
use std::collections::BTreeMap;
use std::marker::PhantomData;
use std::net::TcpStream;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Condvar, Mutex};
use std::time::{Duration, Instant};

pub struct Event<'a> {
    stream: Stream<'a, bool>,
}

impl<'a> Event<'a> {
    pub fn new(stream: Stream<'a, bool>) -> Self {
        Event { stream }
    }

    /// Returns the backing stream for this event
    pub fn stream(&self) -> &Stream<bool> {
        &self.stream
    }

    /// Wait until the event is in the expected state.
    ///
    /// # Arguments
    /// * `triggered` - If `true`, waits until the event is triggered; otherwise, wait until
    /// it's not triggered.
    pub fn wait(&self, triggered: bool) -> KrpcResult<()> {
        if !self.stream.started() {
            self.stream.start()?;
        }

        while self.stream.value()? != triggered {
            self.stream.wait()?;
        }

        Ok(())
    }

    /// Wait for the specified amount of time until the event has occurred.
    ///
    /// # Arguments
    /// * `triggered` - If `true`, waits until the event is triggered; otherwise, wait until
    /// it's not triggered.
    /// * `dur` - The maximum amount of time to wait.
    ///
    /// # Returns
    /// Returns `true` if the event matches the desired triggered state; `false` if it timed-out.
    pub fn wait_timeout(&self, triggered: bool, dur: Duration) -> KrpcResult<bool> {
        if !self.stream.started() {
            self.stream.start()?;
        }

        let start = Instant::now();
        loop {
            if self.stream.value()? == triggered {
                return Ok(true);
            }

            let timeout = match dur.checked_sub(start.elapsed()) {
                Some(timeout) => timeout,
                None => return Ok(false),
            };

            self.stream.wait_timeout(timeout)?;
        }
    }

    /// Returns whether or not the event has occurred.
    pub fn is_triggered(&self) -> KrpcResult<bool> {
        self.stream.value()
    }

    pub fn start(&self) -> KrpcResult<()> {
        self.stream.start()
    }

    pub fn started(&self) -> bool {
        self.stream.started()
    }

    pub fn remove(&mut self) -> KrpcResult<()> {
        self.stream.remove()
    }
}

pub struct Stream<'a, T: Decode<'a>> {
    connection: &'a Connection,
    value: Arc<StreamRaw>,
    removed: bool,
    phantom: PhantomData<T>,
}

impl<'a, T: Decode<'a>> Stream<'a, T> {
    pub(super) fn new(connection: &'a Connection, value: Arc<StreamRaw>) -> Self {
        Stream {
            connection,
            value,
            removed: false,
            phantom: PhantomData,
        }
    }

    /// Returns the id of this stream.
    pub fn id(&self) -> u64 {
        self.value.id()
    }

    /// Returns the current value for the stream.  If the stream is not started, this will
    /// start it.
    pub fn value(&self) -> KrpcResult<T> {
        if !self.value.started() {
            self.start()?;
            self.wait()?;
        }

        self.value
            .value_map(|bytes, _version| Ok(T::decode(bytes, self.connection)?))
    }

    /// Returns whether or not the stream has started.
    pub fn started(&self) -> bool {
        self.value.started()
    }

    /// Starts this stream value if it hs not already been started.  This will cause the stream
    /// to start receiving updates from the KRPC server.
    pub fn start(&self) -> KrpcResult<()> {
        if self.started() {
            return Ok(());
        }

        let args = &vec![self.value.id().encode()?];
        self.connection.invoke("KRPC", "StartStream", &args)?;
        self.value.set_started();

        Ok(())
    }

    /// Returns the current update frequency in hertz.
    pub fn rate(&self) -> f32 {
        self.value.rate()
    }

    /// Sets the update frequency for this stream in hertz.
    ///
    /// # Arguments
    /// * `rate` - The new update in hertz.
    pub fn set_rate(&self, rate: f32) -> KrpcResult<()> {
        if !self.started() {
            return Err(Error::from(StreamError::NotStarted));
        }

        if self.removed {
            return Err(Error::from(StreamError::Removed));
        }

        let args = vec![self.value.id().encode()?, rate.encode()?];
        self.connection.invoke("KRPC", "SetStreamRate", &args)?;

        self.value.set_rate(rate);

        Ok(())
    }

    /// Removes the stream so it no longer receives updates from the KRPC server.  The last
    /// received value for this stream can still be obtained by calling `value()`.
    pub fn remove(&mut self) -> KrpcResult<()> {
        if !self.removed {
            let args = vec![self.id().encode()?];
            self.connection.invoke("KRPC", "RemoveStream", &args)?;

            self.connection.deregister_stream(self.id());
            self.removed = true;
        }

        Ok(())
    }

    /// Returns whether or not the stream has been removed.  The last received value for this
    /// can still be obtained by calling `value()`.
    pub fn is_removed(&self) -> bool {
        self.removed
    }

    /// Waits for the stream value to be updated.
    pub fn wait(&self) -> KrpcResult<()> {
        if self.removed {
            return Err(Error::from(StreamError::Removed));
        }

        self.value.wait()
    }

    /// Waits for the stream value to be updated for the given amount of time.  Returns whether or
    /// not the timeout was reached before the stream was updated.
    ///
    /// # Arguments
    /// * `timeout` - The maximum amount on time to wait for the stream value to be updated.
    pub fn wait_timeout(&self, timeout: Duration) -> KrpcResult<bool> {
        if self.removed {
            return Err(Error::from(StreamError::Removed));
        }

        self.value.wait_timeout(timeout)
    }
}

impl<'a, T: Decode<'a>> Drop for Stream<'a, T> {
    fn drop(&mut self) {
        use std::io::{stderr, Write};
        use std::thread::panicking;

        if !self.removed {
            if let Err(e) = self.remove() {
                if panicking() {
                    write!(stderr(), "Error removing stream value: {:?}", e)
                        .expect("Error writing to `stderr`");
                } else {
                    panic!("Error removing stream value: {:?}", e);
                }
            }
        }
    }
}

pub(super) struct StreamRaw {
    id: u64,
    state: Mutex<StreamState>,
    update_cvar: Condvar,
}

impl StreamRaw {
    pub(super) fn new(id: u64, started: bool) -> StreamRaw {
        StreamRaw {
            id,
            state: Mutex::new(StreamState::new(started)),
            update_cvar: Condvar::new(),
        }
    }

    fn wait(&self) -> KrpcResult<()> {
        let mut state = self.state.lock().unwrap();

        if !state.started {
            return Err(Error::from(StreamError::NotStarted));
        }

        let initial_version = state.version;
        // keep waiting until the state version has been incremented
        while state.version <= initial_version {
            state = self.update_cvar.wait(state).unwrap();
        }

        Ok(())
    }

    fn wait_timeout(&self, dur: Duration) -> KrpcResult<bool> {
        let mut state = self.state.lock().unwrap();

        if !state.started {
            return Err(Error::from(StreamError::NotStarted));
        }

        let initial_version = state.version;
        let start = Instant::now();
        loop {
            if state.version > initial_version {
                return Ok(false);
            }

            let timeout = match dur.checked_sub(start.elapsed()) {
                Some(timeout) => timeout,
                None => return Ok(true),
            };

            state = self.update_cvar.wait_timeout(state, timeout).unwrap().0;
        }
    }

    pub(super) fn id(&self) -> u64 {
        self.id
    }

    fn value_map<F, R>(&self, map: F) -> KrpcResult<R>
    where
        F: FnOnce(&Vec<u8>, u64) -> KrpcResult<R>,
    {
        let state = self.state.lock().unwrap();

        if !state.started {
            return Err(Error::from(StreamError::NotStarted));
        }

        let bytes = state.value.as_ref().map_err(Clone::clone)?;
        map(bytes, state.version)
    }

    fn started(&self) -> bool {
        let state = self.state.lock().unwrap();

        state.started
    }

    fn rate(&self) -> f32 {
        let state = self.state.lock().unwrap();

        state.rate
    }

    fn set_value(&self, bytes: Vec<u8>) {
        let mut state = self.state.lock().unwrap();

        state.version += 1;
        state.value = Ok(bytes);

        self.update_cvar.notify_all();
    }

    fn set_error(&self, err: ResponseError) {
        let mut state = self.state.lock().unwrap();

        state.version += 1;
        state.value = Err(err);

        self.update_cvar.notify_all();
    }

    fn set_rate(&self, rate: f32) {
        let mut state = self.state.lock().unwrap();

        state.rate = rate;
    }

    fn set_started(&self) {
        let mut state = self.state.lock().unwrap();

        state.started = true;
    }
}

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

pub struct StreamManager {
    active_streams: Arc<Mutex<BTreeMap<u64, Arc<StreamRaw>>>>,
    stop_flag: Arc<AtomicBool>,
}

impl StreamManager {
    pub(super) fn connect(client_id: &[u8], host: &str, port: u16) -> KrpcResult<StreamManager> {
        let mut stream_socket = TcpStream::connect((host, port))?;

        let mut request = ConnectionRequest::new();
        request.set_field_type(ConnectionRequest_Type::STREAM);
        request.set_client_identifier(Vec::from(client_id));

        send_msg(&mut stream_socket, &request)?;
        let response: ConnectionResponse = recv_msg(&mut stream_socket)?;

        match response.status {
            ConnectionResponse_Status::OK => {
                let active_streams = Arc::new(Mutex::new(BTreeMap::new()));
                let stop_flag = Arc::new(AtomicBool::new(false));
                Self::start_updater(stream_socket, active_streams.clone(), stop_flag.clone());

                Ok(StreamManager {
                    active_streams,
                    stop_flag,
                })
            }
            ConnectionResponse_Status::TIMEOUT => Err(ConnectionError::Timeout(response.message))?,
            ConnectionResponse_Status::MALFORMED_MESSAGE => {
                Err(ConnectionError::MalformedMessage(response.message))?
            }
            ConnectionResponse_Status::WRONG_TYPE => {
                Err(ConnectionError::WrongType(response.message))?
            }
        }
    }

    fn start_updater(
        socket: TcpStream,
        active_streams: Arc<Mutex<BTreeMap<u64, Arc<StreamRaw>>>>,
        stop_flag: Arc<AtomicBool>,
    ) {
        let mut stream_socket = socket;

        thread::spawn(move || {
            stream_socket
                .set_read_timeout(Some(Duration::from_secs(1)))
                .expect("Unable to set read timeout");

            while !(*stop_flag).load(Ordering::Relaxed) {
                let msg: KrpcResult<StreamUpdate> = recv_msg(&mut stream_socket);
                match msg {
                    Ok(stream_update) => {
                        Self::process_stream_update(stream_update, &active_streams)
                    }
                    Err(ref e) if Self::is_timeout_error(e) => (),
                    Err(ref e) => panic!("Failed to retrieve stream update: {:?}", e),
                }
            }
        });
    }

    fn process_stream_update(
        stream_update: StreamUpdate,
        active_streams: &Mutex<BTreeMap<u64, Arc<StreamRaw>>>,
    ) {
        let streams = active_streams.lock().unwrap();

        for stream_result in stream_update.results.iter() {
            if let Some(stream_value) = streams.get(&stream_result.id) {
                if stream_result.has_result() {
                    match convert_procedure_result(stream_result.get_result()) {
                        Ok(bytes) => stream_value.set_value(bytes),
                        Err(response_err) => stream_value.set_error(response_err),
                    }
                }
            }
        }
    }

    fn is_timeout_error(err: &Error) -> bool {
        if let Some(ref ioe) = err.downcast_ref::<io::Error>() {
            ioe.kind() == io::ErrorKind::TimedOut || ioe.kind() == io::ErrorKind::WouldBlock
        } else {
            false
        }
    }

    pub(super) fn register(&self, stream: Arc<StreamRaw>) {
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
