use crate::*;
use crate::codec::*;

use std::rc::{Rc};
use std::cell::{RefCell};

remote_type!(
/// Provides basic auto-piloting utilities for a vessel. Created by calling `Vessel.getAutoPilot()`.
///
/// # Note
/// If a client engages the auto-pilot and then closes its connection to the server, the
/// auto-pilot will be disengaged and its target reference frame, direction and roll
/// reset to default.
object AutoPilot {}
);