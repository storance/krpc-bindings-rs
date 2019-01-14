use crate::*;
use crate::codec::*;
use super::{CelestialBody, Vessel, Node};

use std::rc::Rc;

remote_type!(
/// Controls the game’s camera. Obtained by calling `SpaceCenter::camera()`.
object SpaceCenter.Camera {
    properties: {
        {
            Mode: CameraMode,
            /// Returns the current mode of the camera.
            ///
            /// **Game Scenes**: Flight
            get: mode,
            /// Sets the current mode of the camera.
            ///
            /// **Game Scenes**: Flight
            set: set_mode
        }
        {
            Pitch: f32,
            /// Returns the pitch of the camera, in degrees.
            ///
            /// **Game Scenes**: Flight
            get: pitch,
            /// Sets the pitch of the camera, in degrees.  A value between
            /// `Camera::min_pitch()` and `Camera::max_pitch()`.
            ///
            /// **Game Scenes**: Flight
            set: set_pitch
        }
        {
            Heading: f32,
            /// Returns the heading of the camera, in degrees.
            ///
            /// **Game Scenes**: Flight
            get: heading,
            /// Sets the heading of the camera, in degrees.
            ///
            /// **Game Scenes**: Flight
            set: set_heading
        }
        {
            Distance: f32,
            /// Returns the distance from the camera to the subject, in meters.
            ///
            /// **Game Scenes**: Flight
            get: distance,
            /// Sets the distance from the camera to the subject, in meters. A value between
            /// `Camera::min_distance()` and `Camera::max_distance()`.
            ///
            /// **Game Scenes**: Flight
            set: set_distance
        }
        {
            MinPitch: f32,
            /// Returns the minimum pitch of the camera.
            ///
            /// **Game Scenes**: Flight
            get: min_pitch
        }
        {
            MaxPitch: f32,
            /// Returns the maximum pitch of the camera.
            ///
            /// **Game Scenes**: Flight
            get: max_pitch
        }
        {
            MinDistance: f32,
            /// Returns the minimum distance from the camera to the subject, in meters.
            ///
            /// **Game Scenes**: Flight
            get: min_distance
        }
        {
            MaxDistance: f32,
            /// Returns the maximum distance from the camera to the subject, in meters.
            ///
            /// **Game Scenes**: Flight
            get: max_distance
        }
        {
            DefaultDistance: f32,
            /// Returns the default distance from the camera to the subject, in meters.
            ///
            /// **Game Scenes**: Flight
            get: default_distance
        }
        {
            FocussedBody: Option<CelestialBody>,
            /// In map mode, returns the celestial body that the camera is focussed on.
            /// Returns `None` if the camera is not focussed on a celestial body.
            /// Returns an error if the camera is not in map mode.
            ///
            /// **Game Scenes**: Flight
            get: focussed_body,
            /// Sets the celestial body that the camera is focussed on.  Returns an error if
            /// the camera is not in map mode.
            ///
            /// **Game Scenes**: Flight
            set: set_focussed_body
        }
        {
            FocussedVessel: Option<Vessel>,
            /// In map mode, returns the vessel that the camera is focussed on.
            /// Returns `None` if the camera is not focussed on a vessel.
            /// Returns an error if the camera is not in map mode.
            ///
            /// **Game Scenes**: Flight
            get: focussed_vessel,
            /// Sets the vessel that the camera is focussed on.  Returns an error if
            /// the camera is not in map mode.
            ///
            /// **Game Scenes**: Flight
            set: set_focussed_vessel
        }
        {
            FocussedNode: Option<Node>,
            /// In map mode, returns the maneuver node that the camera is focussed on.
            /// Returns `None` if the camera is not focussed on a maneuver node.
            /// Returns an error if the camera is not in map mode.
            ///
            /// **Game Scenes**: Flight
            get: focussed_node,
            /// Sets the maneuver node that the camera is focussed on.  Returns an error if
            /// the camera is not in map mode.
            ///
            /// **Game Scenes**: Flight
            set: set_focussed_node
        }
    }
});

remote_type!(
    /// See `Camera::mode()`.
    enum CameraMode {
        /// The camera is showing the active vessel, in "auto" mode.
        Automatic => 0,
        /// The camera is showing the active vessel, in "free" mode.
        Free => 1,
        /// The camera is showing the active vessel, in "chase" mode.
        Chase => 2,
        /// The camera is showing the active vessel, in "locked" mode.
        Locked => 3,
        /// The camera is showing the active vessel, in "orbital" mode.
        Orbital => 4,
        /// The Intra-Vehicular Activity view is being shown.
        IVA => 5,
        /// The map view is being shown.
        Map => 6
    }
);