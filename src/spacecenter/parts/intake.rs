use super::Part;
use crate::codec::*;
use crate::*;

use std::rc::Rc;

remote_type!(
/// An intake. Obtained by calling `Part::intake().`
object SpaceCenter.Intake {
    properties: {
        {
            Part: Part,
            /// The part object for this intake
            get: part
        }
    }
});
