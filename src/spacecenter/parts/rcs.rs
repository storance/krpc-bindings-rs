use crate::*;
use crate::codec::*;
use super::{Part};

use std::rc::{Rc};
use std::cell::{RefCell};

remote_type!(
/// An rcs block or thruster. Obtained by calling `Part::rcs().`
object RCS {});

impl RCS {
    rpc_property!(
        Part: Part {
            service: SpaceCenter,
            class: RCS,
            /// The part object for this rcs block or thruster.
            part
        }
    );
}