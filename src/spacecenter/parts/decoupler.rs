use crate::*;
use crate::codec::*;
use super::{Part};

use std::rc::{Rc};
use std::cell::{RefCell};

remote_type!(
/// A decoupler. Obtained by calling `Part::decoupler().`
object Decoupler {
    service: SpaceCenter,
    properties: {
        {
            Part: Part,
            /// The part object for this decoupler.
            get: part
        }
    }
});
