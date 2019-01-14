use super::Part;
use crate::codec::*;
use crate::*;

use std::rc::Rc;

remote_type!(
/// An reaction wheel. Obtained by calling `Part::reaction_wheel().`
object SpaceCenter.ReactionWheel {
    properties: {
        {
            Part: Part,
            /// The part object for this reaction wheel.
            get: part
        }
    }
});
