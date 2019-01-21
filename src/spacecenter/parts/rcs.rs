use super::Part;
use crate::codec::*;
use crate::spacecenter::{Propellant, Thruster};
use crate::*;

use std::collections::BTreeMap;

remote_type!(
/// An rcs block or thruster. Obtained by calling `Part::rcs().`
object SpaceCenter.RCS {
    properties: {
        {
            Part: Part,
            /// Returns the part object for this rcs block or thruster.
            ///
            /// **Game Scenes**: All
            get: part
        }
        {
            Active: bool,
            /// Returns whether the RCS thrusters are active. An RCS thruster is inactive if
            /// the RCS action group is disabled (`Control::rcs()`), the RCS thruster itself
            /// is not enabled (`RCS::enabled()`) or it is covered by a
            /// fairing (`Part::shielded()`).
            ///
            /// **Game Scenes**: All
            get: is_active
        }
        {
            Enabled: bool,
            /// Returns whether the RCS thrusters are enabled.
            ///
            /// **Game Scenes**: All
            get: is_enabled,
            /// Sets whether the RCS thrusters are enabled.
            ///
            /// **Game Scenes**: All
            set: set_enabled
        }
        {
            PitchEnabled: bool,
            /// Returns whether the RCS thruster will fire when pitch control input is given.
            ///
            /// **Game Scenes**: All
            get: is_pitch_enabled,
            /// Sets whether the RCS thruster will fire when pitch control input is given.
            ///
            /// **Game Scenes**: All
            set: set_pitch_enabled
        }
        {
            YawEnabled: bool,
            /// Returns whether the RCS thruster will fire when yaw control input is given.
            ///
            /// **Game Scenes**: All
            get: is_yaw_enabled,
            /// Sets whether the RCS thruster will fire when yaw control input is given.
            ///
            /// **Game Scenes**: All
            set: set_yaw_enabled
        }
        {
            RollEnabled: bool,
            /// Returns whether the RCS thruster will fire when roll control input is given.
            ///
            /// **Game Scenes**: All
            get: is_roll_enabled,
            /// Sets whether the RCS thruster will fire when roll control input is given.
            ///
            /// **Game Scenes**: All
            set: set_roll_enabled
        }
        {
            ForwardEnabled: bool,
            /// Returns whether the RCS thruster will fire when forward control input is given.
            ///
            /// **Game Scenes**: All
            get: is_forward_enabled,
            /// Sets whether the RCS thruster will fire when forward control input is given.
            ///
            /// **Game Scenes**: All
            set: set_forward_enabled
        }
        {
            UpEnabled: bool,
            /// Returns whether the RCS thruster will fire when up control input is given.
            ///
            /// **Game Scenes**: All
            get: is_up_enabled,
            /// Sets whether the RCS thruster will fire when up control input is given.
            ///
            /// **Game Scenes**: All
            set: set_up_enabled
        }
        {
            RightEnabled: bool,
            /// Returns whether the RCS thruster will fire when right control input is given.
            ///
            /// **Game Scenes**: All
            get: is_right_enabled,
            /// Sets whether the RCS thruster will fire when right control input is given.
            ///
            /// **Game Scenes**: All
            set: set_right_enabled
        }
        {
            AvailableTorque: (Vector3, Vector3),
            /// Returns the available torque, in Newton meters, that can be produced by this RCS,
            /// in the positive and negative pitch, roll and yaw axes of the vessel. These axes
            /// correspond to the coordinate axes of the `Vessel.reference_frame()`.
            /// Returns zero if RCS is disable.
            ///
            /// **Game Scenes**: All
            get: available_torque
        }
        {
            MaxThrust: f32,
            /// Returns the maximum amount of thrust that can be produced by the RCS
            /// thrusters when active, in Newtons.
            ///
            /// **Game Scenes**: All
            get: max_thrust
        }
        {
            MaxVacuumThrust: f32,
            /// Returns the maximum amount of thrust that can be produced by the RCS thrusters
            /// when active in a vacuum, in Newtons.
            ///
            /// **Game Scenes**: All
            get: max_vacuum_thrust
        }
        {
            Thrusters: Vec<Thruster>,
            /// Returns a list of thrusters, one of each nozzel in the RCS part.
            ///
            /// **Game Scenes**: All
            get: thrusters
        }
        {
            SpecificImpulse: f32,
            /// Returns the current specific impulse of the RCS, in seconds. Returns zero if
            /// the engine is not active.
            ///
            /// **Game Scenes**: All
            get: specific_impulse
        }
        {
            VacuumSpecificImpulse: f32,
            /// Returns the vacuum specific impulse of the RCS, in seconds.
            ///
            /// **Game Scenes**: All
            get: vacuum_specific_impulse
        }
        {
            KerbinSeaLevelSpecificImpulse: f32,
            /// Returns the specific impulse of the RCS at sea level on Kerbin, in seconds.
            ///
            /// **Game Scenes**: All
            get: asl_specific_impulse
        }
        {
            PropellantRatios: BTreeMap<String, f32>,
            /// Returns the ratio of resources that the RCS consumes. A dictionary mapping
            /// resource names to the ratio at which they are consumed by the RCS.
            ///
            /// **Game Scenes**: All
            get: propellant_ratios
        }
        {
            Propellants: Vec<Propellant>,
            /// Returns the propellants that the engine consumes.
            ///
            /// **Game Scenes**: All
            get: propellants
        }
        {
            HasFuel: bool,
            /// Returns whether the RCS has any fuel available.
            ///
            /// **Game Scenes**: All
            ///
            /// # Note
            /// The RCS thruster must be activated for this property to update correctly.
            get: has_fuel
        }
    }
});
