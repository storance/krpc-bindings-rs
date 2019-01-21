use super::Part;
use crate::codec::*;
use crate::*;

remote_type!(
/// A light. Obtained by calling `Part::light().`
object SpaceCenter.Light {
    properties: {
        {
            Part: Part,
            /// Returns the part object for this light.
            ///
            /// **Game Scenes**: All
            get: part
        }
        {
            Active: bool,
            /// Returns whether the light is switched on.
            ///
            /// **Game Scenes**: All
            get: is_active,
            /// Sets whether the light is switched on.
            ///
            /// **Game Scenes**: All
            set: set_active
        }
        {
            Color: (f32, f32, f32),
            /// Returns the color of the light, as an RGB triple.
            ///
            /// **Game Scenes**: All
            get: color,
            /// Sets the color of the light, as an RGB triple.
            ///
            /// **Game Scenes**: All
            set: set_color
        }
        {
            PowerUsage: f32,
            /// Returns the current power usage, in units of charge per second.
            ///
            /// **Game Scenes**: All
            get: power_usage
        }
    }
});
