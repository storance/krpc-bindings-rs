use super::Part;
use crate::codec::{Decode, Encode};
use crate::spacecenter::Thruster;
use crate::{remote_type, RemoteObject, Vector3};

use std::collections::BTreeMap;

remote_type!(
/// An engine, including ones of various types. For example liquid fuelled gimballed engines,
/// solid rocket boosters and jet engines. Obtained by calling `Part::engine()`.
object SpaceCenter.Engine {
    properties: {
        {
            Part {
                /// Returns the part object for this engine.
                ///
                /// **Game Scenes**: All
                get: part -> Part
            }
        }
        {
            Active {
                /// Returns whether the engine is active.
                ///
                /// **Game Scenes**: All
                get: is_active -> bool,
                /// Sets whether the engine is active.  Setting this attribute may have no effect,
                /// depending on `Engine::can_shutdown()` and `Engine.can_restart()`.
                ///
                /// **Game Scenes**: All
                set: set_active(bool)
            }
        }
        {
            Thrust {
                /// Returns the current amount of thrust being produced by the engine, in Newtons.
                ///
                /// **Game Scenes**: All
                get: thrust -> f32
            }
        }
        {
            AvailableThrust {
                /// Returns the amount of thrust, in Newtons, that would be produced by the engine
                /// when activated and with its throttle set to 100%. Returns zero if the engine
                /// does not have any fuel. Takes the engine’s current `Engine::thrust_limit()`
                /// and atmospheric conditions into account.
                ///
                /// **Game Scenes**: All
                get: available_thrust -> f32
            }
        }
        {
            MaxThrust {
                /// Returns the amount of thrust, in Newtons, that would be produced by the engine
                /// when activated and fueled, with its throttle and throttle limiter set to 100%.
                ///
                /// **Game Scenes**: All
                get: max_thrust -> f32
            }
        }
        {
            MaxVacuumThrust {
                /// Returns the maximum amount of thrust that can be produced by the engine in a
                /// vacuum, in Newtons. This is the amount of thrust produced by the engine when
                /// activated, `Engine.thrust_limit()` is set to 100%, the main vessel’s throttle is
                /// set to 100% and the engine is in a vacuum.
                ///
                /// **Game Scenes**: All
                get: max_vacuum_thrust -> f32
            }
        }
        {
            ThrustLimit {
                /// Returns the thrust limiter of the engine.
                ///
                /// **Game Scenes**: All
                ///
                /// # Returns
                /// A value between 0 and 1.
                get: thrust_limit -> f32,
                /// Sets the thrust limiter of the engine.  Setting this
                /// attribute may have no effect, for example the thrust limit for a solid rocket
                /// booster cannot be changed in flight.
                ///
                /// **Game Scenes**: All
                ///
                /// # Arguments
                /// * `value` - The thrust limit to set as a value between 0 and 1.
                set: set_thrust_limit(f32)
            }
        }
        {
            Thrusters {
                /// Returns the components of the engine that generate thrust.
                ///
                /// **Game Scenes**: All
                ///
                /// # Note
                /// For example, this corresponds to the rocket nozzel on a solid rocket booster, or
                /// the individual nozzels on a RAPIER engine. The overall thrust produced by the
                /// engine, as reported by `Engine::available_thrust()`, `Engine::max_thrust()` and
                /// others, is the sum of the thrust generated by each thruster.
                get: thrusters -> Vec<Thruster>
            }
        }
        {
            SpecificImpulse {
                /// Returns the current specific impulse of the engine, in seconds. Returns zero if
                /// the engine is not active.
                ///
                /// **Game Scenes**: All
                get: isp -> f32
            }
        }
        {
            VacuumSpecificImpulse {
                /// Returns the vacuum specific impulse of the engine, in seconds.
                ///
                /// **Game Scenes**: All
                get: vacuum_isp -> f32
            }
        }
        {
            KerbinSeaLevelSpecificImpulse {
                /// Returns the specific impulse of the engine at sea level on Kerbin, in seconds.
                ///
                /// **Game Scenes**: All
                get: kerbin_sea_level_isp -> f32
            }
        }
        {
            PropellantNames {
                /// Returns the names of the propellants that the engine consumes.
                ///
                /// **Game Scenes**: All
                get: propellant_names -> Vec<String>
            }
        }
        {
            PropellantRatios {
                /// Returns the ratio of resources that the engine consumes. A dictionary mapping
                /// resource names to the ratio at which they are consumed by the engine.
                ///
                /// **Game Scenes**: All
                ///
                /// # Note
                /// For example, if the ratios are 0.6 for LiquidFuel and 0.4 for Oxidizer, then for
                /// every 0.6 units of LiquidFuel that the engine burns, it will burn 0.4 units
                /// of Oxidizer.
                get: propellant_ratios -> BTreeMap<String, f32>
            }
        }
        {
            Propellants {
                /// Returns the propellants that the engine consumes.
                ///
                /// **Game Scenes**: All
                get: propellants -> Vec<Propellant>
            }
        }
        {
            HasFuel {
                /// Returns whether the engine has any fuel available.
                ///
                /// **Game Scenes**: All
                ///
                /// # Note
                /// The engine must be activated for this property to update correctly.
                get: has_fuel -> bool
            }
        }
        {
            Throttle {
                /// Returns the current throttle setting for the engine. A value between 0 and 1.
                /// This is not necessarily the same as the vessel’s main throttle setting, as some
                /// engines take time to adjust their throttle (such as jet engines).
                ///
                /// **Game Scenes**: All
                get: throttle -> f32
            }
        }
        {
            ThrottleLocked {
                /// Returns whether the `Control::throttle()` affects the engine. For example, this
                /// is true for liquid fueled rockets, and false for solid rocket boosters.
                ///
                /// **Game Scenes**: All
                get: is_throttle_locked -> bool
            }
        }
        {
            CanRestart {
                /// Returns whether he engine can be restarted once shutdown. If the engine cannot be
                /// shutdown, returns `false`. For example, this is `true` for liquid fueled rockets
                /// and `false` for solid rocket boosters.
                ///
                /// **Game Scenes**: All
                get: can_restart -> bool
            }
        }
        {
            CanShutdown {
                /// Returns whether the engine can be shutdown once activated. For example, this is
                /// `true` for liquid fueled rockets and `false` for solid rocket boosters.
                ///
                /// **Game Scenes**: All
                get: can_shutdown -> bool
            }
        }
        {
            HasModes {
                /// Returns whether the engine has multiple modes of operation.
                ///
                /// **Game Scenes**: All
                get: has_modes -> bool
            }
        }
        {
            Mode {
                /// Returns the name of the current engine mode.
                ///
                /// **Game Scenes**: All
                get: mode -> String,
                /// Sets the name of the current engine mode.
                ///
                /// **Game Scenes**: All
                set: set_mode(&str)
            }
        }
        {
            Modes {
                /// Returns the available modes for the engine. A dictionary mapping mode
                /// names to Engine objects.
                ///
                /// **Game Scenes**: All
                get: modes -> BTreeMap<String, Engine>
            }
        }
        {
            AutoModeSwitch {
                /// Returns whether the engine will automatically switch modes.
                ///
                /// **Game Scenes**: All
                get: is_auto_mode_switch -> bool,
                /// Sets whether the engine will automatically switch modes.
                ///
                /// **Game Scenes**: All
                set: set_auto_mode_switch(bool)
            }
        }
        {
            Gimballed {
                /// Returns whether the engine is gimballed.
                ///
                /// **Game Scenes**: All
                get: is_gimballed -> bool
            }
        }
        {
            GimbalRange {
                /// Returns the range over which the gimbal can move, in degrees. Returns 0 if the
                /// engine is not gimballed.
                ///
                /// **Game Scenes**: All
                get: gimbal_range -> f32
            }
        }
        {
            GimbalLocked {
                /// Returns whether the engines gimbal is locked in place.
                ///
                /// **Game Scenes**: All
                get: is_gimbal_locked -> bool,
                /// Sets whether the engines gimbal is locked in place. Setting this attribute has no
                /// effect if the engine is not gimballed.
                ///
                /// **Game Scenes**: All
                set: set_gimbal_locked(bool)
            }
        }
        {
            GimbalLimit {
                /// Returns the gimbal limiter of the engine. A value between 0 and 1.
                /// Returns 0 if the gimbal is locked.
                ///
                /// **Game Scenes**: All
                get: gimbal_limit -> f32,
                /// Sets the gimbal limiter of the engine. A value between 0 and 1.
                ///
                /// **Game Scenes**: All
                set: set_gimbal_limit(f32)
            }
        }
        {
            AvailableTorque {
                /// Returns the available torque, in Newton meters, that can be produced by this
                /// engine, in the positive and negative pitch, roll and yaw axes of the vessel.
                /// These axes correspond to the coordinate axes of the `Vessel::reference_frame()`.
                /// Returns zero if the engine is inactive, or not gimballed.
                ///
                /// **Game Scenes**: All
                get: available_torque -> (Vector3, Vector3)
            }
        }
    }
    methods: {
        {
            /// Toggle the current engine mode.
            ///
            /// **Game Scenes**: All
            fn toggle_mode() {
                ToggleMode()
            }
        }
    }
});

