use super::Vessel;
use crate::codec::{Decode, Encode};
use crate::{remote_type, RemoteEnum, RemoteObject};

remote_type!(
/// Used to interact with CommNet for a given vessel. Obtained by calling `Vessel::comms()`.
object SpaceCenter.Comms {
    properties: {
        {
            CanCommunicate {
                /// Returns whether the vessel can communicate with KSC.
                ///
                /// **Game Scenes**: Flight
                get: can_communicate -> bool
            }
        }
        {
            CanTransmitScience {
                /// Returns whether the vessel can transmit science data to KSC.
                ///
                /// **Game Scenes**: Flight
                get: can_transmit_science -> bool
            }
        }
        {
            SignalStrength {
                /// Returns the signal strength to KSC.
                ///
                /// **Game Scenes**: Flight
                get: signal_strength -> f64
            }
        }
        {
            SignalDelay {
                /// Returns the signal delay to KSC in seconds.
                ///
                /// **Game Scenes**: Flight
                get: signal_delay -> f64
            }
        }
        {
            Power {
                /// Returns the combined power of all active antennae on the vessel.
                ///
                /// **Game Scenes**: Flight
                get: power -> f64
            }
        }
        {
            ControlPath {
                /// Returns the combined power of all active antennae on the vessel.
                ///
                /// **Game Scenes**: Flight
                get: control_path -> Vec<CommLink>
            }
        }
    }
});

remote_type!(
/// Represents a communication node in the network. For example, a vessel or the KSC.
object SpaceCenter.CommLink {
    properties: {
        {
            Type {
                /// Returns the type of link
                ///
                /// **Game Scenes**: Flight
                get: link_type -> CommLinkType
            }
        }
        {
            SignalStrength {
                /// Returns the signal strength of the link.
                ///
                /// **Game Scenes**: Flight
                get: signal_strength -> f64
            }
        }
        {
            Start {
                /// Returns the start point of the link.
                ///
                /// **Game Scenes**: Flight
                get: start -> CommNode
            }
        }
        {
            End {
                /// Returns the end point of the link.
                ///
                /// **Game Scenes**: Flight
                get: end -> CommNode
            }
        }
    }

});

remote_type!(
/// Represents a communication node in the network. For example, a vessel or the KSC.
object SpaceCenter.CommNode {
    properties: {
        {
            Name {
                /// Returns the name of the communication node.
                ///
                /// **Game Scenes**: Flight
                get: name -> String
            }
        }
        {
            IsHome {
                /// Returns whether the communication node is on Kerbin.
                ///
                /// **Game Scenes**: Flight
                get: is_home -> bool
            }
        }
        {
            IsControlPoint {
                /// Returns whether the communication node is a control point, for example
                /// a manned vessel.
                ///
                /// **Game Scenes**: Flight
                get: is_control_point -> bool
            }
        }
        {
            IsVessel {
                /// Returns whether the communication node is a vessel.
                ///
                /// **Game Scenes**: Flight
                get: is_vessel -> bool
            }
        }
        {
            Vessel {
                /// Returns the vessel for this communication node.
                ///
                /// **Game Scenes**: Flight
                get: vessel -> Option<Vessel>
            }
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
