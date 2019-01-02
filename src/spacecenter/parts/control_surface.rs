use crate::*;
use crate::codec::*;
use super::{Part};

use std::rc::{Rc};
use std::cell::{RefCell};

remote_type!(
/// A control surface. Obtained by calling `Part::control_surface().`
object ControlSurface {});

impl ControlSurface {
    rpc_property!(
        Part: Part {
            service: SpaceCenter,
            class: ControlSurface,
            /// The part object for this control surface.
            part
        }
    );
}