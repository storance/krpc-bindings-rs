use super::RectTransform;
use crate::codec::{Decode, Encode};
use crate::{remote_type, RemoteEnum, RemoteObject, Vector3};

remote_type!(
/// A text label.
object UI.Text {
    properties: {
        {
            RectTransform: RectTransform,
            /// Returns the rect transform for the text.
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
            Content: String,
            /// Returns the text string.
            ///
            /// **Game Scenes**: All
            get: content,
            /// Sets the text string.
            ///
            /// **Game Scenes**: All
            set: set_content
        }
        {
            Font: String,
            /// Returns the name of the font.
            ///
            /// **Game Scenes**: All
            get: font,
            /// Sets the name of the font.
            ///
            /// **Game Scenes**: All
            set: set_font
        }
        {
            AvailableFonts: Vec<String>,
            /// Returns a list of all available fonts.
            ///
            /// **Game Scenes**: All
            get: available_fonts
        }
        {
            Size: i32,
            /// Returns the font size.
            ///
            /// **Game Scenes**: All
            get: size,
            /// Sets the font size.
            ///
            /// **Game Scenes**: All
            set: set_size
        }
        {
            FontStyle: FontStyle,
            /// Returns the font style.
            ///
            /// **Game Scenes**: All
            get: font_style,
            /// Sets the font style.
            ///
            /// **Game Scenes**: All
            set: set_font_style
        }
        {
            Color: Vector3,
            /// Returns the color.
            ///
            /// **Game Scenes**: All
            get: color,
            /// Sets the color.
            ///
            /// **Game Scenes**: All
            set: set_color
        }
        {
            Alignment: TextAnchor,
            /// Returns the text alignment.
            ///
            /// **Game Scenes**: All
            get: alignment,
            /// Sets the text alignment.
            ///
            /// **Game Scenes**: All
            set: set_alignment
        }
        {
            LineSpacing: f32,
            /// Returns the line spacing.
            ///
            /// **Game Scenes**: All
            get: line_spacing,
            /// Sets the line spacing.
            ///
            /// **Game Scenes**: All
            set: set_line_spacing
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
