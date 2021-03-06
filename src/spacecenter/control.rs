use super::{Node, Vessel};
use crate::codec::{Decode, Encode};
use crate::{remote_type, RemoteEnum, RemoteObject};

remote_type!(
/// Used to manipulate the controls of a vessel. This includes adjusting the throttle,
/// enabling/disabling systems such as SAS and RCS, or altering the direction in which the vessel
/// is pointing. Obtained by calling `Vessel::control()`.
///
/// # Note
/// Control inputs (such as pitch, yaw and roll) are zeroed when all clients that have set one or
/// more of these inputs are no longer connected.
object SpaceCenter.Control {
    properties: {
        {
            Source {
                /// Returns the source of the vessels control, for example by a kerbal or a probe core.
                ///
                /// **Game Scenes**: Flight
                get: source -> ControlSource
            }
        }
        {
            State {
                /// Returns the control state of the vessel.
                ///
                /// **Game Scenes**: Flight
                get: state -> ControlState
            }
        }
        {
            SAS {
                /// Returns whether or not SAS is enabled.
                ///
                /// **Game Scenes**: Flight
                get: is_sas_enabled -> bool,
                /// Sets whether or not SAS is enabled.
                ///
                /// **Game Scenes**: Flight
                set: set_sas_enabled(bool)
            }
        }
        {
            SASMode {
                /// Returns the current SASMode. These modes are equivalent to the mode buttons to
                /// the left of the navball that appear when SAS is enabled.
                ///
                /// **Game Scenes**: Flight
                get: sas_mode -> SASMode,
                /// Sets the current SASMode. These modes are equivalent to the mode buttons to the
                /// left of the navball that appear when SAS is enabled.
                ///
                /// **Game Scenes**: Flight
                set: set_sas_mode(SASMode)
            }
        }
        {
            SpeedMode {
                /// Returns the current SpeedMode of the navball. This is the mode displayed next to
                /// the speed at the top of the navball.
                ///
                /// **Game Scenes**: Flight
                get: speed_mode -> SpeedMode,
                /// Sets the current SpeedMode of the navball. This is the mode displayed next to
                /// the speed at the top of the navball.
                ///
                /// **Game Scenes**: Flight
                set: set_speed_mode(SpeedMode)
            }
        }
        {
            RCS {
                /// Returns whether or not RCS is enabled.
                ///
                /// **Game Scenes**: Flight
                get: is_rcs_enabled -> bool,
                /// Sets whether or not RCS is enabled.
                ///
                /// **Game Scenes**: Flight
                set: set_rcs_enabled(bool)
            }
        }
        {
            ReactionWheels {
                /// Returns whether or not all reaction wheels are active.
                ///
                /// **Game Scenes**: Flight
                get: is_reaction_wheels_active -> bool,
                /// Sets whether or not all reaction wheels are active.
                ///
                /// **Game Scenes**: Flight
                set: set_reaction_wheels_active(bool)
            }
        }
        {
            Gear {
                /// Returns whether or not landing legs/gear are deployed.
                ///
                /// **Game Scenes**: Flight
                get: is_gear_deployed -> bool,
                /// Sets whether or not landing legs/gears are deployed.
                ///
                /// **Game Scenes**: Flight
                set: set_gear_deployed(bool)
            }
        }
        {
            Legs {
                /// Returns whether all landing legs on the vessel are deployed. Does not include wheels
                /// (for example landing gear). See `Leg::is_deployed()`.
                ///
                /// **Game Scenes**: Flight
                get: is_legs_deployed -> bool,
                /// Sets the deployment state of all landing legs. Does not include wheels
                /// (for example landing gear).
                ///
                /// **Game Scenes**: Flight
                set: set_legs_deployed(bool)
            }
        }
        {
            Wheels {
                /// Returns whether all wheels on the vessel are deployed. Does not include
                /// landing legs. See `Wheel::is_deployed()`.
                ///
                /// **Game Scenes**: Flight
                get: is_wheels_deployed -> bool,
                /// Sets the deployment state of all wheels. Does not include landing legs.
                ///
                /// **Game Scenes**: Flight
                set: set_wheels_deployed(bool)
            }
        }
        {
            Lights {
                /// Returns whether or not the lights are turned on.
                ///
                /// **Game Scenes**: Flight
                get: is_lights_on -> bool,
                /// Sets whether or not the lights are turned on.
                ///
                /// **Game Scenes**: Flight
                set: set_lights_on(bool)
            }
        }
        {
            Brakes {
                /// Returns whether or not the wheel brakes are active.
                ///
                /// **Game Scenes**: Flight
                get: is_brakes_active -> bool,
                /// Sets whether or not the wheel brakes are active.
                ///
                /// **Game Scenes**: Flight
                set: set_braked_active(bool)
            }
        }
        {
            Antennas {
                /// Returns whether all antennas on the vessel are deployed.
                /// See `Antenna::is_deployed()`.
                ///
                /// **Game Scenes**: Flight
                get: is_antennas_deployed -> bool,
                /// Sets the deployment state of all antennas.
                ///
                /// **Game Scenes**: Flight
                set: set_antennas_deployed(bool)
            }
        }
        {
            CargoBays {
                /// Returns whether any of the cargo bays on the vessel are open.
                /// See `CargoBay::is_open()`.
                ///
                /// **Game Scenes**: Flight
                get: is_cargo_bays_open -> bool,
                /// Sets the open state of all cargo bays.
                ///
                /// **Game Scenes**: Flight
                set: set_cargo_bays_open(bool)
            }
        }
        {
            Intakes {
                /// Returns whether all of the air intakes on the vessel are open.
                /// See `Intakes::is_open()`.
                ///
                /// **Game Scenes**: Flight
                get: is_intakes_open -> bool,
                /// Sets the open state of all air intakes.
                ///
                /// **Game Scenes**: Flight
                set: set_intakes_open(bool)
            }
        }
        {
            Parachutes {
                /// Returns whether all parachutes on the vessel are deployed.
                /// See `Parachute::is_deployed()`.
                ///
                /// **Game Scenes**: Flight
                get: is_parachutes_deployed -> bool,
                /// Sets the deployment state of all parachutes. Cannot be set to false.
                ///
                /// **Game Scenes**: Flight
                set: set_parachutes_deployed(bool)
            }
        }
        {
            Radiators {
                /// Returns whether all radiators on the vessel are deployed.
                /// See `Radiator::is_deployed()`.
                ///
                /// **Game Scenes**: Flight
                get: is_radiators_deployed -> bool,
                /// Sets the deployment state of all radiators.
                ///
                /// **Game Scenes**: Flight
                set: set_radiators_deployed(bool)
            }
        }
        {
            ResourceHarvesters {
                /// Returns whether all of the resource harvesters on the vessel are deployed.
                /// See `ResourceHarvester::is_deployed()`.
                ///
                /// **Game Scenes**: Flight
                get: is_resource_harvesters_deployed -> bool,
                /// Sets the deployment state of all resource harvesters.
                ///
                /// **Game Scenes**: Flight
                set: set_resource_harvesters_deployed(bool)
            }
        }
        {
            ResourceHarvestersActive {
                /// Returns whether all of the resource harvesters on the vessel are active.
                /// See `ResourceHarvester::is_active()`.
                ///
                /// **Game Scenes**: Flight
                get: is_resource_harvesters_active -> bool,
                /// Sets the deployment state of all resource harvesters.
                ///
                /// **Game Scenes**: Flight
                set: set_resource_harvesters_active(bool)
            }
        }
        {
            SolarPanels {
                /// Returns whether all solar panels on the vessel are deployed.
                /// See `SolarPanel::is_deployed()`.
                ///
                /// **Game Scenes**: Flight
                get: is_solar_panels_deployed -> bool,
                /// Sets the deployment state of all solar panels.
                ///
                /// **Game Scenes**: Flight
                set: set_solar_panels_deployed(bool)
            }
        }
        {
            Abort {
                /// Returns whether or not the abort action grouped has been activated.
                ///
                /// **Game Scenes**: Flight
                get: is_abort_active -> bool,
                /// Sets whether or not the abort action grouped has been activated.
                ///
                /// **Game Scenes**: Flight
                set: set_abort_active(bool)
            }
        }
        {
            Throttle {
                /// Returns the state of the throttle. A value between 0 and 1.
                ///
                /// **Game Scenes**: Flight
                get: throttle -> f32,
                /// Sets the state of the throttle. A value between 0 and 1.
                ///
                /// **Game Scenes**: Flight
                set: set_throttle(f32)
            }
        }
        {
            InputMode {
                /// Returns the behavior of the pitch, yaw, roll and translation control inputs.
                ///
                /// **Game Scenes**: Flight
                get: input_mode -> ControlInputMode,
                /// Sets the behavior of the pitch, yaw, roll and translation control inputs.
                /// When set to additive, these inputs are added to the vessels current inputs.
                /// This mode is the default. When set to override, these inputs (if non-zero)
                /// override the vessels inputs. This mode prevents keyboard control, or SAS,
                /// from interfering with the controls when they are set.
                ///
                /// **Game Scenes**: Flight
                set: set_input_mode(ControlInputMode)
            }
        }
        {
            Pitch {
                /// Returns the pitch control. A value between -1 and 1. Equivalent to the w and s keys.
                ///
                /// **Game Scenes**: Flight
                get: pitch -> f32,
                /// Sets the pitch control. A value between -1 and 1. Equivalent to the w and s keys.
                ///
                /// **Game Scenes**: Flight
                set: set_pitch(f32)
            }
        }
        {
            Yaw {
                /// Returns the yaw control. A value between -1 and 1. Equivalent to the a and d keys.
                ///
                /// **Game Scenes**: Flight
                get: yaw -> f32,
                /// Sets the yaw control. A value between -1 and 1. Equivalent to the a and d keys.
                ///
                /// **Game Scenes**: Flight
                set: set_yaw(f32)
            }
        }
        {
            Roll {
                /// Returns the yaw control. A value between -1 and 1. Equivalent to the a and d keys.
                ///
                /// **Game Scenes**: Flight
                get: roll -> f32,
                /// Sets the yaw control. A value between -1 and 1. Equivalent to the a and d keys.
                ///
                /// **Game Scenes**: Flight
                set: set_roll(f32)
            }
        }
        {
            Forward {
                /// Returns the forward translation control. A value between -1 and 1.
                /// Equivalent to the h and n keys.
                ///
                /// **Game Scenes**: Flight
                get: translation_forward -> f32,
                /// Sets the forward translation control. A value between -1 and 1.
                /// Equivalent to the h and n keys.
                ///
                /// **Game Scenes**: Flight
                set: set_translation_forward(f32)
            }
        }
        {
            Up {
                /// Returns the up translation control. A value between -1 and 1.
                /// Equivalent to the i and k keys.
                ///
                /// **Game Scenes**: Flight
                get: translation_up -> f32,
                /// Sets the up translation control. A value between -1 and 1.
                /// Equivalent to the i and k keys.
                ///
                /// **Game Scenes**: Flight
                set: set_translation_up(f32)
            }
        }
        {
            Right {
                /// Returns the right translation control. A value between -1 and 1.
                /// Equivalent to the j and l keys.
                ///
                /// **Game Scenes**: Flight
                get: translation_right -> f32,
                /// Sets the right translation control. A value between -1 and 1.
                /// Equivalent to the j and l keys.
                ///
                /// **Game Scenes**: Flight
                set: set_translation_right(f32)
            }
        }
        {
            WheelThrottle {
                /// Returns the wheel throttle control. A value between -1 and 1.
                ///
                /// **Game Scenes**: Flight
                get: wheel_throttle -> f32,
                /// Sets the wheel throttle control. A value between -1 and 1.
                /// A value of 1 rotates the wheels forwards, a value of -1 rotates
                /// the wheels backwards.
                ///
                /// **Game Scenes**: Flight
                set: set_wheel_throttle(f32)
            }
        }
        {
            WheelSteering {
                /// Returns the wheel steering control. A value between -1 and 1.
                ///
                /// **Game Scenes**: Flight
                get: wheel_steering -> f32,
                /// Sets the wheel steering control. A value between -1 and 1.
                /// A value of 1 steers to the left, and a value of -1 steers to the right.
                ///
                /// **Game Scenes**: Flight
                set: set_wheel_steering(f32)
            }
        }
        {
            CurrentStage {
                /// Returns the current stage of the vessel. Corresponds to the stage
                /// number in the in-game UI.
                ///
                /// **Game Scenes**: Flight
                get: current_stage -> i32
            }
        }
        {
            Nodes {
                /// Returns a list of all existing maneuver nodes, ordered by time from first to last.
                ///
                /// **Game Scenes**: Flight
                get: nodes -> Vec<Node>
            }
        }
    }
    methods: {
        {
            /// Activates the next stage. Equivalent to pressing the space bar in-game.
            ///
            /// **Game Scenes**: Flight
            ///
            /// # Return
            /// A list of vessel objects that are jettisoned from the active vessel.
            ///
            /// # Note
            /// When called, the active vessel may change. It is therefore possible that, after
            /// calling this function, the object(s) returned by previous call(s) to `active_vessel()`
            /// no longer refer to the active vessel.
            fn activate_next_stage() -> Vec<Vessel> {
                ActivateNextStage()
            }
        }
        {
            /// Returns `true` if the given action group is enabled.
            ///
            /// **Game Scenes**: Flight
            ///
            /// # Arguments
            /// * `group` - A number between 0 and 9 inclusive, or between 0 and 250 inclusive
            /// when the Extended Action Groups mod is installed.
            fn is_action_group_enabled(group: i32) -> bool {
                GetActionGroup(group)
            }
        }
        {
            /// Sets whether the action group is enabled.
            ///
            /// **Game Scenes**: Flight
            ///
            /// # Arguments
            /// * `group` - A number between 0 and 9 inclusive, or between 0 and 250 inclusive
            /// when the Extended Action Groups mod is installed.
            /// * `enabled` - Whether or not to enable the action group
            fn set_action_group_enabled(group: i32, enabled: bool) {
                SetActionGroup(group, enabled)
            }
        }
        {
            /// Toggles the enablement of the given action group.
            ///
            /// **Game Scenes**: Flight
            ///
            /// # Arguments
            /// * `group` - A number between 0 and 9 inclusive, or between 0 and 250 inclusive
            /// when the Extended Action Groups mod is installed.
            fn toggle_action_group(group: i32) {
                ToggleActionGroup(group)
            }
        }
        {
            /// Creates a maneuver node at the given universal time, and returns a Node object that
            /// can be used to modify it. Optionally sets the magnitude of the delta-v for the
            /// maneuver node in the prograde, normal and radial directions.
            ///
            /// **Game Scenes**: Flight
            ///
            /// # Arguments
            /// * `ut` – Universal time of the maneuver node.
            /// * `prograde` – Delta-v in the prograde direction.
            /// * `normal` – Delta-v in the normal direction.
            /// * `radial` – Delta-v in the radial direction.
            fn add_node(ut: f64, prograde: f32, normal: f32, radial: f32) -> Node {
                AddNode(ut, prograde, normal, radial)
            }
        }
        {
            /// Remove all maneuver nodes.
            ///
            /// **Game Scenes**: Flight
            fn remove_nodes() {
                RemoveNodes()
            }
        }
    }
}
);

