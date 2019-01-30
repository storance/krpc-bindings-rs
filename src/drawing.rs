use crate::codec::{Decode, Encode};
use crate::spacecenter::ReferenceFrame;
use crate::ui::{FontStyle, TextAlignment, TextAnchor};
use crate::{remote_type, Quaternion, RemoteObject, Vector3};

remote_type!(
/// Provides functionality for drawing objects in the flight scene.
service Drawing {
    properties: {}
    methods: {
        {
            /// Draw a line in the scene.
            ///
            /// **Game Scenes**: Flight
            ///
            /// # Arguments
            /// * `start` – Position of the start of the line.
            /// * `end` – Position of the end of the line.
            /// * `reference_frame` – Reference frame that the positions are in.
            /// * `visible` – Whether the line is visible.
            fn add_line(start: Vector3, end: Vector3, reference_frame: &ReferenceFrame, visible: bool) -> Line{
                AddLine(start, end, reference_frame, visible)
            }
        }
        {
            /// Draw a direction vector in the scene, from the center of mass of the active vessel.
            ///
            /// **Game Scenes**: Flight
            ///
            /// # Arguments
            /// * `direction` – Direction to draw the line in.
            /// * `reference_frame` – Reference frame that the positions are in.
            /// * `length` - The length of the line.
            /// * `visible` – Whether the line is visible.
            fn add_direction(direction: Vector3, reference_frame: &ReferenceFrame, length: f32, visible: bool) -> Line {
                AddDirection(direction, reference_frame, length, visible)
            }
        }
        {
            /// Draw a polygon in the scene, defined by a list of vertices.
            ///
            /// **Game Scenes**: Flight
            ///
            /// # Arguments
            /// * `vertices`  – Vertices of the polygon.
            /// * `reference_frame` – Reference frame that the vertices are in.
            /// * `visible` – Whether the polygon is visible.
            fn add_polygon(vertices: &[Vector3], reference_frame: &ReferenceFrame, visible: bool) -> Polygon {
                AddPolygon(vertices, reference_frame, visible)
            }
        }
        {
            /// Draw text in the scene.
            ///
            /// **Game Scenes**: Flight
            ///
            /// # Arguments
            /// * `text`  – The string to draw.
            /// * `reference_frame` – Reference frame that the text position is in.
            /// * `position` – Position of the text.
            /// * `rotation` – Rotation of the text, as a quaternion.
            /// * `visible` – Whether the polygon is visible.
            fn add_text(text: &str, reference_frame: &ReferenceFrame, position: Vector3, rotation: Quaternion, visible: bool) -> Text {
                AddText(text, reference_frame, position, rotation, visible)
            }
        }
        {
            /// Remove all objects being drawn.
            ///
            /// **Game Scenes**: Flight
            ///
            /// # Arguments
            /// * `client_only` - If true, only remove objects created by the calling client.
            fn clear(client_only: bool) {
                Clear(client_only)
            }
        }
    }
});

remote_type!(
/// A line.
object Drawing.Line {
    properties: {
        {
            Start: Vector3,
            /// Returns the start position of the line.
            ///
            /// **Game Scenes**: Flight
            get: start,
            /// Sets the start position of the line.
            ///
            /// **Game Scenes**: Flight
            set: set_start
        }
        {
            End: Vector3,
            /// Returns the end position of the line.
            ///
            /// **Game Scenes**: Flight
            get: end,
            /// Sets the end position of the line.
            ///
            /// **Game Scenes**: Flight
            set: set_end
        }
        {
            ReferenceFrame: ReferenceFrame,
            /// Returns the reference frame for the positions of the object.
            ///
            /// **Game Scenes**: Flight
            get: reference_frame,
            /// Sets the reference frame for the positions of the object.
            ///
            /// **Game Scenes**: Flight
            set: set_reference_frame
        }
        {
            Visible: bool,
            /// Returns whether the object is visible.
            ///
            /// **Game Scenes**: Flight
            get: is_visible,
            /// Sets whether the object is visible.
            ///
            /// **Game Scenes**: Flight
            set: set_visible
        }
        {
            Color: Vector3,
            /// Returns the color.
            ///
            /// **Game Scenes**: Flight
            get: color,
            /// Sets the color.
            ///
            /// **Game Scenes**: Flight
            set: set_color
        }
        {
            Material: String,
            /// Returns the material used to render the object.
            ///
            /// **Game Scenes**: Flight
            get: material,
            /// Sets the material used to render the object. Creates the material from
            /// a shader with the given name.
            ///
            /// **Game Scenes**: Flight
            set: set_material
        }
        {
            Thickness: f32,
            /// Returns the thickness.
            ///
            /// **Game Scenes**: Flight
            get: thickness,
            /// Sets the thickness.
            ///
            /// **Game Scenes**: Flight
            set: set_thickness
        }
    }
    methods: {
        {
            /// Remove the object.
            ///
            /// **Game Scenes**: Flight
            fn remove() {
                Remove()
            }
        }
    }
});

