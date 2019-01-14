extern crate protobuf;
extern crate failure;
#[macro_use]
extern crate failure_derive;

pub mod spacecenter;
pub mod client;
#[macro_use]
mod macros;
mod codec;

use std::rc::Rc;

pub use self::client::{Connection, RpcResult};

pub trait RemoteObject {
    fn new(connection: Rc<Connection>, id: u64) -> Self where Self: Sized;

    fn id(&self) -> u64;
}

pub trait RemoteEnum {
    fn from_value(value: i64) -> Option<Self> where Self: Sized;

    fn value(&self) -> i64;
}

/// Type alias for a 3-dimension Vector.
pub type Vector3 = (f64, f64, f64);
/// Type alias for a Quaternion.
pub type Quaternion = (f64, f64, f64, f64);