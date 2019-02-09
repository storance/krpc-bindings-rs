use super::Part;
use crate::codec::{Decode, Encode};
use crate::{remote_type, RemoteEnum, RemoteObject};

remote_type!(
/// A wheel. Includes landing gear and rover wheels. Obtained by calling `Part::wheel()`. Can be
/// used to control the motors, steering and deployment of wheels, among other things.
object SpaceCenter.Wheel {
    properties: {
        {
            Part {
                /// Returns the part object for this wheel.
                ///
                /// **Game Scenes**: All
                get: part -> Part
            }
        }
        {
            State {
                /// Returns the current state of the wheel.
                ///
                /// **Game Scenes**: All
                get: state -> WheelState
            }
        }
        {
            Radius {
                /// Returns the radius of the wheel, in meters.
                ///
                /// **Game Scenes**: All
                get: radius -> f32
            }
        }
        {
            Grounded {
                /// Returns whether the wheel is touching the ground.
                ///
                /// **Game Scenes**: All
                get: is_grounded -> bool
            }
        }
        {
            HasBrakes {
                /// Returns whether the wheel has brakes.
                ///
                /// **Game Scenes**: All
                get: has_brakes -> bool
            }
        }
        {
            Brakes {
                /// Returns the braking force, as a percentage of maximum, when the brakes are applied.
                ///
                /// **Game Scenes**: All
                get: braking_force -> f32,
                /// Sets the braking force, as a percentage of maximum, when the brakes are applied.
                ///
                /// **Game Scenes**: All
                set: set_braking_force(f32)
            }
        }
        {
            AutoFrictionControl {
                /// Returns whether automatic friction control is enabled.
                ///
                /// **Game Scenes**: All
                get: is_auto_friction_control_enabled -> bool,
                /// Sets whether automatic friction control is enabled.
                ///
                /// **Game Scenes**: All
                set: set_auto_friction_control_enabled(bool)
            }
        }
        {
            ManualFrictionControl {
                /// Returns the manual friction control value. A value between 0 and 5 inclusive.
                ///
                /// **Game Scenes**: All
                get: manual_friction_control -> f32,
                /// Sets the manual friction control value. Only has an effect if automatic friction
                /// control is disabled. A value between 0 and 5 inclusive.
                ///
                /// **Game Scenes**: All
                set: set_manual_friction_control(f32)
            }
        }
        {
            Deployable {
                /// Returns whether the wheel is deployable.
                ///
                /// **Game Scenes**: All
                get: is_deployable -> bool
            }
        }
        {
            Deployed {
                /// Returns whether the wheel is deployed.
                ///
                /// **Game Scenes**: All
                get: is_deployed -> bool,
                /// Sets whether the wheel is deployed.
                ///
                /// **Game Scenes**: All
                set: set_deployed(bool)
            }
        }
        {
            MotorEnabled {
                /// Returns whether the motor is enabled.
                ///
                /// **Game Scenes**: All
                get: is_motor_enabled -> bool,
                /// Sets whether the motor is enabled.
                ///
                /// **Game Scenes**: All
                set: set_motor_enabled(bool)
            }
        }
        {
            MotorInverted {
                /// Returns whether the motor is inverted.
                ///
                /// **Game Scenes**: All
                get: is_motor_inverted -> bool,
                /// Sets whether the motor is inverted.
                ///
                /// **Game Scenes**: All
                set: set_motor_inverted(bool)
            }
        }
        {
            MotorState {
                /// Returns the motor state.
                ///
                /// **Game Scenes**: All
                get: motor_state -> MotorState
            }
        }
        {
            MotorOutput {
                /// Returns the output of the motor. This is the torque currently being generated,
                /// in Newton meters.
                ///
                /// **Game Scenes**: All
                get: motor_output -> f32
            }
        }
        {
            TractionControlEnabled {
                /// Returns whether automatic traction control is enabled. A wheel only has traction
                /// control if it is powered.
                ///
                /// **Game Scenes**: All
                get: is_traction_control_enabled -> bool,
                /// Sets whether automatic traction control is enabled. A wheel only has traction
                /// control if it is powered.
                ///
                /// **Game Scenes**: All
                set: set_traction_control_enabled(bool)
            }
        }
        {
            TractionControl {
                /// Returns the setting for the traction control. A value between 0 and 5 inclusive.
                ///
                /// **Game Scenes**: All
                get: traction_control -> f32,
                /// Sets the setting for the traction control. Only takes effect if the wheel has
                /// automatic traction control enabled. A value between 0 and 5 inclusive.
                ///
                /// **Game Scenes**: All
                set: set_traction_control(f32)
            }
        }
        {
            DriveLimiter {
                /// Returns the manual setting for the motor limiter. A value between
                /// 0 and 100 inclusive.
                ///
                /// **Game Scenes**: All
                get: drive_limiter -> f32,
                /// Sets the manual setting for the motor limiter. Only takes effect if the
                /// wheel has automatic traction control disabled. A value between 0 and 100 inclusive.
                ///
                /// **Game Scenes**: All
                set: set_drive_limiter(f32)
            }
        }
        {
            Steerable {
                /// Returns whether the wheel has steering.
                ///
                /// **Game Scenes**: All
                get: is_steerable -> bool
            }
        }
        {
            SteeringEnabled {
                /// Returns whether the wheel steering is enabled.
                ///
                /// **Game Scenes**: All
                get: is_steering_enabled -> bool,
                /// Sets whether the wheel steering is enabled.
                ///
                /// **Game Scenes**: All
                set: set_steering_enabled(bool)
            }
        }
        {
            SteeringInverted {
                /// Returns whether the wheel steering is inverted.
                ///
                /// **Game Scenes**: All
                get: is_steering_inverted -> bool,
                /// Sets whether the wheel steering is inverted.
                ///
                /// **Game Scenes**: All
                set: set_steering_inverted(bool)
            }
        }
        {
            HasSuspension {
                /// Returns whether the wheel has suspension.
                ///
                /// **Game Scenes**: All
                get: has_suspension -> bool
            }
        }
        {
            SuspensionSpringStrength {
                /// Returns the suspension spring strength, as set in the editor.
                ///
                /// **Game Scenes**: All
                get: suspension_spring_strength -> f32
            }
        }
        {
            SuspensionDamperStrength {
                /// Returns the suspension damper strength, as set in the editor.
                ///
                /// **Game Scenes**: All
                get: suspension_damper_strength -> f32
            }
        }
        {
            Broken {
                /// Returns whether the wheel is broken.
                ///
                /// **Game Scenes**: All
                get: is_broken -> bool
            }
        }
        {
            Repairable {
                /// Returns whether the wheel is repairable.
                ///
                /// **Game Scenes**: All
                get: is_repairable -> bool
            }
        }
        {
            Stress {
                /// Returns the current stress on the wheel.
                ///
                /// **Game Scenes**: All
                get: stress -> f32
            }
        }
        {
            StressTolerance {
                /// Returns the stress tolerance of the wheel.
                ///
                /// **Game Scenes**: All
                get: stress_tolerance -> f32
            }
        }
        {
            StressPercentage {
                /// Returns the current stress on the wheel as a percentage of its stress tolerance.
                ///
                /// **Game Scenes**: All
                get: stress_percentage -> f32
            }
        }
        {
            Deflection {
                /// Returns the current deflection of the wheel.
                ///
                /// **Game Scenes**: All
                get: deflection -> f32
            }
        }
        {
            Slip {
                /// Returns the current slip of the wheel.
                ///
                /// **Game Scenes**: All
                get: slip -> f32
            }
        }
    }
});

remote_type!(
    /// The state of a wheel. See `Wheel:state`.
    enum WheelState {
        /// Wheel is fully deployed.
        Deployed = 0,
        /// Wheel is fully retracted.
        Retracted = 1,
        /// Wheel is being deployed.
        Deploying = 2,
        /// Wheel is being retracted.
        Retracting = 3,
        /// Wheel is broken.
        Broken = 4,
    }
);

remote_type!(
    /// The state of the motor on a powered wheel. See `Wheel::motor_state()`.
    enum MotorState {
        /// The motor is idle.
        Idle = 0,
        /// The motor is running.
        Running = 1,
        /// The motor is disabled.
        Disabled = 2,
        /// The motor is inoperable.
        Inoperable = 3,
        /// The motor does not have enough resources to run.
        NotEnoughResources = 4,
    }
);
