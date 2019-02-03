use crate::codec::{Decode, Encode};
use crate::spacecenter::Part;
use crate::{remote_type, RemoteObject};

remote_type!(
/// Represents a servo.
object InfernalRobotics.Servo {
    properties: {
        {
            Name: String,
            /// Returns the name of the servo.
            ///
            /// **Game Scenes**: Flight
            get: name,
            /// Sets the name of the servo.
            ///
            /// **Game Scenes**: Flight
            set: set_name
        }
        {
            Part: Part,
            /// Returns the part containing the servo.
            ///
            /// **Game Scenes**: Flight
            get: part
        }
        {
            Position: f32,
            /// Returns the position of the servo.
            ///
            /// **Game Scenes**: Flight
            get: position
        }
        {
            MinConfigPosition: f32,
            /// Returns the minimum position of the servo, specified by the part configuration.
            ///
            /// **Game Scenes**: Flight
            get: min_config_position
        }
        {
            MaxConfigPosition: f32,
            /// Returns the maximum position of the servo, specified by the part configuration.
            ///
            /// **Game Scenes**: Flight
            get: max_config_position
        }
        {
            MinPosition: f32,
            /// Returns the minimum position of the servo, specified by the in-game tweak menu.
            ///
            /// **Game Scenes**: Flight
            get: min_position,
            /// Sets the minimum position of the servo, specified by the in-game tweak menu.
            ///
            /// **Game Scenes**: Flight
            set: set_min_position
        }
        {
            MaxPosition: f32,
            /// Returns the maximum position of the servo, specified by the in-game tweak menu.
            ///
            /// **Game Scenes**: Flight
            get: max_position,
            /// Sets the maximum position of the servo, specified by the in-game tweak menu.
            ///
            /// **Game Scenes**: Flight
            set: set_max_position
        }
        {
            ConfigSpeed: f32,
            /// Returns the speed multiplier of the servo, specified by the part configuration.
            ///
            /// **Game Scenes**: Flight
            get: config_speed
        }
        {
            Speed: f32,
            /// Returns the speed multiplier of the servo, specified by the in-game tweak menu.
            ///
            /// **Game Scenes**: Flight
            get: speed,
            /// Sets the speed multiplier of the servo, specified by the in-game tweak menu.
            ///
            /// **Game Scenes**: Flight
            set: set_speed
        }
        {
            CurrentSpeed: f32,
            /// Returns the current speed at which the servo is moving.
            ///
            /// **Game Scenes**: Flight
            get: current_speed,
            /// Sets the current speed at which the servo is moving.
            ///
            /// **Game Scenes**: Flight
            set: set_current_speed
        }
        {
            Acceleration: f32,
            /// Returns the acceleration of the servo.
            ///
            /// **Game Scenes**: Flight
            get: acceleration,
            /// Sets the acceleration of the servo.
            ///
            /// **Game Scenes**: Flight
            set: set_acceleration
        }
        {
            IsMoving: bool,
            /// Returns whether the servo is moving.
            ///
            /// **Game Scenes**: Flight
            get: is_moving
        }
        {
            IsFreeMoving: bool,
            /// Returns whether the servo is freely moving.
            ///
            /// **Game Scenes**: Flight
            get: is_free_moving
        }
        {
            IsLocked: bool,
            /// Returns whether the servo is locked.
            ///
            /// **Game Scenes**: Flight
            get: is_locked,
            /// Sets whether the servo is locked.
            ///
            /// **Game Scenes**: Flight
            set: set_locked
        }
        {
            IsAxisInverted: bool,
            /// Returns whether the servos axis is inverted.
            ///
            /// **Game Scenes**: Flight
            get: is_axis_inverted,
            /// Sets whether the servos axis is inverted.
            ///
            /// **Game Scenes**: Flight
            set: set_axis_inverted
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
