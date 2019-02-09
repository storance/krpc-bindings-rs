use super::{ReferenceFrame, Resources, Vessel};
use crate::codec::{Decode, Encode};
use crate::{remote_type, Quaternion, RemoteObject, Vector3};

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
object SpaceCenter.Parts {
    properties: {
        {
            All {
                /// Returns a list of all of the vessels parts.
                ///
                /// **Game Scenes**: All
                get: all -> Vec<Part>
            }
        }
        {
            Root {
                /// Returns the vessel's root part.
                ///
                /// **Game Scenes**: All
                get: root -> Part
            }
        }
        {
            ControllingPart {
                /// Returns the part from which the vessel is controlled.
                ///
                /// **Game Scenes**: All
                get: controlling_part -> Part,
                /// Sets the part from which the vessel is controlled.
                ///
                /// **Game Scenes**: All
                set: set_controlling_part(&Part)
            }
        }
        {
            Antennas {
                /// Returns a list of all antennas in the vessel.
                ///
                /// **Game Scenes**: All
                get: antennas -> Vec<Antenna>
            }
        }
        {
            CargoBays {
                /// Returns a list of all cargo bays in the vessel.
                ///
                /// **Game Scenes**: All
                get: cargo_bays -> Vec<CargoBay>
            }
        }
        {
            ControlSurfaces {
                /// Returns a list of all control surfaces in the vessel.
                ///
                /// **Game Scenes**: All
                get: control_surfaces -> Vec<ControlSurface>
            }
        }
        {
            Decouplers {
                /// Returns a list of all decouplers in the vessel.
                ///
                /// **Game Scenes**: All
                get: decouplers -> Vec<Decoupler>
            }
        }
        {
            DockingPorts {
                /// Returns a list of all docking ports in the vessel.
                ///
                /// **Game Scenes**: All
                get: docking_ports -> Vec<DockingPort>
            }
        }
        {
            Engines {
                /// Returns a list of all engines in the vessel.
                ///
                /// **Game Scenes**: All
                ///
                /// # Note
                /// This includes any part that generates thrust. This covers many different types
                /// of engine, including liquid fuel rockets, solid rocket boosters,
                /// jet engines and RCS thrusters.
                get: engines -> Vec<Engine>
            }
        }
        {
            Experiments {
                /// Returns a list of all experiments in the vessel.
                ///
                /// **Game Scenes**: All
                get: experiments -> Vec<Experiment>
            }
        }
        {
            Fairings {
                /// Returns a list of all fairings in the vessel.
                ///
                /// **Game Scenes**: All
                get: fairings -> Vec<Fairing>
            }
        }
        {
            Intakes {
                /// Returns a list of all intakes in the vessel.
                ///
                /// **Game Scenes**: All
                get: intakes -> Vec<Intake>
            }
        }
        {
            Legs {
                /// Returns a list of all landing legs in the vessel.
                ///
                /// **Game Scenes**: All
                get: legs -> Vec<Leg>
            }
        }
        {
            LaunchClamps {
                /// Returns a list of all launch clamps attached to the vessel.
                ///
                /// **Game Scenes**: All
                get: launch_clamps ->Vec<LaunchClamp>
            }
        }
        {
            Lights {
                /// Returns a list of all lights in the vessel.
                ///
                /// **Game Scenes**: All
                get: lights -> Vec<Light>
            }
        }
        {
            Parachutes {
                /// Returns a list of all parachutes in the vessel.
                ///
                /// **Game Scenes**: All
                get: parachutes -> Vec<Parachute>
            }
        }
        {
            Radiators {
                /// Returns a list of all radiators in the vessel.
                ///
                /// **Game Scenes**: All
                get: radiators -> Vec<Radiator>
            }
        }
        {
            RCS {
                /// Returns a list of all RCS blocks/thrusters in the vessel.
                ///
                /// **Game Scenes**: All
                get: rcs -> Vec<RCS>
            }
        }
        {
            ReactionWheels {
                /// Returns a list of all reaction wheels in the vessel.
                ///
                /// **Game Scenes**: All
                get: reaction_wheels -> Vec<ReactionWheel>
            }
        }
        {
            ResourceConverters {
                /// Returns a list of all resource converters in the vessel.
                ///
                /// **Game Scenes**: All
                get: resource_converters -> Vec<ResourceConverter>
            }
        }
        {
            ResourceHarvesters {
                /// Returns a list of all resource harvesters in the vessel.
                ///
                /// **Game Scenes**: All
                get: resource_harvesters -> Vec<ResourceHarvester>
            }
        }
        {
            Sensors {
                /// Returns a list of all sensors in the vessel.
                ///
                /// **Game Scenes**: All
                get: sensors -> Vec<Sensor>
            }
        }
        {
            SolarPanels {
                /// Returns a list of all solar panels in the vessel.
                ///
                /// **Game Scenes**: All
                get: solar_panels -> Vec<SolarPanel>
            }
        }
        {
            Wheels {
                /// Returns a list of all wheels in the vessel.
                ///
                /// **Game Scenes**: All
                get: wheels -> Vec<Wheel>
            }
        }
    }
    methods: {
        {
            /// Returns a list of parts whose `Part::name()` is `name`.
            ///
            /// **Game Scenes**: All
            ///
            /// # Arguments
            /// * `name` - The name of the parts to find.
            fn with_name(name: &str) -> Vec<Part> {
                WithName(name)
            }
        }
        {
            /// Returns a list of parts whose `Part::title()` is `title`.
            ///
            /// **Game Scenes**: All
            ///
            /// # Arguments
            /// * `title` - The title of the parts to find.
            fn with_title(title: &str) -> Vec<Part> {
                WithTitle(title)
            }
        }
        {
            /// Returns a list of parts whose `Part::tag()` is `tag`.
            ///
            /// **Game Scenes**: All
            ///
            /// # Arguments
            /// * `tag` - The tag of the parts to find.
            fn with_tag(tag: &str) -> Vec<Part> {
                WithTag(tag)
            }
        }
        {
            /// Returns a list of all parts that contain a `Module` whose `Module::name()`
            /// is `module_name`.
            ///
            /// **Game Scenes**: All
            ///
            /// # Arguments
            /// * `module_name` - The name of the module.
            fn with_module(module_name: &str) -> Vec<Part> {
                WithModule(module_name)
            }
        }
        {
            /// Returns a list of all parts that are activated in the given stage.
            ///
            /// **Game Scenes**: All
            ///
            /// # Arguments
            /// * `stage` - The stage number.
            fn in_stage(stage: i32) -> Vec<Part> {
                InStage(stage)
            }
        }
        {
            /// Returns a list of all parts that are decoupled in the given stage.
            ///
            /// **Game Scenes**: All
            ///
            /// # Arguments
            /// * `stage` - The stage number.
            fn in_decoupler_stage(stage: i32) -> Vec<Part> {
                InDecouplerStage(stage)
            }
        }
        {
            /// Returns a list of modules (combined across all parts in the vessel) whose
            /// `Module::name()` is `module_name`.
            ///
            /// **Game Scenes**: All
            ///
            /// # Arguments
            /// * `module_name` - The name of the module.
            fn modules_with_name(module_name: &str) -> Vec<Module> {
                ModulesWithName(module_name)
            }
        }
    }
});

