use crate::*;
use crate::codec::*;
use super::{Part};

use std::rc::{Rc};
use std::cell::{RefCell};

remote_type!(
/// A landing leg. Obtained by calling `Part::leg().`
object Leg {});

impl Leg {
    rpc_property!(
        Part: Part {
            service: SpaceCenter,
            class: Leg,
            /// The part object for this landing leg.
            part
        }
    );
}

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