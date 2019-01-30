use super::{Button, InputField, Panel, RectTransform, Text};
use crate::codec::{Decode, Encode};
use crate::{remote_type, RemoteObject};

remote_type!(
/// A canvas for user interface elements
object UI.Canvas {
    properties: {
        {
            RectTransform: RectTransform,
            /// Returns the rect transform for the canvas.
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
    }
    methods: {
        {
            /// Create a new container for user interface elements.
            ///
            /// **Game Scenes**: All
            ///
            /// # Arguments
            /// * `visible` - Whether the panel is visible.
            fn add_panel(visible: bool) -> Panel {
                AddPanel(visible)
            }
        }
        {
            /// Add text to the canvas.
            ///
            /// **Game Scenes**: All
            ///
            /// # Arguments
            /// * `content` – The text.
            /// * `visible` - Whether the text is visible.
            fn add_text(content: &str, visible: bool) -> Text {
                AddText(content, visible)
            }
        }
        {
            /// Add an input field to the canvas.
            ///
            /// **Game Scenes**: All
            ///
            /// # Arguments
            /// * `visible` - Whether the input field is visible.
            fn add_input_field(visible: bool) -> InputField {
                AddInputField(visible)
            }
        }
        {
            /// Add a button to the canvas.
            ///
            /// **Game Scenes**: All
            ///
            /// # Arguments
            /// * `content` – The label for the button.
            /// * `visible` - Whether the button is visible.
            fn add_button(content: &str, visible: bool) -> Button {
                AddButton(content, visible)
            }
        }
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
