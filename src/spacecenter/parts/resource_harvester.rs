use super::Part;
use crate::codec::{Decode, Encode};
use crate::{remote_type, RemoteEnum, RemoteObject};

remote_type!(
/// A resource harvester. Obtained by calling `Part::resource_harvester().`
object SpaceCenter.ResourceHarvester {
    properties: {
        {
            Part {
                /// Returns the part object for this resource harvester.
                ///
                /// **Game Scenes**: All
                get: part -> Part
            }
        }
        {
            State {
                /// Returns the state of the harvester.
                ///
                /// **Game Scenes**: All
                get: state -> ResourceHarvesterState
            }
        }
        {
            Deployed {
                /// Returns whether the harvester is deployed.
                ///
                /// **Game Scenes**: All
                get: is_deployed -> bool,
                /// Sets whether the harvester is deployed.
                ///
                /// **Game Scenes**: All
                set: set_deployed(bool)
            }
        }
        {
            Active {
                /// Returns whether the harvester is active.
                ///
                /// **Game Scenes**: All
                get: is_active -> bool,
                /// Sets whether the harvester is active.
                ///
                /// **Game Scenes**: All
                set: set_active(bool)
            }
        }
        {
            ExtractionRate {
                /// Returns the rate at which the drill is extracting ore, in units per second.
                ///
                /// **Game Scenes**: All
                get: extraction_rate -> f32
            }
        }
        {
            OptimumCoreTemperature {
                /// Returns the core temperature at which the drill will operate with
                /// peak efficiency, in Kelvin.
                ///
                /// **Game Scenes**: All
                get: optimum_core_temperature -> f32
            }
        }
        {
            CoreTemperature {
                /// Returns the core temperature of the drill, in Kelvin.
                ///
                /// **Game Scenes**: All
                get: core_temperature -> f32
            }
        }
        {
            ThermalEfficiency {
                /// Returns the thermal efficiency of the drill, as a percentage of its maximum.
                ///
                /// **Game Scenes**: All
                get: thermal_efficiency -> f32
            }
        }
    }
});

remote_type!(
    /// The state of a resource harvester.
    enum ResourceHarvesterState {
        /// The drill is deploying.
        Deploying = 0,
        /// The drill is deployed and ready.
        Deployed = 1,
        /// The drill is retracting.
        Retracting = 2,
        /// The drill is retracted.
        Retracted = 3,
        /// The drill is running.
        Active = 4,
    }
);
