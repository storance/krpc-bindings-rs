use super::Part;
use crate::codec::{Decode, Encode};
use crate::{remote_type, RemoteEnum, RemoteObject};

remote_type!(
/// A cargo bay. Obtained by calling `Part::cargo_bay().`
object SpaceCenter.CargoBay {
    properties: {
        {
            Part {
                /// Returns the part object for this cargo bay.
                ///
                /// **Game Scenes**: All
                get: part -> Part
            }
        }
        {
            State {
                /// Returns the state of the cargo bay.
                ///
                /// **Game Scenes**: All
                get: state -> CargoBayState
            }
        }
        {
            Open {
                /// Returns whether the cargo bay is open.
                ///
                /// **Game Scenes**: All
                get: is_open -> bool,
                /// Sets whether the cargo bay is open.
                ///
                /// **Game Scenes**: All
                set: set_open(bool)
            }
        }
    }
});

remote_type!(
    /// The state of a cargo bay. See `CargoBay::state()`.
    enum CargoBayState {
        /// Cargo bay is fully open.
        Open = 0,
        /// Cargo bay closed and locked.
        Closed = 1,
        /// Cargo bay is opening.
        Opening = 2,
        /// Cargo bay is closing.
        Closing = 3,
    }
);
