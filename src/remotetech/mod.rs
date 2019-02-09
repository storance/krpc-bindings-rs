use crate::codec::{Decode, Encode};
use crate::remote_type;
use crate::spacecenter::{Part, Vessel};

mod antenna;
mod comms;

pub use self::antenna::*;
pub use self::comms::*;

remote_type!(
/// This service provides functionality to interact with RemoteTech.
service RemoteTech {
    properties: {
        {
            Available {
                /// Returns whether RemoteTech is installed.
                ///
                /// **Game Scenes**: All
                get: is_available -> bool
            }
        }
        {
            GroundStations {
                /// Returns the names of the ground stations.
                ///
                /// **Game Scenes**: All
                get: ground_stations -> Vec<String>
            }
        }
    }
    methods: {
        {
            /// Get the antenna object for a particular part.
            ///
            /// **Game Scenes**: All
            fn antenna(part: &Part) -> Antenna {
                Antenna(part)
            }
        }
        {
            /// Get a communications object, representing the communication capability of
            /// a particular vessel.
            ///
            /// **Game Scenes**: All
            fn comms(vessel: &Vessel) -> Comms {
                Comms(vessel)
            }
        }
    }
});
