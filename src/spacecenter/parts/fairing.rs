use crate::*;
use crate::codec::*;
use super::{Part};

use std::rc::{Rc};
use std::cell::{RefCell};

remote_type!(
/// A fairing. Obtained by calling `Part::fairing().`
object Fairing {});

impl Fairing {
    rpc_property!(
        Part: Part {
            service: SpaceCenter,
            class: Fairing,
            /// The part object for this fairing.
            part
        }
    );
}