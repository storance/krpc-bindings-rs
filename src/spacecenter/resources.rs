use crate::*;
use crate::codec::*;

use std::rc::{Rc};
use std::cell::{RefCell};

remote_type!(
    /// Represents the collection of resources stored in a vessel, stage or part. Created by
    /// calling `Vessel::resources()`, `Vessel::resources_in_decouple_stage()` or
    /// `Part::resources()`.
    object Resources {}
);

remote_type!(
    /// An individual resource stored within a part. Created using methods in the Resources class.
    object Resource {}
);

remote_type!(
    /// Transfer resources between parts.
    object ResourceTransfer {}
);

remote_type!(
    /// The way in which a resource flows between parts. See `Resources.flow_mode(String)`.
    enum ResourceFlowMode {
        /// The resource flows to any part in the vessel. For example, electric charge.
        Vessel => 0,
        /// The resource flows from parts in the first stage, followed by the second,
        /// and so on. For example, mono-propellant.
        Stage => 1,
        /// The resource flows between adjacent parts within the vessel. For example,
        /// liquid fuel or oxidizer.
        Adjacent => 2,
        /// The resource does not flow. For example, solid fuel.
        None => 3
    }
);