use crate::*;
use crate::codec::*;

use std::rc::{Rc};
use std::cell::{RefCell};

remote_type!(
/// Represents a maneuver node.  Can be created using `Control::add_node()`.
object Node {}
);