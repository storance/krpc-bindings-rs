use crate::*;
use crate::codec::*;
use super::{Vessel, Node};

use std::rc::{Rc};
use std::cell::{RefCell};


remote_type!(
/// Used to manipulate the controls of a vessel. This includes adjusting the throttle,
/// enabling/disabling systems such as SAS and RCS, or altering the direction in which the vessel
/// is pointing. Obtained by calling `Vessel::control()`.
///
/// # Note
/// Control inputs (such as pitch, yaw and roll) are zeroed when all clients that have set one or
/// more of these inputs are no longer connected.
object Control {}
);

impl Control {
    rpc_property!(
        Source: ControlSource {
            service: SpaceCenter,
            class: Control,
            /// Returns the source of the vessels control, for example by a kerbal or a probe core.
            ///
            /// **Game Scenes**: Flight
            source
        }
    );

    rpc_property!(
        State: ControlState {
            service: SpaceCenter,
            class: Control,
            /// Returns the control state of the vessel.
            ///
            /// **Game Scenes**: Flight
            state
        }
    );


    rpc_property!(
        SAS: bool {
            service: SpaceCenter,
            class: Control,
            /// Returns whether or not SAS is enabled.
            ///
            /// **Game Scenes**: Flight
            is_sas_enabled,
            /// Sets whether or not SAS is enabled.
            ///
            /// **Game Scenes**: Flight
            set_sas_enabled(is_sas_enabled)
        }
    );

    rpc_property!(
        sas_mode: SASMode {
            service: SpaceCenter,
            class: Control,
            /// Returns the current SASMode. These modes are equivalent to the mode buttons to
            /// the left of the navball that appear when SAS is enabled.
            ///
            /// **Game Scenes**: Flight
            sas_mode,
            /// Sets the current SASMode. These modes are equivalent to the mode buttons to the
            /// left of the navball that appear when SAS is enabled.
            ///
            /// **Game Scenes**: Flight
            set_sas_mode(sas_mode)
        }
    );

    rpc_property!(
        SpeedMode: SpeedMode {
            service: SpaceCenter,
            class: Control,
            /// Returns the current SpeedMode of the navball. This is the mode displayed next to
            /// the speed at the top of the navball.
            ///
            /// **Game Scenes**: Flight
            speed_mode,
            /// Sets the current SpeedMode of the navball. This is the mode displayed next to
            /// the speed at the top of the navball.
            ///
            /// **Game Scenes**: Flight
            set_speed_mode(speed_mode)
        }
    );

    rpc_property!(
        RCS: bool {
            service: SpaceCenter,
            class: Control,
            /// Returns whether or not RCS is enabled.
            ///
            /// **Game Scenes**: Flight
            is_rcs_enabled,
            /// Sets whether or not RCS is enabled.
            ///
            /// **Game Scenes**: Flight
            set_rc_enabled(enabled)
        }
    );

    rpc_property!(
        ReactionWheels: bool {
            service: SpaceCenter,
            class: Control,
            /// Returns whether or not all reaction wheels are active.
            ///
            /// **Game Scenes**: Flight
            is_reaction_wheels_active,
            /// Sets whether or not all reaction wheels are active.
            ///
            /// **Game Scenes**: Flight
            set_reaction_wheels_active(enabled)
        }
    );

    rpc_property!(
        Gear: bool {
            service: SpaceCenter,
            class: Control,
            /// Returns whether or not landing legs/gear are deployed.
            ///
            /// **Game Scenes**: Flight
            is_gear_deployed,
            /// Sets whether or not landing legs/gears are deployed.
            ///
            /// **Game Scenes**: Flight
            set_gear_deployed(deploy)
        }
    );

    rpc_property!(
        Legs: bool {
            service: SpaceCenter,
            class: Control,
            /// Returns whether all landing legs on the vessel are deployed. Does not include wheels
            /// (for example landing gear). See `Leg::is_deployed()`.
            ///
            /// **Game Scenes**: Flight
            is_legs_deployed,
            /// Sets the deployment state of all landing legs. Does not include wheels
            /// (for example landing gear).
            ///
            /// **Game Scenes**: Flight
            set_legs_deployed(deploy)
        }
    );

