use super::{Orbit, ReferenceFrame};
use crate::codec::*;
use crate::*;

remote_type!(
/// Represents a maneuver node.  Can be created using `Control::add_node()`.
object SpaceCenter.Node {
    properties: {
        {
            Prograde: f64,
            /// Returns the magnitude of the maneuver nodes delta-v in the prograde direction,
            /// in meters per second.
            ///
            /// **Game Scenes**: Flight
            get: prograde,
            /// Sets the magnitude of the maneuver nodes delta-v in the prograde direction,
            /// in meters per second.
            ///
            /// **Game Scenes**: Flight
            set: set_prograde
        }
        {
            Normal: f64,
            /// Returns the magnitude of the maneuver nodes delta-v in the normal direction,
            /// in meters per second.
            ///
            /// **Game Scenes**: Flight
            get: normal,
            /// Sets the magnitude of the maneuver nodes delta-v in the normal direction,
            /// in meters per second.
            ///
            /// **Game Scenes**: Flight
            set: set_normal
        }
        {
            Radial: f64,
            /// Returns the magnitude of the maneuver nodes delta-v in the radial direction,
            /// in meters per second.
            ///
            /// **Game Scenes**: Flight
            get: radial,
            /// Sets the magnitude of the maneuver nodes delta-v in the radial direction,
            /// in meters per second.
            ///
            /// **Game Scenes**: Flight
            set: set_radial
        }
        {
            DeltaV: f64,
            /// Returns the delta-v of the maneuver node, in meters per second.
            ///
            /// **Game Scenes**: Flight
            ///
            /// # Note
            /// Does not change when executing the maneuver node. See `Node::remaining_deltav()`.
            get: deltav,
            /// Sets the delta-v of the maneuver node, in meters per second.
            ///
            /// **Game Scenes**: Flight
            set: set_deltav
        }
        {
            RemainingDeltaV: f64,
            /// Gets the remaining delta-v of the maneuver node, in meters per second.
            /// Changes as the node is executed. This is equivalent to the delta-v reported in-game.
            ///
            /// **Game Scenes**: Flight
            get: remaining_deltav
        }
        {
            UT: f64,
            /// Returns the universal time at which the maneuver will occur, in seconds.
            ///
            /// **Game Scenes**: Flight
            get: ut,
            /// Sets the universal time at which the maneuver will occur, in seconds.
            ///
            /// **Game Scenes**: Flight
            set: set_ut
        }
        {
            TimeTo: f64,
            /// Returns the time until the maneuver node will be encountered, in seconds.
            ///
            /// **Game Scenes**: Flight
            get: time_to
        }
        {
            Orbit: Orbit,
            /// Returns the orbit that results from executing the maneuver node.
            ///
            /// **Game Scenes**: Flight
            get: orbit
        }
        {
            ReferenceFrame: ReferenceFrame,
            /// Returns the reference frame that is fixed relative to the maneuver node’s burn.
            ///
            /// * The origin is at the position of the maneuver node.
            /// * The y-axis points in the direction of the burn.
            /// * The x-axis and z-axis point in arbitrary but fixed directions.
            ///
            /// **Game Scenes**: Flight
            get: reference_frame
        }
        {
            OrbitalReferenceFrame: ReferenceFrame,
            /// Returns the reference frame that is fixed relative to the maneuver node, and
            /// orientated with the orbital prograde/normal/radial directions of the original
            /// orbit at the maneuver node’s position.
            ///
            /// * The origin is at the position of the maneuver node.
            /// * The x-axis points in the orbital anti-radial direction of the original orbit,
            /// at the position of the maneuver node.
            /// * The y-axis points in the orbital prograde direction of the original orbit,
            /// at the position of the maneuver node.
            /// * The z-axis points in the orbital normal direction of the original orbit,
            /// at the position of the maneuver node.
            ///
            /// **Game Scenes**: Flight
            get: orbital_reference_frame
        }
    }
    methods: {
        {
            /// Returns the burn vector for the maneuver node.
            ///
            /// **Game Scenes**: Flight
            ///
            /// # Arguments
            /// * `reference_frame` - The reference frame that the returned vector is in.
            /// Defaults to `Vessel::orbital_reference_frame()`.
            ///
            /// # Returns
            /// A vector whose direction is the direction of the maneuver node burn, and
            /// magnitude is the delta-v of the burn in meters per second.
            ///
            /// # Note
            /// Does not change when executing the maneuver node.
            fn burn_vector(reference_frame: Option<&ReferenceFrame>) -> Vector3 {
                BurnVector(reference_frame)
            }
        }
        {
            /// Returns the remaining burn vector for the maneuver node.
            ///
            /// **Game Scenes**: Flight
            ///
            /// # Arguments
            /// * `reference_frame` - The reference frame that the returned vector is in.
            /// Defaults to `Vessel::orbital_reference_frame()`.
            ///
            /// # Returns
            /// A vector whose direction is the direction of the maneuver node burn, and
            /// magnitude is the delta-v of the burn in meters per second.
            ///
            /// # Note
            /// Changes as the maneuver node is executed.
            fn remaining_burn_vector(reference_frame: Option<&ReferenceFrame>) -> Vector3 {
                RemainingBurnVector(reference_frame)
            }
        }
        {
            /// Removes the maneuver node.
            ///
            /// **Game Scenes**: Flight
            fn remove() {
                Remove()
            }
        }
        {
            /// Returns the position vector of the maneuver node in the given reference frame.
            ///
            /// **Game Scenes**: Flight
            ///
            /// # Arguments
            /// * `reference_frame` - The reference frame that the returned position vector is in.
            ///
            /// # Returns
            /// The position as a vector.
            fn position(reference_frame: &ReferenceFrame) -> Vector3 {
                Position(reference_frame)
            }
        }
        {
            /// Returns the direction of the maneuver nodes burn.
            ///
            /// **Game Scenes**: Flight
            ///
            /// # Arguments
            /// * `reference_frame` - The reference frame that the returned direction is in.
            ///
            /// # Returns
            /// The direction as a unit vector.
            fn direction(reference_frame: &ReferenceFrame) -> Vector3 {
                Direction(reference_frame)
            }
        }
    }
});
