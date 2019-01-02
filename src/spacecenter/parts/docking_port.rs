use crate::*;
use crate::codec::*;
use super::{Part};

use std::rc::{Rc};
use std::cell::{RefCell};

remote_type!(
/// A docking port. Obtained by calling `Part::docking_port().`
object DockingPort {});

impl DockingPort {
    rpc_property!(
        Part: Part {
            service: SpaceCenter,
            class: DockingPort,
            /// The part object for this docking port.
            part
        }
    );
}

remote_type!(enum DockingPortState {
    Ready => 0,
    Docked => 1,
    Docking => 2,
    Undocking => 3,
    Shielded => 4,
    Moving => 5
});