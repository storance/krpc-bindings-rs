use super::Part;
use crate::codec::{Decode, Encode};
use crate::{remote_type, RemoteObject, Vector3};

remote_type!(
/// A control surface. Obtained by calling `Part::control_surface().`
object SpaceCenter.ControlSurface {
    properties: {
        {
            Part {
                /// Returns the part object for this control surface.
                ///
                /// **Game Scenes**: All
                get: part -> Part
            }
        }
        {
            PitchEnabled {
                /// Returns whether the control surface has pitch control enabled.
                ///
                /// **Game Scenes**: All
                get: is_pitch_enabled -> bool,
                /// Sets whether the control surface has pitch control enabled.
                ///
                /// **Game Scenes**: All
                set: set_pitch_enabled(bool)
            }
        }
        {
            YawEnabled {
                /// Returns whether the control surface has yaw control enabled.
                ///
                /// **Game Scenes**: All
                get: is_yaw_enabled -> bool,
                /// Sets whether the control surface has yaw control enabled.
                ///
                /// **Game Scenes**: All
                set: set_yaw_enabled(bool)
            }
        }
        {
            RollEnabled {
                /// Returns whether the control surface has roll control enabled.
                ///
                /// **Game Scenes**: All
                get: is_roll_enabled -> bool,
                /// Sets whether the control surface has roll control enabled.
                ///
                /// **Game Scenes**: All
                set: set_roll_enabled(bool)
            }
        }
        {
            AuthorityLimiter {
                /// Returns the authority limiter for the control surface, which controls how
                /// far the control surface will move.
                ///
                /// **Game Scenes**: All
                get: authority_limiter -> f32,
                /// Sets the authority limiter for the control surface, which controls how
                /// far the control surface will move.
                ///
                /// **Game Scenes**: All
                set: set_authority_limiter(f32)
            }
        }
        {
            Inverted {
                /// Returns whether the control surface movement is inverted.
                ///
                /// **Game Scenes**: All
                get: is_inverted -> bool,
                /// Sets whether the control surface movement is inverted.
                ///
                /// **Game Scenes**: All
                set: set_inverted(bool)
            }
        }
        {
            Deployed {
                /// Returns whether the control surface has been fully deployed.
                ///
                /// **Game Scenes**: All
                get: is_deployed -> bool,
                /// Sets whether the control surface has been fully deployed.s
                ///
                /// **Game Scenes**: All
                set: set_deployed(bool)
            }
        }
        {
            SurfaceArea {
                /// Returns the surface area of the control surface in m<sup>2</sup>.
                ///
                /// **Game Scenes**: All
                get: surface_area -> f32
            }
        }
        {
            AvailableTorque {
                /// Returns the available torque, in Newton meters, that can be produced by this
                /// control surface, in the positive and negative pitch, roll and yaw axes of the
                /// vessel. These axes correspond to the coordinate axes of the
                /// `Vessel::reference_frame()`.
                ///
                /// **Game Scenes**: All
                get: available_torque -> (Vector3, Vector3)
            }
        }
    }
});
