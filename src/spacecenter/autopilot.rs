use crate::*;
use crate::codec::*;
use super::{ReferenceFrame, SASMode};

use std::rc::Rc;

remote_type!(
/// Provides basic auto-piloting utilities for a vessel. Created by calling `Vessel::auto_pilot()`.
///
/// # Note
/// If a client engages the auto-pilot and then closes its connection to the server, the
/// auto-pilot will be disengaged and its target reference frame, direction and roll
/// reset to default.
object SpaceCenter.AutoPilot {
    properties: {
        {
            Error: f32,
            /// Returns the error, in degrees, between the direction the ship has been asked to
            /// point in and the direction it is pointing in. Returns an error if the auto-pilot
            /// has not been engaged and SAS is not enabled or is in stability assist mode.
            ///
            /// **Game Scenes**: Flight
            get: error
        }
        {
            PitchError: f32,
            /// Returns the error, in degrees, between the vessels current and target pitch. Returns an
            /// error if the auto-pilot has not been engaged.
            ///
            /// **Game Scenes**: Flight
            get: pitch_error
        }
        {
            HeadingError: f32,
            /// Returns the error, in degrees, between the vessels current and target heading.
            /// Returns an error if the auto-pilot has not been engaged.
            ///
            /// **Game Scenes**: Flight
            get: heading_error
        }
        {
            RollError: f32,
            /// Returns the error, in degrees, between the vessels current and target roll.
            /// Returns an error if the auto-pilot has not been engaged.
            ///
            /// **Game Scenes**: Flight
            get: roll_error
        }
        {
            ReferenceFrame: ReferenceFrame,
            /// Returns the reference frame for the target direction (`AutoPilot::target_direction()`).
            ///
            /// **Game Scenes**: Flight
            get: reference_frame,
            /// Sets the reference frame for the target direction (`AutoPilot::target_direction()`).
            ///
            /// **Game Scenes**: Flight
            ///
            /// # Note
            /// An error will be thrown if this property is set to a reference frame that rotates with
            /// the vessel being controlled, as it is impossible to rotate the vessel in such
            /// a reference frame.
            set: set_reference_frame
        }
        {
            TargetPitch: f32,
            /// Returns the target pitch, in degrees, between -90° and +90°.
            ///
            /// **Game Scenes**: Flight
            get: target_pitch,
            /// Sets the target pitch, in degrees, between -90° and +90°.
            ///
            /// **Game Scenes**: Flight
            set: set_target_pitch
        }
        {
            TargetHeading: f32,
            /// Returns the target heading, in degrees, between 0° and 360°.
            ///
            /// **Game Scenes**: Flight
            get: target_heading,
            /// Sets the target heading, in degrees, between 0° and 360°.
            ///
            /// **Game Scenes**: Flight
            set: set_target_heading
        }
        {
            TargetRoll: f32,
            /// Returns the target roll or `NaN` if no target roll is set.
            ///
            /// **Game Scenes**: Flight
            get: target_roll,
            /// Sets the target roll.
            ///
            /// **Game Scenes**: Flight
            set: set_target_roll
        }
        {
            TargetDirection: Vector3,
            /// Returns the direction vector corresponding to the target pitch and heading. This is in
            /// the reference frame specified by ReferenceFrame.
            ///
            /// **Game Scenes**: Flight
            get: target_direction,
            /// Sets the direction vector corresponding to the target pitch and heading. This is in the
            /// reference frame specified by `ReferenceFrame`.
            ///
            /// **Game Scenes**: Flight
            set:set_target_direction
        }
        {
            SAS: bool,
            /// Returns whether or not SAS is enabled.
            ///
            /// **Game Scenes**: Flight
            get: is_sas_enabled,
            /// Sets whether or not SAS is enabled.
            ///
            /// **Game Scenes**: Flight
            set: set_sas_enabled
        }
        {
            SASMode: SASMode,
            /// Returns the current SASMode. These modes are equivalent to the mode buttons to the
            /// left of the navball that appear when SAS is enabled.
            ///
            /// **Game Scenes**: Flight
            get: sas_mode,
            /// Sets the current SASMode. These modes are equivalent to the mode buttons to the
            /// left of the navball that appear when SAS is enabled.
            ///
            /// **Game Scenes**: Flight
            set: set_sas_mode
        }
        {
            RollThreshold: f32,
            /// Returns the threshold at which the autopilot will try to match the target roll angle,
            /// if any. Defaults to 5 degrees.
            ///
            /// **Game Scenes**: Flight
            get: roll_threshold,
            /// Sets the threshold at which the autopilot will try to match the target roll angle,
            /// if any. Defaults to 5 degrees.
            ///
            /// **Game Scenes**: Flight
            set: set_roll_threshold
        }
        {
            StoppingTime : Vector3,
            /// Returns the maximum amount of time that the vessel should need to come to a
            /// complete stop. This determines the maximum angular velocity of the vessel.
            ///
            /// **Game Scenes**: Flight
            ///
            /// # Return
            /// A vector of three stopping times, in seconds, one for each of the pitch, roll and
            /// yaw axes. Defaults to 0.5 seconds for each axis.
            get: stopping_time,
            /// Sets the maximum amount of time that the vessel should need to come to a
            /// complete stop. This determines the maximum angular velocity of the vessel.
            ///
            /// **Game Scenes**: Flight
            ///
            /// # Arguments
            /// *`value` - A vector of three stopping times, in seconds, one for each
            /// of the pitch, roll and yaw axes.
            set: set_stopping_time
        }
        {
            DecelerationTime: Vector3,
            /// Returns the time the vessel should take to come to a stop pointing in the
            /// target direction. This determines the angular acceleration used to decelerate
            /// the vessel.
            ///
            /// **Game Scenes**: Flight
            ///
            /// # Return
            /// A vector of three times, in seconds, one for each of the pitch, roll and yaw axes.
            /// Defaults to 5 seconds for each axis.
            get: deceleration_time,
            /// Sets the time the vessel should take to come to a stop pointing in the
            /// target direction. This determines the angular acceleration used to decelerate
            /// the vessel.
            ///
            /// **Game Scenes**: Flight
            ///
            /// # Arguments
            /// *`value` - A vector of three times, in seconds, one for each of the
            /// pitch, roll and yaw axes.
            set: set_deceleration_time
        }
        {
            AttenuationAngle : Vector3,
            /// Returns the angle at which the autopilot considers the vessel to be pointing
            /// close to the target. This determines the midpoint of the target velocity
            /// attenuation function.
            ///
            /// **Game Scenes**: Flight
            ///
            /// # Return
            /// A vector of three angles, in degrees, one for each of the pitch, roll and yaw
            /// axes. Defaults to 1° for each axis.
            get: attenuation_angle,
            /// Sets the angle at which the autopilot considers the vessel to be pointing close
            /// to the target. This determines the midpoint of the target velocity
            /// attenuation function.
            ///
            /// **Game Scenes**: Flight
            ///
            /// # Arguments
            /// *`value` - A vector of three angles, in degrees, one for each of the
            /// pitch, roll and yaw axes.
            set: set_attenuation_angle
        }
        {
            AutoTune: bool,
            /// Returns whether the rotation rate controllers PID parameters should be
            /// automatically tuned using the vessels moment of inertia and available torque.
            /// Defaults to true
            ///
            /// **Game Scenes**: Flight
            get: is_auto_tune,
            /// Sets whether the rotation rate controllers PID parameters should be
            /// automatically tuned using the vessels moment of inertia and available torque.
            ///
            /// **Game Scenes**: Flight
            set: set_auto_tune
        }
        {
            TimeToPeak: Vector3,
            /// Returns the target time to peak used to autotune the PID controllers.
            ///
            /// **Game Scenes**: Flight
            ///
            /// # Return
            /// A vector of three times, in seconds, for each of the pitch, roll and yaw axes.
            /// Defaults to 3 seconds for each axis.
            get: time_to_peak,
            /// Sets the target time to peak used to autotune the PID controllers.
            ///
            /// **Game Scenes**: Flight
            ///
            /// # Arguments
            /// *`value` - A vector of three times, in seconds, for each of the
            /// pitch, roll and yaw axes.
            set: set_time_to_peak
        }
        {
            Overshoot: Vector3,
            /// Returns the target overshoot percentage used to autotune the PID controllers.
            ///
            /// **Game Scenes**: Flight
            ///
            /// # Return
            /// A vector of three values, between 0 and 1, for each of the pitch, roll and yaw
            /// axes. Defaults to 0.01 for each axis.
            get: overshoot,
            /// Sets the target overshoot percentage used to autotune the PID controllers.
            ///
            /// **Game Scenes**: Flight
            ///
            /// # Arguments
            /// *`value` - A vector of three values, between 0 and 1, for each of
            /// the pitch, roll and yaw axes.
            set: set_overshoot
        }
        {
            PitchPIDGains: Vector3,
            /// Returns the gains for the pitch PID controller.
            ///
            /// **Game Scenes**: Flight
            get: pitch_pid_gains,
            /// Sets the gains for the pitch PID controller.
            ///
            /// **Game Scenes**: Flight
            ///
            /// # Note
            /// When `AutoPilot::is_auto_tune()` is true, these values are updated automatically,
            /// which will overwrite any manual changes.
            set: set_pitch_pid_gains
        }
        {
            RollPIDGains: Vector3,
            /// Returns the gains for the roll PID controller.
            ///
            /// **Game Scenes**: Flight
            get: roll_pid_gains,
            /// Sets the gains for the roll PID controller.
            ///
            /// **Game Scenes**: Flight
            ///
            /// # Note
            /// When `AutoPilot::is_auto_tune()` is true, these values are updated automatically,
            /// which will overwrite any manual changes.
            set: set_roll_pid_gains
        }
        {
            YawPIDGains: Vector3,
            /// Returns the gains for the yaw PID controller.
            ///
            /// **Game Scenes**: Flight
            get: yaw_pid_gains,
            /// Sets the gains for the yaw PID controller.
            ///
            /// **Game Scenes**: Flight
            ///
            /// # Note
            /// When `AutoPilot::is_auto_tune()` is true, these values are updated automatically,
            /// which will overwrite any manual changes.
            set: set_yaw_pid_gains
        }
    }
    methods: {
        {
            /// Engage the auto-pilot.
            ///
            /// **Game Scenes**: Flight
            fn engage() {
                Engage()
            }
        }
        {
            /// Disengage the auto-pilot.
            ///
            /// **Game Scenes**: Flight
            fn disengage() {
                Disengage()
            }
        }
        {
            /// Blocks until the vessel is pointing in the target direction and has the target roll
            /// (if set). Returns an error if the auto-pilot has not been engaged.
            ///
            /// **Game Scenes**: Flight
            fn wait() {
                Wait()
            }
        }
        {
            /// Set target pitch and heading angles.
            ///
            /// **Game Scenes**: Flight
            ///
            /// # Arguments
            /// * `pitch` - Target pitch angle, in degrees between -90° and +90°.
            /// * `heading` - Target heading angle, in degrees between 0° and 360°.
            fn target_pitch_and_heading(pitch: f32, heading: f32) {
                TargetPitchAndHeading(pitch, heading)
            }
        }
    }
});
