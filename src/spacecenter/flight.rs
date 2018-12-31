use crate::*;
use crate::codec::*;

use std::rc::{Rc};
use std::cell::{RefCell};

remote_type!(
/// Used to get flight telemetry for a vessel, by calling `Vessel::flight(ReferenceFrame)`. All of
/// the information returned by this class is given in the reference frame passed to that method.
/// Obtained by calling `Vessel::flight(ReferenceFrame)`.
object Flight {}
);