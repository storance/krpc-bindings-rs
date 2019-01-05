use crate::*;
use crate::codec::*;
use super::{Part};

use std::rc::{Rc};
use std::cell::{RefCell};

remote_type!(
/// An rcs block or thruster. Obtained by calling `Part::rcs().`
object RCS {
    service: SpaceCenter,
    properties: {
        {
            Part: Part,
            /// The part object for this rcs block or thruster.
            get: part
        }
    }
});