use crate::*;
use crate::codec::*;
use super::{ReferenceFrame, SASMode};

use std::rc::{Rc};
use std::cell::{RefCell};

remote_type!(
/// Provides basic auto-piloting utilities for a vessel. Created by calling `Vessel::auto_pilot()`.
///
/// # Note
/// If a client engages the auto-pilot and then closes its connection to the server, the
/// auto-pilot will be disengaged and its target reference frame, direction and roll
/// reset to default.
object AutoPilot {}
);

impl AutoPilot {
    rpc_method!(
    /// Engage the auto-pilot.
    ///
    /// **Game Scenes**: Flight
    fn engage(&self) {
        SpaceCenter.AutoPilot_Engage(self)
    });

    rpc_method!(
    /// Disengage the auto-pilot.
    ///
    /// **Game Scenes**: Flight
    fn disengage(&self) {
        SpaceCenter.AutoPilot_Disengage(self)
    });

    rpc_method!(
    /// Blocks until the vessel is pointing in the target direction and has the target roll
    /// (if set). Returns an error if the auto-pilot has not been engaged.
    ///
    /// **Game Scenes**: Flight
    fn wait(&self) {
        SpaceCenter.AutoPilot_Wait(self)
    });

    rpc_property!(
        Error: f32 {
            service: SpaceCenter,
            class: AutoPilot,
            /// Returns the error, in degrees, between the direction the ship has been asked to
            /// point in and the direction it is pointing in. Returns an error if the auto-pilot
            /// has not been engaged and SAS is not enabled or is in stability assist mode.
            ///
            /// **Game Scenes**: Flight
            error
        }
    );

    rpc_property!(
        PitchError: f32 {
            service: SpaceCenter,
            class: AutoPilot,
            /// Returns the error, in degrees, between the vessels current and target pitch. Returns an
            /// error if the auto-pilot has not been engaged.
            ///
            /// **Game Scenes**: Flight
            pitch_error
        }
    );

    rpc_property!(
        HeadingError: f32 {
            service: SpaceCenter,
            class: AutoPilot,
            /// Returns the error, in degrees, between the vessels current and target heading.
            /// Returns an error if the auto-pilot has not been engaged.
            ///
            /// **Game Scenes**: Flight
            heading_error
        }
    );

    rpc_property!(
        RollError: f32 {
            service: SpaceCenter,
            class: AutoPilot,
            /// Returns the error, in degrees, between the vessels current and target roll.
            /// Returns an error if the auto-pilot has not been engaged.
            ///
            /// **Game Scenes**: Flight
            roll_error
        }
    );

    rpc_property!(
        ReferenceFrame: ReferenceFrame {
            service: SpaceCenter,
            class: AutoPilot,
            /// Returns the reference frame for the target direction (`AutoPilot::target_direction()`).
            ///
            /// **Game Scenes**: Flight
            reference_frame,
            /// Sets the reference frame for the target direction (`AutoPilot::target_direction()`).
            ///
            /// **Game Scenes**: Flight
            ///
            /// # Note
            /// An error will be thrown if this property is set to a reference frame that rotates with
            /// the vessel being controlled, as it is impossible to rotate the vessel in such
            /// a reference frame.
            set_reference_frame(reference_frame)
        }
    );

    rpc_property!(
        TargetPitch: f32 {
            service: SpaceCenter,
            class: AutoPilot,
            /// Returns the target pitch, in degrees, between -90° and +90°.
            ///
            /// **Game Scenes**: Flight
            target_pitch,
            /// Sets the target pitch, in degrees, between -90° and +90°.
            ///
            /// **Game Scenes**: Flight
            set_target_pitch(pitch)
        }
    );

    rpc_property!(
        TargetHeading: f32 {
            service: SpaceCenter,
            class: AutoPilot,
            /// Returns the target heading, in degrees, between 0° and 360°.
            ///
            /// **Game Scenes**: Flight
            target_heading,
            /// Sets the target heading, in degrees, between 0° and 360°.
            ///
            /// **Game Scenes**: Flight
            set_target_heading(heading)
        }
    );

    rpc_property!(
        TargetRoll: f32 {
            service: SpaceCenter,
            class: AutoPilot,
            /// Returns the target roll or `NaN` if no target roll is set.
            ///
            /// **Game Scenes**: Flight
            target_roll,
            /// Sets the target roll.
            ///
            /// **Game Scenes**: Flight
            set_target_roll(target_roll)
        }
    );

