use super::{Orbit, ReferenceFrame};
use crate::codec::*;
use crate::*;
use crate::krpc::Expression;

use std::collections::HashSet;

remote_type!(
    /// Represents a celestial body (such as a planet or moon).
    object SpaceCenter.CelestialBody {
        properties: {
            {
                Name: String,
                /// Returns the name of the body.
                ///
                /// **Game Scenes**: All
                get: name
            }
            {
                Satellites: Vec<CelestialBody>,
                /// Returns a list of celestial bodies that are in orbit around this celestial body.
                ///
                /// **Game Scenes**: All
                get: satellites
            }
            {
                Orbit: Orbit,
                /// Returns the orbit of the body.
                ///
                /// **Game Scenes**: All
                get: orbit
            }
            {
                Mass: f32,
                /// Returns the mass of the body, in kilograms.
                ///
                /// **Game Scenes**: All
                get: mass
            }
            {
                GravitationalParameter: f32,
                /// Returns the [standard gravitational parameter](https://en.wikipedia.org/wiki/Standard_gravitational_parameter)
                /// of the body in m<sup>3</sup>/s<sup>2</sup>.
                ///
                /// **Game Scenes**: All
                get: gravitational_parameter
            }
            {
                SurfaceGravity: f32,
                /// Returns the acceleration due to gravity at sea level (mean altitude) on the
                /// body, in m/s<sup>2</sup>.
                ///
                /// **Game Scenes**: All
                get: surface_gravity
            }
            {
                RotationalPeriod: f32,
                /// Returns the sidereal rotational period of the body, in seconds.
                ///
                /// **Game Scenes**: All
                get: rotational_period
            }
            {
                RotationalSpeed: f32,
                /// Returns the rotational speed of the body, in radians per second.
                ///
                /// **Game Scenes**: All
                get: rotational_speed
            }
            {
                RotationalAngle: f32,
                /// Returns the current rotation angle of the body, in radians.
                ///
                /// **Game Scenes**: All
                get: rotational_angle
            }
            {
                InitialRotation: f32,
                /// Returns the initial rotation angle of the body (at UT 0), in radians.
                ///
                /// **Game Scenes**: All
                get: initial_rotation
            }
            {
                EquatorialRadius: f32,
                /// Returns the equatorial radius of the body, in meters.
                ///
                /// **Game Scenes**: All
                get: equatorial_radius
            }
            {
                SphereOfInfluence: f32,
                /// Returns the radius of the sphere of influence of the body, in meters.
                ///
                /// **Game Scenes**: All
                get: sphere_of_influence
            }
            {
                HasAtmosphere: bool,
                /// Returns `true` if the body has an atmosphere.
                ///
                /// **Game Scenes**: All
                get: has_atmosphere
            }
            {
                AtmosphereDepth: f32,
                /// Returns the depth of the atmosphere, in meters.
                ///
                /// **Game Scenes**: All
                get: atmosphere_depth
            }
            {
                Biomes: HashSet<String>,
                /// Returns the biomes present on this body.
                ///
                /// **Game Scenes**: All
                get: biomes
            }
            {
                FlyingHighAltitudeThreshold: f32,
                /// Returns the altitude, in meters, above which a vessel is considered to be
                /// flying “high” when doing science.
                ///
                /// **Game Scenes**: All
                get: flying_high_altitude_threshold
            }
            {
                SpaceHighAltitudeThreshold: f32,
                /// Returns the altitude, in meters, above which a vessel is considered to be
                /// in “high” space when doing science.
                ///
                /// **Game Scenes**: All
                get: space_high_altitude_threshold
            }
            {
                ReferenceFrame: ReferenceFrame,
                /// Returns the reference frame that is fixed relative to the celestial body.
                ///
                /// * The origin is at the center of the body.
                /// * The axes rotate with the body.
                /// * The x-axis points from the center of the body towards the intersection of the prime
                /// meridian and equator (the position at 0° longitude, 0° latitude).
                /// * The y-axis points from the center of the body towards the north pole.
                /// * The z-axis points from the center of the body towards the equator at 90°E longitude.
                ///
                /// **Game Scenes**: All
                ///
                /// ![Reference frame](https://krpc.github.io/krpc/_images/celestial-body.png)
                ///
                /// *Celestial body reference frame origin and axes. The equator is shown in
                /// blue, and the prime meridian in red.*
                get: reference_frame
            }
            {
                NonRotatingReferenceFrame: ReferenceFrame,
                /// Returns the reference frame that is fixed relative to this celestial body,
                /// and orientated in a fixed direction (it does not rotate with the body).
                ///
                /// * The origin is at the center of the body.
                /// * The axes do not rotate.
                /// * The x-axis points in an arbitrary direction through the equator.
                /// * The y-axis points from the center of the body towards the north pole.
                /// * The z-axis points in an arbitrary direction through the equator.
                ///
                /// **Game Scenes**: All
                get: non_rotating_reference_frame
            }
            {
                OrbitalReferenceFrame: ReferenceFrame,
                /// Returns the reference frame that is fixed relative to this celestial body,
                /// but orientated with the body’s orbital prograde/normal/radial directions.
                ///
                /// * The origin is at the center of the body.
                /// * The axes rotate with the orbital prograde/normal/radial directions.
                /// * The x-axis points in the orbital anti-radial direction.
                /// * The y-axis points in the orbital prograde direction.
                /// * The z-axis points in the orbital normal direction.
                ///
                /// **Game Scenes**: All
                get: orbital_reference_frame
            }
            {
                HasAtmosphericOxygen: bool,
                /// Returns `true` if there is oxygen in the atmosphere, required for
                /// air-breathing engines.
                ///
                /// **Game Scenes**: All
                get: has_atmospheric_oxygen
            }
        }
        methods: {
            {
                /// Returns the height of the surface relative to mean sea level, in meters, at the
                /// given position. When over water this is equal to 0.
                ///
                /// **Game Scenes**: All
                ///
                /// # Arguments
                /// * `latitude` - Latitude in degrees.
                /// * `longitude` – Longitude in degrees.
                fn surface_height(latitude: f64, longitude: f64) -> f64 {
                    SurfaceHeight(latitude, longitude)
                }
            }
            {
                /// Returns the height of the surface relative to mean sea level, in meters, at
                /// the given position. When over water, this is the height of the sea-bed and
                /// is therefore a negative value.
                ///
                /// # Arguments
                /// * `latitude` - Latitude in degrees.
                /// * `longitude` – Longitude in degrees.
                fn bedrock_height(latitude: f64, longitude: f64) -> f64 {
                    BedrockHeight(latitude, longitude)
                }
            }
            {
                /// Returns the position at mean sea level at the given latitude and longitude,
                /// in the given reference frame.
                ///
                /// **Game Scenes**: All
                ///
                /// # Arguments
                /// * `latitude` - Latitude in degrees.
                /// * `longitude` - Longitude in degrees.
                /// * `reference_frame` - Reference frame for the returned position vector.
                fn mean_sea_level_position(latitude: f64, longitude: f64,
                    reference_frame: &ReferenceFrame) -> Vector3 {
                    MSLPosition(latitude, longitude, reference_frame)
                }
            }
            {
                /// Returns the position of the surface at the given latitude and longitude, in
                /// the given reference frame. When over water, this is the position of the
                /// surface of the water.
                ///
                /// **Game Scenes**: All
                ///
                /// # Arguments
                /// * `latitude` - Latitude in degrees.
                /// * `longitude` - Longitude in degrees.
                /// * `reference_frame` - Reference frame for the returned position vector.
                fn surface_position(latitude: f64, longitude: f64,
                        reference_frame: &ReferenceFrame) -> Vector3 {
                    SurfacePosition(latitude, longitude, reference_frame)
                }
            }
            {
                /// Returns the position of the surface at the given latitude and longitude, in
                /// the given reference frame. When over water, this is the position of the
                /// surface of the sea-bed.
                ///
                /// **Game Scenes**: All
                ///
                /// # Arguments
                /// * `latitude` - Latitude in degrees.
                /// * `longitude` - Longitude in degrees.
                /// * `reference_frame` - Reference frame for the returned position vector.
                fn bedrock_position(latitude: f64, longitude: f64,
                        reference_frame: &ReferenceFrame) -> Vector3 {
                    BedrockPosition(latitude, longitude, reference_frame)
                }
            }
            {
                /// Returns the position at the given latitude, longitude and altitude, in the
                /// given reference frame.
                ///
                /// **Game Scenes**: All
                ///
                /// # Arguments
                /// * `latitude` - Latitude in degrees.
                /// * `longitude` - Longitude in degrees.
                /// * `altitude` - Altitude in meters above sea level.
                /// * `reference_frame` - Reference frame for the returned position vector.
                fn position_at_altitude(latitude: f64, longitude: f64,
                        altitude: f64, reference_frame: &ReferenceFrame) -> Vector3 {
                    PositionAtAltitude(latitude, longitude, altitude, reference_frame)
                }
            }
            {
                /// Returns the altitude, in meters, of the given position in the given
                /// reference frame.
                ///
                /// **Game Scenes**: All
                ///
                /// # Arguments
                /// * `position` - Position as a vector.
                /// * `reference_frame` - Reference frame for the position vector.
                fn altitude_at_position(position: Vector3, reference_frame: &ReferenceFrame) -> f64 {
                    AltitudeAtPosition(position, reference_frame)
                }
            }
            {
                /// Returns the latitude of the given position, in the given reference frame.
                ///
                /// **Game Scenes**: All
                ///
                /// # Arguments
                /// * `position` - Position as a vector.
                /// * `reference_frame` - Reference frame for the position vector.
                fn latitude_at_position(position: Vector3, reference_frame: &ReferenceFrame) -> f64 {
                    LatitudeAtPosition(position, reference_frame)
                }
            }
            {
                /// Returns the longitude of the given position, in the given reference frame.
                ///
                /// **Game Scenes**: All
                ///
                /// # Arguments
                /// * `position` - Position as a vector.
                /// * `reference_frame` - Reference frame for the position vector.
                fn longitude_at_position(position: Vector3, reference_frame: &ReferenceFrame) -> f64 {
                    LongitudeAtPosition(position, reference_frame)
                }
            }
            {
                /// Returns the atmospheric density at the given position, in kg/m<sup>3</sup>,
                /// in the given reference frame.
                ///
                /// **Game Scenes**: All
                ///
                /// # Arguments
                /// * `position` - Position as a vector.
                /// * `reference_frame` - Reference frame for the position vector.
                fn atmospheric_density_at_position(position: Vector3, reference_frame: &ReferenceFrame) -> f64 {
                    AtmosphericDensityAtPosition(position, reference_frame)
                }
            }
            {
                /// Returns the temperature on the body at the given position, in the
                /// given reference frame.
                ///
                /// **Game Scenes**: All
                ///
                /// # Arguments
                /// * `position` - Position as a vector.
                /// * `reference_frame` - Reference frame for the position vector.
                ///
                /// # Note
                /// This calculation is performed using the bodies current position, which means
                /// that the value could be wrong if you want to know the temperature in the
                /// far future.
                fn temperature_at(position: Vector3, reference_frame: &ReferenceFrame) -> f64 {
                    TemperatureAt(position, reference_frame)
                }
            }
            {
                /// Returns the air density, in kg/m<sup>3</sup>, for the specified altitude above sea level,
                /// in meters.
                ///
                /// **Game Scenes**: All
                ///
                /// # Arguments
                /// * `altitude` - The altitude above sea level, in meters.
                ///
                /// # Note
                /// This is an approximation, because actual calculations, taking sun exposure
                /// into account to compute air temperature, require us to know the exact point
                /// on the body where the density is to be computed (knowing the altitude is
                /// not enough). However, the difference is small for high altitudes, so it
                /// makes very little difference for trajectory prediction.
                fn density_at(altitude: f64) -> f64 {
                    DensityAt(altitude)
                }
            }
            {
                /// Returns the air pressure, in Pascals, for the specified altitude above sea level, in meters.
                ///
                /// **Game Scenes**: All
                ///
                /// # Arguments
                /// * `altitude` - The altitude above sea level, in meters.
                fn pressure_at(altitude: f64) -> f64 {
                    PressureAt(altitude)
                }
            }
            {
                /// Returns the biome at the given latitude and longitude, in degrees.
                ///
                /// **Game Scenes**: All
                ///
                /// # Arguments
                /// * `latitude` - Latitude in degrees.
                /// * `longitude` – Longitude in degrees.
                fn biome_at(latitude: f64, longitude: f64) -> String {
                    BiomeAt(latitude, longitude)
                }
            }
            {
                /// Returns the position of the center of the body, in the specified reference frame.
                ///
                /// **Game Scenes**: All
                ///
                /// # Arguments
                /// * `reference_frame` - The reference frame that the returned position
                /// vector is in.
                ///
                /// # Return
                /// The position as a vector.
                fn position(reference_frame: &ReferenceFrame) -> Vector3 {
                    Position(reference_frame)
                }
            }
            {
                /// Returns the linear velocity of the body, in the specified reference frame.
                ///
                /// **Game Scenes**: All
                ///
                /// # Arguments
                /// * `reference_frame` - The reference frame that the returned velocity
                /// vector is in.
                ///
                /// # Return
                /// The velocity as a vector. The vector points in the direction of travel,
                /// and its magnitude is the speed of the body in meters per second.
                fn velocity(reference_frame: &ReferenceFrame) -> Vector3 {
                    Velocity(reference_frame)
                }
            }
            {
                /// Returns the rotation of the body, in the specified reference frame.
                ///
                /// **Game Scenes**: All
                ///
                /// # Arguments
                /// * `reference_frame` - The reference frame that the returned rotation is in.
                ///
                /// # Return
                /// The rotation as a quaternion of the form (x,y,z,w).
                fn rotation(reference_frame: &ReferenceFrame) -> Quaternion {
                    Rotation(reference_frame)
                }
            }
            {
                /// Returns the direction in which the north pole of the celestial body is pointing,
                /// in the specified reference frame.
                ///
                /// # Arguments
                /// * `reference_frame` - The reference frame that the returned direction vector
                /// is in.
                ///
                /// # Return
                /// The direction as a unit vector.
                fn direction(reference_frame: &ReferenceFrame) -> Vector3 {
                    Direction(reference_frame)
                }
            }
            {
                /// Returns the angular velocity of the body in the specified reference frame.
                /// right-hand rule.
                ///
                /// # Arguments
                /// * `reference_frame` - The reference frame that the returned angular velocity
                /// vector is in.
                ///
                /// # Return
                /// The angular velocity as a vector. The magnitude of the vector is the
                /// rotational speed of the body, in radians per second. The direction of the
                /// vector indicates the axis of rotation, using the right-hand rule.
                fn angular_velocity(reference_frame: &ReferenceFrame) -> Vector3 {
                    AngularVelocity(reference_frame)
                }
            }
        }
    }
);
