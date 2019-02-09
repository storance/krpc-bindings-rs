use super::Part;
use crate::codec::{Decode, Encode};
use crate::{remote_type, RemoteEnum, RemoteObject};

remote_type!(
    /// Represents the collection of resources stored in a vessel, stage or part. Created by
    /// calling `Vessel::resources()`, `Vessel::resources_in_decouple_stage()` or
    /// `Part::resources()`.
    object SpaceCenter.Resources {
        properties: {
            {
                All {
                    /// Returns all the individual resources that can be stored.
                    ///
                    /// **Game Scenes**: Flight
                    get: all -> Vec<Resource>
                }
            }
            {
                Names {
                    /// Returns a list of resource names that can be stored.
                    ///
                    /// **Game Scenes**: Flight
                    get: names -> Vec<String>
                }
            }
            {
                Enabled {
                    /// Returns whether use of all the resources are enabled.
                    ///
                    /// **Game Scenes**: Flight
                    get: is_enabled -> bool,
                    /// Sets whether use of all the resources are enabled.
                    ///
                    /// **Game Scenes**: Flight
                    set: set_enabled(bool)
                }
            }
        }
        methods: {
            {
                /// Returns all the individual resources with the given name that can be stored.
                ///
                /// **Game Scenes**: Flight
                ///
                /// # Arguments
                /// * `name` - The name of the resource.
                fn with_resource(name: &str) -> Vec<Resource> {
                    WithResource(name)
                }
            }
            {
                /// Check whether the named resource can be stored.
                ///
                /// **Game Scenes**: Flight
                ///
                /// # Arguments
                /// * `name` - The name of the resource.
                fn has_resource(name: &str) -> bool {
                    HasResource(name)
                }
            }
            {
                /// Returns the amount of a resource that is currently stored.
                ///
                /// **Game Scenes**: Flight
                ///
                /// # Arguments
                /// * `name` - The name of the resource.
                fn amount(name: &str) -> f32 {
                    Amount(name)
                }
            }
            {
                /// Returns the amount of a resource that can be stored.
                ///
                /// **Game Scenes**: Flight
                ///
                /// # Arguments
                /// * `name` - The name of the resource.
                fn max(name: &str) -> f32 {
                    Max(name)
                }
            }
        }
        static_methods: {
            {
                /// Returns the density of a resource, in kg/l.
                ///
                /// **Game Scenes**: Flight
                ///
                /// # Arguments
                /// * `name` - The name of the resource.
                fn density(name: &str) -> f32 {
                    Density(name)
                }
            }
            {
                /// Returns the flow mode of a resource.
                ///
                /// **Game Scenes**: Flight
                ///
                /// # Arguments
                /// * `name` - The name of the resource.
                fn flow_mode(name: &str) -> ResourceFlowMode {
                    FlowMode(name)
                }
            }
        }
    }
);

remote_type!(
    /// An individual resource stored within a part. Created using methods in the Resources class.
    object SpaceCenter.Resource {
        properties: {
            {
                Name {
                    /// Returns the name of the resource.
                    ///
                    /// **Game Scenes**: All
                    get: name -> String
                }
            }
            {
                Part {
                    /// Returns the part containing the resource.
                    ///
                    /// **Game Scenes**: All
                    get: part -> Part
                }
            }
            {
                Amount {
                    /// Returns the amount of the resource that is currently stored in the part.
                    ///
                    /// **Game Scenes**: All
                    get: amount -> f32
                }
            }
            {
                Max {
                    /// Returns the total amount of the resource that can be stored in the part.
                    ///
                    /// **Game Scenes**: All
                    get: max -> f32
                }
            }
            {
                Density {
                    /// Returns the density of the resource, in kg/l.
                    ///
                    /// **Game Scenes**: All
                    get: density -> f32
                }
            }
            {
                FlowMode {
                    /// Returns the flow mode of the resource.
                    ///
                    /// **Game Scenes**: All
                    get: flow_mode -> ResourceFlowMode
                }
            }
            {
                Enabled {
                    /// Returns whether use of this resource is enabled.
                    ///
                    /// **Game Scenes**: All
                    get: is_enabled -> bool,
                    /// Sets whether use of this resource is enabled.
                    ///
                    /// **Game Scenes**: All
                    set: set_enabled(bool)
                }
            }
        }
    }
);

remote_type!(
    /// Transfer resources between parts.
    object SpaceCenter.ResourceTransfer {
        properties: {
            {
                Amount {
                    /// Returns the amount of the resource that has been transferred.
                    ///
                    /// **Game Scenes**: All
                    get: amount -> f32
                }
            }
            {
                Complete {
                    /// Returns whether the transfer has completed.
                    ///
                    /// **Game Scenes**: All
                    get: complete -> bool
                }
            }
        }
        static_methods: {
            {
                /// Start transferring a resource transfer between a pair of parts.
                /// The transfer will move at most `max_amount` units of the resource, depending
                /// on how much of the resource is available in the source part and how much
                /// storage is available in the destination part. Use `ResourceTransfer::complete()`
                /// to check if the transfer is complete. Use `ResourceTransfer::amount()`
                /// to see how much of the resource has been transferred.
                ///
                /// **Game Scenes**: All
                ///
                /// # Arguments
                /// * `from_part` - The part to transfer from.
                /// * `to_part` - The part to transfer to.
                /// * `resource` - The name of the resource to transfer.
                /// * `max_amount` - The maximum amount of resource to transfer.
                fn start(from_part: &Part, to_part: &Part, resource: &str, max_amount: f32) -> ResourceTransfer<'a> {
                    Start(from_part, to_part, resource, max_amount)
                }
            }
        }
    }
);

remote_type!(
    /// The way in which a resource flows between parts. See `Resources.flow_mode(String)`.
    enum ResourceFlowMode {
        /// The resource flows to any part in the vessel. For example, electric charge.
        Vessel = 0,
        /// The resource flows from parts in the first stage, followed by the second,
        /// and so on. For example, mono-propellant.
        Stage = 1,
        /// The resource flows between adjacent parts within the vessel. For example,
        /// liquid fuel or oxidizer.
        Adjacent = 2,
        /// The resource does not flow. For example, solid fuel.
        None = 3,
    }
);
