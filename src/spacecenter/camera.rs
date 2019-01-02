use crate::*;
use crate::codec::*;

use std::rc::{Rc};
use std::cell::{RefCell};

remote_type!(
/// Controls the game’s camera. Obtained by calling `SpaceCenter::camera()`.
object Camera {}
);