use crate::*;
use crate::codec::*;

use std::collections::{BTreeMap};
use std::rc::{Rc};
use std::cell::{RefCell};

pub mod vessel;
pub mod body;
pub mod orbit;
pub mod parts;
pub mod reference_frame;

pub use crate::spacecenter::vessel::*;
pub use crate::spacecenter::body::*;
pub use crate::spacecenter::orbit::*;
pub use crate::spacecenter::parts::*;
pub use crate::spacecenter::reference_frame::*;

/// The VAB (Vehicle Assembly Building) craft directory
pub const VAB: &'static str = "VAB";
/// The SPH (Space Plane Hanger) craft directory.
pub const SPH: &'static str = "SPH";

/// The Runway launch site.
pub const RUNWAY: &'static str = "Runway";
/// The LaunchPad launch site.
pub const LAUNCH_PAD: &'static str = "LaunchPad";

/// SpaceCenter service.
pub struct SpaceCenter {
    client : Rc<RefCell<KrpcClient>>
}

impl SpaceCenter {
    /// Creates a new SpaceCenter service using the given `client`.
    pub fn new(client: Rc<RefCell<KrpcClient>>) -> SpaceCenter {
        SpaceCenter{client}
    }

    rpc_method!(
    /// Returns the current amount of science.
    fn get_science(&self) -> f64 {
        if let Some(value) = SpaceCenter.get_Science() as f64 {
            value
        } else {
            return Err(KrpcError::NullResponseValue)
        }
    });

    rpc_method!(
    /// Returns the current amount of funds.
    fn get_funds(&self) -> f64 {
        if let Some(value) = SpaceCenter.get_Funds() as f64 {
            value
        } else {
            return Err(KrpcError::NullResponseValue)
        }
    });

    rpc_method!(
    /// Returns the current amount of reputation.
    fn get_reputation(&self) -> f32 {
        if let Some(value) = SpaceCenter.get_Reputation() as f32 {
            value
        } else {
            return Err(KrpcError::NullResponseValue)
        }
    });

    rpc_method!(
    /// Returns the currently active vessel or None if there isn't one.
    fn get_active_vessel(&self) -> Option<Vessel> {
        if let Some(value) = SpaceCenter.get_ActiveVessel() as Option<Vessel> {
            value
        } else {
            None
        }
    });

    rpc_method!(
    /// Sets the active vessel to the given vessel.
    /// # Arguments
    /// * `vessel` - The vessel to make active.
    fn set_active_vessel(&self, vessel: &vessel::Vessel) {
        SpaceCenter.set_ActiveVessel(vessel)
    });

    rpc_method!(
    /// Returns a list of all the vessels in the game.
    fn get_vessels(&self) -> Vec<Vessel> {
        if let Some(value) = SpaceCenter.get_Vessels() as Vec<Vessel> {
            value
        } else {
            return Err(KrpcError::NullResponseValue)
        }
    });

    rpc_method!(
    /// A map of all celestial bodies (planets, moons, etc.) in the game,
    /// keyed by the name of the body.
    fn get_bodies(&self) -> BTreeMap<String, CelestialBody> {
        if let Some(value) = SpaceCenter.get_Bodies() as BTreeMap<String, CelestialBody> {
            value
        } else {
            return Err(KrpcError::NullResponseValue)
        }
    });

    rpc_method!(
    /// Returns the currently targeted celestial body or None if there isn't one.
    fn get_target_body(&self) -> Option<CelestialBody> {
        if let Some(value) = SpaceCenter.get_TargetBody() as Option<CelestialBody> {
            value
        } else {
            None
        }
    });

    rpc_method!(
    /// Sets the current target to the given `body`.
    /// # Arguments
    /// * `body` - The celestial body to target.
    fn set_target_body(&self, body: &CelestialBody) {
        SpaceCenter.set_TargetBody(body)
    });

    rpc_method!(
    /// Returns the currently targeted vessel or None if there isn't one.
    fn get_target_vessel(&self) -> Option<Vessel> {
        if let Some(value) = SpaceCenter.get_TargetVessel() as Option<Vessel> {
            value
        } else {
            None
        }
    });

    rpc_method!(
    /// Sets the current target to the given `vessel`.
    /// # Arguments
    /// * `vessel` - The vessel to target.
    fn set_target_vessel(&self, vessel: &Vessel) {
        SpaceCenter.set_TargetVessel(vessel)
    });

