use crate::*;
use crate::codec::*;
use super::{CelestialBody, ReferenceFrame};

use std::rc::{Rc};
use std::cell::{RefCell};

use uom::si::f64::{Length, Velocity, Time};
use uom::si::length::{meter};
use uom::si::time::{second};

use angular_units::Rad;

remote_type!(
/// Describes an orbit. For example, the orbit of a vessel or a celestial body.
object Orbit {});

impl Orbit {
    rpc_method!(
    /// Returns the celestial body (e.g. planet or moon) around which the object is orbiting.
    fn get_body(&self) -> CelestialBody {
        if let Some(value) = SpaceCenter.Orbit_get_Body(self) as CelestialBody {
            value
        } else {
            return Err(KrpcError::NullResponseValue)
        }
    });

    rpc_method!(
    /// Gets the apoapsis of the orbit, in meters, from the center of mass of the body being orbited.
    ///
    /// *Note* For the apoapsis altitude reported on the in-game map view, use `get_apoapsis_altitude`.
    fn get_apoapsis(&self) -> Length {
        if let Some(value) = SpaceCenter.Orbit_get_Apoapsis(self) as Length {
            value
        } else {
            return Err(KrpcError::NullResponseValue)
        }
    });

    rpc_method!(
    /// Gets the periapsis of the orbit, in meters, from the center of mass of the body being orbited.
    ///
    /// *Note* For the periapsis altitude reported on the in-game map view, use `get_periapsis_altitude`.
    fn get_periapsis(&self) -> Length {
        if let Some(value) = SpaceCenter.Orbit_get_Periapsis(self) as Length {
            value
        } else {
            return Err(KrpcError::NullResponseValue)
        }
    });

    rpc_method!(
    /// The apoapsis of the orbit, in meters, above the sea level of the body being orbited.
    ///
    /// *Note* &self is equal to `get_apoapsis` minus the equatorial radius of the body.
    fn get_apoapsis_altitude(&self) -> Length {
        if let Some(value) = SpaceCenter.Orbit_get_ApoapsisAltitude(self) as Length {
            value
        } else {
            return Err(KrpcError::NullResponseValue)
        }
    });

    rpc_method!(
    /// The periapsis of the orbit, in meters, above the sea level of the body being orbited.
    /// *Note* &self is equal to `get_periapsis` minus the equatorial radius of the body.
    fn get_periapsis_altitude(&self) -> Length {
        if let Some(value) = SpaceCenter.Orbit_get_PeriapsisAltitude(self) as Length {
            value
        } else {
            return Err(KrpcError::NullResponseValue)
        }
    });

    rpc_method!(
    /// Returns the semi-major axis of the orbit, in meters.
    fn get_semi_major_axis(&self) -> Length {
        if let Some(value) = SpaceCenter.Orbit_get_SemiMajorAxis(self) as Length {
            value
        } else {
            return Err(KrpcError::NullResponseValue)
        }
    });

    rpc_method!(
    /// Returns the semi-minor axis of the orbit, in meters.
    fn get_semi_minor_axis(&self) -> Length {
        if let Some(value) = SpaceCenter.Orbit_get_SemiMinorAxis(self) as Length {
            value
        } else {
            return Err(KrpcError::NullResponseValue)
        }
    });

    rpc_method!(
    /// The current radius of the orbit, in meters. &self is the distance between the center of mass
    /// of the object in orbit, and the center of mass of the body around which it is orbiting.
    ///
    /// *Note* &self value will change over time if the orbit is elliptical.
    fn get_radius(&self) -> Length {
        if let Some(value) = SpaceCenter.Orbit_get_Radius(self) as Length {
            value
        } else {
            return Err(KrpcError::NullResponseValue)
        }
    });

    rpc_method!(
    /// The orbital radius, in meters, at the given time.
    ///
    /// # Arguments
    /// * `ut` - The universal time to measure the radius at.
    fn radius_at(&self, ut: Time) -> Length {
        if let Some(value) = SpaceCenter.Orbit_RadiusAt(self, ut) as Length {
            value
        } else {
            return Err(KrpcError::NullResponseValue)
        }
    });

    rpc_method!(
    /// The position as a vector at a given time, in the specified reference frame.
    ///
    /// # Arguments
    /// * `ut` - The universal time to measure the position at.
    /// * `reference_frame` - The reference frame that the returned position vector is in.
    fn position_at(&self, ut: Time, reference_frame: &ReferenceFrame) -> Vector3 {
        if let Some(value) = SpaceCenter.Orbit_PositionAt(self, ut, reference_frame) as Vector3 {
            value
        } else {
            return Err(KrpcError::NullResponseValue)
        }
    });

