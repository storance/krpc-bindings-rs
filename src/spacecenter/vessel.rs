use crate::*;
use crate::codec::*;

use std::rc::{Rc};
use std::cell::{RefCell};

remote_type!(object Vessel {});

impl Vessel {
    rpc_method!(fn get_name(&self) -> String {
        if let Some(value) = SpaceCenter.Vessel_get_Name(self) as String {
            value
        } else {
            return Err(KrpcError::NullResponseValue)
        }
    });
}