    rpc_method!(
    /// Returns the currently targeted docking port or None if there isn't one.
    fn get_target_docking_port(&self) -> Option<DockingPort> {
        if let Some(value) = SpaceCenter.get_TargetDockingPort() as Option<DockingPort> {
            value
        } else {
            None
        }
    });

    rpc_method!(
    /// Sets the current target to the given `docking_port`.
    /// # Arguments
    /// * `docking_port` - The docking port to target.
    fn set_target_docking_port(&self, docking_port: &DockingPort) {
        SpaceCenter.set_TargetDockingPort(docking_port)
    });

    rpc_method!(
    /// Clears the current target.
    fn clear_target(&self) {
        SpaceCenter.ClearTarget()
    });

    rpc_method!(
    /// Returns a list of vessels from the given `craft_dir` that can be launched.
    /// # Arguments
    /// * `craft_dir` - Name of the directory in the current saves "Ships" directory. For example `"VAB"` or `"SPH"`.
    fn launchable_vessels(&self, craft_dir: String) -> Vec<String> {
        if let Some(value) = SpaceCenter.LaunchableVessels(craft_dir) as Vec<String> {
            value
        } else {
            return Err(KrpcError::NullResponseValue)
        }
    });

    rpc_method!(
    /// Launch a vessel.
    /// # Arguments
    /// * `craft_dir` - Name of the directory in the current saves "Ships" directory, that contains the craft file. For example `"VAB"` or `"SPH"`.
    /// * `name` - Name of the vessel to launch. This is the name of the ".craft" file in the save directory, without the ".craft" file extension.
    /// * `launch_site` - Name of the launch site. For example `"LaunchPad"` or `"Runway"`.
    /// * `recover` - If true and there is a vessel on the launch site, recover it before launching.
    ///
    /// # Errors
    /// Returns an error if any of the games pre-flight checks fail.
    fn launch_vessel(&self, craft_dir: String, name: String, launch_site : String, recover: bool) {
        SpaceCenter.LaunchVessel(craft_dir, name, launch_site, recover)
    });

    rpc_method!(
    /// Launch a new vessel from the VAB onto the launchpad.
    /// # Arguments
    /// * `name` - Name of the vessel to launch. This is the name of the ".craft" file in the save directory, without the ".craft" file extension.
    /// * `recover` - If true and there is a vessel on the launch site, recover it before launching.
    ///
    /// # Errors
    /// Returns an error if any of the games pre-flight checks fail.
    ///
    /// *Note* This is equivalent to calling `launch_vessel` with the craft directory
    /// set to `"VAB"` and the launch site set to `"LaunchPad"`.
    fn launch_vessel_from_vab(&self, name: String, recover: bool) {
        SpaceCenter.LaunchVesselFromVAB(name, recover)
    });

    rpc_method!(
    /// Launch a new vessel from the SPH onto the runway.
    /// # Arguments
    /// * `name` - Name of the vessel to launch. This is the name of the ".craft" file in the save directory, without the ".craft" file extension.
    /// * `recover` - If true and there is a vessel on the launch site, recover it before launching.
    ///
    /// # Errors
    /// Returns an error if any of the games pre-flight checks fail.
    ///
    /// *Note* This is equivalent to calling `launch_vessel` with the craft directory
    /// set to `"SPH"` and the launch site set to `"Runway"`.
    fn launch_vessel_from_sph(&self, name: String, recover: bool) {
        SpaceCenter.LaunchVesselFromSPH(name, recover)
    });
}

remote_type!(
/// The game mode.
enum GameMode {
    /// Sandbox mode.
    Sandbox => 0,
    /// Career mode.
    Career => 1,
    /// Science career mode.
    Science => 2,
    /// Science sandbox mode.
    ScienceSandbox => 3,
    /// Mission mode.
    Mission => 4,
    /// Mission builder mode.
    MissionBuilder => 5,
    /// Scenario mode.
    Scenario => 6,
    /// Scenario mode that cannot be resumed.
    ScenarioNonResumable => 7
});

remote_type!(
/// The time warp mode.
enum WarpMode {
    /// Time warp is active, and in regular "on-rails" mode.
    Rails => 0,
    /// Time warp is active, and in physical time warp mode.
    Physics => 1,
    /// Time warp is not active.
    None => 2
});