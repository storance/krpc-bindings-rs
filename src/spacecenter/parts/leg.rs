use super::Part;
use crate::codec::*;
use crate::*;

use std::rc::Rc;

remote_type!(
/// A landing leg. Obtained by calling `Part::leg().`
object SpaceCenter.Leg {
    properties: {
        {
            Part: Part,
            /// The part object for this leg.
            get: part
        }
    }
});

remote_type!(
/// The state of a landing leg.
enum LegState {
    /// Landing leg is fully deployed.
    Deployed => 0,
    /// Landing leg is fully retracted.
    Retracted => 1,
    /// Landing leg is being deployed.
    Deploying => 2,
    /// Landing leg is being retracted.
    Retracting => 3,
    /// Landing leg is broken.
    Broken => 4
});
