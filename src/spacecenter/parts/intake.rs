use super::Part;
use crate::codec::*;
use crate::*;
use crate::krpc::Expression;

remote_type!(
/// An intake. Obtained by calling `Part::intake().`
object SpaceCenter.Intake {
    properties: {
        {
            Part: Part,
            /// Returns the part object for this intake.
            ///
            /// **Game Scenes**: All
            get: part
        }
        {
            Open: bool,
            /// Returns whether the intake is open.
            ///
            /// **Game Scenes**: All
            get: is_open,
            /// Sets whether the intake is open.
            ///
            /// **Game Scenes**: All
            set: set_open
        }
        {
            Speed: f32,
            /// Returns the speed of the flow into the intake, in m/s.
            ///
            /// **Game Scenes**: All
            get: speed
        }
        {
            Flow: f32,
            /// Returns the rate of flow into the intake, in units of resource per second.
            ///
            /// **Game Scenes**: All
            get: flow
        }
        {
            Area: f32,
            /// Returns the area of the intake’s opening, in square meters.
            ///
            /// **Game Scenes**: All
            get: area
        }
    }
});
