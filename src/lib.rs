#[macro_use]
extern crate failure;
extern crate paste;
extern crate protobuf;

mod client;
pub mod krpc;
pub mod spacecenter;
#[macro_use]
mod macros;
mod codec;

pub use self::client::schema::{Services, Status};
pub use self::client::{
    Connection, ConnectionError, ResponseError, RpcError, RpcResult, StreamError, StreamResult,
    StreamValue,
};

pub trait RemoteObject {
    fn new(connection: &Connection, id: u64) -> Self
    where
        Self: Sized;

    fn id(&self) -> u64;
}

pub trait RemoteEnum {
    fn from_value(value: i64) -> Option<Self>
    where
        Self: Sized;

    fn value(&self) -> i64;
}

/// Type alias for a 3-dimension Vector.
pub type Vector3 = (f64, f64, f64);
/// Type alias for a Quaternion.
pub type Quaternion = (f64, f64, f64, f64);