remote_type!(
/// A propellant for an engine. Obtained by calling `Engine::propellants()`.
object SpaceCenter.Propellant {
    properties: {
        {
            Name {
                /// Returns the name of the propellant.
                ///
                /// **Game Scenes**: All
                get: name -> String
            }
        }
        {
            CurrentAmount {
                /// Returns the current amount of propellant.
                ///
                /// **Game Scenes**: All
                get: current_amount -> f64
            }
        }
        {
            CurrentRequirement {
                /// Returns the required amount of propellant.
                ///
                /// **Game Scenes**: All
                get: current_requirement -> f64
            }
        }
        {
            TotalResourceAvailable {
                /// Returns the total amount of the underlying resource currently reachable
                /// given resource flow rules.
                ///
                /// **Game Scenes**: All
                get: total_resource_available -> f64
            }
        }
        {
            TotalResourceCapacity {
                /// Returns the total vehicle capacity for the underlying propellant resource,
                /// restricted by resource flow rules.
                ///
                /// **Game Scenes**: All
                get: total_resource_capacity -> f64
            }
        }
        {
            IgnoreForIsp {
                /// Returns if this propellant should be ignored when calculating required mass
                /// flow given specific impulse.
                ///
                /// **Game Scenes**: All
                get: is_ignore_for_isp -> bool
            }
        }
        {
            IgnoreForThrustCurve {
                /// Returns if this propellant should be ignored for thrust curve calculations.
                ///
                /// **Game Scenes**: All
                get: is_ignore_for_thrust_curve -> bool
            }
        }
        {
            DrawStackGauge {
                /// Returns if this propellant has a stack gauge or not.
                ///
                /// **Game Scenes**: All
                get: is_draw_stack_gauge -> bool
            }
        }
        {
            IsDeprived {
                /// Returns if this propellant is deprived.
                ///
                /// **Game Scenes**: All
                get: is_deprived -> bool
            }
        }
        {
            Ratio {
                /// Returns the propellant ratio.
                ///
                /// **Game Scenes**: All
                get: ratio -> f32
            }
        }
    }
});
