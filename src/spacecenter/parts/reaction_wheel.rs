use crate::*;
use crate::codec::*;
use super::{Part};

use std::rc::{Rc};
use std::cell::{RefCell};

remote_type!(
/// An reaction wheel. Obtained by calling `Part::reaction_wheel().`
object ReactionWheel {
    service: SpaceCenter,
    properties: {
        {
            Part: Part,
            /// The part object for this reaction wheel.
            get: part
        }
    }
});
