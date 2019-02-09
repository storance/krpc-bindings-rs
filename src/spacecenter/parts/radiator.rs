use super::Part;
use crate::codec::{Decode, Encode};
use crate::{remote_type, RemoteEnum, RemoteObject};

remote_type!(
/// A radiator. Obtained by calling `Part::radiator().`
object SpaceCenter.Radiator {
    properties: {
        {
            Part {
                /// Returns the part object for this radiator.
                ///
                /// **Game Scenes**: All
                get: part -> Part
            }
        }
        {
            Deployable {
                /// Returns whether the radiator is deployable.
                ///
                /// **Game Scenes**: All
                get: is_deployable -> bool
            }
        }
        {
            Deployed {
                /// Returns whether the radiator is extended.  Returns `true` if the radiator is
                /// not deployable.
                ///
                /// **Game Scenes**: All
                get: is_deployed -> bool,
                /// Sets whether the radiator is extended.
                ///
                /// **Game Scenes**: All
                set: set_deployed(bool)
            }
        }
        {
            State {
                /// Returns the current state of the radiator.
                ///
                /// **Game Scenes**: All
                ///
                /// # Note
                /// A fixed radiator is always `RadiatorState::Extended`.
                get: state -> RadiatorState
            }
        }
    }
});

remote_type!(
    /// The state of a radiator.
    enum RadiatorState {
        /// Radiator is fully extended.
        Extended = 0,
        /// Radiator is fully retracted.
        Retracted = 1,
        /// Radiator is being extended.
        Extending = 2,
        /// Radiator is being retracted.
        Retracting = 3,
        /// Radiator is being broken.
        Broken = 4,
    }
);
