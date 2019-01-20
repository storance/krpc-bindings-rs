use crate::client::schema::{Services, Status};
use crate::codec::*;
use crate::*;

use std::rc::Rc;

remote_type!(
    /// KRPC service.
    service KRPC {
        properties: {
            {
                ClientID: Vec<u8>,
                /// Returns the identifier for the current client.
                ///
                /// **Game Scenes**: All
                get: client_id
            }
            {
                ClientName: String,
                /// Returns the name of the current client. This is an empty string if the
                /// client has no name.
                ///
                /// **Game Scenes**: All
                get: client_name
            }
            {
                Clients: Vec<(Vec<u8>, String, String)>,
                /// Returns a list of RPC clients that are currently connected to the server.
                /// Each entry in the list is a clients identifier, name and address.
                ///
                /// **Game Scenes**: All
                get: clients
            }
            {
                Status: Status,
                /// Returns some information about the server, such as the version.
                ///
                /// **Game Scenes**: All
                get: status
            }
            {
                Services: Services,
                /// Returns information on all services, procedures, classes, properties etc.
                /// provided by the server. Can be used by client libraries to automatically
                /// create functionality such as stubs.
                ///
                /// **Game Scenes**: All
                get: services
            }
            {
                CurrentGameScene: GameScene,
                /// Returns the current game scene.
                ///
                /// **Game Scenes**: All
                get: current_game_scene
            }
            {
                Paused: bool,
                /// Returns whether the game is paused.
                ///
                /// **Game Scenes**: All
                get: is_paused,
                /// Sets whether the game is paused.
                ///
                /// **Game Scenes**: All
                set: set_paused
            }
        }
        methods: {}
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