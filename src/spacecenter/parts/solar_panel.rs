use crate::*;
use crate::codec::*;
use super::{Part};

use std::rc::Rc;

remote_type!(
/// A solar panel. Obtained by calling `Part::solar_panel().`
object SpaceCenter.SolarPanel {
    properties: {
        {
            Part: Part,
            /// The part object for this solar panel.
            get: part
        }
    }
});

remote_type!(
/// The state of a solar panel.
enum SolarPanelState {
    /// Solar panel is fully extended.
    Extended => 0,
    /// Solar panel is fully retracted.
    Retracted => 1,
    /// Solar panel is being extended.
    Extending => 2,
    /// Solar panel is being retracted.
    Retracting => 3,
    /// Solar panel is broken
    Broken => 4
});