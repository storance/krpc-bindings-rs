use super::Part;
use crate::codec::*;
use crate::*;
use crate::krpc::Expression;

remote_type!(
/// A sensor, such as a thermometer. Obtained by calling `Part::sensor().`
object SpaceCenter.Sensor {
    properties: {
        {
            Part: Part,
            /// Returns the part object for this sensor.
            ///
            /// **Game Scenes**: All
            get: part
        }
        {
            Active: bool,
            /// Returns whether the sensor is active.
            ///
            /// **Game Scenes**: All
            get: is_active,
            /// Sets whether the sensor is active.
            ///
            /// **Game Scenes**: All
            set: set_active
        }
        {
            Value: String,
            /// Returns the current value of the sensor.
            ///
            /// **Game Scenes**: All
            get: value
        }
    }
});