    rpc_property!(
        Wheels: bool {
            service: SpaceCenter,
            class: Control,
            /// Returns whether all wheels on the vessel are deployed. Does not include
            /// landing legs. See `Wheel::is_deployed()`.
            ///
            /// **Game Scenes**: Flight
            is_wheels_deployed,
            /// Sets the deployment state of all wheels. Does not include landing legs.
            ///
            /// **Game Scenes**: Flight
            set_wheels_deployed(deploy)
        }
    );

    rpc_property!(
        Lights: bool {
            service: SpaceCenter,
            class: Control,
            /// Returns whether or not the lights are turned on.
            ///
            /// **Game Scenes**: Flight
            is_lights_on,
            /// Sets whether or not the lights are turned on.
            ///
            /// **Game Scenes**: Flight
            set_lights_on(on)
        }
    );

    rpc_property!(
        Brakes: bool {
            service: SpaceCenter,
            class: Control,
            /// Returns whether or not the wheel brakes are active.
            ///
            /// **Game Scenes**: Flight
            is_brakes_active,
            /// Sets whether or not the wheel brakes are active.
            ///
            /// **Game Scenes**: Flight
            set_braked_active(active)
        }
    );

    rpc_property!(
        Antennas: bool {
            service: SpaceCenter,
            class: Control,
            /// Returns whether all antennas on the vessel are deployed.
            /// See `Antenna::is_deployed()`.
            ///
            /// **Game Scenes**: Flight
            is_antennas_deployed,
            /// Sets the deployment state of all antennas.
            ///
            /// **Game Scenes**: Flight
            set_antennas_deployed(deploy)
        }
    );

    rpc_property!(
        CargoBays: bool {
            service: SpaceCenter,
            class: Control,
            /// Returns whether any of the cargo bays on the vessel are open.
            /// See `CargoBay::is_open()`.
            ///
            /// **Game Scenes**: Flight
            is_cargo_bays_open,
            /// Sets the open state of all cargo bays.
            ///
            /// **Game Scenes**: Flight
            set_cargo_bays_open(open)
        }
    );

    rpc_property!(
        Intakes: bool {
            service: SpaceCenter,
            class: Control,
            /// Returns whether all of the air intakes on the vessel are open.
            /// See `Intakes::is_open()`.
            ///
            /// **Game Scenes**: Flight
            is_intakes_open,
            /// Sets the open state of all air intakes.
            ///
            /// **Game Scenes**: Flight
            set_intakes_open(open)
        }
    );

    rpc_property!(
        Parachutes: bool {
            service: SpaceCenter,
            class: Control,
            /// Returns whether all parachutes on the vessel are deployed.
            /// See `Parachute::is_deployed()`.
            ///
            /// **Game Scenes**: Flight
            is_parachutes_deployed,
            /// Sets the deployment state of all parachutes. Cannot be set to false.
            ///
            /// **Game Scenes**: Flight
            set_parachutes_deployed(deploy)
        }
    );

    rpc_property!(
        Radiators: bool {
            service: SpaceCenter,
            class: Control,
            /// Returns whether all radiators on the vessel are deployed.
            /// See `Radiator::is_deployed()`.
            ///
            /// **Game Scenes**: Flight
            is_radiators_deployed,
            /// Sets the deployment state of all radiators.
            ///
            /// **Game Scenes**: Flight
            set_radiators_deployed(deploy)
        }
    );

    rpc_property!(
        ResourceHarvesters: bool {
            service: SpaceCenter,
            class: Control,
            /// Returns whether all of the resource harvesters on the vessel are deployed.
            /// See `ResourceHarvester::is_deployed()`.
            ///
            /// **Game Scenes**: Flight
            is_resource_harvesters_deployed,
            /// Sets the deployment state of all resource harvesters.
            ///
            /// **Game Scenes**: Flight
            set_resource_harvesters_deployed(deploy)
        }
    );

    rpc_property!(
        ResourceHarvestersActive: bool {
            service: SpaceCenter,
            class: Control,
            /// Returns whether all of the resource harvesters on the vessel are active.
            /// See `ResourceHarvester::is_active()`.
            ///
            /// **Game Scenes**: Flight
            is_resource_harvesters_active,
            /// Sets the deployment state of all resource harvesters.
            ///
            /// **Game Scenes**: Flight
            set_resource_harvesters_active(active)
        }
    );