remote_type!(
/// Represents an individual part. Vessels are made up of multiple parts. Instances of this
/// class can be obtained by several methods in `Parts`.
object SpaceCenter.Part {
    properties: {
        {
            Name {
                /// Returns the internal name of the part, as used in part cfg files.
                /// For example “Mark1-2Pod”.
                ///
                /// **Game Scenes**: All
                get: name -> String
            }
        }
        {
            Title {
                /// Returns the title of the part, as shown when the part is right clicked in-game.
                /// For example “Mk1-2 Command Pod”.
                ///
                /// **Game Scenes**: All
                get: title -> String
            }
        }
        {
            Tag {
                /// Returns the name tag for the part. Can be set to a custom string using
                /// the in-game user interface.
                ///
                /// **Game Scenes**: All
                get: tag -> String,
                /// Sets the name tag for the part. Can be set to a custom string using the
                /// in-game user interface.
                ///
                /// **Game Scenes**: All
                set: set_tag(&str)
            }
        }
        {
            Highlighted {
                /// Returns whether the part is highlighted.
                ///
                /// **Game Scenes**: All
                get: is_highlighted -> bool,
                /// Sets whether the part is highlighted.
                ///
                /// **Game Scenes**: All
                set: set_highlighted(bool)
            }
        }
        {
            HighlightColor {
                /// Returns the color used to highlight the part, as an RGB triple.
                ///
                /// **Game Scenes**: All
                get: highlight_color -> Vector3,
                /// Sets the color used to highlight the part, as an RGB triple.
                ///
                /// **Game Scenes**: All
                set: set_highlight_color(Vector3)
            }
        }
        {
            Cost {
                /// Returns the cost of the part, in units of funds.
                ///
                /// **Game Scenes**: All
                get: cost -> f64
            }
        }
        {
            Vessel {
                /// Returns the cost of the part, in units of funds.
                ///
                /// **Game Scenes**: All
                get: vessel -> Vessel
            }
        }
        {
            Parent {
                /// Returns the part's parent or `None` if the part does not have a parent.
                /// This, in combination with `Part::children()`, can be used to traverse
                /// the vessels parts tree.
                ///
                /// **Game Scenes**: All
                get: parent -> Option<Part>
            }
        }
        {
            Children {
                /// Returns the part's children. Returns an empty list if the part has no children.
                /// This, in combination with `Part::parent()`, can be used to traverse the
                /// vessels parts tree.
                ///
                /// **Game Scenes**: All
                get: children -> Vec<Part>
            }
        }
        {
            AxiallyAttached {
                /// Returns whether the part is axially attached to its parent, i.e. on the top
                /// or bottom of its parent. If the part has no parent, returns `false`.
                ///
                /// **Game Scenes**: All
                get: is_axially_attached -> bool
            }
        }
        {
            RadiallyAttached {
                /// Returns whether the part is radially attached to its parent, i.e. on the side
                /// of its parent. If the part has no parent, returns `false`.
                ///
                /// **Game Scenes**: All
                get: is_radially_attached -> bool
            }
        }
        {
            Stage {
                /// Returns the stage in which this part will be activated. Returns -1 if the
                /// part is not activated by staging.
                ///
                /// **Game Scenes**: All
                get: stage -> i32
            }
        }
        {
            DecoupleStage {
                /// Returns the stage in which this part will be decoupled. Returns -1 if the part is
                /// never decoupled from the vessel.
                ///
                /// **Game Scenes**: All
                get: decouple_stage -> i32
            }
        }
        {
            Massless {
                /// Returns whether the part is [massless](https://wiki.kerbalspaceprogram.com/wiki/Massless_part).
                ///
                /// **Game Scenes**: All
                get: is_massless -> bool
            }
        }
        {
            Mass {
                /// Returns the current mass of the part, including resources it contains,
                /// in kilograms. Returns zero if the part is massless.
                ///
                /// **Game Scenes**: All
                get: mass -> f64
            }
        }
        {
            DryMass {
                /// Returns the mass of the part, not including any resources it contains,
                /// in kilograms. Returns zero if the part is massless.
                ///
                /// **Game Scenes**: All
                get: dry_mass -> f64
            }
        }
        {
            Shielded {
                /// Returns whether the part is shielded from the exterior of the vessel,
                /// for example by a fairing.
                ///
                /// **Game Scenes**: All
                get: is_shielded -> bool
            }
        }
        {
            DynamicPressure {
                /// Returns the dynamic pressure acting on the part, in Pascals.
                ///
                /// **Game Scenes**: All
                get: dynamic_pressure -> f64
            }
        }
        {
            ImpactTolerance {
                /// Returns the impact tolerance of the part, in meters per second.
                ///
                /// **Game Scenes**: All
                get: impact_tolerance -> f64
            }
        }
        {
            Temperature {
                /// Returns the temperature of the part, in Kelvin.
                ///
                /// **Game Scenes**: All
                get: temperature -> f64
            }
        }
        {
            SkinTemperature {
                /// Returns the temperature of the skin of the part, in Kelvin.
                ///
                /// **Game Scenes**: All
                get: skin_temperature -> f64
            }
        }
        {
            MaxTemperature {
                /// Returns the maximum temperature that the part can survive, in Kelvin.
                ///
                /// **Game Scenes**: All
                get: max_temperature -> f64
            }
        }
        {
            MaxSkinTemperature {
                /// Returns the maximum temperature that the skin of the part can survive, in Kelvin.
                ///
                /// **Game Scenes**: All
                get: max_skin_temperature -> f64
            }
        }
        {
            ThermalMass {
                /// Returns the amount of energy it takes to increase the internal
                /// temperature of the part, in Joules per Kelvin.
                ///
                /// **Game Scenes**: All
                get: thermal_mass -> f32
            }
        }
        {
            ThermalSkinMass {
                /// Returns the amount of energy it takes to increase the skin temperature of
                /// the part, in Joules per Kelvin.
                ///
                /// **Game Scenes**: All
                get: thermal_skin_mass -> f32
            }
        }
        {
            ThermalResourceMass {
                /// Returns the amount of energy it takes to increase the temperature of the
                /// resources contained in the part, in Joules per Kelvin.
                ///
                /// **Game Scenes**: All
                get: thermal_resource_mass -> f32
            }
        }
        {
            ThermalConductionFlux {
                /// Returns the rate at which heat energy is conducting into or out of the part
                /// via contact with other parts. Measured in energy per unit time, or power,
                /// in Watts. A positive value means the part is gaining heat energy, and
                /// negative means it is losing heat energy.
                ///
                /// **Game Scenes**: All
                get: thermal_conduction_flux -> f32
            }
        }
        {
            ThermalConvectionFlux {
                /// Returns the rate at which heat energy is convecting into or out of the part
                /// from the surrounding atmosphere. Measured in energy per unit time, or power,
                /// in Watts. A positive value means the part is gaining heat energy, and negative
                /// means it is losing heat energy.
                ///
                /// **Game Scenes**: All
                get: thermal_convection_flux -> f32
            }
        }
        {
            ThermalRadiationFlux {
                /// Returns the ate at which heat energy is radiating into or out of the part from
                /// the surrounding environment. Measured in energy per unit time, or power, in Watts.
                /// A positive value means the part is gaining heat energy, and negative means
                /// it is losing heat energy.
                ///
                /// **Game Scenes**: All
                get: thermal_radiation_flux -> f32
            }
        }
        {
            ThermalInternalFlux {
                /// Returns the rate at which heat energy is begin generated by the part. For
                /// example, some engines generate heat by combusting fuel. Measured in energy
                /// per unit time, or power, in Watts. A positive value means the part is gaining
                /// heat energy, and negative means it is losing heat energy.
                ///
                /// **Game Scenes**: All
                get: thermal_internal_flux -> f32
            }
        }
        {
            ThermalSkinToInternalFlux {
                /// Returns the rate at which heat energy is transferring between the part’s
                /// skin and its internals. Measured in energy per unit time, or power, in Watts.
                /// A positive value means the part’s internals are gaining heat energy, and
                /// negative means its skin is gaining heat energy.
                ///
                /// **Game Scenes**: All
                get: thermal_skin_to_internal_flux -> f32
            }
        }
        {
            Resources {
                /// Returns a `Resources` object for the part.
                ///
                /// **Game Scenes**: All
                get: resources -> Resources
            }
        }
        {
            Crossfeed {
                /// Returns whether this part is crossfeed capable.
                ///
                /// **Game Scenes**: All
                get: is_crossfeed_capable -> bool
            }
        }
        {
            IsFuelLine {
                /// Returns whether this part is a fuel line.
                ///
                /// **Game Scenes**: All
                get: is_fuel_line -> bool
            }
        }
        {
            FuelLinesFrom {
                /// Returns the parts that are connected to this part via fuel lines, where
                /// the direction of the fuel line is into this part.
                ///
                /// **Game Scenes**: All
                get: fuel_lines_from -> Vec<Part>
            }
        }
        {
            FuelLinesTo {
                /// Returns the parts that are connected to this part via fuel lines, where
                /// the direction of the fuel line is out of this part.
                ///
                /// **Game Scenes**: All
                get: fuel_lines_to -> Vec<Part>
            }
        }
        {
            Modules {
                /// Returns the modules for this part.
                ///
                /// **Game Scenes**: All
                get: modules -> Vec<Module>
            }
        }
        {
            Antenna {
                /// Returns an `Antenna` if this part is an antenna, otherwise `None`.
                ///
                /// **Game Scenes**: All
                get: antenna -> Option<Antenna>
            }
        }
        {
            CargoBay {
                /// Returns a `CargoBay` if this part is a cargo bay, otherwise `None`.
                ///
                /// **Game Scenes**: All
                get: cargo_bay -> Option<CargoBay>
            }
        }
        {
            ControlSurface {
                /// Returns a `ControlSurface` if this part is a control surface, otherwise `None`.
                ///
                /// **Game Scenes**: All
                get: control_surface -> Option<ControlSurface>
            }
        }
        {
            Decoupler {
                /// Returns a `Decoupler` if this part is a decoupler, otherwise `None`.
                ///
                /// **Game Scenes**: All
                get: decoupler -> Option<Decoupler>
            }
        }
        {
            DockingPort {
                /// Returns a `DockingPort` if this part is a docking port, otherwise `None`.
                ///
                /// **Game Scenes**: All
                get: docking_port -> Option<DockingPort>
            }
        }
        {
            Engine {
                /// Returns an `Engine` if this part is an engine, otherwise `None`.
                ///
                /// **Game Scenes**: All
                get: engine -> Option<Engine>
            }
        }
        {
            Experiment {
                /// Returns an `Experiment` if this part is an experiment, otherwise `None`.
                ///
                /// **Game Scenes**: All
                get: experiment -> Option<Experiment>
            }
        }
        {
            Fairing {
                /// Returns a `Fairing` if this part is a fairing, otherwise `None`.
                ///
                /// **Game Scenes**: All
                get: fairing -> Option<Fairing>
            }
        }
        {
            Intake {
                /// Returns an `Intake` if this part is an intake, otherwise `None`.
                ///
                /// **Game Scenes**: All
                get: intake -> Option<Intake>
            }
        }
        {
            Leg {
                /// Returns a `Leg` if this part is a landing leg, otherwise `None`.
                ///
                /// **Game Scenes**: All
                get: leg -> Option<Leg>
            }
        }
        {
            LaunchClamp {
                /// Returns a `LaunchClamp` if this part is a launch clamp, otherwise `None`.
                ///
                /// **Game Scenes**: All
                get: launch_clamp -> Option<LaunchClamp>
            }
        }
        {
            Light {
                /// Returns a `Light` if this part is a light, otherwise `None`.
                ///
                /// **Game Scenes**: All
                get: light -> Option<Light>
            }
        }
        {
            Parachute {
                /// Returns a `Parachute` if this part is an parachute, otherwise `None`.
                ///
                /// **Game Scenes**: All
                get: parachute -> Option<Parachute>
            }
        }
        {
            Radiator {
                /// Returns a `Radiator` if this part is a radiator, otherwise `None`.
                ///
                /// **Game Scenes**: All
                get: radiator -> Option<Radiator>
            }
        }
        {
            RCS {
                /// Returns an `RCS` if this part is an rcs block or thruster, otherwise `None`..
                ///
                /// **Game Scenes**: All
                get: rcs -> Option<RCS>
            }
        }
        {
            ReactionWheel {
                /// Returns a `ReactionWheel` if this part is a reaction wheel, otherwise `None`.
                ///
                /// **Game Scenes**: All
                get: reaction_wheel -> Option<ReactionWheel>
            }
        }
        {
            ResourceConverter {
                /// Returns a `ResourceConverter` if this part is a resource converter, otherwise `None`.
                ///
                /// **Game Scenes**: All
                get: resource_converter -> Option<ResourceConverter>
            }
        }
        {
            ResourceHarvester {
                /// Returns a `ResourceHarvester` if this part is a resource harvester, otherwise `None`.
                ///
                /// **Game Scenes**: All
                get: resource_harvester -> Option<ResourceHarvester>
            }
        }
        {
            Sensor {
                /// Returns a `Sensor` if this part is a sensor, otherwise `None`.
                ///
                /// **Game Scenes**: All
                get: sensor -> Option<Sensor>
            }
        }
        {
            SolarPanel {
                /// Returns a `SolarPanel` if this part is a solar panel, otherwise `None`.
                ///
                /// **Game Scenes**: All
                get: solar_panel -> Option<SolarPanel>
            }
        }
        {
            Wheel {
                /// Returns a `Wheel` if this part is a wheel, otherwise `None`.
                ///
                /// **Game Scenes**: All
                get: wheel -> Option<Wheel>
            }
        }
        {
            MomentOfInertia {
                /// Returns the moment of inertia of the part in kg.m2 around its center of mass in
                /// the parts reference frame (`ReferenceFrame`).
                ///
                /// **Game Scenes**: All
                get: moment_of_inertia -> Vector3
            }
        }
        {
            InertiaTensor {
                /// Returns the inertia tensor of the part in the parts reference frame
                /// (`ReferenceFrame`). Returns the 3x3 matrix as a list of elements,
                /// in row-major order.
                ///
                /// **Game Scenes**: All
                get: inertia_tensor -> Vec<f64>
            }
        }
        {
            ReferenceFrame {
                /// Returns the reference frame that is fixed relative to this part, and centered
                /// on a fixed position within the part, defined by the parts model.
                ///
                /// * The origin is at the position of the part, as returned by
                /// `Part::position()`.
                /// * The axes rotate with the part.
                /// * The x, y and z axis directions depend on the design of the part.
                ///
                /// **Game Scenes**: All
                ///
                /// ![ Reference Frame](https://krpc.github.io/krpc/_images/part.png)
                /// *Mk1 Command Pod reference frame origin and axes*
                ///
                /// # Note
                /// For docking port parts, this reference frame is not necessarily equivalent to
                /// the reference frame for the docking port, returned by
                /// `DockingPort::reference_frame()`.
                get: reference_frame -> ReferenceFrame
            }
        }
        {
            CenterOfMassReferenceFrame {
                /// Returns the reference frame that is fixed relative to this part, and
                /// centered on its center of mass.
                ///
                /// * The origin is at the center of mass of the part, as returned by
                /// `Part::center_of_mass()`.
                /// * The axes rotate with the part.
                /// * The x, y and z axis directions depend on the design of the part.
                ///
                /// **Game Scenes**: All
                ///
                /// # Note
                /// For docking port parts, this reference frame is not necessarily equivalent to
                /// the reference frame for the docking port, returned by
                /// `DockingPort::reference_frame()`.
                get: center_of_mass_reference_frame -> ReferenceFrame
            }
        }
    }
    methods: {
        {
            /// The position of the part in the given reference frame.
            ///
            /// **Game Scenes**: All
            ///
            /// # Arguments
            /// * `reference_frame` -  The reference frame that the returned position vector is in.
            ///
            /// # Returns
            /// The position as a vector.
            ///
            /// # Note
            /// This is a fixed position in the part, defined by the parts model. It s not
            /// necessarily the same as the parts center of mass. Use
            /// `Part.center_of_mass()` to get the parts center of mass.
            fn position(reference_frame: &ReferenceFrame) -> Vector3 {
                Position(reference_frame)
            }
        }
        {
            /// The position of the parts center of mass in the given reference frame. If the
            /// part is physicsless, this is equivalent to `Part::position()`.
            ///
            /// **Game Scenes**: All
            ///
            /// # Arguments
            /// * `reference_frame` -  The reference frame that the returned position vector is in.
            ///
            /// # Returns
            /// The position as a vector.
            fn center_of_mass(reference_frame: &ReferenceFrame) -> Vector3 {
                CenterOfMass(reference_frame)
            }
        }
        {
            /// The axis-aligned bounding box of the part in the given reference frame.
            ///
            /// **Game Scenes**: All
            ///
            /// # Arguments
            /// * `reference_frame` -  The reference frame that the returned position
            /// vectors are in.
            ///
            /// # Returns
            /// The positions of the minimum and maximum vertices of the box, as position vectors.
            ///
            /// # Note
            /// This is computed from the collision mesh of the part. If the part is not
            /// collidable, the box has zero volume and is centered on the
            /// `Part::position()` of the part.
            fn bounding_box(reference_frame: &ReferenceFrame) -> (Vector3, Vector3) {
                BoundingBox(reference_frame)
            }
        }
        {
            /// The direction the part points in, in the given reference frame.
            ///
            /// **Game Scenes**: All
            ///
            /// # Arguments
            /// * `reference_frame` -  The reference frame that the returned direction is in.
            ///
            /// # Returns
            /// The direction as a unit vector.
            fn direction(reference_frame: &ReferenceFrame) -> Vector3 {
                Direction(reference_frame)
            }
        }
        {
            /// The linear velocity of the part in the given reference frame.
            ///
            /// **Game Scenes**: All
            ///
            /// # Arguments
            /// * `reference_frame` -  The reference frame that the returned velocity vector is in.
            ///
            /// # Returns
            /// The velocity as a vector. The vector points in the direction of travel, and
            /// its magnitude is the speed of the body in meters per second.
            fn velocity(reference_frame: &ReferenceFrame) -> Vector3 {
                Velocity(reference_frame)
            }
        }
        {
            /// The rotation of the part, in the given reference frame.
            ///
            /// **Game Scenes**: All
            ///
            /// # Arguments
            /// * `reference_frame` -  The reference frame that the returned rotation is in.
            ///
            /// # Returns
            /// The rotation as a quaternion of the form (*x*,*y*,*z*,*w*).
            fn rotation(reference_frame: &ReferenceFrame) -> Quaternion {
                Rotation(reference_frame)
            }
        }
        {
            /// Exert a constant force on the part, acting at the given position.
            ///
            /// **Game Scenes**: All
            ///
            /// # Arguments
            /// * `force` - A vector pointing in the direction that the force acts, with its
            /// magnitude equal to the strength of the force in Newtons.
            /// * `position` - The position at which the force acts, as a vector.
            /// * `reference_frame` -  The reference frame that the force and position are in.
            ///
            /// # Returns
            /// An object that can be used to remove or modify the force.
            fn add_force(force: Vector3, position: Vector3, reference_frame: &ReferenceFrame) -> Force {
                AddForce(force, position, reference_frame)
            }
        }
        {
            /// Exert an instantaneous force on the part, acting at the given position.
            ///
            /// **Game Scenes**: All
            ///
            /// # Arguments
            /// * `force` - A vector pointing in the direction that the force acts, with its
            /// magnitude equal to the strength of the force in Newtons.
            /// * `position` - The position at which the force acts, as a vector.
            /// * `reference_frame` -  The reference frame that the force and position are in.
            ///
            /// # Returns
            /// An object that can be used to remove or modify the force.
            ///
            /// # Note
            /// The force is applied instantaneously in a single physics update.
            fn instantaneous_force(force: Vector3, position: Vector3, reference_frame: &ReferenceFrame) -> Force {
                InstantaneousForce(force, position, reference_frame)
            }
        }
    }
});

