use crate::*;
use crate::codec::*;
use super::{Part};

use std::rc::Rc;

remote_type!(
/// A cargo bay. Obtained by calling `Part::cargo_bay().`
object SpaceCenter.CargoBay {
    properties: {
        {
            Part: Part,
            /// Returns the part object for this cargo bay.
            ///
            /// **Game Scenes**: All
            get: part
        }
        {
            State: CargoBayState,
            /// Returns the state of the cargo bay.
            ///
            /// **Game Scenes**: All
            get: state
        }
        {
            Open: bool,
            /// Returns whether the cargo bay is open.
            ///
            /// **Game Scenes**: All
            get: is_open,
            /// Sets whether the cargo bay is open.
            ///
            /// **Game Scenes**: All
            set: set_open
        }
    }
});

remote_type!(
/// The state of a cargo bay. See `CargoBay::state()`.
enum CargoBayState {
    /// Cargo bay is fully open.
    Open => 0,
    /// Cargo bay closed and locked.
    Closed => 1,
    /// Cargo bay is opening.
    Opening => 2,
    /// Cargo bay is closing.
    Closing => 3
});