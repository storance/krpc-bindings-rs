use crate::codec::{Decode, Encode};
use crate::remote_type;
use crate::spacecenter::Vessel;

mod servo;
mod servogroup;

pub use self::servo::*;
pub use self::servogroup::*;

remote_type!(
/// This service provides functionality to interact with Infernal Robotics.
service InfernalRobotics {
    properties: {
        {
            Available {
                /// Returns whether Infernal Robotics is installed.
                ///
                /// **Game Scenes**: Flight
                get: is_available -> bool
            }
        }
        {
            Ready {
                /// Returns whether the Infernal Robotics API is ready.
                ///
                /// **Game Scenes**: Flight
                get: is_ready -> bool
            }
        }
    }
    methods: {
        {
            /// A list of all the servo groups in the given vessel.
            ///
            /// **Game Scenes**: Flight
            ///
            /// # Arguments
            /// * `vessel` - Vessel to check.
            fn servo_groups(vessel: &Vessel) -> Vec<ServoGroup> {
                ServoGroups(vessel)
            }
        }
        {
            /// Returns the servo group in the given vessel with the given name,
            /// or `None` if none exists. If multiple servo groups have the same name,
            /// only one of them is returned.
            ///
            /// **Game Scenes**: Flight
            ///
            /// # Arguments
            /// * `vessel` - Vessel to check.
            /// * `name` - Name of servo group to find.
            fn servo_group_with_name(vessel: &Vessel, name: &str) -> Option<ServoGroup> {
                ServoGroupWithName(vessel, name)
            }
        }
        {
            /// Returns the servo in the given vessel with the given name,
            /// or `None` if none exists. If multiple servos have the same name,
            /// only one of them is returned.
            ///
            /// **Game Scenes**: Flight
            ///
            /// # Arguments
            /// * `vessel` - Vessel to check.
            /// * `name` - Name of servo to find.
            fn servo_with_name(vessel: &Vessel, name: &str) -> Option<Servo> {
                ServoWithName(vessel, name)
            }
        }
    }
});
