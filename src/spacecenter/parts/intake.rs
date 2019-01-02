use crate::*;
use crate::codec::*;
use super::{Part};

use std::rc::{Rc};
use std::cell::{RefCell};

remote_type!(
/// An intake. Obtained by calling `Part::intake().`
object Intake {});

impl Intake {
    rpc_property!(
        Part: Part {
            service: SpaceCenter,
            class: Intake,
            /// The part object for this intake.
            part
        }
    );
}