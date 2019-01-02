use crate::*;
use crate::codec::*;
use super::{Part};

use std::rc::{Rc};
use std::cell::{RefCell};

remote_type!(
/// A light. Obtained by calling `Part::light().`
object Light {});

impl Light {
    rpc_property!(
        Part: Part {
            service: SpaceCenter,
            class: Light,
            /// The part object for this light.
            part
        }
    );
}