use crate::*;
use crate::codec::*;
use super::{CelestialBody, ReferenceFrame};

use std::rc::Rc;

remote_type!(
/// Describes an orbit. For example, the orbit of a vessel or a celestial body.
object SpaceCenter.Orbit {
    properties: {
        {
            Body: CelestialBody,
            /// Returns the celestial body (e.g. planet or moon) around which the object is orbiting.
            ///
            /// **Game Scenes**: All
            get: body
        }
        {
            Apoapsis: f64,
            /// Gets the apoapsis of the orbit, in meters, from the center of mass of the body
            /// being orbited.
            ///
            /// **Game Scenes**: All
            ///
            /// # Note
            /// For the apoapsis altitude reported on the in-game map view, use `apoapsis_altitude()`.
            get: apoapsis
        }
        {
            Periapsis: f64,
            /// Gets the periapsis of the orbit, in meters, from the center of mass of the body
            /// being orbited.
            ///
            /// **Game Scenes**: All
            ///
            /// # Note
            /// For the periapsis altitude reported on the in-game map view, use `periapsis_altitude()`.
            get: periapsis
        }

        {
            ApoapsisAltitude: f64,
            /// The apoapsis of the orbit, in meters, above the sea level of the body being orbited.
            ///
            /// **Game Scenes**: All
            ///
            /// # Note
            /// This is equal to `apoapsis()` minus the equatorial radius of the body.
            get: apoapsis_altitude
        }
        {
            PeriapsisAltitude: f64,
            /// The periapsis of the orbit, in meters, above the sea level of the body being orbited.
            ///
            /// **Game Scenes**: All
            ///
            /// # Note
            /// This is equal to `periapsis()` minus the equatorial radius of the body.
            get: periapsis_altitude
        }
        {
            SemiMajorAxis: f64,
            /// Returns the semi-major axis of the orbit, in meters.
            ///
            /// **Game Scenes**: All
            get: semi_major_axis
        }
        {
            SemiMinorAxis: f64,
            /// Returns the semi-minor axis of the orbit, in meters.
            ///
            /// **Game Scenes**: All
            get: semi_minor_axis
        }
        {
            Radius: f64,
            /// The current radius of the orbit, in meters. This is the distance between
            /// the center of mass of the object in orbit, and the center of mass of the body
            /// around which it is orbiting.
            ///
            /// **Game Scenes**: All
            ///
            /// # Note
            /// This value will change over time if the orbit is elliptical.
            get: radius
        }
        {
            Speed: f64,
            /// The current orbital speed of the object in meters per second.
            ///
            /// **Game Scenes**: All
            ///
            /// # Note
            /// This value will change over time if the orbit is elliptical.
            get: speed
        }
        {
            TimeToApoapsis: f64,
            /// Returns the time until the object reaches apoapsis, in seconds.
            ///
            /// **Game Scenes**: All
            get: time_to_apoapsis
        }
        {
            TimeToPeriapsis: f64,
            /// Returns the time until the object reaches periapsis, in seconds.
            ///
            /// **Game Scenes**: All
            get: time_to_periapsis
        }
        {
            Eccentricity: f64,
            /// Returns the [eccentricity](https://en.wikipedia.org/wiki/Orbital_eccentricity) of
            /// the orbit.
            ///
            /// **Game Scenes**: All
            get: eccentricity
        }
        {
            Inclination: f64,
            /// Returns the [inclination](https://en.wikipedia.org/wiki/Orbital_inclination) of
            /// the orbit, in radians.
            ///
            /// **Game Scenes**: All
            get: inclination
        }

        {
            LongitudeOfAscendingNode: f64,
            /// Returns the [longitude of the ascending node](https://en.wikipedia.org/wiki/Longitude_of_the_ascending_node),
            /// in radians.
            ///
            /// **Game Scenes**: All
            get: longitude_of_ascending_node
        }
        {
            ArgumentOfPeriapsis: f64,
            /// Returns the [argument of periapsis](https://en.wikipedia.org/wiki/Argument_of_periapsis),
            /// in radians.
            ///
            /// **Game Scenes**: All
            get: argument_of_periapsis
        }
        {
            MeanAnomalyAtEpoch: f64,
            /// Returns the [mean anomaly at epoch](https://en.wikipedia.org/wiki/Mean_anomaly).
            ///
            /// **Game Scenes**: All
            get: mean_anomaly_at_epoch
        }
        {
            Epoch: f64,
            /// Returns the time since the epoch (the point at which the
            /// [mean anomaly at epoch](https://en.wikipedia.org/wiki/Mean_anomaly) was
            /// measured), in seconds.
            ///
            /// **Game Scenes**: All
            get: epoch
        }
        {
            MeanAnomaly: f64,
            /// Returns the [mean anomaly](https://en.wikipedia.org/wiki/Mean_anomaly).
            ///
            /// **Game Scenes**: All
            get: mean_anomaly
        }
        {
            EccentricAnomaly: f64,
            /// Returns the [eccentric anomaly](https://en.wikipedia.org/wiki/Eccentric_anomaly).
            ///
            /// **Game Scenes**: All
            get: eccentric_anomaly
        }
        {
            TrueAnomaly: f64,
            /// Returns the [true anomaly](https://en.wikipedia.org/wiki/True_anomaly).
            ///
            /// **Game Scenes**: All
            get: true_anomaly
        }
        {
            OrbitalSpeed: f64,
            /// Returns the current orbital speed in meters per second.
            ///
            /// **Game Scenes**: All
            get: orbital_speed
        }
        {
            TimeToSOIChange: f64,
            /// Returns the time until the object changes sphere of influence, in seconds or `NaN`
            /// if the object is not going to change sphere of influence.
            ///
            /// **Game Scenes**: All
            get: time_to_soi_change
        }
    }
    methods: {
        {
            /// The orbital radius, in meters, at the given time.
            ///
            /// **Game Scenes**: All
            ///
            /// # Arguments
            /// * `ut` - The universal time to measure the radius at.
            fn radius_at(ut: f64) -> f64 {
                RadiusAt(ut)
            }
        }
        {
            /// The position as a vector at a given time, in the specified reference frame.
            ///
            /// **Game Scenes**: All
            ///
            /// # Arguments
            /// * `ut` - The universal time to measure the position at.
            /// * `reference_frame` - The reference frame that the returned position vector is in.
            fn position_at(ut: f64, reference_frame: &ReferenceFrame) -> Vector3 {
                PositionAt(ut, reference_frame)
            }
        }
        {
            /// Returns the mean anomaly at the given time.
            ///
            /// **Game Scenes**: All
            ///
            /// # Arguments
            /// * `ut` - The universal time in seconds.
            fn mean_anomaly_at_ut(ut: f64) -> f64 {
                MeanAnomalyAtUT(ut)
            }
        }
        {
            /// Returns the eccentric anomaly at the given time.
            ///
            /// **Game Scenes**: All
            ///
            /// # Arguments
            /// * `ut` - The universal time in seconds.
            fn eccentric_anomaly_at_ut(ut: f64) -> f64 {
                EccentricAnomalyAtUT(ut)
            }
        }
        {
            /// Returns the true anomaly at the given time.
            ///
            /// **Game Scenes**: All
            ///
            /// # Arguments
            /// * `ut` - The universal time in seconds.
            fn true_anomaly_at_ut(ut: f64) -> f64 {
                TrueAnomalyAtUT(ut)
            }
        }
        {
            /// Returns the true anomaly at the given orbital radius.
            ///
            /// **Game Scenes**: All
            ///
            /// # Arguments
            /// * `radius` - The orbital radius in meters.
            fn true_anomaly_at_radius(radius: f64) -> f64 {
                TrueAnomalyAtRadius(radius)
            }
        }
        {
            /// Returns the universal time, in seconds, corresponding to the given true anomaly.
            ///
            /// **Game Scenes**: All
            ///
            /// # Arguments
            /// * `true_anomaly` - True anomaly.
            fn ut_at_true_anomaly(true_anomaly: f64) -> f64 {
                UTAtTrueAnomaly(true_anomaly)
            }
        }
        {
            /// Returns the orbital radius at the point in the orbit given by the true anomaly.
            ///
            /// **Game Scenes**: All
            ///
            /// # Arguments
            /// * `true_anomaly` - True anomaly.
            fn radius_at_true_anomaly(true_anomaly: f64) -> f64 {
                RadiusAtTrueAnomaly(true_anomaly)
            }
        }
        {
            /// Return the true anomaly of the ascending node with the given target orbit.
            ///
            /// **Game Scenes**: All
            ///
            /// # Arguments
            /// * `target` - Target orbit.
            fn true_anomaly_at_an() -> f64 {
                TrueAnomalyAtAN()
            }
        }
        {
            /// Return the true anomaly of the descending node with the given target orbit.
            ///
            /// **Game Scenes**: All
            ///
            /// # Arguments
            /// * `target` - Target orbit.
            fn true_anomaly_at_dn() -> f64 {
                TrueAnomalyAtDN()
            }
        }
        {
            /// The orbital speed at the given time, in meters per second.
            ///
            /// **Game Scenes**: All
            ///
            /// # Arguments
            /// * `time` - f64 from now, in seconds.
            fn orbital_speed_at(time: f64) -> f64 {
                OrbitalSpeedAt(time)
            }
        }
        {
            /// Returns the relative inclination of &self orbit and the target orbit, in radians.
            ///
            /// **Game Scenes**: All
            ///
            /// # Arguments
            /// * `target` - Target orbit.
            fn relative_inclincation(target: &Orbit) -> f64 {
                RelativeInclination(target)
            }
        }
        {
            /// Estimates and returns the universal time, in seconds, at closest approach to a target orbit.
            ///
            /// **Game Scenes**: All
            ///
            /// # Arguments
            /// * `target` - Target orbit.
            fn time_of_closest_approach(target: &Orbit) -> f64 {
                TimeOfClosestApproach(target)
            }
        }
        {
            /// Estimates and returns the distance at closest approach to a target orbit, in meters.
            ///
            /// **Game Scenes**: All
            ///
            /// # Arguments
            /// * `target` - Target orbit.
            fn distance_at_closest_approach(target: &Orbit) -> f64 {
                DistanceAtClosestApproach(target)
            }
        }
        {
            ///  Returns the times at closest approach and corresponding distances, to a target orbit.
            ///
            /// **Game Scenes**: All
            ///
            /// # Arguments
            /// * `target` - Target orbit.
            /// * `orbits` - The number of future orbits to search.
            ///
            /// # Return
            /// A list of two lists. The first is a list of times at closest approach, as universal
            /// times in seconds. The second is a list of corresponding distances at closest approach,
            /// in meters.
            fn list_closest_approaches(target: &Orbit, orbits: i32) -> Vec<Vec<f64>> {
                ListClosestApproaches(target, orbits)
            }
        }
    }
    static_methods: {
        {
            /// Returns the direction that is normal to the orbits reference plane,
            /// in the given reference frame. The reference plane is the plane from which the
            /// orbits inclination is measured.
            ///
            /// **Game Scenes**: All
            ///
            /// # Arguments
            /// * `client` - A KRPC client object.
            /// * `reference_frame` - The reference frame that the returned direction is in.
            ///
            /// # Return
            /// The direction as a unit vector
            fn reference_plane_normal(reference_frame: &ReferenceFrame) -> Vector3 {
                ReferencePlaneNormal(reference_frame)
            }
        }
        {
            /// Returns the direction from which the orbits longitude of ascending node is measured,
            /// in the given reference frame.
            ///
            /// **Game Scenes**: All
            ///
            /// # Arguments
            /// * `client` - A KRPC client object.
            /// * `reference_frame` - The reference frame that the returned direction is in.
            ///
            /// # Return
            /// The direction as a unit vector
            fn reference_plane_direction(reference_frame: &ReferenceFrame) -> Vector3 {
                ReferencePlaneDirection(reference_frame)
            }
        }
    }
});
