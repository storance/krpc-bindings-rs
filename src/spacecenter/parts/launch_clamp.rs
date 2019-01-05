use crate::*;
use crate::codec::*;
use super::{Part};

use std::rc::{Rc};
use std::cell::{RefCell};

remote_type!(
/// A launch clamp. Obtained by calling `Part::launch_clamp().`
object LaunchClamp {
    service: SpaceCenter,
    properties: {
        {
            Part: Part,
            /// The part object for this launch clamp.
            get: part
        }
    }
});
