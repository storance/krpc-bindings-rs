use crate::*;
use crate::codec::*;
use super::{Part};

use std::rc::Rc;

remote_type!(
/// A resource harvester. Obtained by calling `Part::resource_harvester().`
object SpaceCenter.ResourceHarvester {
    properties: {
        {
            Part: Part,
            /// The part object for this resource harvester.
            get: part
        }
    }
});

remote_type!(
/// The state of a resource harvester.
enum ResourceHarvesterState {
    /// The drill is deploying.
    Deploying => 0,
    /// The drill is deployed and ready.
    Deployed => 1,
    /// The drill is retracting.
    Retracting => 2,
    /// The drill is retracted.
    Retracted => 3,
    /// The drill is running.
    Active => 4
});