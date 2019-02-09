use crate::codec::{Decode, Encode};
use crate::{remote_type, RemoteEnum, RemoteObject};

use crate::spacecenter::CelestialBody;

remote_type!(
    /// Represents an alarm.
    object KerbalAlarmClock.Alarm {
        properties: {
            {
                Action {
                    /// Returns the action that the alarm triggers.
                    ///
                    /// **Game Scenes**: All
                    get: action -> AlarmAction,
                    /// Sets the action that the alarm triggers.
                    ///
                    /// **Game Scenes**: All
                    set: set_action(AlarmAction)
                }
            }
            {
                Margin {
                    /// Returns the number of seconds before the event that the alarm will fire.
                    ///
                    /// **Game Scenes**: All
                    get: margin -> f64,
                    /// Sets the number of seconds before the event that the alarm will fire.
                    ///
                    /// **Game Scenes**: All
                    set: set_margin(f64)
                }
            }
            {
                Time {
                    /// Returns the time at which the alarm will fire.
                    ///
                    /// **Game Scenes**: All
                    get: time -> f64,
                    /// Sets the time at which the alarm will fire.
                    ///
                    /// **Game Scenes**: All
                    set: set_time(f64)
                }
            }
            {
                Type {
                    /// Returns the type of the alarm.
                    ///
                    /// **Game Scenes**: All
                    get: alarm_type -> AlarmType,
                    /// Sets the type of the alarm.
                    ///
                    /// **Game Scenes**: All
                    set: set_alarm_type(AlarmType)
                }
            }
            {
                ID {
                    /// Returns the unique identifier for the alarm.
                    ///
                    /// **Game Scenes**: All
                    get: alarm_id -> String
                }
            }
            {
                Name {
                    /// Returns the short name of the alarm.
                    ///
                    /// **Game Scenes**: All
                    get: name -> String,
                    /// Sets the short name of the alarm.
                    ///
                    /// **Game Scenes**: All
                    set: set_name(&str)
                }
            }
            {
                Notes {
                    /// Returns the long description of the alarm.
                    ///
                    /// **Game Scenes**: All
                    get: notes -> String,
                    /// Sets the long description of the alarm.
                    ///
                    /// **Game Scenes**: All
                    set: set_notes(&str)
                }
            }
            {
                Remaining {
                    /// Returns the number of seconds until the alarm will fire.
                    ///
                    /// **Game Scenes**: All
                    get: remaining -> f64
                }
            }
            {
                Repeat {
                    /// Returns whether the alarm will be repeated after it has fired.
                    ///
                    /// **Game Scenes**: All
                    get: is_repeating -> bool,
                    /// Sets the whether the alarm will be repeated after it has fired.
                    ///
                    /// **Game Scenes**: All
                    set: set_repeating(bool)
                }
            }
            {
                RepeatPeriod {
                    /// Returns the time delay to automatically create an alarm after it has fired.
                    ///
                    /// **Game Scenes**: All
                    get: repeat_period -> f64,
                    /// Sets the time delay to automatically create an alarm after it has fired.
                    ///
                    /// **Game Scenes**: All
                    set: set_repeat_period(f64)
                }
            }
            {
                TransferOriginBody {
                    /// Returns the celestial body the vessel is departing from.
                    ///
                    /// **Game Scenes**: All
                    get: transfer_origin_body -> Option<CelestialBody>,
                    /// Sets the celestial body the vessel is departing from.
                    ///
                    /// **Game Scenes**: All
                    set: set_transfer_origin_body(Option<&CelestialBody>)
                }
            }
            {
                TransferTargetBody {
                    /// Returns the celestial body the vessel is arriving at.
                    ///
                    /// **Game Scenes**: All
                    get: transfer_target_body -> Option<CelestialBody>,
                    /// Sets the celestial body the vessel is arriving at.
                    ///
                    /// **Game Scenes**: All
                    set: set_transfer_target_body(Option<&CelestialBody>)
                }
            }
        }
        methods: {
            {
                /// Removes the alarm.
                ///
                /// **Game Scenes**: All
                fn remove() {
                    Remove()
                }
            }
        }
    }
);

remote_type!(
    /// The type of an alarm.
    enum AlarmType {
        /// An alarm for a specific date/time or a specific period in the future.
        Raw = 0,
        /// An alarm based on the next maneuver node on the current ships flight path.
        /// This node will be stored and can be restored when you come back to the ship.
        Maneuver = 1,
        /// See `Maneuver`.
        ManeuverAuto = 2,
        /// An alarm for furthest part of the orbit from the planet.
        Apoapsis = 3,
        /// An alarm for nearest part of the orbit from the planet.
        Periapsis = 4,
        /// Ascending node for the targeted object, or equatorial ascending node.
        AscendingNode = 5,
        /// Descending node for the targeted object, or equatorial descending node.
        DescendingNode = 6,
        /// An alarm based on the closest approach of this vessel to the targeted
        /// vessel, some number of orbits into the future.
        Closest = 7,
        /// An alarm based on the expiry or deadline of contracts in career modes.
        Contract = 8,
        /// See `Contract`.
        ContractAuto = 9,
        /// An alarm that is attached to a crew member.
        Crew = 10,
        /// An alarm that is triggered when a selected target comes within a chosen distance.
        Distance = 11,
        /// An alarm based on the time in the "Earth" alternative Universe (aka the Real World).
        EarthTime = 12,
        /// An alarm that fires as your landed craft passes under the orbit of your target.
        LaunchRendevous = 13,
        /// An alarm manually based on when the next SOI point is on the flight path
        /// or set to continually monitor the active flight path and add alarms as it
        /// detects SOI changes.
        SOIChange = 14,
        /// See `SOIChange`.
        SOIChangeAuto = 15,
        /// An alarm based on Interplanetary Transfer Phase Angles, i.e. when should
        /// I launch to planet X? Based on Kosmo Not's post and used in Olex's
        /// Calculator.
        Transfer = 16,
        /// See `Transfer`.
        TransferModelled = 17,
    }
);

remote_type!(
    /// The action performed by an alarm when it fires.
    enum AlarmAction {
        /// Don't do anything at all...
        DoNothing = 0,
        /// Don't do anything, and delete the alarm.
        DoNothingDeleteWhenPassed = 1,
        /// Drop out of time warp.
        KillWarp = 2,
        /// Drop out of time warp.
        KillWarpOnly = 3,
        /// Display a message.
        MessageOnly = 4,
        /// Pause the game.
        PauseGame = 5,
    }
);
