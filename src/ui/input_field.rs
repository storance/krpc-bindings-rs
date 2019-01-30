use super::RectTransform;
use crate::codec::{Decode, Encode};
use crate::{remote_type, RemoteObject};

remote_type!(
/// An input field.
object UI.InputField {
    properties: {
        {
            RectTransform: RectTransform,
            /// Returns the rect transform for the input field.
            ///
            /// **Game Scenes**: All
            get: rect_transform
        }
        {
            Visible: bool,
            /// Returns whether the UI object is visible.
            ///
            /// **Game Scenes**: All
            get: is_visible,
            /// Sets whether the UI object is visible.
            ///
            /// **Game Scenes**: All
            set: set_visible
        }
        {
            Text: String,
            /// Returns the text component of the input field.
            ///
            /// **Game Scenes**: All
            get: text
        }
        {
            Value: String,
            /// Returns the value of the input field.
            ///
            /// **Game Scenes**: All
            get: value,
            /// Sets the value of the input field.
            ///
            /// **Game Scenes**: All
            set: set_value
        }
        {
            Changed: bool,
            /// Returns whether the input field has been changed.
            ///
            /// **Game Scenes**: All
            ///
            /// # Note
            /// This property is set to true when the user modifies the value of the input field.
            /// A client script should reset the property to false in order to detect
            /// subsequent changes.
            get: is_changed,
            /// Sets whether the input field has been changed.
            ///
            /// **Game Scenes**: All
            set: set_changed
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
