use crate::codec::{Decode, Encode};
use crate::{remote_type, RemoteEnum, Vector3};

mod button;
mod canvas;
mod input_field;
mod panel;
mod rect_transform;
mod text;

pub use self::button::*;
pub use self::canvas::*;
pub use self::input_field::*;
pub use self::panel::*;
pub use self::rect_transform::*;
pub use self::text::*;

remote_type!(
/// Provides functionality for drawing and interacting with in-game user interface elements.
service UI {
    properties: {
        {
            StockCanvas: Canvas,
            /// The stock UI canvas.
            ///
            /// **Game Scenes**: All
            get: stock_canvas
        }
    }
    methods: {
        {
            /// Add a new canvas.
            ///
            /// **Game Scenes**: All
            ///
            /// # Note
            /// If you want to add UI elements to KSPs stock UI canvas, use `stock_canvas()`.
            fn add_canvas() -> Canvas {
                AddCanvas()
            }
        }
        {
            /// Display a message on the screen.
            ///
            /// **Game Scenes**: All
            ///
            /// # Arguments
            /// * `content` – Message content.
            /// * `duration` – Duration before the message disappears, in seconds.
            /// * `position` – Position to display the message.
            /// * `color` – The color of the message.
            /// * `size` – Size of the message, differs per position.
            ///
            /// # Note
            /// The message appears just like a stock message, for example quicksave
            /// or quickload messages.
            fn message(content: &str,
                       duration: f32,
                       position: MessagePosition,
                       color: Vector3,
                       size: f32) -> Canvas {
                Message(content, duration, position, color, size)
            }
        }
        {
            /// Remove all user interface elements.
            ///
            /// **Game Scenes**: All
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
    /// Message position.
    enum MessagePosition {
        /// Top Left.
        TopLeft = 0,
        /// Top Center.
        TopCenter = 1,
        /// Top Right.
        TopRight = 2,
        /// Bottom Center.
        BottomCenter = 3,
    }
);
