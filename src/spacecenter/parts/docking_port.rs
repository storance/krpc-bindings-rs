use super::Part;
use crate::codec::*;
use crate::*;

use std::rc::Rc;

remote_type!(
/// A docking port. Obtained by calling `Part::docking_port().`
object SpaceCenter.DockingPort {
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
