use super::Part;
use crate::codec::*;
use crate::*;

use std::rc::Rc;

remote_type!(
/// A launch clamp. Obtained by calling `Part::launch_clamp().`
object SpaceCenter.LaunchClamp {
    properties: {
        {
            Part: Part,
            /// The part object for this launch clamp.
            get: part
        }
    }
});
