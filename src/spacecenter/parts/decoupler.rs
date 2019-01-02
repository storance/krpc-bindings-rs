use crate::*;
use crate::codec::*;
use super::{Part};

use std::rc::{Rc};
use std::cell::{RefCell};

remote_type!(
/// A decoupler. Obtained by calling `Part::decoupler().`
object Decoupler {});

impl Decoupler {
    rpc_property!(
        Part: Part {
            service: SpaceCenter,
            class: Decoupler,
            /// The part object for this decoupler.
            part
        }
    );
}