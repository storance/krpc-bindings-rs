use crate::*;
use crate::codec::*;
use super::{Part};

use std::rc::{Rc};
use std::cell::{RefCell};

remote_type!(
/// A docking port. Obtained by calling `Part::docking_port().`
object DockingPort {
    service: SpaceCenter,
    properties: {
        {
            Part: Part,
            /// The part object for this docking part.
            get: part
        }
    }
});

remote_type!(enum DockingPortState {
    Ready => 0,
    Docked => 1,
    Docking => 2,
    Undocking => 3,
    Shielded => 4,
    Moving => 5
});