remote_type!(
/// A polygon.
object Drawing.Polygon {
    properties: {
        {
            Vertices: Vec<Vector3>,
            /// Returns the vertices for the polygon.
            ///
            /// **Game Scenes**: Flight
            get: vertices,
            /// Sets the vertices for the polygon.
            ///
            /// **Game Scenes**: Flight
            set: set_vertices
        }
        {
            ReferenceFrame: ReferenceFrame,
            /// Returns the reference frame for the positions of the object.
            ///
            /// **Game Scenes**: Flight
            get: reference_frame,
            /// Sets the reference frame for the positions of the object.
            ///
            /// **Game Scenes**: Flight
            set: set_reference_frame
        }
        {
            Visible: bool,
            /// Returns whether the object is visible.
            ///
            /// **Game Scenes**: Flight
            get: is_visible,
            /// Sets whether the object is visible.
            ///
            /// **Game Scenes**: Flight
            set: set_visible
        }
        {
            Color: Vector3,
            /// Returns the color.
            ///
            /// **Game Scenes**: Flight
            get: color,
            /// Sets the color.
            ///
            /// **Game Scenes**: Flight
            set: set_color
        }
        {
            Material: String,
            /// Returns the material used to render the object.
            ///
            /// **Game Scenes**: Flight
            get: material,
            /// Sets the material used to render the object. Creates the material from
            /// a shader with the given name.
            ///
            /// **Game Scenes**: Flight
            set: set_material
        }
        {
            Thickness: f32,
            /// Returns the thickness.
            ///
            /// **Game Scenes**: Flight
            get: thickness,
            /// Sets the thickness.
            ///
            /// **Game Scenes**: Flight
            set: set_thickness
        }
    }
    methods: {
        {
            /// Remove the object.
            ///
            /// **Game Scenes**: Flight
            fn remove() {
                Remove()
            }
        }
    }
});

remote_type!(
/// Text.
object Drawing.Text {
    properties: {
        {
            Position: Vector3,
            /// Returns the position of the text.
            ///
            /// **Game Scenes**: Flight
            get: position,
            /// Sets the position of the text.
            ///
            /// **Game Scenes**: Flight
            set: set_position
        }
        {
            Rotation: Quaternion,
            /// Returns the position of the text.
            ///
            /// **Game Scenes**: Flight
            get: rotation,
            /// Sets the position of the text.
            ///
            /// **Game Scenes**: Flight
            set: set_rotation
        }
        {
            ReferenceFrame: ReferenceFrame,
            /// Returns the reference frame for the positions of the object.
            ///
            /// **Game Scenes**: Flight
            get: reference_frame,
            /// Sets the reference frame for the positions of the object.
            ///
            /// **Game Scenes**: Flight
            set: set_reference_frame
        }
        {
            Visible: bool,
            /// Returns whether the object is visible.
            ///
            /// **Game Scenes**: Flight
            get: is_visible,
            /// Sets whether the object is visible.
            ///
            /// **Game Scenes**: Flight
            set: set_visible
        }
        {
            Content: String,
            /// Returns the text string.
            ///
            /// **Game Scenes**: Flight
            get: content,
            /// Sets the text string.
            ///
            /// **Game Scenes**: Flight
            set: set_content
        }
        {
            Font: String,
            /// Returns the name of the font.
            ///
            /// **Game Scenes**: Flight
            get: font,
            /// Sets the name of the font.
            ///
            /// **Game Scenes**: Flight
            set: set_font
        }
        {
            Size: i32,
            /// Returns the font size.
            ///
            /// **Game Scenes**: Flight
            get: size,
            /// Sets the font size.
            ///
            /// **Game Scenes**: Flight
            set: set_size
        }
        {
            CharacterSize: f32,
            /// Returns the character size.
            ///
            /// **Game Scenes**: Flight
            get: character_size,
            /// Sets the character size.
            ///
            /// **Game Scenes**: Flight
            set: set_character_size
        }
        {
            Style: FontStyle,
            /// Returns the font style.
            ///
            /// **Game Scenes**: Flight
            get: style,
            /// Sets the font style.
            ///
            /// **Game Scenes**: Flight
            set: set_style
        }
        {
            Material: String,
            /// Returns the material used to render the object.
            ///
            /// **Game Scenes**: Flight
            get: material,
            /// Sets the material used to render the object. Creates the material from
            /// a shader with the given name.
            ///
            /// **Game Scenes**: Flight
            set: set_material
        }
        {
            Alignment: TextAlignment,
            /// Returns the text alignment.
            ///
            /// **Game Scenes**: Flight
            get: alignment,
            /// Sets the text alignment.
            ///
            /// **Game Scenes**: Flight
            set: set_alignment
        }
        {
            Anchor: TextAnchor,
            /// Returns the text anchor.
            ///
            /// **Game Scenes**: Flight
            get: anchor,
            /// Sets the text anchor.
            ///
            /// **Game Scenes**: Flight
            set: set_anchor
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
            /// Remove the object.
            ///
            /// **Game Scenes**: Flight
            fn remove() {
                Remove()
            }
        }
    }
    static_methods: {
        {
            /// Remove the object.
            ///
            /// **Game Scenes**: Flight
            fn available_fonts() -> Vec<String> {
                AvailableFonts()
            }
        }
    }
});
