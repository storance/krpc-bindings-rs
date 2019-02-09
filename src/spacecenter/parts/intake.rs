use super::Part;
use crate::codec::{Decode, Encode};
use crate::{remote_type, RemoteObject};

remote_type!(
/// An intake. Obtained by calling `Part::intake().`
object SpaceCenter.Intake {
    properties: {
        {
            Part {
                /// Returns the part object for this intake.
                ///
                /// **Game Scenes**: All
                get: part -> Part
            }
        }
        {
            Open {
                /// Returns whether the intake is open.
                ///
                /// **Game Scenes**: All
                get: is_open -> bool,
                /// Sets whether the intake is open.
                ///
                /// **Game Scenes**: All
                set: set_open(bool)
            }
        }
        {
            Speed {
                /// Returns the speed of the flow into the intake, in m/s.
                ///
                /// **Game Scenes**: All
                get: speed -> f32
            }
        }
        {
            Flow {
                /// Returns the rate of flow into the intake, in units of resource per second.
                ///
                /// **Game Scenes**: All
                get: flow -> f32
            }
        }
        {
            Area {
                /// Returns the area of the intake’s opening, in square meters.
                ///
                /// **Game Scenes**: All
                get: area -> f32
            }
        }
    }
});
