use crate::*;
use crate::codec::*;
use super::{Part};

use std::rc::{Rc};
use std::cell::{RefCell};

remote_type!(
/// An engine. Obtained by calling `Part::engine().`
object Engine {});

impl Engine {
    rpc_property!(
        Part: Part {
            service: SpaceCenter,
            class: Engine,
            /// The part object for this engine.
            part
        }
    );
}

remote_type!(
/// A propellant for an engine. Obtained by calling `Engine::propellants()`.
object Propellant {});