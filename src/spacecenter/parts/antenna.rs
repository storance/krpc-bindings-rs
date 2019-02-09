use super::Part;
use crate::codec::{Decode, Encode};
use crate::{remote_type, RemoteEnum, RemoteObject};

remote_type!(
/// An antenna. Obtained by calling `Part::antenna().`
object SpaceCenter.Antenna {
    properties: {
        {
            Part {
                /// Returns the part object for this antenna.
                ///
                /// **Game Scenes**: All
                get: part -> Part
            }
        }
        {
            State {
                /// Returns the current state of the antenna.
                ///
                /// **Game Scenes**: All
                get: state -> AntennaState
            }
        }
        {
            Deployable {
                /// Returns whether the antenna is deployable.
                ///
                /// **Game Scenes**: All
                get: is_deployable -> bool
            }
        }
        {
            Deployed {
                /// Returns whether the antenna is deployed.
                ///
                /// **Game Scenes**: All
                get: is_deployed -> bool,
                /// Sets whether the antenna is deployed.
                ///
                /// **Game Scenes**: All
                ///
                /// # Note
                /// Fixed antennas are always deployed. Returns an error if you try to deploy
                /// a fixed antenna.
                set: set_deployed(bool)
            }
        }
        {
            CanTransmit {
                /// Returns whether data can be transmitted by this antenna.
                ///
                /// **Game Scenes**: All
                get: can_transmit -> bool
            }
        }
        {
            AllowPartial {
                /// Returns whether partial data transmission is permitted.
                ///
                /// **Game Scenes**: All
                get: is_allow_partial -> bool,
                /// Sets whether partial data transmission is permitted.
                ///
                /// **Game Scenes**: All
                set: set_allow_partial(bool)
            }
        }
        {
            Power {
                /// Returns the power of the antenna.
                ///
                /// **Game Scenes**: All
                get: power -> f64
            }
        }
        {
            Combinable {
                /// Returns whether the antenna can be combined with other antennae on the
                /// vessel to boost the power.
                ///
                /// **Game Scenes**: All
                get: is_combinable -> bool
            }
        }
        {
            CombinableExponent {
                /// Returns the exponent used to calculate the combined power of multiple
                /// antennae on a vessel.
                ///
                /// **Game Scenes**: All
                get: combinable_exponent -> f64
            }
        }
        {
            PacketInterval {
                /// Returns the interval between sending packets in seconds.
                ///
                /// **Game Scenes**: All
                get: packet_interval -> f32
            }
        }
        {
            PacketSize {
                /// Returns the amount of data sent per packet in Mits.
                ///
                /// **Game Scenes**: All
                get: packet_size -> f32
            }
        }
        {
            PacketResourceCost {
                /// Returns the units of electric charge consumed per packet sent.
                ///
                /// **Game Scenes**: All
                get: packet_resource_cost -> f64
            }
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
        Deployed = 0,
        /// Antenna is fully retracted.
        Retracted = 1,
        /// Antenna is being deployed.
        Deploying = 2,
        /// Antenna is being retracted.
        Retracting = 3,
        /// Antenna is broken.
        Broken = 4,
    }
);
