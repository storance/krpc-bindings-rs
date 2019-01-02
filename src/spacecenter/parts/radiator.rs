use crate::*;
use crate::codec::*;
use super::{Part};

use std::rc::{Rc};
use std::cell::{RefCell};

remote_type!(
/// A radiator. Obtained by calling `Part::radiator().`
object Radiator {});

impl Radiator {
    rpc_property!(
        Part: Part {
            service: SpaceCenter,
            class: Radiator,
            /// The part object for this radiator.
            part
        }
    );
}

remote_type!(
/// The state of a radiator.
enum RadiatorState {
    /// Radiator is fully extended.
    Extended => 0,
    /// Radiator is fully retracted.
    Retracted => 1,
    /// Radiator is being extended.
    Extending => 2,
    /// Radiator is being retracted.
    Retracting => 3,
    /// Radiator is being broken.
    Broken => 4
});