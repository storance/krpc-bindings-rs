use crate::*;
use crate::codec::*;
use super::{Part};

use std::rc::{Rc};
use std::cell::{RefCell};

remote_type!(
/// The component of an Engine or RCS part that generates thrust. Can obtained by
/// calling` Engine::thrusters()` or `RCS::thrusters()`.
///
/// # Note
/// Engines can consist of multiple thrusters. For example, the S3 KS-25x4 “Mammoth” has four
/// rocket nozzels, and so consists of four thrusters.
object Thruster {
    service: SpaceCenter,
    properties: {
        {
            Part: Part,
            /// The part object for this thruster.
            get: part
        }
    }
});
