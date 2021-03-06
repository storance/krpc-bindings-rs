use super::Part;
use crate::codec::{Decode, Encode};
use crate::{remote_type, RemoteObject};

remote_type!(
/// A launch clamp. Obtained by calling `Part::launch_clamp().`
object SpaceCenter.LaunchClamp {
    properties: {
        {
            Part {
                /// Returns the part object for this launch clamp.
                ///
                /// **Game Scenes**: All
                get: part -> Part
            }
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
