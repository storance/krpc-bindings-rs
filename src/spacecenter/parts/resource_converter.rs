use super::Part;
use crate::codec::*;
use crate::krpc::Expression;
use crate::*;

remote_type!(
/// A resource converter. Obtained by calling `Part::resource_converter().`
object SpaceCenter.ResourceConverter {
    properties: {
        {
            Part: Part,
            /// Returns the part object for this resource converter.
            ///
            /// **Game Scenes**: All
            get: part
        }
        {
            Count: i32,
            /// Returns the number of converters in the part.
            ///
            /// **Game Scenes**: All
            get: count
        }
        {
            OptimumCoreTemperature: f32,
            /// Returns the core temperature at which the converter will operate with
            /// peak efficiency, in Kelvin.
            ///
            /// **Game Scenes**: All
            get: optimum_core_temperature
        }
        {
            CoreTemperature: f32,
            /// Returns the core temperature of the converter, in Kelvin.
            ///
            /// **Game Scenes**: All
            get: core_temperature
        }
        {
            ThermalEfficiency: f32,
            /// Returns the thermal efficiency of the converter, as a percentage of its maximum.
            ///
            /// **Game Scenes**: All
            get: thermal_efficiency
        }
    }
    methods: {
        {
            /// The name of the specified converter.
            ///
            /// **Game Scenes**: All
            ///
            /// # Arguments
            /// * `index` – Index of the converter.
            fn name(index: i32) -> String {
                Name(index)
            }
        }
        {
            /// True if the specified converter is active.
            ///
            /// **Game Scenes**: All
            ///
            /// # Arguments
            /// * `index` – Index of the converter.
            fn active(index: i32) -> bool {
                Active(index)
            }
        }
        {
            /// Start the specified converter.
            ///
            /// **Game Scenes**: All
            ///
            /// # Arguments
            /// * `index` – Index of the converter.
            fn start(index: i32) {
                Start(index)
            }
        }
        {
            /// Stop the specified converter.
            ///
            /// **Game Scenes**: All
            ///
            /// # Arguments
            /// * `index` – Index of the converter.
            fn stop(index: i32) {
                Stop(index)
            }
        }
        {
            /// The state of the specified converter.
            ///
            /// **Game Scenes**: All
            ///
            /// # Arguments
            /// * `index` – Index of the converter.
            fn state(index: i32) -> ResourceConverterState {
                State(index)
            }
        }
        {
            /// Status information for the specified converter. This is the full status
            /// message shown in the in-game UI.
            ///
            /// **Game Scenes**: All
            ///
            /// # Arguments
            /// * `index` – Index of the converter.
            fn status_info(index: i32) -> String {
                StatusInfo(index)
            }
        }
        {
            /// List of the names of resources consumed by the specified converter.
            ///
            /// **Game Scenes**: All
            ///
            /// # Arguments
            /// * `index` – Index of the converter.
            fn inputs(index: i32) -> Vec<String> {
                Inputs(index)
            }
        }
        {
            /// List of the names of resources produced by the specified converter.
            ///
            /// **Game Scenes**: All
            ///
            /// # Arguments
            /// * `index` – Index of the converter.
            fn outputs(index: i32) -> Vec<String> {
                Output(index)
            }
        }

    }
});

remote_type!(
    /// The state of a resource converter.
    enum ResourceConverterState {
        /// Converter is running.
        Running = 0,
        /// Converter is idle.
        Idle = 1,
        /// Converter is missing a required resource.
        MissingResource = 2,
        /// No available storage for output resource.
        StorageFull = 3,
        /// At preset resource capacity.
        Capacity = 4,
        /// Unknown state. Possible with modified resource converters.
        /// In this case, check `ResourceConverter::status_info()` for more information.
        Unknown = 5,
    }
);
