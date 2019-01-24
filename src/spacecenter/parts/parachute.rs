use super::Part;
use crate::codec::*;
use crate::*;
use crate::krpc::Expression;

remote_type!(
/// A parachute. Obtained by calling `Part::parachute().`
object SpaceCenter.Parachute {
    properties: {
        {
            Part: Part,
            /// Returns the part object for this parachute.
            ///
            /// **Game Scenes**: All
            get: part
        }
        {
            Deployed: bool,
            /// Returns whether the parachute has been deployed.
            ///
            /// **Game Scenes**: All
            get: is_deployed
        }
        {
            Armed: Part,
            /// Returns whether the parachute has been armed or deployed. Only applicable to
            /// RealChutes parachutes.
            ///
            /// **Game Scenes**: All
            get: is_armed
        }
        {
            State: ParachuteState,
            /// Returns the current state of the parachute.
            ///
            /// **Game Scenes**: All
            get: state
        }
        {
            DeployAltitude: f32,
            /// Returns the altitude at which the parachute will full deploy, in meters.
            /// Only applicable to stock parachutes.
            ///
            /// **Game Scenes**: All
            get: deploy_altitude,
            /// Sets the altitude at which the parachute will full deploy, in meters.
            /// Only applicable to stock parachutes.
            ///
            /// **Game Scenes**: All
            set: set_deploy_altitude
        }
        {
            DeployMinPressure: f32,
            /// Returns the minimum pressure at which the parachute will semi-deploy, in
            /// atmospheres. Only applicable to stock parachutes.
            ///
            /// **Game Scenes**: All
            get: deploy_min_pressure,
            /// Sets the minimum pressure at which the parachute will semi-deploy, in
            /// atmospheres. Only applicable to stock parachutes.
            ///
            /// **Game Scenes**: All
            set: set_deploy_min_pressure
        }
    }
});

remote_type!(
    /// The state of a parachute.
    enum ParachuteState {
        /// The parachute is safely tucked away inside its housing.
        Stowed = 0,
        /// The parachute is armed for deployment. (RealChutes only)
        Armed = 1,
        /// The parachute is still stowed, but ready to semi-deploy.
        /// (Stock parachutes only)
        Active = 2,
        /// The parachute has been deployed and is providing some drag,
        /// but is not fully deployed yet. (Stock parachutes only)
        SemiDeployed = 3,
        /// The parachute is fully deployed.
        Deployed = 4,
        /// The parachute has been cut.
        Cut = 5,
    }
);
