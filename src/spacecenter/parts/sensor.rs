use crate::*;
use crate::codec::*;
use super::{Part};

use std::rc::{Rc};
use std::cell::{RefCell};

remote_type!(
/// A sensor, such as a thermometer. Obtained by calling `Part::sensor().`
object Sensor {
    service: SpaceCenter,
    properties: {
        {
            Part: Part,
            /// The part object for this sensor.
            get: part
        }
    }
});
