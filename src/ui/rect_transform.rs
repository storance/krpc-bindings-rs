use crate::codec::{Decode, Encode};
use crate::{remote_type, Quaternion, RemoteObject, Vector3};

remote_type!(
/// A Unity engine Rect Transform for a UI object. See the
/// [Unity manual](https://docs.unity3d.com/Manual/class-RectTransform.html) for more details.
object UI.RectTransform {
    properties: {
        {
            Position: (f64, f64),
            /// Returns the position of the rectangles pivot point relative to the anchors.
            ///
            /// **Game Scenes**: All
            get: position,
            /// Sets the position of the rectangles pivot point relative to the anchors.
            ///
            /// **Game Scenes**: All
            set: set_position
        }
        {
            LocalPosition: Vector3,
            /// Returns the position of the rectangles pivot point relative to the anchors.
            ///
            /// **Game Scenes**: All
            get: local_position,
            /// Sets the position of the rectangles pivot point relative to the anchors.
            ///
            /// **Game Scenes**: All
            set: set_local_position
        }
        {
            Size: (f64, f64),
            /// Returns the width and height of the rectangle.
            ///
            /// **Game Scenes**: All
            get: size,
            /// Sets the width and height of the rectangle.
            ///
            /// **Game Scenes**: All
            set: set_size
        }
        {
            UpperRight: (f64, f64),
            /// Returns the position of the rectangles upper right corner relative to the anchors.
            ///
            /// **Game Scenes**: All
            get: upper_right,
            /// Sets the position of the rectangles upper right corner relative to the anchors.
            ///
            /// **Game Scenes**: All
            set: set_upper_right
        }
        {
            LowerLeft: (f64, f64),
            /// Returns the position of the rectangles lower left corner relative to the anchors.
            ///
            /// **Game Scenes**: All
            get: lower_left,
            /// Sets the position of the rectangles lower left corner relative to the anchors.
            ///
            /// **Game Scenes**: All
            set: set_lower_left
        }
        {
            AnchorMin: (f64, f64),
            /// Returns the anchor point for the upper right corner of the rectangle defined
            /// as a fraction of the size of the parent rectangle.
            ///
            /// **Game Scenes**: All
            get: anchor_min,
            /// Sets the anchor point for the upper right corner of the rectangle defined
            /// as a fraction of the size of the parent rectangle.
            ///
            /// **Game Scenes**: All
            set: set_anchor_min
        }
        {
            AnchorMax: (f64, f64),
            /// Returns the anchor point for the lower left corner of the rectangle defined as
            /// a fraction of the size of the parent rectangle.
            ///
            /// **Game Scenes**: All
            get: anchor_max,
            /// Sets the anchor point for the lower left corner of the rectangle defined as
            /// a fraction of the size of the parent rectangle.
            ///
            /// **Game Scenes**: All
            set: set_anchor_max
        }
        {
            Pivot: (f64, f64),
            /// Returns the location of the pivot point around which the rectangle rotates,
            /// defined as a fraction of the size of the rectangle itself.
            ///
            /// **Game Scenes**: All
            get: pivot,
            /// Sets the location of the pivot point around which the rectangle rotates,
            /// defined as a fraction of the size of the rectangle itself.
            ///
            /// **Game Scenes**: All
            set: set_pivot
        }
        {
            Rotation: Quaternion,
            /// Returns the rotation, as a quaternion, of the object around its pivot point.
            ///
            /// **Game Scenes**: All
            get: rotation,
            /// Sets the rotation, as a quaternion, of the object around its pivot point.
            ///
            /// **Game Scenes**: All
            set: set_rotation
        }
        {
            Scale: Vector3,
            /// Returns the scale factor applied to the object in the x, y and z dimensions.
            ///
            /// **Game Scenes**: All
            get: scale,
            /// Sets the scale factor applied to the object in the x, y and z dimensions.
            ///
            /// **Game Scenes**: All
            set: set_scale
        }
    }
    methods: {
        {
            /// Set the minimum and maximum anchor points as a fraction of the size of
            /// the parent rectangle.
            ///
            /// **Game Scenes**: All
            fn set_anchor(value: (f64, f64)) {
                set_Anchor(value)
            }
        }
    }
});
