use super::Part;
use crate::codec::*;
use crate::*;

use std::rc::Rc;

remote_type!(
/// A sensor, such as a thermometer. Obtained by calling `Part::sensor().`
object SpaceCenter.Sensor {
    properties: {
        {
            Part: Part,
            /// The part object for this sensor.
            get: part
        }
    }
});
