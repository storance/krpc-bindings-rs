use crate::*;
use crate::codec::*;
use super::{Part};

use std::rc::Rc;

remote_type!(
/// A decoupler. Obtained by calling `Part::decoupler().`
object SpaceCenter.Decoupler {
    properties: {
        {
            Part: Part,
            /// The part object for this decoupler.
            get: part
        }
    }
});