remote_type!(
/// Obtained by calling `Part::add_force()`.
object SpaceCenter.Force {
    properties: {
        {
            Part {
                /// Returns the part that this force is applied to.
                ///
                /// **Game Scenes**: All
                get: part -> Part
            }
        }
        {
            ForceVector {
                /// Returns the force vector, in Newtons.
                ///
                /// **Game Scenes**: All
                ///
                /// # Returns
                /// A vector pointing in the direction that the force acts, with its magnitude
                /// equal to the strength of the force in Newtons.
                get: force_vector -> Vector3,
                /// Sets the force vector, in Newtons.
                ///
                /// **Game Scenes**: All
                set: set_force_vector(Vector3)
            }
        }
        {
            Position {
                /// Returns the position at which the force acts, in reference frame `ReferenceFrame`.
                ///
                /// **Game Scenes**: All
                ///
                /// # Returns
                /// The position as a vector.
                get: position -> Vector3,
                /// Sets the position at which the force acts, in reference frame `ReferenceFrame`.
                ///
                /// **Game Scenes**: All
                set: set_positiion(Vector3)
            }
        }
        {
            ReferenceFrame {
                /// Returns the reference frame of the force vector and position.
                ///
                /// **Game Scenes**: All
                get: reference_frame -> ReferenceFrame,
                /// Sets the reference frame of the force vector and position.
                ///
                /// **Game Scenes**: All
                set: set_reference_frame(&ReferenceFrame)
            }
        }
    }
    methods: {
        {
            /// Remove the force.
            ///
            /// **Game Scenes**: All
            fn remove() {
                Remove()
            }
        }
    }
});
