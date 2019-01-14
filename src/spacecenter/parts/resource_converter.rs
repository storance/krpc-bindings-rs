use super::Part;
use crate::codec::*;
use crate::*;

use std::rc::Rc;

remote_type!(
/// A resource converter. Obtained by calling `Part::resource_converter().`
object SpaceCenter.ResourceConverter {
    properties: {
        {
            Part: Part,
            /// The part object for this resource converter.
            get: part
        }
    }
});

remote_type!(
/// The state of a resource converter.
enum ResourceConverterState {
    /// Converter is running.
    Running => 0,
    /// Converter is idle.
    Idle => 1,
    /// Converter is missing a required resource.
    MissingResource => 2,
    /// No available storage for output resource.
    StorageFull => 3,
    /// At preset resource capacity.
    Capacity => 4,
    /// Unknown state. Possible with modified resource converters.
    /// In this case, check `ResourceConverter::status_info()` for more information.
    Unknown => 5
});
