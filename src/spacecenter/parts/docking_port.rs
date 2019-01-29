use super::Part;
use crate::codec::{Decode, Encode};
use crate::spacecenter::{ReferenceFrame, Vessel};
use crate::{remote_type, Quaternion, RemoteEnum, RemoteObject, Vector3};

remote_type!(
/// A docking port. Obtained by calling `Part::docking_port().`
object SpaceCenter.DockingPort {
    properties: {
        {
            Part: Part,
            /// Returns the part object for this docking port.
            ///
            /// **Game Scenes**: All
            get: part
        }
        {
            State: DockingPortState,
            /// Returns the current state of the docking port.
            ///
            /// **Game Scenes**: All
            get: state
        }
        {
            DockedPart: Option<Part>,
            /// Returns the part that this docking port is docked to. Returns `None` if
            /// this docking port is not docked to anything.
            ///
            /// **Game Scenes**: All
            get: docked_part
        }
        {
            ReengageDistance: f32,
            /// Returns the distance a docking port must move away when it undocks before it
            /// becomes ready to dock with another port, in meters.
            ///
            /// **Game Scenes**: All
            get: reengage_distance
        }
        {
            HasShield: bool,
            /// Returns whether the docking port has a shield.
            ///
            /// **Game Scenes**: All
            get: has_shield
        }
        {
            Shielded: bool,
            /// Returns the state of the docking ports shield, if it has one.  Returns `true` if
            /// the docking port has a shield, and the shield is closed. Otherwise returns `false`.
            ///
            /// **Game Scenes**: All
            get: is_shielded,
            /// Sets the state of the docking ports shield.  When set to `true`, the shield is
            /// closed, and when set to `false` the shield is opened. If the docking port does
            /// not have a shield, setting this attribute has no effect.
            ///
            /// **Game Scenes**: All
            set: set_shielded
        }
        {
            ReferenceFrame: ReferenceFrame,
            /// Returns the reference frame that is fixed relative to this docking port, and
            /// oriented with the port.
            ///
            /// * The origin is at the position of the docking port.
            /// * The axes rotate with the docking port.
            /// * The x-axis points out to the right side of the docking port.
            /// * The y-axis points in the direction the docking port is facing.
            /// * The z-axis points out of the bottom off the docking port.
            ///
            /// **Game Scenes**: All
            ///
            /// ![Docking port reference frame origin and axes](https://krpc.github.io/krpc/_images/docking-port.png)
            /// *Docking port reference frame origin and axes*
            ///
            /// ![Inline docking port reference frame origin and axes](https://krpc.github.io/krpc/_images/docking-port-inline.png)
            /// *Inline docking port reference frame origin and axes*
            ///
            /// # Note
            /// This reference frame is not necessarily equivalent to the reference frame
            /// for the part, returned by `Part::reference_frame()`.
            get: reference_frame
        }
    }
    methods: {
        {
            /// Undocks the docking port and returns the new Vessel that is created. This method
            /// can be called for either docking port in a docked pair. Returns an error if
            /// the docking port is not docked to anything.
            ///
            /// **Game Scenes**: All
            ///
            /// # Note
            /// When called, the active vessel may change. It is therefore possible that,
            /// after calling this function, the object(s) returned by previous call(s) to
            /// `active_vessel()` no longer refer to the active vessel.
            fn undock() -> Vessel {
                Undock()
            }
        }
        {
            /// Returns the position of the docking port, in the given reference frame.
            ///
            /// **Game Scenes**: All
            ///
            /// # Arguments
            /// * `reference_frame`  – The reference frame that the returned position vector is in.
            ///
            /// # Returns
            /// The position as a vector.
            fn positiion(reference_frame: &ReferenceFrame) -> Vector3 {
                Position(reference_frame)
            }
        }
        {
            /// Returns the direction that docking port points in, in the given reference frame.
            ///
            /// **Game Scenes**: All
            ///
            /// # Arguments
            /// * `reference_frame`  – The reference frame that the returned direction is in.
            ///
            /// # Returns
            /// The direction as a unit vector.
            fn direction(reference_frame: &ReferenceFrame) -> Vector3 {
                Direction(reference_frame)
            }
        }
        {
            /// Returns the rotation of the docking port, in the given reference frame.
            ///
            /// **Game Scenes**: All
            ///
            /// # Arguments
            /// * `reference_frame`  – The reference frame that the returned rotation is in.
            ///
            /// # Returns
            /// The rotation as a quaternion of the form (*x*,*y*,*z*,*w*).
            fn rotation(reference_frame: &ReferenceFrame) -> Quaternion {
                Rotation(reference_frame)
            }
        }
    }
});

remote_type!(
    /// The state of a docking port. See `DockingPort::state()`.
    enum DockingPortState {
        /// The docking port is ready to dock to another docking port.
        Ready = 0,
        /// The docking port is docked to another docking port, or docked to another part
        /// (from the VAB/SPH).
        Docked = 1,
        /// The docking port is very close to another docking port, but has not docked. It is
        /// using magnetic force to acquire a solid dock.
        Docking = 2,
        /// The docking port has just been undocked from another docking port, and is disabled
        /// until it moves away by a sufficient distance (`DockingPort::reengage_distance()`).
        Undocking = 3,
        /// The docking port has a shield, and the shield is closed.
        Shielded = 4,
        /// The docking ports shield is currently opening/closing.
        Moving = 5,
    }
);
