use super::Part;
use crate::codec::*;
use crate::*;

use std::rc::Rc;

remote_type!(
/// A light. Obtained by calling `Part::light().`
object SpaceCenter.Light {
    properties: {
        {
            Part: Part,
            /// The part object for this light.
            get: part
        }
    }
});
