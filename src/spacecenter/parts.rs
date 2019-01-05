use crate::*;
use crate::codec::*;

use std::rc::{Rc};
use std::cell::{RefCell};

mod antenna;
mod cargo_bay;
mod control_surface;
mod decoupler;
mod docking_port;
mod engine;
mod experiment;
mod fairing;
mod intake;
mod launch_clamp;
mod leg;
mod light;
mod module;
mod parachute;
mod radiator;
mod rcs;
mod reaction_wheel;
mod resource_converter;
mod resource_harvester;
mod sensor;
mod solar_panel;
mod thruster;
mod wheel;

pub use self::antenna::*;
pub use self::cargo_bay::*;
pub use self::control_surface::*;
pub use self::decoupler::*;
pub use self::docking_port::*;
pub use self::engine::*;
pub use self::experiment::*;
pub use self::fairing::*;
pub use self::intake::*;
pub use self::launch_clamp::*;
pub use self::leg::*;
pub use self::light::*;
pub use self::module::*;
pub use self::parachute::*;
pub use self::radiator::*;
pub use self::rcs::*;
pub use self::reaction_wheel::*;
pub use self::resource_converter::*;
pub use self::resource_harvester::*;
pub use self::sensor::*;
pub use self::solar_panel::*;
pub use self::thruster::*;
pub use self::wheel::*;

remote_type!(
/// Instances of this class are used to interact with the parts of a vessel. An instance can
/// be obtained by calling `Vessel::parts()`.
object Parts {
    service: SpaceCenter,
    properties: {
        {
            All: Vec<Part>,
            get: all
        }
    }
});


remote_type!(
/// Represents an individual part. Vessels are made up of multiple parts. Instances of this
/// class can be obtained by several methods in `Parts`.
object Part {
    service: SpaceCenter,
    properties: {
        {
            Name: String,
            get: name
        }
    }
});


remote_type!(
/// Obtained by calling `Part::add_force()`.
object Force {
    service: SpaceCenter,
    properties: {

    }
});