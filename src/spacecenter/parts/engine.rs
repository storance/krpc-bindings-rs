use crate::*;
use crate::codec::*;
use super::{Part};

use std::rc::{Rc};
use std::cell::{RefCell};

remote_type!(
/// An engine. Obtained by calling `Part::engine().`
object Engine {
    service: SpaceCenter,
    properties: {
        {
            Part: Part,
            /// The part object for this engine.
            get: part
        }
    }
});

remote_type!(
/// A propellant for an engine. Obtained by calling `Engine::propellants()`.
object Propellant {});