    rpc_method!(
    /// The current orbital speed of the object in meters per second.
    ///
    /// *Note* &self value will change over time if the orbit is elliptical.
    fn get_speed(&self) -> Velocity {
        if let Some(value) = SpaceCenter.Orbit_get_Speed(self) as Velocity {
            value
        } else {
            return Err(KrpcError::NullResponseValue)
        }
    });

    rpc_method!(
    /// Returns the time until the object reaches apoapsis, in seconds.
    fn get_time_to_apoapsis(&self) -> Time {
        if let Some(value) = SpaceCenter.Orbit_get_TimeToApoapsis(self) as Time {
            value
        } else {
            return Err(KrpcError::NullResponseValue)
        }
    });

    rpc_method!(
    /// Returns the time until the object reaches periapsis, in seconds.
    fn get_time_to_periapsis(&self) -> Time {
        if let Some(value) = SpaceCenter.Orbit_get_TimeToPeriapsis(self) as Time {
            value
        } else {
            return Err(KrpcError::NullResponseValue)
        }
    });

    rpc_method!(
    /// Returns the [eccentricity](https://en.wikipedia.org/wiki/Orbital_eccentricity) of the orbit.
    fn get_eccentricity(&self) -> f64 {
        if let Some(value) = SpaceCenter.Orbit_get_Eccentricity(self) as f64 {
            value
        } else {
            return Err(KrpcError::NullResponseValue)
        }
    });

    rpc_method!(
    /// Returns the [inclination](https://en.wikipedia.org/wiki/Orbital_inclination) of the orbit,
    /// in Rad<f64>.
    fn get_inclination(&self) -> Rad<f64> {
        if let Some(value) = SpaceCenter.Orbit_get_Inclination(self) as Rad<f64> {
            value
        } else {
            return Err(KrpcError::NullResponseValue)
        }
    });

    rpc_method!(
    /// Returns the [longitude of the ascending node](https://en.wikipedia.org/wiki/Longitude_of_the_ascending_node),
    /// in Rad<f64>.
    fn get_longitude_of_ascending_node(&self) -> Rad<f64> {
        if let Some(value) = SpaceCenter.Orbit_get_LongitudeOfAscendingNode(self) as Rad<f64> {
            value
        } else {
            return Err(KrpcError::NullResponseValue)
        }
    });

    rpc_method!(
    /// Returns the [argument of periapsis](https://en.wikipedia.org/wiki/Argument_of_periapsis), in Rad<f64>.
    fn get_argument_of_periapsis(&self) -> Rad<f64> {
        if let Some(value) = SpaceCenter.Orbit_get_ArgumentOfPeriapsis(self) as Rad<f64> {
            value
        } else {
            return Err(KrpcError::NullResponseValue)
        }
    });

    rpc_method!(
    /// Returns the [mean anomaly at epoch](https://en.wikipedia.org/wiki/Mean_anomaly).
    fn get_mean_anomaly_at_epoch(&self) -> f64 {
        if let Some(value) = SpaceCenter.Orbit_get_MeanAnomalyAtEpoch(self) as f64 {
            value
        } else {
            return Err(KrpcError::NullResponseValue)
        }
    });

    rpc_method!(
    /// Returns the time since the epoch (the point at which the
    /// [mean anomaly at epoch](https://en.wikipedia.org/wiki/Mean_anomaly) was measured), in seconds.
    fn get_epoch(&self) -> Time {
        if let Some(value) = SpaceCenter.Orbit_get_Epoch(self) as Time {
            value
        } else {
            return Err(KrpcError::NullResponseValue)
        }
    });

    rpc_method!(
    /// Returns the [mean anomaly](https://en.wikipedia.org/wiki/Mean_anomaly).
    fn get_mean_anomaly(&self) -> f64 {
        if let Some(value) = SpaceCenter.Orbit_get_MeanAnomaly(self) as f64 {
            value
        } else {
            return Err(KrpcError::NullResponseValue)
        }
    });

    rpc_method!(
    /// Returns the mean anomaly at the given time.
    ///
    /// # Arguments
    /// * `ut` - The universal time in seconds.
    fn mean_anomaly_at_ut(&self, ut: Time) -> f64 {
        if let Some(value) = SpaceCenter.Orbit_MeanAnomalyAtUT(self, ut) as f64 {
            value
        } else {
            return Err(KrpcError::NullResponseValue)
        }
    });