    rpc_property!(
        TargetDirection: Vector3 {
            service: SpaceCenter,
            class: AutoPilot,
            /// Returns the direction vector corresponding to the target pitch and heading. This is in
            /// the reference frame specified by ReferenceFrame.
            ///
            /// **Game Scenes**: Flight
            target_direction,
            /// Sets the direction vector corresponding to the target pitch and heading. This is in the
            /// reference frame specified by `ReferenceFrame`.
            ///
            /// **Game Scenes**: Flight
            set_target_direction(direction)
        }
    );

    rpc_method!(
    /// Set target pitch and heading angles.
    ///
    /// **Game Scenes**: Flight
    ///
    /// # Arguments
    /// * `pitch` - Target pitch angle, in degrees between -90° and +90°.
    /// * `heading` - Target heading angle, in degrees between 0° and 360°.
    fn target_pitch_and_heading(&self, pitch: f32, heading: f32) {
        SpaceCenter.AutoPilot_TargetPitchAndHeading(self, pitch, heading)
    });


    rpc_property!(
        SAS: bool {
            service: SpaceCenter,
            class: AutoPilot,
            /// Returns whether or not SAS is enabled.
            ///
            /// **Game Scenes**: Flight
            is_sas_enabled,
            /// Sets whether or not SAS is enabled.
            ///
            /// **Game Scenes**: Flight
            set_sas_enabled(enabled)
        }
    );

    rpc_property!(
        SASMode: SASMode {
            service: SpaceCenter,
            class: AutoPilot,
            /// Returns the current SASMode. These modes are equivalent to the mode buttons to the
            /// left of the navball that appear when SAS is enabled.
            ///
            /// **Game Scenes**: Flight
            sas_mode,
            /// Sets the current SASMode. These modes are equivalent to the mode buttons to the
            /// left of the navball that appear when SAS is enabled.
            ///
            /// **Game Scenes**: Flight
            set_sas_mode(mode)
        }
    );

    rpc_property!(
        RollThreshold: f32 {
            service: SpaceCenter,
            class: AutoPilot,
            /// Returns the threshold at which the autopilot will try to match the target roll angle,
            /// if any. Defaults to 5 degrees.
            ///
            /// **Game Scenes**: Flight
            roll_threshold,
            /// Sets the threshold at which the autopilot will try to match the target roll angle,
            /// if any. Defaults to 5 degrees.
            ///
            /// **Game Scenes**: Flight
            set_roll_threshold(roll_threshold)
        }
    );

    rpc_property!(
        StoppingTime : Vector3 {
            service: SpaceCenter,
            class: AutoPilot,
            /// Returns the maximum amount of time that the vessel should need to come to a
            /// complete stop. This determines the maximum angular velocity of the vessel.
            ///
            /// **Game Scenes**: Flight
            ///
            /// # Return
            /// A vector of three stopping times, in seconds, one for each of the pitch, roll and
            /// yaw axes. Defaults to 0.5 seconds for each axis.
            stopping_time,
            /// Sets the maximum amount of time that the vessel should need to come to a
            /// complete stop. This determines the maximum angular velocity of the vessel.
            ///
            /// **Game Scenes**: Flight
            ///
            /// # Arguments
            /// *`stopping_time` - A vector of three stopping times, in seconds, one for each
            /// of the pitch, roll and yaw axes.
            set_stopping_time(stopping_time)
        }
    );

    rpc_property!(
        DecelerationTime: Vector3 {
            service: SpaceCenter,
            class: AutoPilot,
            /// Returns the time the vessel should take to come to a stop pointing in the
            /// target direction. This determines the angular acceleration used to decelerate
            /// the vessel.
            ///
            /// **Game Scenes**: Flight
            ///
            /// # Return
            /// A vector of three times, in seconds, one for each of the pitch, roll and yaw axes.
            /// Defaults to 5 seconds for each axis.
            deceleration_time,
            /// Sets the time the vessel should take to come to a stop pointing in the
            /// target direction. This determines the angular acceleration used to decelerate
            /// the vessel.
            ///
            /// **Game Scenes**: Flight
            ///
            /// # Arguments
            /// *`deceleration_time` - A vector of three times, in seconds, one for each of the
            /// pitch, roll and yaw axes.
            set_deceleration_time(deceleration_time)
        }
    );

