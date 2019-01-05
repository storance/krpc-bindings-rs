use crate::*;
use crate::codec::*;

use std::rc::{Rc};
use std::cell::{RefCell};

remote_type!(
/// Used to get flight telemetry for a vessel, by calling `Vessel::flight()`. All of
/// the information returned by this class is given in the reference frame passed to that method.
object Flight {
    service: SpaceCenter,
    properties: {
        {
            GForce: f32,
            /// Returns the current G force acting on the vessel in *g*.
            ///
            /// **Game Scenes**: Flight
            get: g_force
        }

        {
            MeanAltitude: f64,
            /// Returns the altitude above sea level, in meters. Measured from the center of
            /// mass of the vessel.
            ///
            /// **Game Scenes**: Flight
            get: mean_altitude
        }

        {
            SurfaceAltitude: f64,
            /// Returns the altitude above the surface of the body or sea level, whichever
            /// is closer, in meters. Measured from the center of mass of the vessel.
            ///
            /// **Game Scenes**: Flight
            get: surface_altitude
        }

        {
            BedrockAltitude: f64,
            /// Returns the altitude above the surface of the body, in meters. When over water,
            /// this is the altitude above the sea floor. Measured from the center of mass of
            /// the vessel.
            ///
            /// **Game Scenes**: Flight
            get: bedrock_altitude
        }

        {
            Elevation: f64,
            /// Returns the elevation of the terrain under the vessel, in meters. This is the
            /// height of the terrain above sea level, and is negative when the vessel is
            /// over the sea.
            ///
            /// **Game Scenes**: Flight
            get: elevation
        }

        {
            Latitude: f64,
            /// Returns the [latitude](https://en.wikipedia.org/wiki/Latitude) of the vessel for
            /// the body being orbited, in degrees.
            ///
            /// **Game Scenes**: Flight
            get: latitude
        }

        {
            Longitude: f64,
            /// Returns the [longitude](https://en.wikipedia.org/wiki/Longitude) of the vessel for
            /// the body being orbited, in degrees.
            ///
            /// **Game Scenes**: Flight
            get: longitude
        }
        {
            Velocity: Vector3,
            /// Returns the velocity of the vessel, in the reference frame `ReferenceFrame`.
            ///
            /// **Game Scenes**: Flight
            ///
            /// # Return
            /// The velocity as a vector. The vector points in the direction of travel, and
            /// its magnitude is the speed of the vessel in meters per second.
            get: velocity
        }
        {
            Speed: f64,
            /// Returns the speed of the vessel in meters per second, in the reference
            /// frame `ReferenceFrame`.
            ///
            /// **Game Scenes**: Flight
            get: speed
        }
        {
            HorizontalSpeed: f64,
            /// Returns the horizontal speed of the vessel in meters per second, in the reference
            /// frame `ReferenceFrame`.
            ///
            /// **Game Scenes**: Flight
            get: horizontal_speed
        }
        {
            VerticalSpeed: f64,
            /// Returns the vertical speed of the vessel in meters per second, in the reference
            /// frame `ReferenceFrame`.
            ///
            /// **Game Scenes**: Flight
            get: vertical_speed
        }

        {
            CenterOfMass: Vector3,
            /// Returns the position of the center of mass of the vessel, in the
            /// reference frame `ReferenceFrame`.
            ///
            /// **Game Scenes**: Flight
            ///
            /// # Return
            /// The position as a vector.
            get: center_of_mass
        }
    }

});
