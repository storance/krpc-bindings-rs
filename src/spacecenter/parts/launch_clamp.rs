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
            /// Returns the part object for this launch clamp.
            ///
            /// **Game Scenes**: All
            get: part
        }
    }
    methods: {
        {
            /// Releases the docking clamp. Has no effect if the clamp has already been released.
            ///
            /// **Game Scenes**: All
            fn release() {
                Release()
            }
        }
    }
});
