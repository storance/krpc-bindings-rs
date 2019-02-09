use super::RectTransform;
use crate::codec::{Decode, Encode};
use crate::{remote_type, RemoteEnum, RemoteObject, Vector3};

remote_type!(
/// A text label.
object UI.Text {
    properties: {
        {
            RectTransform {
                /// Returns the rect transform for the text.
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
            Content {
                /// Returns the text string.
                ///
                /// **Game Scenes**: All
                get: content -> String,
                /// Sets the text string.
                ///
                /// **Game Scenes**: All
                set: set_content(&str)
            }
        }
        {
            Font {
                /// Returns the name of the font.
                ///
                /// **Game Scenes**: All
                get: font -> String,
                /// Sets the name of the font.
                ///
                /// **Game Scenes**: All
                set: set_font(&str)
            }
        }
        {
            AvailableFonts {
                /// Returns a list of all available fonts.
                ///
                /// **Game Scenes**: All
                get: available_fonts -> Vec<String>
            }
        }
        {
            Size {
                /// Returns the font size.
                ///
                /// **Game Scenes**: All
                get: size -> i32,
                /// Sets the font size.
                ///
                /// **Game Scenes**: All
                set: set_size(i32)
            }
        }
        {
            FontStyle {
                /// Returns the font style.
                ///
                /// **Game Scenes**: All
                get: font_style -> FontStyle,
                /// Sets the font style.
                ///
                /// **Game Scenes**: All
                set: set_font_style(FontStyle)
            }
        }
        {
            Color {
                /// Returns the color.
                ///
                /// **Game Scenes**: All
                get: color -> Vector3,
                /// Sets the color.
                ///
                /// **Game Scenes**: All
                set: set_color(Vector3)
            }
        }
        {
            Alignment {
                /// Returns the text alignment.
                ///
                /// **Game Scenes**: All
                get: alignment -> TextAnchor,
                /// Sets the text alignment.
                ///
                /// **Game Scenes**: All
                set: set_alignment(TextAnchor)
            }
        }
        {
            LineSpacing {
                /// Returns the line spacing.
                ///
                /// **Game Scenes**: All
                get: line_spacing -> f32,
                /// Sets the line spacing.
                ///
                /// **Game Scenes**: All
                set: set_line_spacing(f32)
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

remote_type!(
    /// Font Style.
    enum FontStyle {
        /// Normal.
        Normal = 0,
        /// Bold.
        Bold = 1,
        /// Italic.
        Italic = 2,
        /// Bold and Italic.
        BoldAndItalic = 3,
    }
);

remote_type!(
    /// Text alignment.
    enum TextAlignment {
        /// Left aligned.
        Left = 0,
        /// Right aligned.
        Right = 1,
        /// Center aligned.
        Center = 2,
    }
);

remote_type!(
    /// Text anchor.
    enum TextAnchor {
        /// Lower center.
        LowerCenter = 0,
        /// Lower left.
        LowerLeft = 1,
        /// Lower right.
        LowerRight = 2,
        /// Middle center.
        MiddleCenter = 3,
        /// Middle left.
        MiddleLeft = 4,
        /// Middle right.
        MiddleRight = 5,
        /// Upper center.
        UpperCenter = 6,
        /// Upper left.
        UpperLeft = 7,
        /// Upper right.
        UpperRight = 8,
    }
);
