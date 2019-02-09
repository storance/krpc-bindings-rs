use super::RectTransform;
use crate::codec::{Decode, Encode};
use crate::{remote_type, RemoteObject};

remote_type!(
/// An input field.
object UI.InputField {
    properties: {
        {
            RectTransform {
                /// Returns the rect transform for the input field.
                ///
                /// **Game Scenes**: All
                get: rect_transform -> RectTransform
            }
        }
        {
            Visible {
                /// Returns whether the UI object is visible.
                ///
                /// **Game Scenes**: All
                get: is_visible -> bool,
                /// Sets whether the UI object is visible.
                ///
                /// **Game Scenes**: All
                set: set_visible(bool)
            }
        }
        {
            Text {
                /// Returns the text component of the input field.
                ///
                /// **Game Scenes**: All
                get: text -> String
            }
        }
        {
            Value {
                /// Returns the value of the input field.
                ///
                /// **Game Scenes**: All
                get: value -> String,
                /// Sets the value of the input field.
                ///
                /// **Game Scenes**: All
                set: set_value(&str)
            }
        }
        {
            Changed {
                /// Returns whether the input field has been changed.
                ///
                /// **Game Scenes**: All
                ///
                /// # Note
                /// This property is set to true when the user modifies the value of the input field.
                /// A client script should reset the property to false in order to detect
                /// subsequent changes.
                get: is_changed -> bool,
                /// Sets whether the input field has been changed.
                ///
                /// **Game Scenes**: All
                set: set_changed(bool)
            }
        }
    }
    methods: {
        {
            /// Remove the UI object.
            ///
            /// **Game Scenes**: All
            fn remove() {
                Remove()
            }
        }
    }
});
