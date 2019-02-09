use super::{Button, InputField, RectTransform, Text};
use crate::codec::{Decode, Encode};
use crate::{remote_type, RemoteObject};

remote_type!(
/// A container for user interface elements.
object UI.Panel {
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
            /// Add text to the panel.
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
            /// Add an input field to the panel.
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
            /// Add a button to the panel.
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