    rpc_property!(
        SolarPanels: bool {
            service: SpaceCenter,
            class: Control,
            /// Returns whether all solar panels on the vessel are deployed.
            /// See `SolarPanel::is_deployed()`.
            ///
            /// **Game Scenes**: Flight
            is_solar_panels_deployed,
            /// Sets the deployment state of all solar panels.
            ///
            /// **Game Scenes**: Flight
            set_solar_panels_deployed(deploy)
        }
    );

    rpc_property!(
        Abort: bool {
            service: SpaceCenter,
            class: Control,
            /// Returns whether or not the abort action grouped has been activated.
            ///
            /// **Game Scenes**: Flight
            is_abort,
            /// Sets whether or not the abort action grouped has been activated.
            ///
            /// **Game Scenes**: Flight
            set_abort(abort)
        }
    );

    rpc_property!(
        Throttle: f32 {
            service: SpaceCenter,
            class: Control,
            /// Returns the state of the throttle. A value between 0 and 1.
            ///
            /// **Game Scenes**: Flight
            throttle,
            /// Sets the state of the throttle. A value between 0 and 1.
            ///
            /// **Game Scenes**: Flight
            set_throttle(throttle)
        }
    );

    rpc_property!(
        InputMode: f32 {
            service: SpaceCenter,
            class: Control,
            /// Returns the behavior of the pitch, yaw, roll and translation control inputs.
            ///
            /// **Game Scenes**: Flight
            input_mode,
            /// Sets the behavior of the pitch, yaw, roll and translation control inputs.
            /// When set to additive, these inputs are added to the vessels current inputs.
            /// This mode is the default. When set to override, these inputs (if non-zero)
            /// override the vessels inputs. This mode prevents keyboard control, or SAS,
            /// from interfering with the controls when they are set.
            ///
            /// **Game Scenes**: Flight
            set_input_mode(input_mode)
        }
    );

    rpc_property!(
        Pitch: f32 {
            service: SpaceCenter,
            class: Control,
            /// Returns the pitch control. A value between -1 and 1. Equivalent to the w and s keys.
            ///
            /// **Game Scenes**: Flight
            pitch,
            /// Sets the pitch control. A value between -1 and 1. Equivalent to the w and s keys.
            ///
            /// **Game Scenes**: Flight
            set_pitch(pitch)
        }
    );

    rpc_property!(
        Yaw: f32 {
            service: SpaceCenter,
            class: Control,
            /// Returns the yaw control. A value between -1 and 1. Equivalent to the a and d keys.
            ///
            /// **Game Scenes**: Flight
            yaw,
            /// Sets the yaw control. A value between -1 and 1. Equivalent to the a and d keys.
            ///
            /// **Game Scenes**: Flight
            set_yaw(yaw)
        }
    );

    rpc_property!(
        Roll: f32 {
            service: SpaceCenter,
            class: Control,
            /// Returns the yaw control. A value between -1 and 1. Equivalent to the a and d keys.
            ///
            /// **Game Scenes**: Flight
            roll,
            /// Sets the yaw control. A value between -1 and 1. Equivalent to the a and d keys.
            ///
            /// **Game Scenes**: Flight
            set_roll(roll)
        }
    );

    rpc_property!(
        Forward: f32 {
            service: SpaceCenter,
            class: Control,
            /// Returns the forward translation control. A value between -1 and 1.
            /// Equivalent to the h and n keys.
            ///
            /// **Game Scenes**: Flight
            translation_forward,
            /// Sets the forward translation control. A value between -1 and 1.
            /// Equivalent to the h and n keys.
            ///
            /// **Game Scenes**: Flight
            set_translation_forward(forward)
        }
    );

    rpc_property!(
        Up: f32 {
            service: SpaceCenter,
            class: Control,
            /// Returns the up translation control. A value between -1 and 1.
            /// Equivalent to the i and k keys.
            ///
            /// **Game Scenes**: Flight
            translation_up,
            /// Sets the up translation control. A value between -1 and 1.
            /// Equivalent to the i and k keys.
            ///
            /// **Game Scenes**: Flight
            set_translation_up(up)
        }
    );

