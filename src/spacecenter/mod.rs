use crate::*;
use crate::codec::*;

use std::collections::{BTreeMap};
use std::rc::{Rc};
use std::cell::{RefCell};

pub mod autopilot;
pub mod body;
pub mod communication;
pub mod control;
pub mod flight;
pub mod node;
pub mod orbit;
pub mod parts;
pub mod reference_frame;
pub mod resources;
pub mod vessel;

pub use self::autopilot::*;
pub use self::body::*;
pub use self::communication::*;
pub use self::control::*;
pub use self::flight::*;
pub use self::node::*;
pub use self::orbit::*;
pub use self::parts::*;
pub use self::reference_frame::*;
pub use self::resources::*;
pub use self::vessel::*;


/// The VAB (Vehicle Assembly Building) craft directory
pub const VAB: &'static str = "VAB";
/// The SPH (Space Plane Hanger) craft directory.
pub const SPH: &'static str = "SPH";

/// The Runway launch site.
pub const RUNWAY: &'static str = "Runway";
/// The LaunchPad launch site.
pub const LAUNCH_PAD: &'static str = "LaunchPad";

remote_type!(
    /// SpaceCenter service.
    service SpaceCenter {
        properties: {
            {
                Science: f64,
                /// Returns the current amount of science.
                get: science
            }
            {
                Funds: f64,
                /// Returns the current amount of funds.
                get: funds
            }
            {
                Reputation: f32,
                /// Returns the current amount of reputation.
                get: reputation
            }
            {
                ActiveVessel: Option<Vessel>,
                /// Returns the currently active vessel or None if there isn't one.
                get: active_vessel,
                /// Sets the active vessel to the given vessel.
                ///
                /// # Arguments
                /// * `value` - The vessel to make active.
                set: set_active_vessel
            }
            {
                Vessels: Vec<Vessel>,
                /// Returns a list of all the vessels in the game.
                get: vessels
            }
            {
                Bodies: BTreeMap<String, CelestialBody>,
                /// A map of all celestial bodies (planets, moons, etc.) in the game,
                /// keyed by the name of the body.
                get: bodies
            }
            {
                TargetBody: Option<CelestialBody>,
                /// Returns the currently targeted celestial body or `None` if there isn't one.
                get: target_body,
                /// Sets the current target to the given `body`.
                ///
                /// # Arguments
                /// * `value` - The celestial body to target.
                set: set_target_body
            }
            {
                TargetVessel: Option<Vessel>,
                /// Returns the currently targeted vessel or `None` if there isn't one.
                get: target_vessel,
                /// Sets the current target to the given vessel.
                ///
                /// # Arguments
                /// * `value` - The vessel to target.
                set: set_target_vessel
            }
            {
                TargetDockingPort: Option<DockingPort>,
                /// Returns the currently targeted docking port or `None` if there isn't one.
                get: target_docking_port,
                /// Sets the current target to the given docking port.
                ///
                /// # Arguments
                /// * `value` - The docking port to target.
                set: set_target_docking_port
            }
        },
        methods: {
            {
                /// Clears the current target.
                fn clear_target() {
                    ClearTarget()
                }
            }
            {
                /// Returns a list of vessels from the given `craft_dir` that can be launched.
                ///
                /// # Arguments
                /// * `craft_dir` - Name of the directory in the current saves "Ships"
                /// directory. For example `"VAB"` or `"SPH"`.
                fn launchable_vessels(craft_dir: String) -> Vec<String> {
                    LaunchableVessels(craft_dir)
                }
            }
            {
                /// Launch a vessel.
                ///
                /// # Arguments
                /// * `craft_dir` - Name of the directory in the current saves "Ships"
                /// directory, that contains the craft file. For example `"VAB"` or `"SPH"`.
                /// * `name` - Name of the vessel to launch. This is the name of the ".craft"
                /// file in the save directory, without the ".craft" file extension.
                /// * `launch_site` - Name of the launch site. For example `"LaunchPad"`
                /// or `"Runway"`.
                /// * `recover` - If true and there is a vessel on the launch site, recover
                /// it before launching.
                ///
                /// # Errors
                /// Returns an error if any of the games pre-flight checks fail.
                fn launch_vessel(craft_dir: String, name: String, launch_site : String, recover: bool) {
                    LaunchVessel(craft_dir, name, launch_site, recover)
                }
            }
            {
                /// Launch a new vessel from the VAB onto the launchpad.
                ///
                /// # Arguments
                /// * `name` - Name of the vessel to launch. This is the name of the ".craft"
                /// file in the save directory, without the ".craft" file extension.
                /// * `recover` - If true and there is a vessel on the launch site, recover it
                /// before launching.
                ///
                /// # Errors
                /// Returns an error if any of the games pre-flight checks fail.
                ///
                /// *Note* This is equivalent to calling `launch_vessel` with the craft directory
                /// set to `"VAB"` and the launch site set to `"LaunchPad"`.
                fn launch_vessel_from_vab(name: String, recover: bool) {
                    LaunchVesselFromVAB(name, recover)
                }
            }
            {
                /// Launch a new vessel from the SPH onto the runway.
                ///
                /// # Arguments
                /// * `name` - Name of the vessel to launch. This is the name of the ".craft"
                /// file in the save directory, without the ".craft" file extension.
                /// * `recover` - If true and there is a vessel on the launch site, recover
                /// it before launching.
                ///
                /// # Errors
                /// Returns an error if any of the games pre-flight checks fail.
                ///
                /// *Note* This is equivalent to calling `launch_vessel` with the craft directory
                /// set to `"SPH"` and the launch site set to `"Runway"`.
                fn launch_vessel_from_sph(name: String, recover: bool) {
                    LaunchVesselFromSPH(name, recover)
                }
            }
        }
    }
);

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