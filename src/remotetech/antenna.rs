use crate::codec::{Decode, Encode};
use crate::spacecenter::{CelestialBody, Part, Vessel};
use crate::{remote_type, RemoteEnum, RemoteObject};

remote_type!(
/// A RemoteTech antenna.
object RemoteTech.Antenna {
    properties: {
        {
            Part: Part,
            /// Returns the part containing this antenna.
            ///
            /// **Game Scenes**: All
            get: part
        }
        {
            HasConnection: bool,
            /// Returns whether the antenna has a connection.
            ///
            /// **Game Scenes**: All
            get: has_connection
        }
        {
            Target: Target,
            /// Returns the object that the antenna is targetting.
            ///
            /// **Game Scenes**: All
            get: target,
            /// Sets the object that the antenna is targetting.  This property can be used to set
            /// the target to Target.NONE or Target.ACTIVE_VESSEL. To set the target to a
            /// celestial body, ground station or vessel see `Antenna::set_target_body()`,
            /// `Antenna::set_target_ground_station()` and `Antenna::set_target_vessel()`.
            ///
            /// **Game Scenes**: All
            set: set_target
        }
        {
            TargetBody: Option<CelestialBody>,
            /// Returns the celestial body the antenna is targetting.
            ///
            /// **Game Scenes**: All
            get: target_body,
            /// Sets the celestial body the antenna is targetting.
            ///
            /// **Game Scenes**: All
            set: set_target_body
        }
        {
            TargetGroundStation: String,
            /// Returns the ground station the antenna is targetting.
            ///
            /// **Game Scenes**: All
            get: target_ground_station,
            /// Sets the ground station the antenna is targetting.
            ///
            /// **Game Scenes**: All
            set: set_target_ground_station
        }
        {
            TargetVessel: Option<Vessel>,
            /// Returns the vessel the antenna is targetting.
            ///
            /// **Game Scenes**: All
            get: target_vessel,
            /// Sets the vessel the antenna is targetting.
            ///
            /// **Game Scenes**: All
            set: set_target_vessel
        }
    }
});

remote_type!(
    /// The type of object an antenna is targetting.
    enum Target {
        /// The active vessel.
        ActiveVessel = 0,
        /// A celestial body.
        CelestialBody = 1,
        /// A ground station.
        GroundStation = 2,
        /// A specific vessel.
        Vessel = 3,
        /// No target.
        None = 4,
    }
);
