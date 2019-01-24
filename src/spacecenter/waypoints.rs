use super::{CelestialBody, Contract};
use crate::codec::*;
use crate::*;
use crate::krpc::Expression;

use std::collections::BTreeMap;

remote_type!(
/// Waypoints are the location markers you can see on the map view showing you where contracts
/// are targeted for. With this structure, you can obtain coordinate data for the locations of
/// these waypoints. Obtained by calling `SpaceCenter::waypoint_manager()`.
object SpaceCenter.WaypointManager {
    properties: {
        {
            Waypoints: Vec<Waypoint>,
            /// Returns a list of all existing waypoints.
            ///
            /// **Game Scenes**: All
            get: waypoints
        }
        {
            Colors: BTreeMap<String, i32>,
            /// Returns an example map of known color - seed pairs. Any other integers
            /// may be used as seed.
            ///
            /// **Game Scenes**: All
            get: colors
        }
        {
            Icons: Vec<String>,
            /// Returns all available icons (from “GameData/Squad/Contracts/Icons/”).
            ///
            /// **Game Scenes**: All
            get: icons
        }
    }
    methods: {
        {
            /// Creates a waypoint at the given position at ground level, and
            /// returns a `Waypoint` object that can be used to modify it.
            ///
            /// **Game Scenes**: All
            ///
            /// # Arguments
            /// * `latitude` - Longitude of the waypoint.
            /// * `longitude` - Longitude of the waypoint.
            /// * `body` - Celestial body the waypoint is attached to.
            /// * `name` - Name of the waypoint.
            fn add_waypoint(latitude: f64, longitude: f64, body: &CelestialBody,
                name: &str) -> Waypoint {
                AddWaypoint(latitude, longitude, body, name)
            }
        }
        {
            /// Creates a waypoint at the given position and altitude, and
            /// returns a `Waypoint` object that can be used to modify it.
            ///
            /// **Game Scenes**: All
            ///
            /// # Arguments
            /// * `latitude` - Longitude of the waypoint.
            /// * `longitude` - Longitude of the waypoint.
            /// * `altitude` - Altitude (above sea level) of the waypoint.
            /// * `body` - Celestial body the waypoint is attached to.
            /// * `name` - Name of the waypoint.
            fn add_waypoint_at_altitude(latitude: f64, longitude: f64, altitude: f64,
                body: &CelestialBody, name: &str) -> Waypoint {
                AddWaypointAtAltitude(latitude, longitude, altitude, body, name)
            }
        }
    }
});

remote_type!(
/// Represents a waypoint. Can be created using `WaypointManager::add_waypoint()`.
object SpaceCenter.Waypoint {
    properties: {
        {
            Body: CelestialBody,
            /// Returns the celestial body the waypoint is attached to.
            ///
            /// **Game Scenes**: Flight
            get: body,
            /// Sets the celestial body the waypoint is attached to.
            ///
            /// **Game Scenes**: Flight
            set: set_body
        }
        {
            Name: String,
            /// Returns the name of the waypoint as it appears on the map and the contract.
            ///
            /// **Game Scenes**: Flight
            get: name,
            /// Sets the name of the waypoint as it appears on the map and the contract.
            ///
            /// **Game Scenes**: Flight
            set: set_name
        }
        {
            Color: i32,
            /// Returns the seed of the icon color. See `WaypointManager::colors()`
            /// for example colors.
            ///
            /// **Game Scenes**: Flight
            get: color,
            /// Sets the seed of the icon color. See `WaypointManager::colors()`
            /// for example colors.
            ///
            /// **Game Scenes**: Flight
            set: set_color
        }
        {
            Icon: String,
            /// Returns the icon of the waypoint.
            ///
            /// **Game Scenes**: Flight
            get: icon,
            /// Sets the icon of the waypoint.
            ///
            /// **Game Scenes**: Flight
            set: set_icon
        }
        {
            Latitude: f64,
            /// Returns the latitude of the waypoint.
            ///
            /// **Game Scenes**: Flight
            get: latitude,
            /// Sets the latitude of the waypoint.
            ///
            /// **Game Scenes**: Flight
            set: set_latitude
        }
        {
            Longitude: f64,
            /// Returns the longitude of the waypoint.
            ///
            /// **Game Scenes**: Flight
            get: longitude,
            /// Sets the longitude of the waypoint.
            ///
            /// **Game Scenes**: Flight
            set: set_longitude
        }
        {
            MeanAltitude: f64,
            /// Returns the altitude of the waypoint above sea level, in meters.
            ///
            /// **Game Scenes**: Flight
            get: mean_altitude,
            /// Sets the altitude of the waypoint above sea level, in meters.
            ///
            /// **Game Scenes**: Flight
            set: set_mean_altitude
        }
        {
            SurfaceAltitude: f64,
            /// Returns the altitude of the waypoint above the surface of the body or
            /// sea level, whichever is closer, in meters.
            ///
            /// **Game Scenes**: Flight
            get: surface_altitude,
            /// Sets the altitude of the waypoint above the surface of the body or
            /// sea level, whichever is closer, in meters.
            ///
            /// **Game Scenes**: Flight
            set: set_surface_altitude
        }
        {
            BedrockAltitude: f64,
            /// Returns the altitude of the waypoint above the surface of the body, in meters.
            /// When over water, this is the altitude above the sea floor.
            ///
            /// **Game Scenes**: Flight
            get: bedrock_altitude,
            /// Sets the altitude of the waypoint above the surface of the body, in meters.
            /// When over water, this is the altitude above the sea floor.
            ///
            /// **Game Scenes**: Flight
            set: set_bedrock_altitude
        }
        {
            NearSurface: bool,
            /// Returns `true` if the waypoint is near to the surface of a body.
            ///
            /// **Game Scenes**: Flight
            get: is_near_surface
        }
        {
            Grounded: bool,
            /// Returns `true` if the waypoint is attached to the ground.
            ///
            /// **Game Scenes**: Flight
            get: is_grounded
        }
        {
            Index: i32,
            /// Returns the integer index of this waypoint within its cluster of sibling
            /// waypoints. In other words, when you have a cluster of waypoints called
            /// “Somewhere Alpha”, “Somewhere Beta” and “Somewhere Gamma”, the alpha site has
            /// index 0, the beta site has index 1 and the gamma site has index 2.
            /// When `Waypoint::is_clustered()` is `false`, this is zero.
            ///
            /// **Game Scenes**: Flight
            get: index
        }
        {
            Clustered: bool,
            /// Returns `true` if this waypoint is part of a set of clustered waypoints with
            /// greek letter names appended (Alpha, Beta, Gamma, etc). If true, there is a
            /// one-to-one correspondence with the greek letter name and the `Waypoint::index()`.
            ///
            /// **Game Scenes**: Flight
            get: is_clustered
        }
        {
            HasContract: bool,
            /// Returns whether the waypoint belongs to a contract.
            ///
            /// **Game Scenes**: Flight
            get: has_contract
        }
        {
            Contract: Option<Contract>,
            /// Returns the associated contract.
            ///
            /// **Game Scenes**: Flight
            get: contract
        }
    }
    methods: {
        {
            /// Removes the waypoint.
            ///
            /// **Game Scenes**: Flight
            fn remove() {
                Remove()
            }
        }
    }
});
