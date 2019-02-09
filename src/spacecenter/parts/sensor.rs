use super::Part;
use crate::codec::{Decode, Encode};
use crate::{remote_type, RemoteObject};

remote_type!(
/// A sensor, such as a thermometer. Obtained by calling `Part::sensor().`
object SpaceCenter.Sensor {
    properties: {
        {
            Part {
                /// Returns the part object for this sensor.
                ///
                /// **Game Scenes**: All
                get: part -> Part
            }
        }
        {
            Active {
                /// Returns whether the sensor is active.
                ///
                /// **Game Scenes**: All
                get: is_active -> bool,
                /// Sets whether the sensor is active.
                ///
                /// **Game Scenes**: All
                set: set_active(bool)
            }
        }
        {
            Value {
                /// Returns the current value of the sensor.
                ///
                /// **Game Scenes**: All
                get: value -> String
            }
        }
    }
});
