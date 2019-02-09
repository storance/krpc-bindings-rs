use crate::codec::{Decode, Encode};
use crate::spacecenter::Part;
use crate::{remote_type, RemoteObject};

remote_type!(
/// Represents a servo.
object InfernalRobotics.Servo {
    properties: {
        {
            Name {
                /// Returns the name of the servo.
                ///
                /// **Game Scenes**: Flight
                get: name -> String,
                /// Sets the name of the servo.
                ///
                /// **Game Scenes**: Flight
                set: set_name(&str)
            }
        }
        {
            Part {
                /// Returns the part containing the servo.
                ///
                /// **Game Scenes**: Flight
                get: part -> Part
            }
        }
        {
            Position {
                /// Returns the position of the servo.
                ///
                /// **Game Scenes**: Flight
                get: position -> f32
            }
        }
        {
            MinConfigPosition {
                /// Returns the minimum position of the servo, specified by the part configuration.
                ///
                /// **Game Scenes**: Flight
                get: min_config_position -> f32
            }
        }
        {
            MaxConfigPosition {
                /// Returns the maximum position of the servo, specified by the part configuration.
                ///
                /// **Game Scenes**: Flight
                get: max_config_position -> f32
            }
        }
        {
            MinPosition {
                /// Returns the minimum position of the servo, specified by the in-game tweak menu.
                ///
                /// **Game Scenes**: Flight
                get: min_position -> f32,
                /// Sets the minimum position of the servo, specified by the in-game tweak menu.
                ///
                /// **Game Scenes**: Flight
                set: set_min_position(f32)
            }
        }
        {
            MaxPosition {
                /// Returns the maximum position of the servo, specified by the in-game tweak menu.
                ///
                /// **Game Scenes**: Flight
                get: max_position -> f32,
                /// Sets the maximum position of the servo, specified by the in-game tweak menu.
                ///
                /// **Game Scenes**: Flight
                set: set_max_position(f32)
            }
        }
        {
            ConfigSpeed {
                /// Returns the speed multiplier of the servo, specified by the part configuration.
                ///
                /// **Game Scenes**: Flight
                get: config_speed -> f32
            }
        }
        {
            Speed {
                /// Returns the speed multiplier of the servo, specified by the in-game tweak menu.
                ///
                /// **Game Scenes**: Flight
                get: speed -> f32,
                /// Sets the speed multiplier of the servo, specified by the in-game tweak menu.
                ///
                /// **Game Scenes**: Flight
                set: set_speed(f32)
            }
        }
        {
            CurrentSpeed {
                /// Returns the current speed at which the servo is moving.
                ///
                /// **Game Scenes**: Flight
                get: current_speed -> f32,
                /// Sets the current speed at which the servo is moving.
                ///
                /// **Game Scenes**: Flight
                set: set_current_speed(f32)
            }
        }
        {
            Acceleration {
                /// Returns the acceleration of the servo.
                ///
                /// **Game Scenes**: Flight
                get: acceleration -> f32,
                /// Sets the acceleration of the servo.
                ///
                /// **Game Scenes**: Flight
                set: set_acceleration(f32)
            }
        }
        {
            IsMoving {
                /// Returns whether the servo is moving.
                ///
                /// **Game Scenes**: Flight
                get: is_moving -> bool
            }
        }
        {
            IsFreeMoving {
                /// Returns whether the servo is freely moving.
                ///
                /// **Game Scenes**: Flight
                get: is_free_moving -> bool
            }
        }
        {
            IsLocked {
                /// Returns whether the servo is locked.
                ///
                /// **Game Scenes**: Flight
                get: is_locked -> bool,
                /// Sets whether the servo is locked.
                ///
                /// **Game Scenes**: Flight
                set: set_locked(bool)
            }
        }
        {
            IsAxisInverted {
                /// Returns whether the servos axis is inverted.
                ///
                /// **Game Scenes**: Flight
                get: is_axis_inverted -> bool,
                /// Sets whether the servos axis is inverted.
                ///
                /// **Game Scenes**: Flight
                set: set_axis_inverted(bool)
            }
        }
    }
    methods: {
        {
            /// Moves the servo to the right.
            ///
            /// **Game Scenes**: Flight
            fn move_right() {
                MoveRight()
            }
        }
        {
            /// Moves the servo to the left.
            ///
            /// **Game Scenes**: Flight
            fn move_left() {
                MoveLeft()
            }
        }
        {
            /// Moves the servo to the center.
            ///
            /// **Game Scenes**: Flight
            fn move_center() {
                MoveCenter()
            }
        }
        {
            /// Moves the servo to the next preset.
            ///
            /// **Game Scenes**: Flight
            fn move_next_preset() {
                MoveNextPreset()
            }
        }
        {
            /// Moves the servo to the previous preset.
            ///
            /// **Game Scenes**: Flight
            fn move_prev_preset() {
                MovePrevPreset()
            }
        }
        {
            /// Moves the servo to position and sets the speed multiplier to speed.
            ///
            /// **Game Scenes**: Flight
            ///
            /// # Arguments
            /// * `position` - The position to move the servo to.
            /// * `speed` - Speed multiplier for the movement.
            fn move_to(position: f32, speed: f32) {
                MoveTo(position, speed)
            }
        }
        {
            /// Stops the servo.
            ///
            /// **Game Scenes**: Flight
            fn stop() {
                Stop()
            }
        }
        {
            /// Sets whether the servo should be highlighted in-game.
            ///
            /// **Game Scenes**: Flight
            fn set_highlight(highlight: bool) {
                set_Highlight(highlight)
            }
        }
    }
});
