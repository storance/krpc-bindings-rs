use crate::*;
use crate::codec::*;

use std::rc::{Rc};
use std::cell::{RefCell};

remote_type!(
/// Used to manipulate the controls of a vessel. This includes adjusting the throttle,
/// enabling/disabling systems such as SAS and RCS, or altering the direction in which the vessel
/// is pointing. Obtained by calling `Vessel.getControl()`.
///
/// # Note
/// Control inputs (such as pitch, yaw and roll) are zeroed when all clients that have set one or
/// more of these inputs are no longer connected.
object Control {}
);

remote_type!(
    /// The behavior of the SAS auto-pilot.
    enum SASMode {
        /// Stability assis mode. Dampen out any rotation.
        StabilityAssist => 0,
        /// Point in the burn direction of the next maneuver node.
        Maneuver => 1,
        /// Point in the prograde direction.
        Prograde => 2,
        /// Point in the retrograde direction.
        Retrograde => 3,
        /// Point in the orbit normal direction.
        Normal => 4,
        /// Point in the orbit anti-normal direction.
        AntiNormal => 5,
        /// Point in the orbit radial direction.
        Radial => 6,
        /// Point in the orbit anti-radial direction.
        AntiRadial => 7,
        /// Point in the direction of the current target.
        Target => 8,
        /// Point away from the current target.
        AntiTarget => 9
    }
);

remote_type!(
    /// The control source of a vessel.
    enum ControlSource {
        /// Vessel is controlled by a Kerbal.
        Kernal => 0,
        /// Vessel is controlled by a probe core.
        Probe => 1,
        /// Vessel is not controlled.
        None => 2
    }
);

remote_type!(
    /// The control state of a vessel.
    enum ControlState {
        /// Full controllable.
        Full => 0,
        /// Partially controllable.
        Partial => 1,
        /// Not controllable.
        None => 2
    }
);

remote_type!(
    /// The mode of the speed reported in the navball.
    enum SpeedMode {
        /// Speed is relative to the vessel's orbit.
        Orbit => 0,
        /// Speed is relative to the surface of the body being orbited.
        Surface => 1,
        /// Speed is relative to the current target.
        Taerget => 2
    }
);

remote_type!(
    /// The control input mode.
    enum ControlInputMode {
        /// Control inputs are added to the vessel's current control inputs.
        Additive => 0,
        /// Control inputs (when they are non-zero) override the vessel's current control inputs.
        Override => 1
    }
);