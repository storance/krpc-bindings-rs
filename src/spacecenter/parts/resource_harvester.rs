use super::Part;
use crate::codec::*;
use crate::krpc::Expression;
use crate::*;

remote_type!(
/// A resource harvester. Obtained by calling `Part::resource_harvester().`
object SpaceCenter.ResourceHarvester {
    properties: {
        {
            Part: Part,
            /// Returns the part object for this resource harvester.
            ///
            /// **Game Scenes**: All
            get: part
        }
        {
            State: ResourceHarvesterState,
            /// Returns the state of the harvester.
            ///
            /// **Game Scenes**: All
            get: state
        }
        {
            Deployed: bool,
            /// Returns whether the harvester is deployed.
            ///
            /// **Game Scenes**: All
            get: is_deployed,
            /// Sets whether the harvester is deployed.
            ///
            /// **Game Scenes**: All
            set: set_deployed
        }
        {
            Active: bool,
            /// Returns whether the harvester is active.
            ///
            /// **Game Scenes**: All
            get: is_active,
            /// Sets whether the harvester is active.
            ///
            /// **Game Scenes**: All
            set: set_active
        }
        {
            ExtractionRate: f32,
            /// Returns the rate at which the drill is extracting ore, in units per second.
            ///
            /// **Game Scenes**: All
            get: extraction_rate
        }
        {
            OptimumCoreTemperature: f32,
            /// Returns the core temperature at which the drill will operate with
            /// peak efficiency, in Kelvin.
            ///
            /// **Game Scenes**: All
            get: optimum_core_temperature
        }
        {
            CoreTemperature: f32,
            /// Returns the core temperature of the drill, in Kelvin.
            ///
            /// **Game Scenes**: All
            get: core_temperature
        }
        {
            ThermalEfficiency: f32,
            /// Returns the thermal efficiency of the drill, as a percentage of its maximum.
            ///
            /// **Game Scenes**: All
            get: thermal_efficiency
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
