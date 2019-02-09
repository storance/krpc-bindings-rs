use super::{CelestialBody, Node, Vessel};
use crate::codec::{Decode, Encode};
use crate::{remote_type, RemoteEnum, RemoteObject};

remote_type!(
/// Controls the game’s camera. Obtained by calling `SpaceCenter::camera()`.
object SpaceCenter.Camera {
    properties: {
        {
            Mode {
                /// Returns the current mode of the camera.
                ///
                /// **Game Scenes**: Flight
                get: mode -> CameraMode,
                /// Sets the current mode of the camera.
                ///
                /// **Game Scenes**: Flight
                set: set_mode(CameraMode)
            }
        }
        {
            Pitch {
                /// Returns the pitch of the camera, in degrees.
                ///
                /// **Game Scenes**: Flight
                get: pitch -> f32,
                /// Sets the pitch of the camera, in degrees.  A value between
                /// `Camera::min_pitch()` and `Camera::max_pitch()`.
                ///
                /// **Game Scenes**: Flight
                set: set_pitch(f32)
            }
        }
        {
            Heading {
                /// Returns the heading of the camera, in degrees.
                ///
                /// **Game Scenes**: Flight
                get: heading -> f32,
                /// Sets the heading of the camera, in degrees.
                ///
                /// **Game Scenes**: Flight
                set: set_heading(f32)
            }
        }
        {
            Distance {
                /// Returns the distance from the camera to the subject, in meters.
                ///
                /// **Game Scenes**: Flight
                get: distance -> f32,
                /// Sets the distance from the camera to the subject, in meters. A value between
                /// `Camera::min_distance()` and `Camera::max_distance()`.
                ///
                /// **Game Scenes**: Flight
                set: set_distance(f32)
            }
        }
        {
            MinPitch {
                /// Returns the minimum pitch of the camera.
                ///
                /// **Game Scenes**: Flight
                get: min_pitch -> f32
            }
        }
        {
            MaxPitch {
                /// Returns the maximum pitch of the camera.
                ///
                /// **Game Scenes**: Flight
                get: max_pitch -> f32
            }
        }
        {
            MinDistance {
                /// Returns the minimum distance from the camera to the subject, in meters.
                ///
                /// **Game Scenes**: Flight
                get: min_distance -> f32
            }
        }
        {
            MaxDistance {
                /// Returns the maximum distance from the camera to the subject, in meters.
                ///
                /// **Game Scenes**: Flight
                get: max_distance -> f32
            }
        }
        {
            DefaultDistance {
                /// Returns the default distance from the camera to the subject, in meters.
                ///
                /// **Game Scenes**: Flight
                get: default_distance -> f32
            }
        }
        {
            FocussedBody {
                /// In map mode, returns the celestial body that the camera is focussed on.
                /// Returns `None` if the camera is not focussed on a celestial body.
                /// Returns an error if the camera is not in map mode.
                ///
                /// **Game Scenes**: Flight
                get: focussed_body -> Option<CelestialBody>,
                /// Sets the celestial body that the camera is focussed on.  Returns an error if
                /// the camera is not in map mode.
                ///
                /// **Game Scenes**: Flight
                set: set_focussed_body(&CelestialBody)
            }
        }
        {
            FocussedVessel {
                /// In map mode, returns the vessel that the camera is focussed on.
                /// Returns `None` if the camera is not focussed on a vessel.
                /// Returns an error if the camera is not in map mode.
                ///
                /// **Game Scenes**: Flight
                get: focussed_vessel -> Option<Vessel>,
                /// Sets the vessel that the camera is focussed on.  Returns an error if
                /// the camera is not in map mode.
                ///
                /// **Game Scenes**: Flight
                set: set_focussed_vessel(&Vessel)
            }
        }
        {
            FocussedNode {
                /// In map mode, returns the maneuver node that the camera is focussed on.
                /// Returns `None` if the camera is not focussed on a maneuver node.
                /// Returns an error if the camera is not in map mode.
                ///
                /// **Game Scenes**: Flight
                get: focussed_node -> Option<Node>,
                /// Sets the maneuver node that the camera is focussed on.  Returns an error if
                /// the camera is not in map mode.
                ///
                /// **Game Scenes**: Flight
                set: set_focussed_node(&Node)
            }
        }
    }
});

remote_type!(
    /// See `Camera::mode()`.
    enum CameraMode {
        /// The camera is showing the active vessel, in "auto" mode.
        Automatic = 0,
        /// The camera is showing the active vessel, in "free" mode.
        Free = 1,
        /// The camera is showing the active vessel, in "chase" mode.
        Chase = 2,
        /// The camera is showing the active vessel, in "locked" mode.
        Locked = 3,
        /// The camera is showing the active vessel, in "orbital" mode.
        Orbital = 4,
        /// The Intra-Vehicular Activity view is being shown.
        IVA = 5,
        /// The map view is being shown.
        Map = 6,
    }
);
