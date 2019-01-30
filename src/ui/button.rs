use super::RectTransform;
use crate::codec::{Decode, Encode};
use crate::{remote_type, RemoteObject};

remote_type!(
/// A button.
object UI.Button {
    properties: {
        {
            RectTransform: RectTransform,
            /// Returns the rect transform for the button.
            ///
            /// **Game Scenes**: All
            get: rect_transform
        }
        {
            Text: String,
            /// Returns the text for the button.
            ///
            /// **Game Scenes**: All
            get: text
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
            Clicked: bool,
            /// Returns whether the button has been clicked.
            ///
            /// **Game Scenes**: All
            ///
            /// # Note
            /// This property is set to true when the user clicks the button. A client script
            /// should reset the property to false in order to detect subsequent button presses.
            get: is_clicked,
            /// Sets whether the button has been clicked.
            ///
            /// **Game Scenes**: All
            set: set_clicked
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
