use super::Part;
use crate::codec::*;
use crate::krpc::Expression;
use crate::*;

remote_type!(
/// A fairing. Obtained by calling `Part::fairing().`
object SpaceCenter.Fairing {
    properties: {
        {
            Part: Part,
            /// Returns the part object for this fairing.
            ///
            /// **Game Scenes**: All
            get: part
        }
        {
            Jettisoned: bool,
            /// Returns whether the fairing has been jettisoned.
            ///
            /// **Game Scenes**: All
            get: is_jettisoned
        }
    }
    methods: {
        {
            /// Jettison the fairing. Has no effect if it has already been jettisoned.
            ///
            /// **Game Scenes**: All
            fn jettison() {
                Jettison()
            }
        }
    }
});
