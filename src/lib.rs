#[macro_use]
extern crate failure;
extern crate paste;
extern crate protobuf;

pub mod client;
pub mod drawing;
pub mod kac;
pub mod krpc;
pub mod remotetech;
pub mod spacecenter;
pub mod ui;
#[macro_use]
mod macros;
pub mod codec;

pub trait RemoteObject<'a> {
    fn new(connection: &'a client::Connection, id: u64) -> Self
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
