use super::Part;
use crate::codec::*;
use crate::*;

use std::rc::Rc;

remote_type!(
/// An engine. Obtained by calling `Part::engine().`
object SpaceCenter.Engine {
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
object SpaceCenter.Propellant {});
