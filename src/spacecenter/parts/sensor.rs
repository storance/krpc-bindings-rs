use crate::*;
use crate::codec::*;
use super::{Part};

use std::rc::{Rc};
use std::cell::{RefCell};

remote_type!(
/// A sensor, such as a thermometer. Obtained by calling `Part::sensor().`
object Sensor {});

impl Sensor {
    rpc_property!(
        Part: Part {
            service: SpaceCenter,
            class: Sensor,
            /// The part object for this sensor.
            part
        }
    );
}