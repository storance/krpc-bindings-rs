use crate::codec::*;
use crate::*;

remote_type!(
/// Represents a reference frame for positions, rotations and velocities. Contains:
///
/// * The position of the origin.
/// * The directions of the x, y and z axes.
/// * The linear velocity of the frame.
/// * The angular velocity of the frame.
///
/// # Note
/// This struct does not contain any class methods. It is only to be used as a parameter to
/// other functions.
object SpaceCenter.ReferenceFrame {
    static_methods: {
        {
            /// Create a relative reference frame. This is a custom reference frame whose components
            /// offset the components of a parent reference frame.
            ///
            /// **Game Scenes***: All
            ///
            /// # Arguments
            /// * `client` - The krpc client.
            /// * `reference_frame` - The parent reference frame on which to base this reference frame.
            /// * `position` - The offset of the position of the origin, as a position vector. If `None`,
            /// (0,0,0) is used.
            /// * `rotation` - The rotation to apply to the parent frames rotation, as a quaternion of the
            /// form (x,y,z,w). If `None`, (0,0,0,1) is used (i.e. no rotation)
            /// * `velocity` - The linear velocity to offset the parent frame by, as a vector pointing in
            /// the direction of travel, whose magnitude is the speed in meters per second. If `None`,
            /// (0,0,0) is used.
            /// * `angular_velocity` - The angular velocity to offset the parent frame by, as a vector.
            /// This vector points in the direction of the axis of rotation, and its magnitude is the
            /// speed of the rotation in radians per second. If `None`, (0,0,0) is used.
            fn create_relative(reference_frame: &ReferenceFrame,
                               position: Option<Vector3>,
                               rotation: Option<Quaternion>,
                               velocity: Option<Vector3>,
                               angular_velocity: Option<Vector3>) -> ReferenceFrame<'a> {
                CreateRelative(reference_frame,
                               position.unwrap_or((0.0, 0.0, 0.0)),
                               rotation.unwrap_or((0.0, 0.0, 0.0, 1.0)),
                               velocity.unwrap_or((0.0, 0.0, 0.0)),
                               angular_velocity.unwrap_or((0.0, 0.0, 0.0)))
            }
        }
        {
            /// Create a hybrid reference frame. This is a custom reference frame whose components
            /// inherited from other reference frames.
            ///
            /// **Game Scenes***: All
            ///
            /// # Arguments
            /// * `client` - The krpc client.
            /// * `reference_frame` - The parent reference frame on which to base this reference frame.
            /// * `position` - The reference frame providing the position of the origin.
            /// * `rotation` - The reference frame providing the rotation of the frame. If `None`,
            /// the rotation of the `position` frame is used.
            /// * `velocity` - The reference frame providing the linear velocity of the frame. If `None`,
            /// the linear velocity of the `position` frame is used.
            /// * `angular_velocity` - The reference frame providing the angular velocity of the frame.
            /// If `None`, the angular velocity of the `position` frame is used.
            fn create_hybrid(position: &ReferenceFrame,
                             rotation: Option<&ReferenceFrame>,
                             velocity: Option<&ReferenceFrame>,
                             angular_velocity: Option<&ReferenceFrame>) -> ReferenceFrame<'a> {
                CreateHybrid(position, rotation, velocity, angular_velocity)
            }
        }
    }
});