remote_type!(
    /// The behavior of the SAS auto-pilot.
    enum SASMode {
        /// Stability assis mode. Dampen out any rotation.
        StabilityAssist = 0,
        /// Point in the burn direction of the next maneuver node.
        Maneuver = 1,
        /// Point in the prograde direction.
        Prograde = 2,
        /// Point in the retrograde direction.
        Retrograde = 3,
        /// Point in the orbit normal direction.
        Normal = 4,
        /// Point in the orbit anti-normal direction.
        AntiNormal = 5,
        /// Point in the orbit radial direction.
        Radial = 6,
        /// Point in the orbit anti-radial direction.
        AntiRadial = 7,
        /// Point in the direction of the current target.
        Target = 8,
        /// Point away from the current target.
        AntiTarget = 9,
    }
);

remote_type!(
    /// The control source of a vessel.
    enum ControlSource {
        /// Vessel is controlled by a Kerbal.
        Kernal = 0,
        /// Vessel is controlled by a probe core.
        Probe = 1,
        /// Vessel is not controlled.
        None = 2,
    }
);

remote_type!(
    /// The control state of a vessel.
    enum ControlState {
        /// Full controllable.
        Full = 0,
        /// Partially controllable.
        Partial = 1,
        /// Not controllable.
        None = 2,
    }
);

remote_type!(
    /// The mode of the speed reported in the navball.
    enum SpeedMode {
        /// Speed is relative to the vessel's orbit.
        Orbit = 0,
        /// Speed is relative to the surface of the body being orbited.
        Surface = 1,
        /// Speed is relative to the current target.
        Taerget = 2,
    }
);

remote_type!(
    /// The control input mode.
    enum ControlInputMode {
        /// Control inputs are added to the vessel's current control inputs.
        Additive = 0,
        /// Control inputs (when they are non-zero) override the vessel's current control inputs.
        Override = 1,
    }
);
