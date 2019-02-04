use crate::client::{Services, Status};
use crate::codec::{Decode, Encode};
use crate::{remote_type, RemoteEnum};

mod expression;

pub use self::expression::*;

/// Type alias for client information.  First element is client identifier, second is name, and
/// the third is address.
pub type Client = (Vec<u8>, String, String);

remote_type!(
    /// KRPC service.
    service KRPC {
        properties: {
            {
                Clients {
                    /// Returns a list of RPC clients that are currently connected to the server.
                    /// Each entry in the list is a clients identifier, name and address.
                    ///
                    /// **Game Scenes**: All
                    get: clients -> Vec<Client>
                }
            }
            {
                CurrentGameScene {
                    /// Returns the current game scene.
                    ///
                    /// **Game Scenes**: All
                    get: current_game_scene -> GameScene
                }
            }
            {
                Paused {
                    /// Returns whether the game is paused.
                    ///
                    /// **Game Scenes**: All
                    get: is_paused -> bool,
                    /// Sets whether the game is paused.
                    ///
                    /// **Game Scenes**: All
                    set: set_paused(bool)
                }
            }
        }
        methods: {
            {
                /// Returns the identifier for the current client.
                ///
                /// **Game Scenes**: All
                fn client_id() -> Vec<u8> {
                    GetClientID()
                }
            }
            {
                /// Returns the name of the current client. This is an empty string if the
                /// client has no name.
                ///
                /// **Game Scenes**: All
                fn client_name() -> String {
                    GetClientName()
                }
            }
            {
                /// Returns some information about the server, such as the version.
                ///
                /// **Game Scenes**: All
                fn status() -> Status {
                    GetStatus()
                }
            }
            {
                /// Returns information on all services, procedures, classes, properties etc.
                /// provided by the server. Can be used by client libraries to automatically
                /// create functionality such as stubs.
                ///
                /// **Game Scenes**: All
                fn services() -> Services {
                    GetServices()
                }
            }
        }
    }
);

remote_type!(
    /// The game scene. See `KRPC::current_game_scene()`.
    enum GameScene {
        /// The game scene showing the Kerbal Space Center buildings.
        SpaceCenter = 0,
        /// The game scene showing a vessel in flight (or on the launchpad/runway).
        Flight = 1,
        /// The tracking station.
        TrackingStation = 2,
        /// The Vehicle Assembly Building.
        EditorVAB = 3,
        /// The Space Plane Hangar.
        EditorSPH = 4,
    }
);
