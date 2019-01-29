use crate::codec::{Decode, Encode};
use crate::{remote_type};

mod alarm;

pub use self::alarm::*;

remote_type!(
    /// This service provides functionality to interact with Kerbal Alarm Clock.
    service KerbalAlarmClock {
        properties: {
            {
                Available: bool,
                /// Returns whether Kerbal Alarm Clock is available.
                ///
                /// **Game Scenes**: All
                get: is_available
            }
            {
                Alarms: Vec<Alarm>,
                /// Returns a list of all the alarms.
                ///
                /// **Game Scenes**: All
                get: alarms
            }
        }
        methods: {
            {
                /// Get the alarm with the given name, or `None` if no alarms have that name.
                /// If more than one alarm has the name, only returns one of them.
                ///
                /// **Game Scenes**: All
                ///
                /// # Arguments
                /// `name` - Name of the alarm to search for.
                fn alarm_with_name(name: &str) -> Option<Alarm> {
                    AlarmWithName(name)
                }
            }
            {
                /// Get a list of alarms of the specified type.
                ///
                /// **Game Scenes**: All
                ///
                /// # Arguments
                /// `type` - Type of alarm to return.
                fn alarm_with_type(alarm_type: AlarmType) -> Vec<Alarm> {
                    AlarmWithType(alarm_type)
                }
            }
            {
                /// Get a list of alarms of the specified type.
                ///
                /// **Game Scenes**: All
                ///
                /// # Arguments
                /// `type` - Type of the new alarm.
                /// `name` – Name of the new alarm.
                /// `ut` – Time at which the new alarm should trigger.
                fn create_alarm(alarm_type: AlarmType, name: &str, ut: f64) -> Alarm {
                    CreateAlarm(alarm_type, name, ut)
                }
            }
        }
    }
);
