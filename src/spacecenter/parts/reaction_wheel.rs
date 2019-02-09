use super::Part;
use crate::codec::{Decode, Encode};
use crate::{remote_type, RemoteObject, Vector3};

remote_type!(
/// An reaction wheel. Obtained by calling `Part::reaction_wheel().`
object SpaceCenter.ReactionWheel {
    properties: {
        {
            Part {
                /// Returns the part object for this reaction wheel.
                ///
                /// **Game Scenes**: All
                get: part -> Part
            }
        }
        {
            Active {
                /// Returns whether the reaction wheel is active.
                ///
                /// **Game Scenes**: All
                get: is_active -> bool,
                /// Sets whether the reaction wheel is active.
                ///
                /// **Game Scenes**: All
                set: set_active(bool)
            }
        }
        {
            Broken {
                /// Returns whether the reaction wheel is broken.
                ///
                /// **Game Scenes**: All
                get: is_broken -> bool
            }
        }
        {
            AvailableTorque {
                /// Returns the available torque, in Newton meters, that can be produced by this
                /// engine, in the positive and negative pitch, roll and yaw axes of the vessel.
                /// These axes correspond to the coordinate axes of the `Vessel::reference_frame()`.
                /// Returns zero if the reaction wheel is inactive or broken.
                ///
                /// **Game Scenes**: All
                get: available_torque -> (Vector3, Vector3)
            }
        }
        {
            MaxTorque {
                /// Returns the maximum torque, in Newton meters, that can be produced by this
                /// reaction wheel, when it is active, in the positive and negative pitch, roll
                /// and yaw axes of the vessel.  These axes correspond to the coordinate axes
                /// of the `Vessel::reference_frame()`.
                ///
                /// **Game Scenes**: All
                get: max_torque -> (Vector3, Vector3)
            }
        }
    }
});
