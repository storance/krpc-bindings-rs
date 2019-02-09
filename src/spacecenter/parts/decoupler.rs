use super::Part;
use crate::codec::{Decode, Encode};
use crate::spacecenter::Vessel;
use crate::{remote_type, RemoteObject};

remote_type!(
/// A decoupler. Obtained by calling `Part::decoupler().`
object SpaceCenter.Decoupler {
    properties: {
        {
            Part {
                /// Returns the part object for this decoupler.
                ///
                /// **Game Scenes**: All
                get: part -> Part
            }
        }
        {
            Decoupled {
                /// Returns whether the decoupler has fired.
                ///
                /// **Game Scenes**: All
                get: is_decoupled -> bool
            }
        }
        {
            Staged {
                /// Returns whether the decoupler has fired.
                ///
                /// **Game Scenes**: All
                get: is_staged -> bool
            }
        }
        {
            Impulse {
                /// Returns the impulse that the decoupler imparts when it is fired, in Newton seconds.
                ///
                /// **Game Scenes**: All
                get: impulse -> f32
            }
        }
    }
    methods: {
        {
            /// Fires the decoupler. Returns the new vessel created when the decoupler fires.
            /// Returns an error if the decoupler has already fired.
            ///
            /// **Game Scenes**: All
            ///
            /// # Note
            /// When called, the active vessel may change. It is therefore possible that,
            /// after calling this function, the object(s) returned by previous call(s) to
            /// `active_vessel()` no longer refer to the active vessel.
            fn decouple() -> Vessel {
                Decouple()
            }
        }
    }
});
