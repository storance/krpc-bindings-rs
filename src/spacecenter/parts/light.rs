use crate::*;
use crate::codec::*;
use super::{Part};

use std::rc::{Rc};
use std::cell::{RefCell};

remote_type!(
/// A light. Obtained by calling `Part::light().`
object Light {
    service: SpaceCenter,
    properties: {
        {
            Part: Part,
            /// The part object for this light.
            get: part
        }
    }
});
