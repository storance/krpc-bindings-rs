use crate::codec::*;
use crate::*;

use std::collections::BTreeMap;
use std::rc::Rc;

mod autopilot;
mod body;
mod camera;
mod communication;
mod contracts;
mod control;
mod flight;
mod node;
mod orbit;
mod parts;
mod reference_frame;
mod resources;
mod vessel;
mod waypoints;

pub use self::autopilot::*;
pub use self::body::*;
pub use self::camera::*;
pub use self::communication::*;
pub use self::contracts::*;
pub use self::control::*;
pub use self::flight::*;
pub use self::node::*;
pub use self::orbit::*;
pub use self::parts::*;
pub use self::reference_frame::*;
pub use self::resources::*;
pub use self::vessel::*;
pub use self::waypoints::*;

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
                ///
                /// **Game Scenes**: All
                get: science
            }
            {
                Funds: f64,
                /// Returns the current amount of funds.
                ///
                /// **Game Scenes**: All
                get: funds
            }
            {
                Reputation: f32,
                /// Returns the current amount of reputation.
                ///
                /// **Game Scenes**: All
                get: reputation
            }
            {
                ActiveVessel: Option<Vessel>,
                /// Returns the currently active vessel or None if there isn't one.
                ///
                /// **Game Scenes**: All
                get: active_vessel,
                /// Sets the active vessel to the given vessel.
                ///
                /// **Game Scenes**: All
                ///
                /// # Arguments
                /// * `value` - The vessel to make active.
                set: set_active_vessel
            }
            {
                Vessels: Vec<Vessel>,
                /// Returns a list of all the vessels in the game.
                ///
                /// **Game Scenes**: All
                get: vessels
            }
            {
                Bodies: BTreeMap<String, CelestialBody>,
                /// A map of all celestial bodies (planets, moons, etc.) in the game,
                /// keyed by the name of the body.
                ///
                /// **Game Scenes**: All
                get: bodies
            }
            {
                TargetBody: Option<CelestialBody>,
                /// Returns the currently targeted celestial body or `None` if there isn't one.
                ///
                /// **Game Scenes**: All
                get: target_body,
                /// Sets the current target to the given `body`.
                ///
                /// **Game Scenes**: All
                ///
                /// # Arguments
                /// * `value` - The celestial body to target.
                set: set_target_body
            }
            {
                TargetVessel: Option<Vessel>,
                /// Returns the currently targeted vessel or `None` if there isn't one.
                ///
                /// **Game Scenes**: All
                get: target_vessel,
                /// Sets the current target to the given vessel.
                ///
                /// **Game Scenes**: All
                ///
                /// # Arguments
                /// * `value` - The vessel to target.
                set: set_target_vessel
            }
            {
                TargetDockingPort: Option<DockingPort>,
                /// Returns the currently targeted docking port or `None` if there isn't one.
                ///
                /// **Game Scenes**: All
                get: target_docking_port,
                /// Sets the current target to the given docking port.
                ///
                /// **Game Scenes**: All
                ///
                /// # Arguments
                /// * `value` - The docking port to target.
                set: set_target_docking_port
            }
            {
                UIVisible: bool,
                /// Returns whether the UI is visible.
                ///
                /// **Game Scenes**: Flight
                get: is_ui_visible,
                /// Sets whether the UI is visible.
                ///
                /// **Game Scenes**: Flight
                set: set_ui_visible
            }
            {
                Navball: bool,
                /// Returns whether the navball is visible.
                ///
                /// **Game Scenes**: Flight
                get: is_navball_visible,
                /// Sets whether the navball is visible.
                ///
                /// **Game Scenes**: Flight
                set: set_navball_visible
            }
            {
                G: f64,
                /// Returns the value of the gravitational constant G in N(m/kg)<sup>2</sup>.
                ///
                /// **Game Scenes**: All
                get: g
            }
            {
                WarpRate: f32,
                /// Returns the current warp rate. This is the rate at which time is passing
                /// for either on-rails or physical time warp. For example, a value of 10 means
                /// time is passing 10x faster than normal. Returns 1 if time warp is not active.
                ///
                /// **Game Scenes**: Flight
                get: warp_rate
            }
            {
                WarpFactor: f32,
                /// Returns the current warp factor. This is the index of the rate at which
                /// time is passing for either regular “on-rails” or physical time warp. Returns
                /// 0 if time warp is not active. When in on-rails time warp, this is equal to
                /// `rails_warp_factor()`, and in physics time warp, this is
                /// equal to `physics_warp_factor()`.
                ///
                /// **Game Scenes**: Flight
                get: warp_factor
            }
            {
                RailsWarpFactor: f32,
                /// Returns the time warp rate, using regular “on-rails” time warp. A value
                /// between 0 and 7 inclusive. Returns 0 if no time warp or physical time
                /// warp is active.
                ///
                /// **Game Scenes**: Flight
                get: rails_warp_factor,
                /// Sets the time warp rate, using regular “on-rails” time warp. A value
                /// between 0 and 7 inclusive. 0 means no time warp.
                ///
                /// If requested time warp factor cannot be set, it will be set to the next
                /// lowest possible value. For example, if the vessel is too close to a
                /// planet. See the [KSP wiki](https://wiki.kerbalspaceprogram.com/wiki/Time_warp)
                /// for details.
                ///
                /// **Game Scenes**: Flight
                set: set_rails_warp_factor
            }
            {
                PhysicsWarpFactor: f32,
                /// Returns the physical time warp rate. A value between 0 and 3 inclusive.
                /// Returns 0 if no time warp or regular “on-rails” time warp is active.
                ///
                /// **Game Scenes**: Flight
                get: physics_warp_factor,
                /// Sets the physical time warp rate. A value between 0 and 3 inclusive.
                /// 0 means no time warp.
                ///
                /// **Game Scenes**: Flight
                set: set_physics_warp_factor
            }
            {
                MaximumRailsWarpFactor: f32,
                /// Returns the current maximum regular “on-rails” warp factor that can be set.
                /// A value between 0 and 7 inclusive. See the
                /// [KSP wiki](https://wiki.kerbalspaceprogram.com/wiki/Time_warp) for details.
                ///
                /// **Game Scenes**: Flight
                get: max_rails_warp_factor
            }
            {
                FARAvailable: bool,
                /// Returns whether Ferram Aerospace Research is installed.
                ///
                /// **Game Scenes**: All
                get: is_far_available
            }
            {
                GameMode: GameMode,
                /// Returns the current mode the game is in.
                ///
                /// **Game Scenes**: All
                get: game_mode
            }
            {
                WarpMode: WarpMode,
                /// Returns the current time warp mode. Returns `WarpMode::NONE` if time warp
                /// is not active, `WarpMode::RAILS` if regular “on-rails” time warp is active,
                /// or `WarpMode::PHYSICS` if physical time warp is active.
                ///
                /// **Game Scenes**: Flight
                get: warp_mode
            }
            {
                Camera: Camera,
                /// Returns an object that can be used to control the camera.
                ///
                /// **Game Scenes**: Flight
                get: camera
            }
            {
                WaypointManager: WaypointManager,
                /// Returns the waypoint manager.
                ///
                /// **Game Scenes**: Flight
                get: waypoint_manager
            }
            {
                ContractManager: ContractManager,
                /// Returns the contract manager.
                ///
                /// **Game Scenes**: All
                get: contract_manager
            }
        }
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
                /// **Game Scenes**: All
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
                /// **Game Scenes**: All
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
                /// **Game Scenes**: All
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
                /// **Game Scenes**: All
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
            {
                /// Save the game with a given name. This will create a save file called
                /// `name.sfs` in the folder of the current save game.
                ///
                /// **Game Scenes**: All
                ///
                /// # Arguments
                /// * `name` - Name of the save to create.
                fn save(name: String) {
                    Save(name)
                }
            }
            {
                /// Load the game with the given name. This will create a load a save file called
                /// `name.sfs` from the folder of the current save game.
                ///
                /// **Game Scenes**: All
                ///
                /// # Arguments
                /// * `name` - Name of the save to load.
                fn load(name: String) {
                    Load(name)
                }
            }

            {
                /// Save a quicksave.
                ///
                /// **Game Scenes**: All
                ///
                /// # Note
                /// This is the same as calling `save()` with the name `“quicksave”`.
                fn quicksave() {
                    Quicksave()
                }
            }
            {
                /// Load a quicksave.
                ///
                /// **Game Scenes**: All
                ///
                /// # Note
                /// This is the same as calling `load()` with the name `“quicksave”`.
                fn quickload() {
                    Quickload()
                }
            }
            {
                /// Returns `true` if regular “on-rails” time warp can be used, at the
                /// specified warp factor. The maximum time warp rate is limited by various
                /// things, including how close the active vessel is to a planet.
                ///
                /// **Game Scenes**: Flight
                ///
                /// # Arguments:
                /// * `factor` - The warp factor to check.
                fn can_rails_warp_at(factor: i32) -> bool {
                    CanRailsWarpAt(factor)
                }
            }
            {
                /// Uses time acceleration to warp forward to a time in the future, specified
                /// by universal time `ut`. This call blocks until the desired time is reached.
                /// Uses regular “on-rails” or physical time warp as appropriate. For example,
                /// physical time warp is used when the active vessel is traveling through an
                /// atmosphere. When using regular “on-rails” time warp, the warp rate is
                /// limited by `max_rails_rate`, and when using physical time warp, the warp
                /// rate is limited by `max_physics_rate`.
                ///
                /// **Game Scenes**: Flight
                ///
                /// # Arguments:
                /// * `ut` - The universal time to warp to, in seconds.
                /// * `max_rails_rate` - The maximum warp rate in regular “on-rails” time warp.
                /// * `max_physics_rate` - The maximum warp rate in physical time warp.
                ///
                /// # Returns
                /// When the warp is complete.
                fn warp_to(ut: f64, max_rails_rate: i32, max_physics_rate: i32) {
                    WarpTO(ut, max_rails_rate, max_physics_rate)
                }
            }
            {
                /// Converts a position from one reference frame to another.
                ///
                /// **Game Scenes**: All
                ///
                /// # Arguments
                /// * `position` – Position, as a vector, in reference frame `from`.
                /// * `from` – The reference frame that the position is in.
                /// * `to`  – The reference frame to covert the position to.
                ///
                /// # Returns
                /// The corresponding position, as a vector, in reference frame `to`.
                fn transform_position(position: Vector3, from: ReferenceFrame, to: ReferenceFrame) -> Vector3 {
                    TransformPosition(position, from, to)
                }
            }
            {
                /// Converts a direction from one reference frame to another.
                ///
                /// **Game Scenes**: All
                ///
                /// # Arguments
                /// * `direction` – Direction, as a vector, in reference frame `from`.
                /// * `from` – The reference frame that the direction is in.
                /// * `to`  – The reference frame to covert the direction to.
                ///
                /// # Returns
                /// The corresponding direction, as a vector, in reference frame `to`.
                fn transform_direction(direction: Vector3, from: ReferenceFrame, to: ReferenceFrame) -> Vector3 {
                    TransformDirection(direction, from, to)
                }
            }
            {
                /// Converts a rotation from one reference frame to another.
                ///
                /// **Game Scenes**: All
                ///
                /// # Arguments
                /// * `rotation` – Rotation, as a quaternion of the form (*x*,*y*,*z*,*w*),
                /// in reference frame `from`.
                /// * `from` – The reference frame that the rotation is in.
                /// * `to`  – The reference frame to covert the rotation to.
                ///
                /// # Returns
                /// The corresponding rotation, as a quaternion of the form  (*x*,*y*,*z*,*w*),
                /// in reference frame `to`.
                fn transform_rotation(rotation: Quaternion, from: ReferenceFrame, to: ReferenceFrame) -> Quaternion {
                    TransformRotation(rotation, from, to)
                }
            }
            {
                /// Converts a velocity (acting at the specified position) from one reference
                /// frame to another. The position is required to take the relative angular
                /// velocity of the reference frames into account.
                ///
                /// **Game Scenes**: All
                ///
                /// # Arguments
                /// * `position` – Position, as a vector, in reference frame `from`.
                /// * `velocity` – Velocity, as a vector that points in the direction of travel
                /// and whose magnitude is the speed in meters per second, in reference
                /// frame `from`.
                /// * `from` – The reference frame that the position is in.
                /// * `to`  – The reference frame to covert the position to.
                ///
                /// # Returns
                /// The corresponding velocity, as a vector, in reference frame to.
                fn transform_velocity(position: Vector3, velocity: Vector3, from: ReferenceFrame, to: ReferenceFrame) -> Vector3 {
                    TransformVelocity(position, velocity, from, to)
                }
            }
            {
                /// Cast a ray from a given position in a given direction, and return the
                /// distance to the hit point. If no hit occurs, returns infinity.
                ///
                /// **Game Scenes**: All
                ///
                /// # Arguments
                /// * `position` - Position, as a vector, of the origin of the ray.
                /// * `direction` – Direction of the ray, as a unit vector.
                /// * `reference_frame` - The reference frame that the position and direction are in.
                ///
                /// # Returns
                /// The distance to the hit, in meters, or infinity if there was no hit.
                fn raycast_distance(position: Vector3, direction: Vector3, reference_frame: ReferenceFrame) -> f64 {
                    RaycastDistance(position, direction, reference_frame)
                }
            }
            {
                /// Cast a ray from a given position in a given direction, and return the part
                /// that it hits. If no hit occurs, returns `None`.
                ///
                /// **Game Scenes**: Flight
                ///
                /// # Arguments
                /// * `position` - Position, as a vector, of the origin of the ray.
                /// * `direction` – Direction of the ray, as a unit vector.
                /// * `reference_frame` - The reference frame that the position and direction are in.
                ///
                /// # Returns
                /// The part that was hit or `None` if there was no hit.
                fn raycast_part(position: Vector3, direction: Vector3, reference_frame: ReferenceFrame) -> Option<Part> {
                    RaycastPart(position, direction, reference_frame)
                }
            }
        }
    }
);

remote_type!(
    /// The game mode.
    enum GameMode {
        /// Sandbox mode.
        Sandbox = 0,
        /// Career mode.
        Career = 1,
        /// Science career mode.
        Science = 2,
        /// Science sandbox mode.
        ScienceSandbox = 3,
        /// Mission mode.
        Mission = 4,
        /// Mission builder mode.
        MissionBuilder = 5,
        /// Scenario mode.
        Scenario = 6,
        /// Scenario mode that cannot be resumed.
        ScenarioNonResumable = 7,
    }
);

remote_type!(
    /// The time warp mode.
    enum WarpMode {
        /// Time warp is active, and in regular "on-rails" mode.
        Rails = 0,
        /// Time warp is active, and in physical time warp mode.
        Physics = 1,
        /// Time warp is not active.
        None = 2,
    }
);
