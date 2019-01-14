use crate::*;
use crate::codec::*;
use super::{Part};

use std::rc::Rc;

remote_type!(
/// A parachute. Obtained by calling `Part::parachute().`
object SpaceCenter.Parachute {
    properties: {
        {
            Part: Part,
            /// The part object for this parachute.
            get: part
        }
    }
});

remote_type!(
/// The state of a parachute.
enum ParachuteState {
    /// The parachute is safely tucked away inside its housing.
    Stowed => 0,
    /// The parachute is armed for deployment. (RealChutes only)
    Armed => 1,
    /// The parachute is still stowed, but ready to semi-deploy.
    /// (Stock parachutes only)
    Active => 2,
    /// The parachute has been deployed and is providing some drag,
    /// but is not fully deployed yet. (Stock parachutes only)
    SemiDeployed => 3,
    /// The parachute is fully deployed.
    Deployed => 4,
    /// The parachute has been cut.
    Cut => 5
});