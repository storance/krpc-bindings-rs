use crate::*;
use crate::codec::*;

use std::rc::{Rc};
use std::cell::{RefCell};

remote_type!(object Parts {});

impl Parts {
    rpc_method!(fn get_all(&self) -> Vec<Part> {
        if let Some(value) = SpaceCenter.Parts_get_All(self) as Vec<Part> {
            value
        } else {
            return Err(KrpcError::NullResponseValue)
        }
    });
}

remote_type!(object Part {});

impl Part {
    rpc_method!(fn get_name(&self) -> String {
        if let Some(value) = SpaceCenter.Part_get_Name(self) as String {
            value
        } else {
            return Err(KrpcError::NullResponseValue)
        }
    });
}

remote_type!(object DockingPort {});

impl DockingPort {
    rpc_method!(
    /// The part object for this docking port.
    fn get_part(&self) -> Part {
        if let Some(value) = SpaceCenter.DockingPort_get_Part(self) as Part {
            value
        } else {
            return Err(KrpcError::NullResponseValue)
        }
    });
}

remote_type!(enum DockingPortState {
    Ready => 0,
    Docked => 1,
    Docking => 2,
    Undocking => 3,
    Shielded => 4,
    Moving => 5
});