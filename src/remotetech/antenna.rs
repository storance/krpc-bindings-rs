use crate::codec::{Decode, Encode};
use crate::spacecenter::{CelestialBody, Part, Vessel};
use crate::{remote_type, RemoteEnum, RemoteObject};

remote_type!(
/// A RemoteTech antenna.
object RemoteTech.Antenna {
    properties: {
        {
            Part {
                /// Returns the part containing this antenna.
                ///
                /// **Game Scenes**: All
                get: part -> Part
            }
        }
        {
            HasConnection {
                /// Returns whether the antenna has a connection.
                ///
                /// **Game Scenes**: All
                get: has_connection -> bool
            }
        }
        {
            Target {
                /// Returns the object that the antenna is targetting.
                ///
                /// **Game Scenes**: All
                get: target -> Target,
                /// Sets the object that the antenna is targetting.  This property can be used to set
                /// the target to Target.NONE or Target.ACTIVE_VESSEL. To set the target to a
                /// celestial body, ground station or vessel see `Antenna::set_target_body()`,
                /// `Antenna::set_target_ground_station()` and `Antenna::set_target_vessel()`.
                ///
                /// **Game Scenes**: All
                set: set_target(Target)
            }
        }
        {
            TargetBody {
                /// Returns the celestial body the antenna is targetting.
                ///
                /// **Game Scenes**: All
                get: target_body -> Option<CelestialBody>,
                /// Sets the celestial body the antenna is targetting.
                ///
                /// **Game Scenes**: All
                set: set_target_body(&CelestialBody)
            }
        }
        {
            TargetGroundStation {
                /// Returns the ground station the antenna is targetting.
                ///
                /// **Game Scenes**: All
                get: target_ground_station -> String,
                /// Sets the ground station the antenna is targetting.
                ///
                /// **Game Scenes**: All
                set: set_target_ground_station(&str)
            }
        }
        {
            TargetVessel {
                /// Returns the vessel the antenna is targetting.
                ///
                /// **Game Scenes**: All
                get: target_vessel -> Option<Vessel>,
                /// Sets the vessel the antenna is targetting.
                ///
                /// **Game Scenes**: All
                set: set_target_vessel(&Vessel)
            }
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
