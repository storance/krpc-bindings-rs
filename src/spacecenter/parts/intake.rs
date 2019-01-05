use crate::*;
use crate::codec::*;
use super::{Part};

use std::rc::{Rc};
use std::cell::{RefCell};

remote_type!(
/// An intake. Obtained by calling `Part::intake().`
object Intake {
    service: SpaceCenter,
    properties: {
        {
            Part: Part,
            /// The part object for this intake
            get: part
        }
    }
});