    rpc_property!(
        Right: f32 {
            service: SpaceCenter,
            class: Control,
            /// Returns the right translation control. A value between -1 and 1.
            /// Equivalent to the j and l keys.
            ///
            /// **Game Scenes**: Flight
            translation_right,
            /// Sets the right translation control. A value between -1 and 1.
            /// Equivalent to the j and l keys.
            ///
            /// **Game Scenes**: Flight
            set_translation_right(right)
        }
    );

    rpc_property!(
        WheelThrottle: f32 {
            service: SpaceCenter,
            class: Control,
            /// Returns the wheel throttle control. A value between -1 and 1.
            ///
            /// **Game Scenes**: Flight
            wheel_throttle,
            /// Sets the wheel throttle control. A value between -1 and 1.
            /// A value of 1 rotates the wheels forwards, a value of -1 rotates
            /// the wheels backwards.
            ///
            /// **Game Scenes**: Flight
            set_wheel_throttle(throttle)
        }
    );

    rpc_property!(
        WheelSteering: f32 {
            service: SpaceCenter,
            class: Control,
            /// Returns the wheel steering control. A value between -1 and 1.
            ///
            /// **Game Scenes**: Flight
            wheel_steering,
            /// Sets the wheel steering control. A value between -1 and 1.
            /// A value of 1 steers to the left, and a value of -1 steers to the right.
            ///
            /// **Game Scenes**: Flight
            set_wheel_steering(steering)
        }
    );

    rpc_property!(
        CurrentStage: f32 {
            service: SpaceCenter,
            class: Control,
            /// Returns the current stage of the vessel. Corresponds to the stage
            /// number in the in-game UI.
            ///
            /// **Game Scenes**: Flight
            current_stage
        }
    );

    rpc_method!(
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
    fn activate_next_stage(&self) -> Vec<Vessel> {
        SpaceCenter.Control_ActivateNextStage(self)
            .ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// Returns `true` if the given action group is enabled.
    ///
    /// **Game Scenes**: Flight
    ///
    /// # Arguments
    /// * `group` - A number between 0 and 9 inclusive, or between 0 and 250 inclusive
    /// when the Extended Action Groups mod is installed.
    fn is_action_group_enabled(&self, group: i32) -> bool {
        SpaceCenter.Control_GetActionGroup(self, group)
            .ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// Sets whether the action group is enabled.
    ///
    /// **Game Scenes**: Flight
    ///
    /// # Arguments
    /// * `group` - A number between 0 and 9 inclusive, or between 0 and 250 inclusive
    /// when the Extended Action Groups mod is installed.
    /// * `enabled` - Whether or not to enable the action group
    fn set_action_group_enabled(&self, group: i32, enabled: bool) {
        SpaceCenter.Control_SetActionGroup(self, group, enabled)
    });

    rpc_method!(
    /// Toggles the enablement of the given action group.
    ///
    /// **Game Scenes**: Flight
    ///
    /// # Arguments
    /// * `group` - A number between 0 and 9 inclusive, or between 0 and 250 inclusive
    /// when the Extended Action Groups mod is installed.
    fn toggle_action_group(&self, group: i32) {
        SpaceCenter.Control_ToggleActionGroup(self, group)
    });

    rpc_method!(
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
    fn add_node(&self, ut: f64, prograde: f32, normal: f32, radial: f32) -> Node {
        SpaceCenter.Control_AddNode(self, ut, prograde, normal, radial)
            .ok_or(KrpcError::NullResponseValue)
    });

    rpc_property!(
        Nodes: Vec<Node> {
            service: SpaceCenter,
            class: Control,
            /// Returns a list of all existing maneuver nodes, ordered by time from first to last.
            ///
            /// **Game Scenes**: Flight
            nodes
        }
    );

    rpc_method!(
    /// Remove all maneuver nodes.
    ///
    /// **Game Scenes**: Flight
    fn remove_nodes(&self) {
        SpaceCenter.Control_RemoveNodes(self)
    });
}

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