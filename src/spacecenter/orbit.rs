use crate::*;
use crate::codec::*;
use super::{CelestialBody, ReferenceFrame};

use std::rc::{Rc};
use std::cell::{RefCell};


remote_type!(
/// Describes an orbit. For example, the orbit of a vessel or a celestial body.
object Orbit {});

impl Orbit {
    rpc_method!(
    /// Returns the celestial body (e.g. planet or moon) around which the object is orbiting.
    ///
    /// **Game Scenes**: All
    fn body(&self) -> CelestialBody {
        SpaceCenter.Orbit_get_Body(self).ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// Gets the apoapsis of the orbit, in meters, from the center of mass of the body being orbited.
    ///
    /// **Game Scenes**: All
    ///
    /// # Note
    /// For the apoapsis altitude reported on the in-game map view, use `get_apoapsis_altitude`.
    fn apoapsis(&self) -> f64 {
        SpaceCenter.Orbit_get_Apoapsis(self).ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// Gets the periapsis of the orbit, in meters, from the center of mass of the body being orbited.
    ///
    /// **Game Scenes**: All
    ///
    /// # Note
    /// For the periapsis altitude reported on the in-game map view, use `get_periapsis_altitude`.
    fn periapsis(&self) -> f64 {
        SpaceCenter.Orbit_get_Periapsis(self).ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// The apoapsis of the orbit, in meters, above the sea level of the body being orbited.
    ///
    /// **Game Scenes**: All
    ///
    /// # Note
    /// This is equal to `get_apoapsis` minus the equatorial radius of the body.
    fn apoapsis_altitude(&self) -> f64 {
        SpaceCenter.Orbit_get_ApoapsisAltitude(self).ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// The periapsis of the orbit, in meters, above the sea level of the body being orbited.
    ///
    /// **Game Scenes**: All
    ///
    /// # Note
    /// This is equal to `get_periapsis` minus the equatorial radius of the body.
    fn periapsis_altitude(&self) -> f64 {
        SpaceCenter.Orbit_get_PeriapsisAltitude(self)
            .ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// Returns the semi-major axis of the orbit, in meters.
    ///
    /// **Game Scenes**: All
    fn semi_major_axis(&self) -> f64 {
        SpaceCenter.Orbit_get_SemiMajorAxis(self).ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// Returns the semi-minor axis of the orbit, in meters.
    ///
    /// **Game Scenes**: All
    fn semi_minor_axis(&self) -> f64 {
        SpaceCenter.Orbit_get_SemiMinorAxis(self).ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// The current radius of the orbit, in meters. &self is the distance between the center of mass
    /// of the object in orbit, and the center of mass of the body around which it is orbiting.
    ///
    /// **Game Scenes**: All
    ///
    /// # Note
    /// This value will change over time if the orbit is elliptical.
    fn radius(&self) -> f64 {
        SpaceCenter.Orbit_get_Radius(self).ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// The orbital radius, in meters, at the given time.
    ///
    /// **Game Scenes**: All
    ///
    /// # Arguments
    /// * `ut` - The universal time to measure the radius at.
    fn radius_at(&self, ut: f64) -> f64 {
        SpaceCenter.Orbit_RadiusAt(self, ut).ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// The position as a vector at a given time, in the specified reference frame.
    ///
    /// **Game Scenes**: All
    ///
    /// # Arguments
    /// * `ut` - The universal time to measure the position at.
    /// * `reference_frame` - The reference frame that the returned position vector is in.
    fn position_at(&self, ut: f64, reference_frame: &ReferenceFrame) -> Vector3 {
        SpaceCenter.Orbit_PositionAt(self, ut, reference_frame).ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// The current orbital speed of the object in meters per second.
    ///
    /// **Game Scenes**: All
    ///
    /// # Note
    /// This value will change over time if the orbit is elliptical.
    fn speed(&self) -> f64 {
        SpaceCenter.Orbit_get_Speed(self).ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// Returns the time until the object reaches apoapsis, in seconds.
    ///
    /// **Game Scenes**: All
    fn time_to_apoapsis(&self) -> f64 {
        SpaceCenter.Orbit_get_f64ToApoapsis(self).ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// Returns the time until the object reaches periapsis, in seconds.
    ///
    /// **Game Scenes**: All
    fn time_to_periapsis(&self) -> f64 {
        SpaceCenter.Orbit_get_f64ToPeriapsis(self).ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// Returns the [eccentricity](https://en.wikipedia.org/wiki/Orbital_eccentricity) of the orbit.
    ///
    /// **Game Scenes**: All
    fn eccentricity(&self) -> f64 {
        SpaceCenter.Orbit_get_Eccentricity(self).ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// Returns the [inclination](https://en.wikipedia.org/wiki/Orbital_inclination) of the orbit,
    /// in radians.
    ///
    /// **Game Scenes**: All
    fn inclination(&self) -> f64 {
        SpaceCenter.Orbit_get_Inclination(self).ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// Returns the [longitude of the ascending node](https://en.wikipedia.org/wiki/Longitude_of_the_ascending_node),
    /// in radians.
    ///
    /// **Game Scenes**: All
    fn longitude_of_ascending_node(&self) -> f64 {
        SpaceCenter.Orbit_get_LongitudeOfAscendingNode(self).ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// Returns the [argument of periapsis](https://en.wikipedia.org/wiki/Argument_of_periapsis),
    /// in radians.
    ///
    /// **Game Scenes**: All
    fn argument_of_periapsis(&self) -> f64 {
        SpaceCenter.Orbit_get_ArgumentOfPeriapsis(self).ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// Returns the [mean anomaly at epoch](https://en.wikipedia.org/wiki/Mean_anomaly).
    ///
    /// **Game Scenes**: All
    fn mean_anomaly_at_epoch(&self) -> f64 {
        SpaceCenter.Orbit_get_MeanAnomalyAtEpoch(self).ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// Returns the time since the epoch (the point at which the
    /// [mean anomaly at epoch](https://en.wikipedia.org/wiki/Mean_anomaly) was measured), in seconds.
    ///
    /// **Game Scenes**: All
    fn epoch(&self) -> f64 {
        SpaceCenter.Orbit_get_Epoch(self).ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// Returns the [mean anomaly](https://en.wikipedia.org/wiki/Mean_anomaly).
    ///
    /// **Game Scenes**: All
    fn mean_anomaly(&self) -> f64 {
        SpaceCenter.Orbit_get_MeanAnomaly(self).ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// Returns the mean anomaly at the given time.
    ///
    /// **Game Scenes**: All
    ///
    /// # Arguments
    /// * `ut` - The universal time in seconds.
    fn mean_anomaly_at_ut(&self, ut: f64) -> f64 {
        SpaceCenter.Orbit_MeanAnomalyAtUT(self, ut).ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// Returns the [eccentric anomaly](https://en.wikipedia.org/wiki/Eccentric_anomaly).
    ///
    /// **Game Scenes**: All
    fn eccentric_anomaly(&self) -> f64 {
        SpaceCenter.Orbit_get_EccentricAnomaly(self).ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// Returns the eccentric anomaly at the given time.
    ///
    /// **Game Scenes**: All
    ///
    /// # Arguments
    /// * `ut` - The universal time in seconds.
    fn eccentric_anomaly_at_ut(&self, ut: f64) -> f64 {
        SpaceCenter.Orbit_EccentricAnomalyAtUT(self, ut).ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// Returns the [true anomaly](https://en.wikipedia.org/wiki/True_anomaly).
    ///
    /// **Game Scenes**: All
    fn true_anomaly(&self) -> f64 {
        SpaceCenter.Orbit_get_TrueAnomaly(self).ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// Returns the true anomaly at the given time.
    ///
    /// **Game Scenes**: All
    ///
    /// # Arguments
    /// * `ut` - The universal time in seconds.
    fn true_anomaly_at_ut(&self, ut: f64) -> f64 {
        SpaceCenter.Orbit_TrueAnomalyAtUT(self, ut).ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// Returns the true anomaly at the given orbital radius.
    ///
    /// **Game Scenes**: All
    ///
    /// # Arguments
    /// * `radius` - The orbital radius in meters.
    fn true_anomaly_at_radius(&self, radius: f64) -> f64 {
        SpaceCenter.Orbit_TrueAnomalyAtRadius(self, radius).ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// Returns the universal time, in seconds, corresponding to the given true anomaly.
    ///
    /// **Game Scenes**: All
    ///
    /// # Arguments
    /// * `true_anomaly` - True anomaly.
    fn ut_at_true_anomaly(&self, true_anomaly: f64) -> f64 {
        SpaceCenter.Orbit_UTAtTrueAnomaly(self, true_anomaly).ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// Returns the orbital radius at the point in the orbit given by the true anomaly.
    ///
    /// **Game Scenes**: All
    ///
    /// # Arguments
    /// * `true_anomaly` - True anomaly.
    fn radius_at_true_anomaly(&self, true_anomaly: f64) -> f64 {
        SpaceCenter.Orbit_RadiusAtTrueAnomaly(self, true_anomaly)
            .ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// Return the true anomaly of the ascending node with the given target orbit.
    ///
    /// **Game Scenes**: All
    ///
    /// # Arguments
    /// * `target` - Target orbit.
    fn true_anomaly_at_an(&self) -> f64 {
        SpaceCenter.Orbit_TrueAnomalyAtAN(self).ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// Return the true anomaly of the descending node with the given target orbit.
    ///
    /// **Game Scenes**: All
    ///
    /// # Arguments
    /// * `target` - Target orbit.
    fn true_anomaly_at_dn(&self) -> f64 {
        SpaceCenter.Orbit_TrueAnomalyAtDN(self)
            .ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// Returns the current orbital speed in meters per second.
    ///
    /// **Game Scenes**: All
    fn orbital_speed(&self) -> f64 {
        SpaceCenter.Orbit_get_OrbitalSpeed(self).ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// The orbital speed at the given time, in meters per second.
    ///
    /// **Game Scenes**: All
    ///
    /// # Arguments
    /// * `time` - f64 from now, in seconds.
    fn orbital_speed_at(&self, time: f64) -> f64 {
        SpaceCenter.Orbit_OrbitalSpeedAt(self, time).ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// Returns the relative inclination of &self orbit and the target orbit, in radians.
    ///
    /// **Game Scenes**: All
    ///
    /// # Arguments
    /// * `target` - Target orbit.
    fn relative_inclincation(&self, target: &Orbit) -> f64 {
        SpaceCenter.Orbit_RelativeInclination(self, target).ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// Returns the time until the object changes sphere of influence, in seconds or `NaN`
    /// if the object is not going to change sphere of influence.
    ///
    /// **Game Scenes**: All
    fn time_to_soi_change(&self) -> f64 {
        SpaceCenter.Orbit_get_f64ToSOIChange(self)
            .ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// Estimates and returns the universal time, in seconds, at closest approach to a target orbit.
    ///
    /// **Game Scenes**: All
    ///
    /// # Arguments
    /// * `target` - Target orbit.
    fn time_of_closest_approach(&self, target: &Orbit) -> f64 {
        SpaceCenter.Orbit_f64OfClosestApproach(self, target).ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// Estimates and returns the distance at closest approach to a target orbit, in meters.
    ///
    /// **Game Scenes**: All
    ///
    /// # Arguments
    /// * `target` - Target orbit.
    fn distance_at_closest_approach(&self, target: &Orbit) -> f64 {
        SpaceCenter.Orbit_DistanceAtClosestApproach(self, target)
            .ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
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
    fn list_closest_approaches(&self, target: &Orbit, orbits: i32) -> Vec<Vec<f64>> {
        SpaceCenter.Orbit_ListClosestApproaches(self, target, orbits)
            .ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
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
    fn reference_plane_normal(client: &KrpcClient, reference_frame: &ReferenceFrame) -> Vector3 {
        SpaceCenter.Orbit_static_ReferencePlaneNormal(reference_frame)
            .ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
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
    fn reference_plane_direction(client: &KrpcClient, reference_frame: &ReferenceFrame) -> Vector3 {
        SpaceCenter.Orbit_static_ReferencePlaneDirection(reference_frame).ok_or(KrpcError::NullResponseValue)
    });
}