use super::CelestialBody;
use crate::codec::*;
use crate::*;
use crate::krpc::Expression;

remote_type!(
/// Used to get flight telemetry for a vessel, by calling `Vessel::flight()`. All of
/// the information returned by this class is given in the reference frame passed to that method.
///
/// # Note
/// To get orbital information, such as the apoapsis or inclination, see `Orbit`.
object SpaceCenter.Flight {
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
        {
            Rotation: Quaternion,
            /// Returns the rotation of the vessel, in the reference frame `ReferenceFrame`
            ///
            /// **Game Scenes**: Flight
            ///
            /// # Return
            /// The rotation as a quaternion of the form (*x*,*y*,*z*,*w*).
            get: rotation
        }
        {
            Direction: Vector3,
            /// Returns the direction that the vessel is pointing in, in the
            /// reference frame `ReferenceFrame`.
            ///
            /// **Game Scenes**: Flight
            ///
            /// # Return
            /// The direction as a unit vector.
            get: direction
        }
        {
            Pitch: f32,
            /// Returns the pitch of the vessel relative to the horizon, in degrees.
            /// A value between -90° and +90°.
            ///
            /// **Game Scenes**: Flight
            get: pitch
        }
        {
            Heading: f32,
            /// Returns the heading of the vessel (its angle relative to north), in degrees.
            /// A value between 0° and 360°.
            ///
            /// **Game Scenes**: Flight
            get: heading
        }
        {
            Roll: f32,
            /// Returns the roll of the vessel relative to the horizon, in degrees.
            /// A value between -180° and +180°.
            ///
            /// **Game Scenes**: Flight
            get: roll
        }
        {
            Prograde: Vector3,
            /// Returns the prograde direction of the vessels orbit, in the
            /// reference frame `ReferenceFrame`.
            ///
            /// **Game Scenes**: Flight
            ///
            /// # Return
            /// The direction as a unit vector.
            get: prograde
        }
        {
            Retrograde: Vector3,
            /// Returns the retrograde direction of the vessels orbit, in the
            /// reference frame `ReferenceFrame`.
            ///
            /// **Game Scenes**: Flight
            ///
            /// # Return
            /// The direction as a unit vector.
            get: retrograde
        }

        {
            Normal: Vector3,
            /// Returns the normal direction of the vessels orbit, in the
            /// reference frame `ReferenceFrame`.
            ///
            /// **Game Scenes**: Flight
            ///
            /// # Return
            /// The direction as a unit vector.
            get: normal
        }
        {
            AntiNormal: Vector3,
            /// Returns the direction opposite to the normal of the vessels orbit, in the
            /// reference frame `ReferenceFrame`.
            ///
            /// **Game Scenes**: Flight
            ///
            /// # Return
            /// The direction as a unit vector.
            get: anti_normal
        }
        {
            Radial: Vector3,
            /// Returns the radial direction of the vessels orbit, in the
            /// reference frame `ReferenceFrame`.
            ///
            /// **Game Scenes**: Flight
            ///
            /// # Return
            /// The direction as a unit vector.
            get: radial
        }
        {
            AntiRadial: Vector3,
            /// Returns the direction opposite to the radial of the vessels orbit, in the
            /// reference frame `ReferenceFrame`.
            ///
            /// **Game Scenes**: Flight
            ///
            /// # Return
            /// The direction as a unit vector.
            get: anti_radial
        }
        {
            AtmosphereDensity: f32,
            /// Returns the current density of the atmosphere around the vessel,
            /// in kg/m<sup>3</sup>.
            ///
            /// **Game Scenes**: Flight
            get: atmosphere_density
        }
        {
            DynamicPressure: f32,
            /// Returns the dynamic pressure acting on the vessel, in Pascals. This is a
            /// measure of the strength of the aerodynamic forces. It is equal to
            /// `1/2 · air density · velocity<sup>2</sup>`. It is commonly denoted Q.
            ///
            /// **Game Scenes**: Flight
            get: dynamic_pressure
        }
        {
            StaticPressure: f32,
            /// Returns the static atmospheric pressure acting on the vessel, in Pascals.
            ///
            /// **Game Scenes**: Flight
            get: static_pressure
        }
        {
            StaticPressureAtMSL: f32,
            /// Returns the static pressure at mean sea level, in Pascals.
            ///
            /// **Game Scenes**: Flight
            get: static_pressure_at_msl
        }
        {
            AerodynamicForce: Vector3,
            /// Returns the total aerodynamic forces acting on the vessel,
            /// in reference frame `ReferenceFrame`.
            ///
            /// **Game Scenes**: Flight
            ///
            /// # Return
            /// A vector pointing in the direction that the force acts, with its magnitude
            /// equal to the strength of the force in Newtons.
            get: aerodynamic_force
        }
        {
            Lift: Vector3,
            /// Returns the [aerodynamic lift](https://en.wikipedia.org/wiki/Aerodynamic_force)
            /// currently acting on the vessel.
            ///
            /// **Game Scenes**: Flight
            ///
            /// # Return
            /// A vector pointing in the direction that the force acts, with its magnitude
            /// equal to the strength of the force in Newtons.
            get: lift
        }
        {
            Drag: Vector3,
            /// Returns the [aerodynamic drag](https://en.wikipedia.org/wiki/Aerodynamic_force)
            /// currently acting on the vessel.
            ///
            /// **Game Scenes**: Flight
            ///
            /// # Return
            /// A vector pointing in the direction that the force acts, with its magnitude
            /// equal to the strength of the force in Newtons.
            get: drag
        }
        {
            SpeedOfSound: f32,
            /// Returns the speed of sound, in the atmosphere around the vessel, in m/s.
            ///
            /// **Game Scenes**: Flight
            get: speed_of_sound
        }
        {
            Mach: f32,
            /// Returns the speed of the vessel, in multiples of the speed of sound.
            ///
            /// **Game Scenes**: Flight
            get: mach
        }
        {
            ReynoldsNumber: f32,
            /// Returns the vessels Reynolds number.
            ///
            /// **Game Scenes**: Flight
            ///
            /// # Note
            /// Requires Ferram Aerospace Research.
            get: reynolds_number
        }
        {
            TrueAirSpeed: f32,
            /// Returns the [true air speed](https://en.wikipedia.org/wiki/True_airspeed)
            /// of the vessel, in meters per second.
            ///
            /// **Game Scenes**: Flight
            get: true_air_speed
        }
        {
            EquivalentAirSpeed: f32,
            /// Returns the [equivalent air speed](https://en.wikipedia.org/wiki/Equivalent_airspeed)
            /// of the vessel, in meters per second.
            ///
            /// **Game Scenes**: Flight
            get: equivalent_air_speed
        }
        {
            TerminalVelocity: f32,
            /// Returns an estimate of the current terminal velocity of the vessel,
            /// in meters per second. This is the speed at which the drag forces cancel
            /// out the force of gravity.
            ///
            /// **Game Scenes**: Flight
            get: terminal_velocity
        }
        {
            AngleOfAttack: f32,
            /// Returns the pitch angle between the orientation of the vessel and its
            /// velocity vector, in degrees..
            ///
            /// **Game Scenes**: Flight
            get: angle_of_attack
        }
        {
            SideslipAngle: f32,
            /// Returns the yaw angle between the orientation of the vessel and its
            /// velocity vector, in degrees.
            ///
            /// **Game Scenes**: Flight
            get: sideslip_angle
        }
        {
            TotalAirTemperature: f32,
            /// Returns the [total air temperature](https://en.wikipedia.org/wiki/Total_air_temperature)
            /// of the atmosphere around the vessel, in Kelvin. This includes the
            /// `Flight::static_air_temperature()` and the vessel’s kinetic energy.
            ///
            /// **Game Scenes**: Flight
            get: total_air_temperature
        }
        {
            StaticAirTemperature: f32,
            /// Returns the [static (ambient) temperature](https://en.wikipedia.org/wiki/Total_air_temperature)
            /// of the atmosphere around the vessel, in Kelvin.
            ///
            /// **Game Scenes**: Flight
            get: static_air_temperature
        }
        {
            StallFraction: f32,
            /// Returns the current amount of stall, between 0 and 1. A value greater than
            /// 0.005 indicates a minor stall and a value greater than 0.5 indicates
            /// a large-scale stall.
            ///
            /// **Game Scenes**: Flight
            ///
            /// # Note
            /// Requires Ferram Aerospace Research.
            get: stall_fraction
        }
        {
            DragCoefficient: f32,
            /// Returns the coefficient of drag. This is the amount of drag produced by the
            /// vessel. It depends on air speed, air density and wing area.
            ///
            /// **Game Scenes**: Flight
            ///
            /// # Note
            /// Requires Ferram Aerospace Research.
            get: drag_coefficient
        }
        {
            LiftCoefficient: f32,
            /// Returns the coefficient of lift. This is the amount of lift produced by the
            /// vessel. It depends on air speed, air density and wing area.
            ///
            /// **Game Scenes**: Flight
            ///
            /// # Note
            /// Requires Ferram Aerospace Research.
            get: lift_coefficient
        }
        {
            BallisticCoefficient: f32,
            /// Returns the [ballistic coefficient](https://en.wikipedia.org/wiki/Ballistic_coefficient).
            ///
            /// **Game Scenes**: Flight
            ///
            /// # Note
            /// Requires Ferram Aerospace Research.
            get: ballistic_coefficient
        }
        {
            ThrustSpecificFuelConsumption: f32,
            /// Returns the efficiency of the engines, with a lower value indicating a
            /// more efficient vessel. This value is the number of Newtons of fuel that
            /// are burned, per hour, to produce one newton of thrust.
            ///
            /// **Game Scenes**: Flight
            ///
            /// # Note
            /// Requires Ferram Aerospace Research.
            get: thrust_specific_fuel_consumption
        }
    }
    methods: {
        {
            /// Simulate and return the total aerodynamic forces acting on the vessel,
            /// if it where to be traveling with the given velocity at the given position
            /// in the atmosphere of the given celestial body.
            ///
            /// **Game Scenes**: Flight
            ///
            /// # Arguments
            /// * `body` - The celestial body.
            /// * `position` - The vessel's position as a vector on the body.
            /// * `velocity` - The vessel's velocity as a vector on the body.
            ///
            /// # Return
            /// A vector pointing in the direction that the force acts, with its magnitude equal
            /// to the strength of the force in Newtons.
            fn simulate_aerodynamic_force_at(body: &CelestialBody, position: Vector3, velocity: Vector3) -> Vector3 {
                SimulateAerodynamicForceAt(body, position, velocity)
            }
        }
    }
});
