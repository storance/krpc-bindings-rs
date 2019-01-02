use crate::*;
use crate::codec::*;

use std::rc::{Rc};
use std::cell::{RefCell};

remote_type!(
/// Used to interact with CommNet for a given vessel. Obtained by calling `Vessel::comms()`.
object Comms {}
);

remote_type!(
/// Represents a communication node in the network. For example, a vessel or the KSC.
object CommNode {}
);

remote_type!(
    /// The type of communication link.
    enum CommLinkType {
        /// Link is to a base station on Kerbin.
        Home => 0,
        /// Link is to a control source, for example a manned spacecraft.
        Control => 1,
        /// Link is to a relay satellite.
        Relay => 2
    }
);