    rpc_property!(
        AttenuationAngle : Vector3 {
            service: SpaceCenter,
            class: AutoPilot,
            /// Returns the angle at which the autopilot considers the vessel to be pointing
            /// close to the target. This determines the midpoint of the target velocity
            /// attenuation function.
            ///
            /// **Game Scenes**: Flight
            ///
            /// # Return
            /// A vector of three angles, in degrees, one for each of the pitch, roll and yaw
            /// axes. Defaults to 1° for each axis.
            attenuation_angle,
            /// Sets the angle at which the autopilot considers the vessel to be pointing close
            /// to the target. This determines the midpoint of the target velocity
            /// attenuation function.
            ///
            /// **Game Scenes**: Flight
            ///
            /// # Arguments
            /// *`attenuation_angle` - A vector of three angles, in degrees, one for each of the
            /// pitch, roll and yaw axes.
            set_attenuation_angle(attenuation_angle)
        }
    );

    rpc_property!(
        AutoTune: bool {
            service: SpaceCenter,
            class: AutoPilot,
            /// Returns whether the rotation rate controllers PID parameters should be
            /// automatically tuned using the vessels moment of inertia and available torque.
            /// Defaults to true
            ///
            /// **Game Scenes**: Flight
            auto_tune,
            /// Sets whether the rotation rate controllers PID parameters should be
            /// automatically tuned using the vessels moment of inertia and available torque.
            ///
            /// **Game Scenes**: Flight
            set_auto_tune(auto_tune)
        }
    );

    rpc_property!(
        TimeToPeak: Vector3 {
            service: SpaceCenter,
            class: AutoPilot,
            /// Returns the target time to peak used to autotune the PID controllers.
            ///
            /// **Game Scenes**: Flight
            ///
            /// # Return
            /// A vector of three times, in seconds, for each of the pitch, roll and yaw axes.
            /// Defaults to 3 seconds for each axis.
            time_to_peak,
            /// Sets the target time to peak used to autotune the PID controllers.
            ///
            /// **Game Scenes**: Flight
            ///
            /// # Arguments
            /// *`time_to_peak` - A vector of three times, in seconds, for each of the
            /// pitch, roll and yaw axes.
            set_time_to_peak(time_to_peak)
        }
    );

    rpc_property!(
        Overshoot: Vector3 {
            service: SpaceCenter,
            class: AutoPilot,
            /// Returns the target overshoot percentage used to autotune the PID controllers.
            ///
            /// **Game Scenes**: Flight
            ///
            /// # Return
            /// A vector of three values, between 0 and 1, for each of the pitch, roll and yaw
            /// axes. Defaults to 0.01 for each axis.
            overshoot,
            /// Sets the target overshoot percentage used to autotune the PID controllers.
            ///
            /// **Game Scenes**: Flight
            ///
            /// # Arguments
            /// *`attenuation_angle` - A vector of three values, between 0 and 1, for each of
            /// the pitch, roll and yaw axes.
            set_overshoot(overshoot)
        }
    );

    rpc_property!(
        PitchPIDGains: Vector3 {
            service: SpaceCenter,
            class: AutoPilot,
            /// Returns the gains for the pitch PID controller.
            ///
            /// **Game Scenes**: Flight
            pitch_pid_gains,
            /// Sets the gains for the pitch PID controller.
            ///
            /// **Game Scenes**: Flight
            ///
            /// # Note
            /// When `AutoPilot::auto_tune()` is true, these values are updated automatically,
            /// which will overwrite any manual changes.
            set_pitch_pid_gains(pid_gains)
        }
    );

    rpc_property!(
        RollPIDGains: Vector3 {
            service: SpaceCenter,
            class: AutoPilot,
            /// Returns the gains for the roll PID controller.
            ///
            /// **Game Scenes**: Flight
            roll_pid_gains,
            /// Sets the gains for the roll PID controller.
            ///
            /// **Game Scenes**: Flight
            ///
            /// # Note
            /// When `AutoPilot::auto_tune()` is true, these values are updated automatically,
            /// which will overwrite any manual changes.
            set_roll_pid_gains(pid_gains)
        }
    );

    rpc_property!(
        YawPIDGains: Vector3 {
            service: SpaceCenter,
            class: AutoPilot,
            /// Returns the gains for the yaw PID controller.
            ///
            /// **Game Scenes**: Flight
            yaw_pid_gains,
            /// Sets the gains for the yaw PID controller.
            ///
            /// **Game Scenes**: Flight
            ///
            /// # Note
            /// When `AutoPilot::auto_tune()` is true, these values are updated automatically,
            /// which will overwrite any manual changes.
            set_yaw_pid_gains(pid_gains)
        }
    );
}