use crate::*;
use crate::codec::*;
use super::{Part};

use std::rc::{Rc};
use std::cell::{RefCell};

remote_type!(
/// An antenna. Obtained by calling `Part::antenna().`
object Antenna {
    service: SpaceCenter,
    properties: {
        {
            Part: Part,
            /// The part object for this antenna.
            get: part
        }
    }
});

remote_type!(
/// The state of an antenna.
enum AntennaState {
    /// Antenna is fully deployed.
    Deployed => 0,
    /// Antenna is fully retracted.
    Retracted => 1,
    /// Antenna is being deployed.
    Deploying => 2,
    /// Antenna is being retracted.
    Retracting => 3,
    /// Antenna is broken.
    Broken => 4
});