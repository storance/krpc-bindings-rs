use crate::codec::{Decode, Encode};
use crate::{remote_type, Quaternion, RemoteObject, Vector3};

remote_type!(
/// A Unity engine Rect Transform for a UI object. See the
/// [Unity manual](https://docs.unity3d.com/Manual/class-RectTransform.html) for more details.
object UI.RectTransform {
    properties: {
        {
            Position {
                /// Returns the position of the rectangles pivot point relative to the anchors.
                ///
                /// **Game Scenes**: All
                get: position -> (f64, f64),
                /// Sets the position of the rectangles pivot point relative to the anchors.
                ///
                /// **Game Scenes**: All
                set: set_position((f64, f64))
            }
        }
        {
            LocalPosition {
                /// Returns the position of the rectangles pivot point relative to the anchors.
                ///
                /// **Game Scenes**: All
                get: local_position -> Vector3,
                /// Sets the position of the rectangles pivot point relative to the anchors.
                ///
                /// **Game Scenes**: All
                set: set_local_position(Vector3)
            }
        }
        {
            Size {
                /// Returns the width and height of the rectangle.
                ///
                /// **Game Scenes**: All
                get: size -> (f64, f64),
                /// Sets the width and height of the rectangle.
                ///
                /// **Game Scenes**: All
                set: set_size((f64, f64))
            }
        }
        {
            UpperRight {
                /// Returns the position of the rectangles upper right corner relative to the anchors.
                ///
                /// **Game Scenes**: All
                get: upper_right -> (f64, f64),
                /// Sets the position of the rectangles upper right corner relative to the anchors.
                ///
                /// **Game Scenes**: All
                set: set_upper_right((f64, f64))
            }
        }
        {
            LowerLeft {
                /// Returns the position of the rectangles lower left corner relative to the anchors.
                ///
                /// **Game Scenes**: All
                get: lower_left -> (f64, f64),
                /// Sets the position of the rectangles lower left corner relative to the anchors.
                ///
                /// **Game Scenes**: All
                set: set_lower_left((f64, f64))
            }
        }
        {
            AnchorMin {
                /// Returns the anchor point for the upper right corner of the rectangle defined
                /// as a fraction of the size of the parent rectangle.
                ///
                /// **Game Scenes**: All
                get: anchor_min -> (f64, f64),
                /// Sets the anchor point for the upper right corner of the rectangle defined
                /// as a fraction of the size of the parent rectangle.
                ///
                /// **Game Scenes**: All
                set: set_anchor_min((f64, f64))
            }
        }
        {
            AnchorMax {
                /// Returns the anchor point for the lower left corner of the rectangle defined as
                /// a fraction of the size of the parent rectangle.
                ///
                /// **Game Scenes**: All
                get: anchor_max -> (f64, f64),
                /// Sets the anchor point for the lower left corner of the rectangle defined as
                /// a fraction of the size of the parent rectangle.
                ///
                /// **Game Scenes**: All
                set: set_anchor_max((f64, f64))
            }
        }
        {
            Pivot {
                /// Returns the location of the pivot point around which the rectangle rotates,
                /// defined as a fraction of the size of the rectangle itself.
                ///
                /// **Game Scenes**: All
                get: pivot -> (f64, f64),
                /// Sets the location of the pivot point around which the rectangle rotates,
                /// defined as a fraction of the size of the rectangle itself.
                ///
                /// **Game Scenes**: All
                set: set_pivot((f64, f64))
            }
        }
        {
            Rotation {
                /// Returns the rotation, as a quaternion, of the object around its pivot point.
                ///
                /// **Game Scenes**: All
                get: rotation -> Quaternion,
                /// Sets the rotation, as a quaternion, of the object around its pivot point.
                ///
                /// **Game Scenes**: All
                set: set_rotation(Quaternion)
            }
        }
        {
            Scale {
                /// Returns the scale factor applied to the object in the x, y and z dimensions.
                ///
                /// **Game Scenes**: All
                get: scale -> Vector3,
                /// Sets the scale factor applied to the object in the x, y and z dimensions.
                ///
                /// **Game Scenes**: All
                set: set_scale(Vector3)
            }
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