    rpc_method!(
    /// Returns the [eccentric anomaly](https://en.wikipedia.org/wiki/Eccentric_anomaly).
    fn get_eccentric_anomaly(&self) -> f64 {
        if let Some(value) = SpaceCenter.Orbit_get_EccentricAnomaly(self) as f64 {
            value
        } else {
            return Err(KrpcError::NullResponseValue)
        }
    });

    rpc_method!(
    /// Returns the eccentric anomaly at the given time.
    ///
    /// # Arguments
    /// * `ut` - The universal time in seconds.
    fn eccentric_anomaly_at_ut(&self, ut: Time) -> f64 {
        if let Some(value) = SpaceCenter.Orbit_EccentricAnomalyAtUT(self, ut) as f64 {
            value
        } else {
            return Err(KrpcError::NullResponseValue)
        }
    });

    rpc_method!(
    /// Returns the [true anomaly](https://en.wikipedia.org/wiki/True_anomaly).
    fn get_true_anomaly(&self) -> f64 {
        if let Some(value) = SpaceCenter.Orbit_get_TrueAnomaly(self) as f64 {
            value
        } else {
            return Err(KrpcError::NullResponseValue)
        }
    });

    rpc_method!(
    /// Returns the true anomaly at the given time.
    ///
    /// # Arguments
    /// * `ut` - The universal time in seconds.
    fn true_anomaly_at_ut(&self, ut: Time) -> f64 {
        if let Some(value) = SpaceCenter.Orbit_TrueAnomalyAtUT(self, ut) as f64 {
            value
        } else {
            return Err(KrpcError::NullResponseValue)
        }
    });

    rpc_method!(
    /// Returns the true anomaly at the given orbital radius.
    ///
    /// # Arguments
    /// * `radius` - The orbital radius in meters.
    fn true_anomaly_at_radius(&self, radius: Length) -> f64 {
        if let Some(value) = SpaceCenter.Orbit_TrueAnomalyAtRadius(self, radius) as f64 {
            value
        } else {
            return Err(KrpcError::NullResponseValue)
        }
    });

    rpc_method!(
    /// Returns the universal time, in seconds, corresponding to the given true anomaly.
    ///
    /// # Arguments
    /// * `true_anomaly` - True anomaly.
    fn ut_at_true_anomaly(&self, true_anomaly: f64) -> Time {
        if let Some(value) = SpaceCenter.Orbit_UTAtTrueAnomaly(self, true_anomaly) as f64 {
            Time::new::<second>(value)
        } else {
            return Err(KrpcError::NullResponseValue)
        }
    });

    rpc_method!(
    /// Returns the orbital radius at the point in the orbit given by the true anomaly.
    ///
    /// # Arguments
    /// * `true_anomaly` - True anomaly.
    fn radius_at_true_anomaly(&self, true_anomaly: f64) -> Length {
        if let Some(value) = SpaceCenter.Orbit_RadiusAtTrueAnomaly(self, true_anomaly) as f64 {
            Length::new::<meter>(value)
        } else {
            return Err(KrpcError::NullResponseValue)
        }
    });

    rpc_method!(
    /// Return the true anomaly of the ascending node with the given target orbit.
    ///
    /// # Arguments
    /// * `target` - Target orbit.
    fn true_anomaly_at_an(&self) -> f64 {
        if let Some(value) = SpaceCenter.Orbit_TrueAnomalyAtAN(self) as f64 {
            value
        } else {
            return Err(KrpcError::NullResponseValue)
        }
    });

    rpc_method!(
    /// Return the true anomaly of the descending node with the given target orbit.
    ///
    /// # Arguments
    /// * `target` - Target orbit.
    fn true_anomaly_at_dn(&self) -> f64 {
        if let Some(value) = SpaceCenter.Orbit_TrueAnomalyAtDN(self) as f64 {
            value
        } else {
            return Err(KrpcError::NullResponseValue)
        }
    });

    rpc_method!(
    /// Returns the current orbital speed in meters per second.
    fn get_orbital_speed(&self) -> Velocity {
        if let Some(value) = SpaceCenter.Orbit_get_OrbitalSpeed(self) as Velocity {
            value
        } else {
            return Err(KrpcError::NullResponseValue)
        }
    });

    rpc_method!(
    /// The orbital speed at the given time, in meters per second.
    ///
    /// # Arguments
    /// * `time` - Time from now, in seconds.
    fn orbital_speed_at(&self, time: Time) -> Velocity {
        if let Some(value) = SpaceCenter.Orbit_OrbitalSpeedAt(self, time) as Velocity {
            value
        } else {
            return Err(KrpcError::NullResponseValue)
        }
    });

