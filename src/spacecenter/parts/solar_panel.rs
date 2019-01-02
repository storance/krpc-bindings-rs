use crate::*;
use crate::codec::*;
use super::{Part};

use std::rc::{Rc};
use std::cell::{RefCell};

remote_type!(
/// A solar panel. Obtained by calling `Part::solar_panel().`
object SolarPanel {});

impl SolarPanel {
    rpc_property!(
        Part: Part {
            service: SpaceCenter,
            class: SolarPanel,
            /// The part object for this solar panel.
            part
        }
    );
}

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