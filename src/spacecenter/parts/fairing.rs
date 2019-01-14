use crate::*;
use crate::codec::*;
use super::{Part};

use std::rc::Rc;

remote_type!(
/// A fairing. Obtained by calling `Part::fairing().`
object SpaceCenter.Fairing {
    properties: {
        {
            Part: Part,
            /// The part object for this fairing.
            get: part
        }
    }
});