    rpc_method!(
    /// Returns the relative inclination of &self orbit and the target orbit, in Rad<f64>.
    ///
    /// # Arguments
    /// * `target` - Target orbit.
    fn relative_inclincation(&self, target: &Orbit) -> Rad<f64> {
        if let Some(value) = SpaceCenter.Orbit_RelativeInclination(self, target) as Rad<f64> {
            value
        } else {
            return Err(KrpcError::NullResponseValue)
        }
    });

    rpc_method!(
    /// Returns the time until the object changes sphere of influence, in secondso or `None`
    /// if the object is not going to change sphere of influence.
    fn get_time_to_soi_change(&self) -> Option<Time> {
        if let Some(value) = SpaceCenter.Orbit_get_TimeToSOIChange(self) as f64 {
            if value.is_nan() {
                None
            } else {
                Some(Time::new::<second>(value))
            }
        } else {
            return Err(KrpcError::NullResponseValue)
        }
    });

    rpc_method!(
    /// Estimates and returns the universal time, in seconds, at closest approach to a target orbit.
    ///
    /// # Arguments
    /// * `target` - Target orbit.
    fn time_of_closest_approach(&self, target: &Orbit) -> Time {
        if let Some(value) = SpaceCenter.Orbit_TimeOfClosestApproach(self, target) as Time {
            value
        } else {
            return Err(KrpcError::NullResponseValue)
        }
    });

    rpc_method!(
    /// Estimates and returns the distance at closest approach to a target orbit, in meters.
    ///
    /// # Arguments
    /// * `target` - Target orbit.
    fn distance_at_closest_approach(&self, target: &Orbit) -> Length {
        if let Some(value) = SpaceCenter.Orbit_DistanceAtClosestApproach(self, target) as Length {
            value
        } else {
            return Err(KrpcError::NullResponseValue)
        }
    });

    rpc_method!(
    ///  Returns the times at closest approach and corresponding distances, to a target orbit.
    ///
    /// # Returns
    /// A list of tuples.
    /// The first element of the tuple is the universal time at the closest approach, in seconds.
    /// The second element of the tuple is the distance at closest approach, in meters.
    ///
    /// # Arguments
    /// * `target` - Target orbit.
    /// * `orbits` - The number of future orbits to search.
    fn list_closest_approaches(&self, target: &Orbit, orbits: i32) -> Vec<(Time, Length)> {
        if let Some(values) = SpaceCenter.Orbit_ListClosestApproaches(self, target, orbits) as Vec<Vec<f64>> {
            {
                if values.len() != 2 {
                    return Err(KrpcError::ResponseHasError("Expected list to have exactly two entries".to_owned()));
                }

                let times = &values[0];
                let distances = &values[1];

                if times.len() != distances.len() {
                    return Err(KrpcError::ResponseHasError("Expected times and distances lists to be the same size".to_owned()));
                }

                let mut time_distances = vec!();
                for (time, distance) in times.iter().zip(distances.iter()) {
                    time_distances.push((
                        Time::new::<second>(*time),
                        Length::new::<meter>(*distance)
                    ))
                }

                time_distances
            }
        } else {
            return Err(KrpcError::NullResponseValue)
        }
    });

    rpc_method!(
    /// Returns the direction that is normal to the orbits reference plane,
    /// in the given reference frame. The reference plane is the plane from which the
    /// orbits inclination is measured.
    ///
    /// # Returns
    /// The direction as a unit vector
    ///
    /// # Arguments
    /// * `client` - A KRPC client object.
    /// * `reference_frame` - The reference frame that the returned direction is in.
    fn reference_plane_normal(client: &KrpcClient, reference_frame: &ReferenceFrame) -> Vector3 {
        if let Some(value) = SpaceCenter.Orbit_static_ReferencePlaneNormal(reference_frame) as Vector3 {
            value
        } else {
            return Err(KrpcError::NullResponseValue)
        }
    });

    rpc_method!(
    /// Returns the direction from which the orbits longitude of ascending node is measured,
    /// in the given reference frame.
    ///
    /// # Returns
    /// The direction as a unit vector
    ///
    /// # Arguments
    /// * `client` - A KRPC client object.
    /// * `reference_frame` - The reference frame that the returned direction is in.
    fn reference_plane_direction(client: &KrpcClient, reference_frame: &ReferenceFrame) -> Vector3 {
        if let Some(value) = SpaceCenter.Orbit_static_ReferencePlaneDirection(reference_frame) as Vector3 {
            value
        } else {
            return Err(KrpcError::NullResponseValue)
        }
    });
}