use crate::*;
use crate::codec::*;
use super::{Part};

use std::rc::Rc;

remote_type!(
/// An antenna. Obtained by calling `Part::antenna().`
object SpaceCenter.Antenna {
    properties: {
        {
            Part: Part,
            /// Returns the part object for this antenna.
            ///
            /// **Game Scenes**: All
            get: part
        }
        {
            State: AntennaState,
            /// Returns the current state of the antenna.
            ///
            /// **Game Scenes**: All
            get: state
        }
        {
            Deployable: bool,
            /// Returns whether the antenna is deployable.
            ///
            /// **Game Scenes**: All
            get: is_deployable
        }
        {
            Deployed: bool,
            /// Returns whether the antenna is deployed.
            ///
            /// **Game Scenes**: All
            get: is_deployed,
            /// Sets whether the antenna is deployed.
            ///
            /// **Game Scenes**: All
            ///
            /// # Note
            /// Fixed antennas are always deployed. Returns an error if you try to deploy
            /// a fixed antenna.
            set: set_deployed
        }
        {
            CanTransmit: bool,
            /// Returns whether data can be transmitted by this antenna.
            ///
            /// **Game Scenes**: All
            get: can_transmit
        }
        {
            AllowPartial: bool,
            /// Returns whether partial data transmission is permitted.
            ///
            /// **Game Scenes**: All
            get: is_allow_partial,
            /// Sets whether partial data transmission is permitted.
            ///
            /// **Game Scenes**: All
            set: set_allow_partial
        }
        {
            Power: f64,
            /// Returns the power of the antenna.
            ///
            /// **Game Scenes**: All
            get: power
        }
        {
            Combinable: bool,
            /// Returns whether the antenna can be combined with other antennae on the
            /// vessel to boost the power.
            ///
            /// **Game Scenes**: All
            get: is_combinable
        }
        {
            CombinableExponent: f64,
            /// Returns the exponent used to calculate the combined power of multiple
            /// antennae on a vessel.
            ///
            /// **Game Scenes**: All
            get: combinable_exponent
        }
        {
            PacketInterval: f32,
            /// Returns the interval between sending packets in seconds.
            ///
            /// **Game Scenes**: All
            get: packet_interval
        }
        {
            PacketSize: f32,
            /// Returns the amount of data sent per packet in Mits.
            ///
            /// **Game Scenes**: All
            get: packet_size
        }
        {
            PacketResourceCost: f64,
            /// Returns the units of electric charge consumed per packet sent.
            ///
            /// **Game Scenes**: All
            get: packet_resource_cost
        }
    }
    methods: {
        {
            /// Transmit data.
            ///
            /// **Game Scenes**: All
            fn transmit() {
                Transmit()
            }
        }
        {
            /// Cancel current transmission of data.
            ///
            /// **Game Scenes**: All
            fn cancel() {
                Cancel()
            }
        }
    }
});

remote_type!(
/// The state of an antenna.
enum AntennaState {
    /// Antenna is fully deployed.
    Deployed => 0,
    /// Antenna is fully retracted.
    Retracted => 1,
    /// Antenna is being deployed.
    Deploying => 2,
    /// Antenna is being retracted.
    Retracting => 3,
    /// Antenna is broken.
    Broken => 4
});