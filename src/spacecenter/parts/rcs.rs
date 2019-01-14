use crate::*;
use crate::codec::*;
use super::{Part};

use std::rc::Rc;

remote_type!(
/// An rcs block or thruster. Obtained by calling `Part::rcs().`
object SpaceCenter.RCS {
    properties: {
        {
            Part: Part,
            /// The part object for this rcs block or thruster.
            get: part
        }
    }
});