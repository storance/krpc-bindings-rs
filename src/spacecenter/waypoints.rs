use crate::*;
use crate::codec::*;

use std::rc::{Rc};
use std::cell::{RefCell};

remote_type!(
/// Waypoints are the location markers you can see on the map view showing you where contracts
/// are targeted for. With this structure, you can obtain coordinate data for the locations of
/// these waypoints. Obtained by calling `SpaceCenter::waypoint_manager()`.
object WaypointManager {}
);

remote_type!(
/// Represents a waypoint. Can be created using `WaypointManager::add_waypoint()`.
object Waypoint {}
);