use crate::*;
use crate::codec::*;

use std::rc::{Rc};
use std::cell::{RefCell};

remote_type!(
/// Used to get flight telemetry for a vessel, by calling `Vessel::flight()`. All of
/// the information returned by this class is given in the reference frame passed to that method.
object Flight {}
);

impl Flight {
    rpc_property!(
        GForce: f32 {
            service: SpaceCenter,
            class: Flight,
            /// Returns the current G force acting on the vessel in *g*.
            ///
            /// **Game Scenes**: Flight
            g_force
        }
    );

    rpc_property!(
        MeanAltitude: f64 {
            service: SpaceCenter,
            class: Flight,
            /// Returns the altitude above sea level, in meters. Measured from the center of
            /// mass of the vessel.
            ///
            /// **Game Scenes**: Flight
            mean_altitude
        }
    );

    rpc_property!(
        SurfaceAltitude: f64 {
            service: SpaceCenter,
            class: Flight,
            /// Returns the altitude above the surface of the body or sea level, whichever
            /// is closer, in meters. Measured from the center of mass of the vessel.
            ///
            /// **Game Scenes**: Flight
            surface_altitude
        }
    );

    rpc_property!(
        BedrockAltitude: f64{
            service: SpaceCenter,
            class: Flight,
            /// Returns the altitude above the surface of the body, in meters. When over water,
            /// this is the altitude above the sea floor. Measured from the center of mass of
            /// the vessel.
            ///
            /// **Game Scenes**: Flight
            bedrock_altitude
        }
    );

    rpc_property!(
        Elevation: f64 {
            service: SpaceCenter,
            class: Flight,
            /// Returns the elevation of the terrain under the vessel, in meters. This is the
            /// height of the terrain above sea level, and is negative when the vessel is
            /// over the sea.
            ///
            /// **Game Scenes**: Flight
            elevation
        }
    );

    rpc_property!(
        Latitude: f64 {
            service: SpaceCenter,
            class: Flight,
            /// Returns the [latitude](https://en.wikipedia.org/wiki/Latitude) of the vessel for
            /// the body being orbited, in degrees.
            ///
            /// **Game Scenes**: Flight
            latitude
        }
    );

    rpc_property!(
        Longitude: f64 {
            service: SpaceCenter,
            class: Flight,
            /// Returns the [longitude](https://en.wikipedia.org/wiki/Longitude) of the vessel for
            /// the body being orbited, in degrees.
            ///
            /// **Game Scenes**: Flight
            longitude
        }
    );

    rpc_property!(
        Velocity: Vector3 {
            service: SpaceCenter,
            class: Flight,
            /// Returns the velocity of the vessel, in the reference frame `ReferenceFrame`.
            ///
            /// **Game Scenes**: Flight
            ///
            /// # Return
            /// The velocity as a vector. The vector points in the direction of travel, and
            /// its magnitude is the speed of the vessel in meters per second.
            velocity
        }
    );

    rpc_property!(
        Speed: f64 {
            service: SpaceCenter,
            class: Flight,
            /// Returns the speed of the vessel in meters per second, in the reference
            /// frame `ReferenceFrame`.
            ///
            /// **Game Scenes**: Flight
            speed
        }
    );

    rpc_property!(
        HorizontalSpeed: f64 {
            service: SpaceCenter,
            class: Flight,
            /// Returns the horizontal speed of the vessel in meters per second, in the reference
            /// frame `ReferenceFrame`.
            ///
            /// **Game Scenes**: Flight
            horizontal_speed
        }
    );

    rpc_property!(
        VerticalSpeed: f64 {
            service: SpaceCenter,
            class: Flight,
            /// Returns the vertical speed of the vessel in meters per second, in the reference
            /// frame `ReferenceFrame`.
            ///
            /// **Game Scenes**: Flight
            vertical_speed
        }
    );

    rpc_property!(
        CenterOfMass: Vector3 {
            service: SpaceCenter,
            class: Flight,
            /// Returns the position of the center of mass of the vessel, in the
            /// reference frame `ReferenceFrame`.
            ///
            /// **Game Scenes**: Flight
            ///
            /// # Return
            /// The position as a vector.
            center_of_mass
        }
    );
}