use crate::*;
use crate::codec::*;
use crate::units::{Degrees, Radians, RadiansPerSecond, Vector3, Quaternion, GravitationalParameter};
use super::{Orbit, ReferenceFrame};

use std::rc::{Rc};
use std::cell::{RefCell};
use std::collections::HashSet;

use uom::si::quantities::{Length, Mass, Time, Acceleration, Density, Pressure, ThermodynamicTemperature};

remote_type!(
    /// Represents a celestial body (such as a planet or moon).
    object CelestialBody {}
);

impl CelestialBody {
    rpc_method!(
    /// Returns the name of the body.
    ///
    /// **Game Scenes**: All
    fn name(&self) -> String {
        SpaceCenter.CelestialBody_get_Name(self).ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// Returns a list of celestial bodies that are in orbit around this celestial body.
    ///
    /// **Game Scenes**: All
    fn satellites(&self) -> Vec<CelestialBody> {
        SpaceCenter.CelestialBody_get_Satellites(self).ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// Returns the orbit of the body.
    ///
    /// **Game Scenes**: All
    fn orbit(&self) -> Orbit {
        SpaceCenter.CelestialBody_get_Orbit(self).ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// Returns the mass of the body, in kilograms.
    ///
    /// **Game Scenes**: All
    fn mass(&self) -> Mass<f32> {
        SpaceCenter.CelestialBody_get_Mass(self).ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// Returns the [standard gravitational parameter](https://en.wikipedia.org/wiki/Standard_gravitational_parameter)
    /// of the body in m<sup>3</sup>/s<sup>2</sup>.
    ///
    /// **Game Scenes**: All
    fn gravitational_parameter(&self) -> GravitationalParameter<f32> {
        SpaceCenter.CelestialBody_get_GravitationalParameter(self).ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// Returns the acceleration due to gravity at sea level (mean altitude) on the body,
    /// in m/s<sup>2</sup>.
    ///
    /// **Game Scenes**: All
    fn surface_gravity(&self) -> Acceleration<f32> {
        SpaceCenter.CelestialBody_get_Mass(self).ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// Returns the sidereal rotational period of the body, in seconds.
    ///
    /// **Game Scenes**: All
    fn rotational_period(&self) -> Time<f32> {
        SpaceCenter.CelestialBody_get_RotationalPeriod(self).ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// Returns the rotational speed of the body, in radians per second.
    ///
    /// **Game Scenes**: All
    fn rotational_speed(&self) -> RadiansPerSecond<f32> {
        SpaceCenter.CelestialBody_get_RotationalSpeed(self).ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// Returns the current rotation angle of the body, in radians.
    ///
    /// **Game Scenes**: All
    fn rotation_angle(&self) -> Radians<f32> {
        SpaceCenter.CelestialBody_get_RotationAngle(self).ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// Returns the initial rotation angle of the body (at UT 0), in radians.
    ///
    /// **Game Scenes**: All
    fn initial_rotation(&self) -> Radians<f32> {
        SpaceCenter.CelestialBody_get_InitialRotation(self).ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// Returns the equatorial radius of the body, in meters.
    ///
    /// **Game Scenes**: All
    fn equatorial_radius(&self) -> Length<f32> {
        SpaceCenter.CelestialBody_get_EquatorialRadius(self).ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// Returns the height of the surface relative to mean sea level, in meters, at the
    /// given position. When over water this is equal to 0.
    ///
    /// **Game Scenes**: All
    ///
    /// # Arguments
    /// * `latitude` - Latitude in degrees.
    /// * `longitude` – Longitude in degrees.
    fn surface_height(&self, latitude: Degrees<f64>, longitude: Degrees<f64>) -> Length<f64> {
        SpaceCenter.CelestialBody_SurfaceHeight(self, latitude, longitude)
            .ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// Returns the height of the surface relative to mean sea level, in meters, at the given
    /// position. When over water, this is the height of the sea-bed and is therefore
    /// a negative value.
    ///
    /// # Arguments
    /// * `latitude` - Latitude in degrees.
    /// * `longitude` – Longitude in degrees.
    fn bedrock_height(&self, latitude: Degrees<f64>, longitude: Degrees<f64>) -> Length<f64> {
        SpaceCenter.CelestialBody_BedrockHeight(self, latitude, longitude)
            .ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// Returns the position at mean sea level at the given latitude and longitude, in the
    /// given reference frame.
    ///
    /// **Game Scenes**: All
    ///
    /// # Arguments
    /// * `latitude` - Latitude in degrees.
    /// * `longitude` - Longitude in degrees.
    /// * `reference_frame` - Reference frame for the returned position vector.
    fn mean_sea_level_position(&self, latitude: Degrees<f64>, longitude: Degrees<f64>,
            reference_frame: &ReferenceFrame) -> Vector3 {
        SpaceCenter.CelestialBody_MSLPosition(self,
            latitude,
            longitude,
            reference_frame).ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// Returns the position of the surface at the given latitude and longitude, in the given
    /// reference frame. When over water, this is the position of the surface of the water.
    ///
    /// **Game Scenes**: All
    ///
    /// # Arguments
    /// * `latitude` - Latitude in degrees.
    /// * `longitude` - Longitude in degrees.
    /// * `reference_frame` - Reference frame for the returned position vector.
    fn surface_position(&self, latitude: Degrees<f64>, longitude: Degrees<f64>,
            reference_frame: &ReferenceFrame) -> Vector3 {
        SpaceCenter.CelestialBody_SurfacePosition(self,
            latitude,
            longitude,
            reference_frame).ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// Returns the position of the surface at the given latitude and longitude, in the given
    /// reference frame. When over water, this is the position of the sea-bed.
    ///
    /// **Game Scenes**: All
    ///
    /// # Arguments
    /// * `latitude` - Latitude in degrees.
    /// * `longitude` - Longitude in degrees.
    /// * `reference_frame` - Reference frame for the returned position vector.
    fn bedrock_position(&self, latitude: Degrees<f64>, longitude: Degrees<f64>,
            reference_frame: &ReferenceFrame) -> Vector3 {
        SpaceCenter.CelestialBody_BedrockPosition(self,
            latitude,
            longitude,
            reference_frame).ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
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
    fn position_at_altitude(&self, latitude: Degrees<f64>, longitude: Degrees<f64>,
            altitude: Length<f64>, reference_frame: &ReferenceFrame) -> Vector3 {
        SpaceCenter.CelestialBody_PositionAtAltitude(self,
            latitude,
            longitude,
            altitude,
            reference_frame).ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// Returns the altitude, in meters, of the given position in the given reference frame.
    ///
    /// **Game Scenes**: All
    ///
    /// # Arguments
    /// * `position` - Position as a vector.
    /// * `reference_frame` - Reference frame for the position vector.
    fn altitude_at_position(&self, position: Vector3, reference_frame: &ReferenceFrame) -> Length<f64> {
        SpaceCenter.CelestialBody_AltitudeAtPosition(self,
            position,
            reference_frame).ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// Returns the latitude of the given position, in the given reference frame.
    ///
    /// **Game Scenes**: All
    ///
    /// # Arguments
    /// * `position` - Position as a vector.
    /// * `reference_frame` - Reference frame for the position vector.
    fn latitude_at_position(&self, position: Vector3, reference_frame: &ReferenceFrame) -> Degrees<f64> {
        SpaceCenter.CelestialBody_LatitudeAtPosition(self,
            position,
            reference_frame).ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// Returns the longitude of the given position, in the given reference frame.
    ///
    /// **Game Scenes**: All
    ///
    /// # Arguments
    /// * `position` - Position as a vector.
    /// * `reference_frame` - Reference frame for the position vector.
    fn longitude_at_position(&self, position: Vector3, reference_frame: &ReferenceFrame) -> Degrees<f64> {
        SpaceCenter.CelestialBody_LongitudeAtPosition(self,
            position,
            reference_frame).ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// Returns the radius of the sphere of influence of the body, in meters.
    ///
    /// **Game Scenes**: All
    fn sphere_of_influence(&self) -> Length<f32> {
        SpaceCenter.CelestialBody_get_SphereOfInfluence(self).ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// Returns `true` if the body has an atmosphere.
    ///
    /// **Game Scenes**: All
    fn has_atmosphere(&self) -> bool {
        SpaceCenter.CelestialBody_get_HasAtmosphere(self).ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// Returns the depth of the atmosphere, in meters.
    ///
    /// **Game Scenes**: All
    fn atmosphere_depth(&self) -> Length<f32> {
        SpaceCenter.CelestialBody_get_AtmosphereDepth(self).ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// Returns the atmospheric density at the given position, in kg/m<sup>3</sup>,
    /// in the given reference frame.
    ///
    /// **Game Scenes**: All
    ///
    /// # Arguments
    /// * `position` - Position as a vector.
    /// * `reference_frame` - Reference frame for the position vector.
    fn atmospheric_density_at_position(&self, position: Vector3, reference_frame: &ReferenceFrame) -> Density<f64> {
        SpaceCenter.CelestialBody_AtmosphericDensityAtPosition(self,
            position,
            reference_frame).ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// Returns `true` if there is oxygen in the atmosphere, required for air-breathing engines.
    fn has_atmospheric_oxygen(&self) -> bool {
        SpaceCenter.CelestialBody_get_HasAtmosphericOxygen(self).ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// Returns the temperature on the body at the given position, in the given reference frame.
    ///
    /// **Game Scenes**: All
    ///
    /// # Arguments
    /// * `position` - Position as a vector.
    /// * `reference_frame` - Reference frame for the position vector.
    ///
    /// # Note
    /// This calculation is performed using the bodies current position, which means that
    /// the value could be wrong if you want to know the temperature in the far future.
    fn temperature_at(&self, position: Vector3, reference_frame: &ReferenceFrame) -> ThermodynamicTemperature<f64> {
        SpaceCenter.CelestialBody_TemperatureAt(self,
            position,
            reference_frame).ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// Returns the air density, in kg/m<sup>3</sup>, for the specified altitude above sea level,
    /// in meters.
    ///
    /// **Game Scenes**: All
    ///
    /// # Arguments
    /// * `altitude` - The altitude above sea level, in meters.
    ///
    /// # Note
    /// This is an approximation, because actual calculations, taking sun exposure into account to
    /// compute air temperature, require us to know the exact point on the body where the density
    /// is to be computed (knowing the altitude is not enough). However, the difference is small
    /// for high altitudes, so it makes very little difference for trajectory prediction.
    fn density_at(&self, altitude: Length<f64>) -> Density<f64> {
        SpaceCenter.CelestialBody_DensityAt(self, altitude).ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// Returns the air pressure, in Pascals, for the specified altitude above sea level, in meters.
    ///
    /// **Game Scenes**: All
    ///
    /// # Arguments
    /// * `altitude` - The altitude above sea level, in meters.
    fn pressure_at(&self, altitude: Length<f64>) -> Pressure<f64> {
        SpaceCenter.CelestialBody_PressureAt(self, altitude).ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// Returns the biomes present on this body.
    ///
    /// **Game Scenes**: All
    fn biomes(&self, altitude: Length<f64>) -> HashSet<String> {
        SpaceCenter.CelestialBody_get_Biomes(self, altitude).ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// Returns the biome at the given latitude and longitude, in degrees.
    ///
    /// **Game Scenes**: All
    ///
    /// # Arguments
    /// * `latitude` - Latitude in degrees.
    /// * `longitude` – Longitude in degrees.
    fn biome_at(&self, latitude: Degrees<f64>, longitude: Degrees<f64>) -> String {
        SpaceCenter.CelestialBody_BiomeAt(self, latitude, longitude).ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// Returns the altitude, in meters, above which a vessel is considered to be flying “high”
    /// when doing science.
    ///
    /// **Game Scenes**: All
    fn flying_high_altitude_threshold(&self) -> Length<f32> {
        SpaceCenter.CelestialBody_get_FlyingHighAltitudeThreshold(self).ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// Returns the altitude, in meters, above which a vessel is considered to be in “high”
    /// space when doing science.
    ///
    /// **Game Scenes**: All
    fn space_high_altitude_threshold(&self) -> Length<f32> {
        SpaceCenter.CelestialBody_get_SpaceHighAltitudeThreshold(self).ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
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
    /// *Celestial body reference frame origin and axes. The equator is shown in blue, and
    /// the prime meridian in red.*
    fn reference_frame(&self) -> ReferenceFrame {
        SpaceCenter.CelestialBody_get_ReferenceFrame(self).ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// Returns the reference frame that is fixed relative to this celestial body, and orientated
    /// in a fixed direction (it does not rotate with the body).
    ///
    /// * The origin is at the center of the body.
    /// * The axes do not rotate.
    /// * The x-axis points in an arbitrary direction through the equator.
    /// * The y-axis points from the center of the body towards the north pole.
    /// * The z-axis points in an arbitrary direction through the equator.
    ///
    /// **Game Scenes**: All
    fn non_rotating_reference_frame(&self) -> ReferenceFrame {
        SpaceCenter.CelestialBody_get_NonRotatingReferenceFrame(self)
            .ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// Returns the reference frame that is fixed relative to this celestial body, but orientated
    /// with the body’s orbital prograde/normal/radial directions.
    ///
    /// * The origin is at the center of the body.
    /// * The axes rotate with the orbital prograde/normal/radial directions.
    /// * The x-axis points in the orbital anti-radial direction.
    /// * The y-axis points in the orbital prograde direction.
    /// * The z-axis points in the orbital normal direction.
    fn orbital_reference_frame(&self) -> ReferenceFrame {
        SpaceCenter.CelestialBody_get_OrbitalReferenceFrame(self)
            .ok_or(KrpcError::NullResponseValue)
    });


    rpc_method!(
    /// Returns the position of the center of the body, in the specified reference frame.
    ///
    /// **Game Scenes**: All
    ///
    /// # Arguments
    /// * `reference_frame` - The reference frame that the returned position vector is in.
    ///
    /// # Return
    /// The position as a vector.
    fn position(&self, reference_frame: &ReferenceFrame) -> Vector3 {
        SpaceCenter.CelestialBody_Position(self, reference_frame)
            .ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// Returns the linear velocity of the body, in the specified reference frame.
    ///
    /// **Game Scenes**: All
    ///
    /// # Arguments
    /// * `reference_frame` - The reference frame that the returned velocity vector is in.
    ///
    /// # Return
    /// The velocity as a vector. The vector points in the direction of travel, and its magnitude
    /// is the speed of the body in meters per second.
    fn velocity(&self, reference_frame: &ReferenceFrame) -> Vector3 {
        SpaceCenter.CelestialBody_Velocity(self, reference_frame)
            .ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// Returns the rotation of the body, in the specified reference frame.
    ///
    /// **Game Scenes**: All
    ///
    /// # Arguments
    /// * `reference_frame` - The reference frame that the returned rotation is in.
    ///
    /// # Return
    /// The rotation as a quaternion of the form (x,y,z,w).
    fn rotation(&self, reference_frame: &ReferenceFrame) -> Quaternion {
        SpaceCenter.CelestialBody_Rotation(self, reference_frame)
            .ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// Returns the direction in which the north pole of the celestial body is pointing,
    /// in the specified reference frame.
    ///
    /// # Arguments
    /// * `reference_frame` - The reference frame that the returned direction vector is in.
    ///
    /// # Return
    /// The direction as a unit vector.
    fn direction(&self, reference_frame: &ReferenceFrame) -> Vector3 {
        SpaceCenter.CelestialBody_Direction(self, reference_frame)
            .ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// Returns the angular velocity of the body in the specified reference frame.
    /// right-hand rule.
    ///
    /// # Arguments
    /// * `reference_frame` - The reference frame that the returned angular velocity vector is in.
    ///
    /// # Return
    /// The angular velocity as a vector. The magnitude of the vector is the rotational speed of
    /// the body, in radians per second. The direction of the vector indicates the axis of
    /// rotation, using the right-hand rule.
    fn angular_velocity(&self, reference_frame: &ReferenceFrame) -> Vector3 {
        SpaceCenter.CelestialBody_AngularVelocity(self, reference_frame)
            .ok_or(KrpcError::NullResponseValue)
    });
}