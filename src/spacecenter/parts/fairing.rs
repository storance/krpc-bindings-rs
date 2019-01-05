use crate::*;
use crate::codec::*;
use super::{Part};

use std::rc::{Rc};
use std::cell::{RefCell};

remote_type!(
/// A fairing. Obtained by calling `Part::fairing().`
object Fairing {
    service: SpaceCenter,
    properties: {
        {
            Part: Part,
            /// The part object for this fairing.
            get: part
        }
    }
});
