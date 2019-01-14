use super::Part;
use crate::codec::*;
use crate::*;

use std::rc::Rc;

remote_type!(
/// A wheel. Includes landing gear and rover wheels. Obtained by calling `Part::wheel()`. Can be
/// used to control the motors, steering and deployment of wheels, among other things.
object SpaceCenter.Wheel {
    properties: {
        {
            Part: Part,
            /// The part object for this wheel.
            get: part
        }
    }
});

remote_type!(
/// The state of a wheel. See `Wheel:state`.
enum WheelState {
    /// Wheel is fully deployed.
    Deployed => 0,
    /// Wheel is fully retracted.
    Retracted => 1,
    /// Wheel is being deployed.
    Deploying => 2,
    /// Wheel is being retracted.
    Retracting => 3,
    /// Wheel is broken.
    Broken => 4
});
