use crate::*;
use crate::codec::*;
use super::spacecenter::{Vessel};

use std::rc::{Rc};
use std::cell::{RefCell};

remote_type!(
/// Used to interact with CommNet for a given vessel. Obtained by calling `Vessel::comms()`.
object Comms {}
);

impl Comms {
    rpc_property!(
        CanCommunicate: bool {
            service: SpaceCenter,
            class: Comms,
            /// Returns whether the vessel can communicate with KSC.
            has_uplink
        }
    );

    rpc_property!(
        CanTransmitScience: bool {
            service: SpaceCenter,
            class: Comms,
            /// Returns whether the vessel can transmit science data to KSC.
            is_science_transmittable
        }
    );

    rpc_property!(
        SignalStrength: f64 {
            service: SpaceCenter,
            class: Comms,
            /// Returns the signal strength to KSC.
            signal_strength
        }
    );

    rpc_property!(
        SignalDelay: f64 {
            service: SpaceCenter,
            class: Comms,
            /// Returns the signal delay to KSC in seconds.
            signal_delay
        }
    );

    rpc_property!(
        Power: f64 {
            service: SpaceCenter,
            class: Comms,
            /// Returns the combined power of all active antennae on the vessel.
            power
        }
    );

    rpc_property!(
        ControlPath: Vec<CommLink> {
            service: SpaceCenter,
            class: Comms,
            /// Returns the combined power of all active antennae on the vessel.
            control_path
        }
    );
}

remote_type!(
/// Represents a communication node in the network. For example, a vessel or the KSC.
object CommLink {}
);

impl CommLink {
    rpc_property!(
        Type: CommLinkType {
            service: SpaceCenter,
            class: CommLink,
            /// Returns the type of link
            link_type
        }
    );

    rpc_property!(
        SignalStrength: f64 {
            service: SpaceCenter,
            class: CommLink,
            /// Returns the signal strength of the link.
            signal_strength
        }
    );

    rpc_property!(
        Start: CommNode {
            service: SpaceCenter,
            class: CommLink,
            /// Returns the start point of the link.
            start
        }
    );

    rpc_property!(
        End: CommNode {
            service: SpaceCenter,
            class: CommLink,
            /// Returns the end point of the link.
            end
        }
    );
}

remote_type!(
/// Represents a communication node in the network. For example, a vessel or the KSC.
object CommNode {}
);

impl CommNode {
    rpc_property!(
        Name: String {
            service: SpaceCenter,
            class: CommNode,
            /// Returns the name of the communication node.
            name
        }
    );

    rpc_property!(
        IsHome: bool {
            service: SpaceCenter,
            class: CommNode,
            /// Returns whether the communication node is on Kerbin.
            is_home
        }
    );

    rpc_property!(
        IsControlPoint: bool {
            service: SpaceCenter,
            class: CommNode,
            /// Returns whether the communication node is a control point, for example
            /// a manned vessel.
            is_control_point
        }
    );

    rpc_property!(
        IsVessel: bool {
            service: SpaceCenter,
            class: CommNode,
            /// Returns whether the communication node is a vessel.
            is_vessel
        }
    );

    rpc_property!(
        Vessel: Option<Vessel> {
            service: SpaceCenter,
            class: CommNode,
            /// Returns the vessel for this communication node.
            vessel
        }
    );
}


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