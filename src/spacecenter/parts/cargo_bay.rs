use crate::*;
use crate::codec::*;
use super::{Part};

use std::rc::{Rc};
use std::cell::{RefCell};

remote_type!(
/// A cargo bay. Obtained by calling `Part::cargo_bay().`
object CargoBay {
    service: SpaceCenter,
    properties: {
        {
            Part: Part,
            /// The part object for this cargo bay.
            get: part
        }
    }
});

remote_type!(
/// The state of a cargo bay. See <see cref="M:SpaceCenter.CargoBay.State" />.
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