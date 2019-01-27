use super::Part;
use crate::codec::*;
use crate::krpc::Expression;
use crate::*;

remote_type!(
/// A radiator. Obtained by calling `Part::radiator().`
object SpaceCenter.Radiator {
    properties: {
        {
            Part: Part,
            /// Returns the part object for this radiator.
            ///
            /// **Game Scenes**: All
            get: part
        }
        {
            Deployable: bool,
            /// Returns whether the radiator is deployable.
            ///
            /// **Game Scenes**: All
            get: is_deployable
        }
        {
            Deployed: bool,
            /// Returns whether the radiator is extended.  Returns `true` if the radiator is
            /// not deployable.
            ///
            /// **Game Scenes**: All
            get: is_deployed,
            /// Sets whether the radiator is extended.
            ///
            /// **Game Scenes**: All
            set: set_deployed
        }
        {
            State: RadiatorState,
            /// Returns the current state of the radiator.
            ///
            /// **Game Scenes**: All
            ///
            /// # Note
            /// A fixed radiator is always `RadiatorState::Extended`.
            get: state
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
