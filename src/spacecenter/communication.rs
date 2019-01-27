use super::spacecenter::Vessel;
use crate::codec::*;
use crate::krpc::Expression;
use crate::*;

remote_type!(
/// Used to interact with CommNet for a given vessel. Obtained by calling `Vessel::comms()`.
object SpaceCenter.Comms {
    properties: {
        {
            CanCommunicate: bool,
            /// Returns whether the vessel can communicate with KSC.
            get: has_uplink
        }
        {
            CanTransmitScience: bool,
            /// Returns whether the vessel can transmit science data to KSC.
            get: is_science_transmittable
        }
        {
            SignalStrength: f64,
            /// Returns the signal strength to KSC.
            get: signal_strength
        }
        {
            SignalDelay: f64,
            /// Returns the signal delay to KSC in seconds.
            get: signal_delay
        }
        {
            Power: f64,
            /// Returns the combined power of all active antennae on the vessel.
            get: power
        }
        {
            ControlPath: Vec<CommLink>,
            /// Returns the combined power of all active antennae on the vessel.
            get: control_path
        }
    }
});

remote_type!(
/// Represents a communication node in the network. For example, a vessel or the KSC.
object SpaceCenter.CommLink {
    properties: {
        {
            Type: CommLinkType,
            /// Returns the type of link
            get: link_type
        }
        {
            SignalStrength: f64,
            /// Returns the signal strength of the link.
            get: signal_strength
        }
        {
            Start: CommNode,
            /// Returns the start point of the link.
            get: start
        }
        {
            End: CommNode,
            /// Returns the end point of the link.
            get: end
        }
    }

});

remote_type!(
/// Represents a communication node in the network. For example, a vessel or the KSC.
object SpaceCenter.CommNode {
    properties: {
        {
            Name: String,
            /// Returns the name of the communication node.
            get: name
        }
        {
            IsHome: bool,
            /// Returns whether the communication node is on Kerbin.
            get: is_home
        }
        {
            IsControlPoint: bool,
            /// Returns whether the communication node is a control point, for example
            /// a manned vessel.
            get: is_control_point
        }
        {
            IsVessel: bool,
            /// Returns whether the communication node is a vessel.
            get: is_vessel
        }
        {
            Vessel: Option<Vessel>,
            /// Returns the vessel for this communication node.
            get: vessel
        }
    }

});

remote_type!(
    /// The type of communication link.
    enum CommLinkType {
        /// Link is to a base station on Kerbin.
        Home = 0,
        /// Link is to a control source, for example a manned spacecraft.
        Control = 1,
        /// Link is to a relay satellite.
        Relay = 2,
    }
);
