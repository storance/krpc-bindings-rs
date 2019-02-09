use super::Part;
use crate::codec::{Decode, Encode};
use crate::{remote_type, RemoteObject};

remote_type!(
/// A fairing. Obtained by calling `Part::fairing().`
object SpaceCenter.Fairing {
    properties: {
        {
            Part {
                /// Returns the part object for this fairing.
                ///
                /// **Game Scenes**: All
                get: part -> Part
            }
        }
        {
            Jettisoned {
                /// Returns whether the fairing has been jettisoned.
                ///
                /// **Game Scenes**: All
                get: is_jettisoned -> bool
            }
        }
    }
    methods: {
        {
            /// Jettison the fairing. Has no effect if it has already been jettisoned.
            ///
            /// **Game Scenes**: All
            fn jettison() {
                Jettison()
            }
        }
    }
});
