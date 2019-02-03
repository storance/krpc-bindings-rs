use super::Antenna;
use crate::codec::{Decode, Encode};
use crate::spacecenter::Vessel;
use crate::{remote_type, RemoteObject};

remote_type!(
/// Communications for a vessel.
object RemoteTech.Comms {
    properties: {
        {
            Vessel: Vessel,
            /// Returns the vessel.
            ///
            /// **Game Scenes**: All
            get: vessel
        }
        {
            HasLocalControl: bool,
            /// Returns whether the vessel can be controlled locally.
            ///
            /// **Game Scenes**: All
            get: has_local_control
        }
        {
            HasFlightComputer: bool,
            /// Returns whether the vessel has a flight computer on board.
            ///
            /// **Game Scenes**: All
            get: has_flight_computer
        }
        {
            HasConnection: bool,
            /// Returns whether the vessel has any connection.
            ///
            /// **Game Scenes**: All
            get: has_connection
        }
        {
            HasConnectionToGroundStation: bool,
            /// Returns whether the vessel has a connection to a ground station.
            ///
            /// **Game Scenes**: All
            get: has_connection_to_ground_station
        }
        {
            SignalDelay: f64,
            /// Returns the shortest signal delay to the vessel, in seconds.
            ///
            /// **Game Scenes**: All
            get: signal_display
        }
        {
            Antennas: Vec<Antenna>,
            /// Returns the antennas for this vessel.
            ///
            /// **Game Scenes**: All
            get: antennas
        }
    }
    methods: {
        {
            /// The signal delay between the this vessel and another vessel, in seconds.
            ///
            /// **Game Scenes**: All
            fn signal_delay_to_vessel(vessel: &Vessel) -> f64 {
                SignalDelayToVessel(vessel)
            }
        }
    }
});
