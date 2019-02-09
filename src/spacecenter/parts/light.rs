use super::Part;
use crate::codec::{Decode, Encode};
use crate::{remote_type, RemoteObject};

remote_type!(
/// A light. Obtained by calling `Part::light().`
object SpaceCenter.Light {
    properties: {
        {
            Part {
                /// Returns the part object for this light.
                ///
                /// **Game Scenes**: All
                get: part -> Part
            }
        }
        {
            Active {
                /// Returns whether the light is switched on.
                ///
                /// **Game Scenes**: All
                get: is_active -> bool,
                /// Sets whether the light is switched on.
                ///
                /// **Game Scenes**: All
                set: set_active(bool)
            }
        }
        {
            Color {
                /// Returns the color of the light, as an RGB triple.
                ///
                /// **Game Scenes**: All
                get: color -> (f32, f32, f32),
                /// Sets the color of the light, as an RGB triple.
                ///
                /// **Game Scenes**: All
                set: set_color((f32, f32, f32))
            }
        }
        {
            PowerUsage {
                /// Returns the current power usage, in units of charge per second.
                ///
                /// **Game Scenes**: All
                get: power_usage -> f32
            }
        }
    }
});
