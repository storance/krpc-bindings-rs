use super::Part;
use crate::codec::*;
use crate::*;

use std::rc::Rc;

remote_type!(
/// A radiator. Obtained by calling `Part::radiator().`
object SpaceCenter.Radiator {
    properties: {
        {
            Part: Part,
            /// The part object for this radiator.
            get: part
        }
    }
});

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
