use super::Part;
use crate::codec::*;
use crate::*;

remote_type!(
/// A landing leg. Obtained by calling `Part::leg().`
object SpaceCenter.Leg {
    properties: {
        {
            Part: Part,
            /// Returns the part object for this leg.
            ///
            /// **Game Scenes**: All
            get: part
        }
        {
            State: LegState,
            /// Returns the current state of the landing leg.
            ///
            /// **Game Scenes**: All
            get: state
        }
        {
            Deployable: bool,
            /// Returns whether the leg is deployable.
            ///
            /// **Game Scenes**: All
            get: is_deployable
        }
        {
            Deployed: bool,
            /// Returns whether the landing leg is deployed.
            ///
            /// **Game Scenes**: All
            get: is_deployed,
            /// Sets whether the landing leg is deployed.
            ///
            /// **Game Scenes**: All
            ///
            /// # Note
            /// Fixed landing legs are always deployed. Returns an error if you try to deploy
            /// fixed landing gear.
            set: set_deployed
        }
        {
            IsGrounded: bool,
            /// Returns whether the leg is touching the ground.
            ///
            /// **Game Scenes**: All
            get: is_grounded
        }
    }
});

remote_type!(
    /// The state of a landing leg.
    enum LegState {
        /// Landing leg is fully deployed.
        Deployed = 0,
        /// Landing leg is fully retracted.
        Retracted = 1,
        /// Landing leg is being deployed.
        Deploying = 2,
        /// Landing leg is being retracted.
        Retracting = 3,
        /// Landing leg is broken.
        Broken = 4,
    }
);
