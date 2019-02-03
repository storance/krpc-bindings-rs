use crate::codec::{Decode, Encode};
use crate::spacecenter::Part;
use crate::{remote_type, RemoteObject};

remote_type!(
/// A group of servos.
object InfernalRobotics.ServoGroup {
    properties: {
        {
            Name: String,
            /// Returns the name of the group.
            ///
            /// **Game Scenes**: Flight
            get: name,
            /// Sets the name of the group.
            ///
            /// **Game Scenes**: Flight
            set: set_name
        }
        {
            ForwardKey: String,
            /// Returns the key assigned to be the “forward” key for the group.
            ///
            /// **Game Scenes**: Flight
            get: forward_key,
            /// Sets the key assigned to be the “forward” key for the group.
            ///
            /// **Game Scenes**: Flight
            set: set_forward_key
        }
        {
            ReverseKey: String,
            /// Returns the key assigned to be the “reverse” key for the group.
            ///
            /// **Game Scenes**: Flight
            get: reverse_key,
            /// Sets the key assigned to be the “reverse” key for the group.
            ///
            /// **Game Scenes**: Flight
            set: set_reverse_key
        }
        {
            Speed: f32,
            /// Returns the speed multiplier for the group.
            ///
            /// **Game Scenes**: Flight
            get: speed,
            /// Sets the speed multiplier for the group.
            ///
            /// **Game Scenes**: Flight
            set: set_speed
        }
        {
            Expanded: bool,
            /// Returns whether the group is expanded in the InfernalRobotics UI.
            ///
            /// **Game Scenes**: Flight
            get: is_expanded,
            /// Sets whether the group is expanded in the InfernalRobotics UI.
            ///
            /// **Game Scenes**: Flight
            set: set_expanded
        }
        {
            Part: Vec<Part>,
            /// Returns the parts containing the servos in the group.
            ///
            /// **Game Scenes**: Flight
            get: parts
        }
    }
    methods: {
        {
            /// Moves all of the servos in the group to the right.
            ///
            /// **Game Scenes**: Flight
            fn move_right() {
                MoveRight()
            }
        }
        {
            /// Moves all of the servos in the group to the left.
            ///
            /// **Game Scenes**: Flight
            fn move_left() {
                MoveLeft()
            }
        }
        {
            /// Moves all of the servos in the group to the center.
            ///
            /// **Game Scenes**: Flight
            fn move_center() {
                MoveCenter()
            }
        }
        {
            /// Moves all of the servos in the group to the next preset.
            ///
            /// **Game Scenes**: Flight
            fn move_next_preset() {
                MoveNextPreset()
            }
        }
        {
            /// Moves all of the servos in the group to the previous preset.
            ///
            /// **Game Scenes**: Flight
            fn move_prev_preset() {
                MovePrevPreset()
            }
        }
        {
            /// Stops the servos in the group.
            ///
            /// **Game Scenes**: Flight
            fn stop() {
                Stop()
            }
        }
    }
});
