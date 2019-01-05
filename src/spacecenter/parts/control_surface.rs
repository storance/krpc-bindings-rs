use crate::*;
use crate::codec::*;
use super::{Part};

use std::rc::{Rc};
use std::cell::{RefCell};

remote_type!(
/// A control surface. Obtained by calling `Part::control_surface().`
object ControlSurface {
    service: SpaceCenter,
    properties: {
        {
            Part: Part,
            /// The part object for this control surface
            get: part
        }
    }
});
