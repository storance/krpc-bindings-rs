use super::Part;
use crate::codec::*;
use crate::krpc::Expression;
use crate::*;

remote_type!(
/// An reaction wheel. Obtained by calling `Part::reaction_wheel().`
object SpaceCenter.ReactionWheel {
    properties: {
        {
            Part: Part,
            /// Returns the part object for this reaction wheel.
            ///
            /// **Game Scenes**: All
            get: part
        }
        {
            Active: bool,
            /// Returns whether the reaction wheel is active.
            ///
            /// **Game Scenes**: All
            get: is_active,
            /// Sets whether the reaction wheel is active.
            ///
            /// **Game Scenes**: All
            set: set_active
        }
        {
            Broken: bool,
            /// Returns whether the reaction wheel is broken.
            ///
            /// **Game Scenes**: All
            get: is_broken
        }
        {
            AvailableTorque: (Vector3, Vector3),
            /// Returns the available torque, in Newton meters, that can be produced by this
            /// engine, in the positive and negative pitch, roll and yaw axes of the vessel.
            /// These axes correspond to the coordinate axes of the `Vessel::reference_frame()`.
            /// Returns zero if the reaction wheel is inactive or broken.
            ///
            /// **Game Scenes**: All
            get: available_torque
        }
        {
            MaxTorque: (Vector3, Vector3),
            /// Returns the maximum torque, in Newton meters, that can be produced by this
            /// reaction wheel, when it is active, in the positive and negative pitch, roll
            /// and yaw axes of the vessel.  These axes correspond to the coordinate axes
            /// of the `Vessel::reference_frame()`.
            ///
            /// **Game Scenes**: All
            get: max_torque
        }
    }
});
