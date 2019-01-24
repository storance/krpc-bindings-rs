use super::Part;
use crate::codec::*;
use crate::*;
use crate::krpc::Expression;

use std::collections::BTreeMap;

remote_type!(
/// This can be used to interact with a specific part module. This includes part modules in
/// stock KSP, and those added by mods.
///
/// In KSP, each part has zero or more PartModules associated with it. Each one contains some of
/// the functionality of the part. For example, an engine has a “ModuleEngines” part module
/// that contains all the functionality of an engine.
object SpaceCenter.Module {
    properties: {
        {
            Name: String,
            /// Returns the name of the PartModule. For example, “ModuleEngines”.
            ///
            /// **Game Scenes**: All
            get: name
        }
        {
            Part: Part,
            /// The part that contains this module.
            ///
            /// **Game Scenes**: All
            get: part
        }
        {
            Fields: BTreeMap<String, String>,
            /// Returns the modules field names and their associated values, as a dictionary.
            /// These are the values visible in the right-click menu of the part.
            ///
            /// **Game Scenes**: All
            get: fields
        }
        {
            Events: Vec<String>,
            /// Returns a list of the names of all of the modules events. Events are the
            /// clickable buttons visible in the right-click menu of the part.
            ///
            /// **Game Scenes**: All
            get: events
        }
        {
            Actions: Vec<String>,
            /// Returns a list of all the names of the modules actions. These are the parts
            /// actions that can be assigned to action groups in the in-game editor.
            ///
            /// **Game Scenes**: All
            get: actions
        }
    }
    methods: {
        {
            /// Returns `true` if the module has a field with the given name.
            ///
            /// **Game Scenes**: All
            ///
            /// # Arguments
            /// * `name` - Name of the field.
            fn has_field(name: &str) -> bool {
                HasField(name)
            }
        }
        {
            /// Returns the value of a field.
            ///
            /// **Game Scenes**: All
            ///
            /// # Arguments
            /// * `name` - Name of the field.
            fn get_field(name: &str) -> String {
                GetField(name)
            }
        }
        {
            /// Set the value of a field to the given integer number.
            ///
            /// **Game Scenes**: All
            ///
            /// # Arguments
            /// * `name` - Name of the field.
            /// * `value` - Value to set.
            fn set_field_int(name: &str, value: i32) {
                SetFieldInt(name, value)
            }
        }
        {
            /// Set the value of a field to the given floating point number.
            ///
            /// **Game Scenes**: All
            ///
            /// # Arguments
            /// * `name` - Name of the field.
            /// * `value` - Value to set.
            fn set_field_float(name: &str, value: f32) {
                SetFieldFloat(name, value)
            }
        }
        {
            /// Set the value of a field to the given string.
            ///
            /// **Game Scenes**: All
            ///
            /// # Arguments
            /// * `name` - Name of the field.
            /// * `value` - Value to set.
            fn set_field_string(name: &str, value: &str) {
                SetFieldString(name, value)
            }
        }
        {
            /// Set the value of a field to its original value.
            ///
            /// **Game Scenes**: All
            ///
            /// # Arguments
            /// * `name` - Name of the field.
            fn reset_field(name: &str) -> String {
                ResetField(name)
            }
        }
        {
            /// Returns `true` if the module has an event with the given name.
            ///
            /// **Game Scenes**: All
            ///
            /// # Arguments
            /// * `name` - Name of the event.
            fn has_event(name: &str) -> bool {
                HasEvent(name)
            }
        }
        {
            /// Trigger the named event. Equivalent to clicking the button in the right-click
            /// menu of the part.
            ///
            /// **Game Scenes**: All
            ///
            /// # Arguments
            /// * `name` - Name of the event.
            fn trigger_event(name: &str) {
                TriggerEvent(name)
            }
        }
        {
            /// Returns `true` if the module has an action with the given name.
            ///
            /// **Game Scenes**: All
            ///
            /// # Arguments
            /// * `name` - Name of the event.
            fn has_action(name: &str) -> bool {
                HasAction(name)
            }
        }
        {
            /// Set the value of an action with the given name.
            ///
            /// **Game Scenes**: All
            ///
            /// # Arguments
            /// * `name` - Name of the event.
            /// * `value` - Value to set.
            fn set_action(name: &str, value: bool) {
                SetAction(name, value)
            }
        }
    }
});
