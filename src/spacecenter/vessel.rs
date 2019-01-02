use crate::*;
use crate::codec::*;
use crate::units::{TorqueTuple, MomentOfInertia, Vector3, Quaternion};
use super::{ReferenceFrame, Orbit, Parts, Resources, Flight, AutoPilot, Control, Comms};

use std::rc::{Rc};
use std::cell::{RefCell};

use uom::si::quantities::{Time, Mass, Force};

remote_type!(
/// These objects are used to interact with vessels in KSP. This includes getting orbital and
/// flight data, manipulating control inputs and managing resources.
object Vessel {}
);

impl Vessel {
    rpc_method!(
    /// Returns the name of the vessel.
    ///
    /// **Game Scenes**: All
    fn name(&self) -> String {
        SpaceCenter.Vessel_get_Name(self).ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// Sets the name of the vessel to given `name`.
    ///
    /// **Game Scenes**: All
    fn set_name(&self, name: String) {
        SpaceCenter.Vessel_set_Name(self, name)
    });

    rpc_method!(
    /// Returns the type of the vessel.
    ///
    /// **Game Scenes**: All
    fn vessel_type(&self) -> VesselType {
        SpaceCenter.Vessel_get_Type(self).ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// Sets the type of the vessel to given `vessel_type`.
    ///
    /// **Game Scenes**: All
    fn set_vessel_type(&self, vessel_type: VesselType) {
        SpaceCenter.Vessel_set_Type(self, vessel_type)
    });

    rpc_method!(
    /// Returns the situation the vessel is in.
    ///
    /// **Game Scenes**: All
    fn situation(&self) -> VesselSituation {
        SpaceCenter.Vessel_get_Situation(self).ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// Returns whether the vessel is recoverable.
    ///
    /// **Game Scenes**: All
    fn is_recoverable(&self) -> bool {
        SpaceCenter.Vessel_get_Recoverable(self).ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// Recover the vessel.
    ///
    /// **Game Scenes**: All
    fn recover(&self) {
        SpaceCenter.Vessel_Recover(self)
    });

    rpc_method!(
    /// Returns the mission elapsed time in seconds.
    ///
    /// **Game Scenes**: All
    fn met(&self) -> Time<f64> {
        SpaceCenter.Vessel_get_MET(self).ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// Returns the name of the biome the vessel is currently in.
    ///
    /// **Game Scenes**: All
    fn biome(&self) -> String {
        SpaceCenter.Vessel_get_Biome(self).ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// Returns a Flight object that can be used to get flight telemetry for the vessel, in
    /// the specified reference frame.
    ///
    /// **Game Scenes**: Flight
    ///
    /// # Arguments
    /// * `reference_frame` - Reference frame. If `None`, uses the vessel’s surface reference
    /// frame (`Vessel::get_surface_reference_frame()`).
    ///
    /// # Note
    /// When this is called with no arguments, the vessel’s surface reference frame is used. This
    /// reference frame moves with the vessel, therefore velocities and speeds returned by the
    /// flight object will be zero. See the [reference frames tutorial](https://krpc.github.io/krpc/tutorials/reference-frames.html#tutorial-reference-frames)
    /// for examples of getting [the orbital and surface speeds of a vessel](https://krpc.github.io/krpc/tutorials/reference-frames.html#tutorial-reference-frames-vessel-speed).
    fn flight(&self, reference_frame: &Option<ReferenceFrame>) -> Flight{
        SpaceCenter.Vessel_Flight(self, reference_frame).ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// Returns the current orbit of the vessel.
    ///
    /// **Game Scenes**: All
    fn orbit(&self) -> Orbit {
        SpaceCenter.Vessel_get_Orbit(self).ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// Returns a `Control` object that can be used to manipulate the vessel’s control inputs.
    /// For example, its pitch/yaw/roll controls, RCS and thrust.
    ///
    /// **Game Scenes**: Flight
    fn control(&self) -> Control {
        SpaceCenter.Vessel_get_Control(self).ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// Returns a `Comms` object that can be used to interact with CommNet for this vessel.
    ///
    /// **Game Scenes**: Flight
    fn comms(&self) -> Comms {
        SpaceCenter.Vessel_get_Comms(self).ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// Returns an `AutoPilot` object, that can be used to perform simple auto-piloting of
    /// the vessel.
    ///
    /// **Game Scenes**: Flight
    fn auto_pilot(&self) -> AutoPilot {
        SpaceCenter.Vessel_get_AutoPilot(self).ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// Returns the number of crew that can occupy the vessel.
    ///
    /// **Game Scenes**: All
    fn crew_capacity(&self) -> i32 {
        SpaceCenter.Vessel_get_CrewCapacity(self).ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// Returns the number of crew that are occupying the vessel.
    ///
    /// **Game Scenes**: All
    fn crew_count(&self) -> i32 {
        SpaceCenter.Vessel_get_CrewCount(self).ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// Returns the crew in the vessel.
    ///
    /// **Game Scenes**: All
    fn crew(&self) -> Vec<CrewMember> {
        SpaceCenter.Vessel_get_Crew(self).ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// Returns a `Resources` object, that can used to get information about resources
    /// stored in the vessel.
    ///
    /// **Game Scenes**: Flight
    fn resources(&self) -> Resources {
        SpaceCenter.Vessel_get_Resources(self).ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// Returns a `Resources` object, that can used to get information about resources
    /// stored in a given stage.
    ///
    /// **Game Scenes**: Flight
    ///
    /// # Arguments
    /// * `stage` - Get resources for parts that are decoupled in this stage.
    /// * `cumulative` - When `false`, returns the resources for parts decoupled in just the given
    /// stage. When `true` returns the resources decoupled in the given stage and all subsequent
    /// stages combined.
    fn resources_in_decouple_stage(&self, stage: i32, cumulative: bool) -> Resources {
        SpaceCenter.Vessel_ResourcesInDecoupleStage(self, stage, cumulative)
            .ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// Returns a `Parts` object, that can used to interact with the parts that make up this vessel.
    ///
    /// **Game Scenes**: Flight
    fn parts(&self) -> Parts {
        SpaceCenter.Vessel_get_Parts(self).ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// Returns the total mass of the vessel, including resources, in kg.
    ///
    /// **Game Scenes**: Flight
    fn mass(&self) -> Mass<f32> {
        SpaceCenter.Vessel_get_Mass(self).ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// Returns the dry mass of the vessel, excluding resources, in kg.
    ///
    /// **Game Scenes**: Flight
    fn dry_mass(&self) -> Mass<f32> {
        SpaceCenter.Vessel_get_DryMass(self).ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// Returns the total thrust currently being produced by the vessel’s engines, in Newtons. This is
    /// computed by summing `Engine.get_thrust()` for every engine in the vessel.
    ///
    /// **Game Scenes**: Flight
    fn thrust(&self) -> Force<f32> {
        SpaceCenter.Vessel_get_Thrust(self).ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// Gets the total available thrust that can be produced by the vessel’s active engines,
    /// in Newtons. This is computed by summing `Engine.get_available_thrust()` for every active
    /// engine in the vessel.
    ///
    /// **Game Scenes**: Flight
    fn available_thrust(&self) -> Force<f32> {
        SpaceCenter.Vessel_get_AvailableThrust(self).ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// Returns the total maximum thrust that can be produced by the vessel’s active engines,
    /// in Newtons. This is computed by summing `Engine.get_max_thrust()` for every active engine.
    ///
    /// **Game Scenes**: Flight
    fn max_thrust(&self) -> Force<f32> {
        SpaceCenter.Vessel_get_MaxThrust(self).ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// Returns the total maximum thrust that can be produced by the vessel’s active engines when
    /// the vessel is in a vacuum, in Newtons. This is computed by summing
    /// `Engine.get_max_vacuum_thrust()` for every active engine.
    ///
    /// **Game Scenes**: Flight
    fn max_vacuum_thrust(&self) -> Force<f32> {
        SpaceCenter.Vessel_get_MaxVacuumThrust(self).ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// The combined specific impulse of all active engines, in seconds. This is computed using
    /// the formula [described here](https://wiki.kerbalspaceprogram.com/wiki/Specific_impulse#Multiple_engines).
    ///
    /// **Game Scenes**: Flight
    fn specific_impulse(&self) -> Time<f32> {
        SpaceCenter.Vessel_get_SpecificImpulse(self).ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// The combined vacuum specific impulse of all active engines, in seconds. This is computed
    /// using the formula [described here](https://wiki.kerbalspaceprogram.com/wiki/Specific_impulse#Multiple_engines).
    ///
    /// **Game Scenes**: Flight
    fn vacuum_specific_impulse(&self) -> Time<f32> {
        SpaceCenter.Vessel_get_VacuumSpecificImpulse(self).ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// The combined vacuum specific impulse of all active engines at sea level on Kerbin,
    /// in seconds. This is compute using the formula
    /// [described here](https://wiki.kerbalspaceprogram.com/wiki/Specific_impulse#Multiple_engines).
    ///
    /// **Game Scenes**: Flight
    fn kerbin_sea_level_specific_impulse(&self) -> Time<f32> {
        SpaceCenter.Vessel_get_KerbinSeaLevelSpecificImpulse(self)
            .ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// The moment of inertia of the vessel around its center of mass in kg·m<sup>2</sup>. The inertia
    /// values in the returned 3-tuple are around the pitch, roll and yaw directions respectively.
    /// This corresponds to the vessels reference frame (`ReferenceFrame`).
    ///
    /// **Game Scenes**: Flight
    fn moment_of_inertia(&self) -> (MomentOfInertia<f64>, MomentOfInertia<f64>, MomentOfInertia<f64>) {
        SpaceCenter.Vessel_get_MomentOfIntertia(self)
            .ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// The inertia tensor of the vessel around its center of mass, in the vessels reference
    /// frame (`ReferenceFrame`). Returns the 3x3 matrix as a list of elements, in row-major order.
    ///
    /// **Game Scenes**: All
    fn inertia_tensor(&self) -> Vec<f64> {
        SpaceCenter.Vessel_get_IntertiaTensor(self)
            .ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// The maximum torque that the vessel generates. Includes contributions from reaction wheels,
    /// RCS, gimballed engines and aerodynamic control surfaces. Returns the torques in N·m
    /// around each of the coordinate axes of the vessels reference frame (`ReferenceFrame`).
    /// These axes are equivalent to the pitch, roll and yaw axes of the vessel.
    ///
    /// **Game Scenes**: Flight
    fn available_torque(&self) -> TorqueTuple<f64> {
        SpaceCenter.Vessel_get_AvailableTorque(self)
            .ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// The maximum torque that the currently active and powered reaction wheels can generate.
    /// Returns the torques in N·m around each of the coordinate axes of the vessels reference
    /// frame (`ReferenceFrame`). These axes are equivalent to the pitch, roll and yaw axes of
    /// the vessel.
    ///
    /// **Game Scenes**: Flight
    fn available_reaction_wheel_torque(&self) -> TorqueTuple<f64> {
        SpaceCenter.Vessel_get_AvailableReactionWheelTorque(self)
            .ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// The maximum torque that the currently active RCS thrusters can generate.
    /// Returns the torques in N·m around each of the coordinate axes of the vessels reference
    /// frame (`ReferenceFrame`). These axes are equivalent to the pitch, roll and yaw axes of
    /// the vessel.
    ///
    /// **Game Scenes**: Flight
    fn available_rcs_torque(&self) -> TorqueTuple<f64> {
        SpaceCenter.Vessel_get_AvailableRCSTorque(self)
            .ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// The maximum torque that the currently active and gimballed engines can generate.
    /// Returns the torques in N·m around each of the coordinate axes of the vessels reference
    /// frame (`ReferenceFrame`). These axes are equivalent to the pitch, roll and yaw axes of
    /// the vessel.
    ///
    /// **Game Scenes**: Flight
    fn available_engine_torque(&self) -> TorqueTuple<f64> {
        SpaceCenter.Vessel_get_AvailableEngineTorque(self)
            .ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// The maximum torque that the aerodynamic control surfaces can generate.
    /// Returns the torques in N·m around each of the coordinate axes of the vessels reference
    /// frame (`ReferenceFrame`).  These axes are equivalent to the pitch, roll and yaw axes of
    /// the vessel.
    ///
    /// **Game Scenes**: Flight
    fn available_control_surface_torque(&self) -> TorqueTuple<f64> {
        SpaceCenter.Vessel_get_AvailableControlSurfaceTorque(self)
            .ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// The maximum torque that parts (excluding reaction wheels, gimballed engines, RCS and
    /// control surfaces) can generate.  Returns the torques in N·m around each of the coordinate
    /// axes of the vessels reference frame (`ReferenceFrame`).  These axes are equivalent to the
    /// pitch, roll and yaw axes of the vessel.
    ///
    /// **Game Scenes**: Flight
    fn available_other_torque(&self) -> TorqueTuple<f64> {
        SpaceCenter.Vessel_get_AvailableOtherTorque(self)
            .ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// Returns the reference frame that is fixed relative to the vessel, and orientated with
    /// the vessel.
    ///
    /// * The origin is at the center of mass of the vessel.
    /// * The axes rotate with the vessel.
    /// * The x-axis points out to the right of the vessel.
    /// * The y-axis points in the forward direction of the vessel.
    /// * The z-axis points out of the bottom off the vessel.
    ///
    /// **Game Scenes**: Flight
    ///
    /// ![Aeris 3A](https://krpc.github.io/krpc/_images/vessel-aircraft.png)
    ///
    /// *Vessel reference frame origin and axes for the Aeris 3A aircraft*
    ///
    /// ![Kerbal-X](https://krpc.github.io/krpc/_images/vessel-rocket.png)
    ///
    /// *Vessel reference frame origin and axes for the Kerbal-X rocket*
    fn reference_frame(&self) -> ReferenceFrame {
        SpaceCenter.Vessel_get_ReferenceFrame(self)
            .ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// Returns the reference frame that is fixed relative to the vessel, and orientated with the
    /// vessels orbital prograde/normal/radial directions.
    ///
    /// * The origin is at the center of mass of the vessel.
    /// * The axes rotate with the orbital prograde/normal/radial directions.
    /// * The x-axis points in the orbital anti-radial direction.
    /// * The y-axis points in the orbital prograde direction.
    /// * The z-axis points in the orbital normal direction.
    ///
    /// **Game Scenes**: Flight
    ///
    /// ![Orbit Reference Frame](https://krpc.github.io/krpc/_images/vessel-orbital.png)
    ///
    /// *Vessel orbital reference frame origin and axes*
    ///
    ///
    /// # Note
    /// Be careful not to confuse this with ‘orbit’ mode on the navball.
    fn orbital_reference_frame(&self) -> ReferenceFrame {
        SpaceCenter.Vessel_get_OrbitalReferenceFrame(self)
            .ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// Returns the reference frame that is fixed relative to the vessel, and orientated with the
    /// surface of the body being orbited.
    ///
    /// * The origin is at the center of mass of the vessel.
    /// * The axes rotate with the north and up directions on the surface of the body.
    /// * The x-axis points in the [zenith](https://en.wikipedia.org/wiki/Zenith) direction
    /// (upwards, normal to the body being orbited from the center of the body towards the
    /// center of mass of the vessel).
    /// * The y-axis points northwards towards the [astronomical horizon](https://en.wikipedia.org/wiki/Horizon)
    /// (north, and tangential to the surface of the body – the direction in which a
    /// compass would point when on the surface).
    /// * The z-axis points eastwards towards the [astronomical horizon](https://en.wikipedia.org/wiki/Horizon)
    /// (east, and tangential to the surface of the body – east on a compass when on the surface).
    ///
    /// **Game Scenes**: Flight
    ///
    /// ![Surface Reference Frame](https://krpc.github.io/krpc/_images/vessel-surface.png)
    ///
    /// *Vessel surface reference frame origin and axes*
    ///
    ///
    /// # Note
    /// Be careful not to confuse this with ‘surface’ mode on the navball.
    fn surface_reference_frame(&self) -> ReferenceFrame {
        SpaceCenter.Vessel_get_SurfaceReferenceFrame(self)
            .ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// Returns the reference frame that is fixed relative to the vessel, and orientated with
    /// the velocity vector of the vessel relative to the surface of the body being orbited.
    ///
    /// * The origin is at the center of mass of the vessel.
    /// * The axes rotate with the vessel’s velocity vector.
    /// * The y-axis points in the direction of the vessel’s velocity vector, relative to the
    /// surface of the body being orbited.
    /// * The z-axis is in the plane of the [astronomical horizon](https://en.wikipedia.org/wiki/Horizon).
    /// * The x-axis is orthogonal to the other two axes.
    ///
    /// **Game Scenes**: Flight
    ///
    /// ![Surface Velocity Reference Frame](https://krpc.github.io/krpc/_images/vessel-surface-velocity.png)
    ///
    /// *Vessel surface velocity reference frame origin and axes*
    fn surface_velocity_reference_frame(&self) -> ReferenceFrame {
        SpaceCenter.Vessel_get_SurfaceVelocityReferenceFrame(self)
            .ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// Returns the position of the center of mass of the vessel, in the given reference frame.
    ///
    /// **Game Scenes**: Flight
    ///
    /// # Arguments
    /// * `reference_frame` - The reference frame that the returned position vector is in.
    ///
    /// # Return
    /// The positions of the minimum and maximum vertices of the box, as position vectors.
    fn position(&self, reference_frame: &ReferenceFrame) -> Vector3 {
        SpaceCenter.Vessel_Position(self, reference_frame)
            .ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// Returns the axis-aligned bounding box of the vessel in the given reference frame.
    ///
    /// **Game Scenes**: Flight
    ///
    /// # Arguments
    /// * `reference_frame` - The reference frame that the returned position vectors are in.
    ///
    /// # Return
    /// The positions of the minimum and maximum vertices of the box, as position vectors.
    fn bounding_box(&self, reference_frame: &ReferenceFrame) -> (Vector3, Vector3) {
        SpaceCenter.Vessel_BoundingBox(self, reference_frame)
            .ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// Returns the velocity of the center of mass of the vessel, in the given reference frame.
    ///
    /// **Game Scenes**: Flight
    ///
    /// # Arguments
    /// * `reference_frame` - The reference frame that the returned velocity is in.
    ///
    /// # Return
    /// The velocity as a vector. The vector points in the direction of travel, and its
    /// magnitude is the speed of the body in meters per second.
    fn velocity(&self, reference_frame: &ReferenceFrame) -> Vector3 {
        SpaceCenter.Vessel_Velocity(self, reference_frame)
            .ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// Returns the rotation of the vessel, in the given reference frame.
    ///
    /// **Game Scenes**: Flight
    ///
    /// # Arguments
    /// * `reference_frame` - The reference frame that the returned rotation is in.
    ///
    /// # Return
    /// The rotation as a quaternion of the form (*x*,*y*,*z*,*w*).
    fn rotation(&self, reference_frame: &ReferenceFrame) -> Quaternion {
        SpaceCenter.Vessel_Rotation(self, reference_frame)
            .ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// Returns the direction in which the vessel is pointing, in the given reference frame..
    ///
    /// **Game Scenes**: Flight
    ///
    /// # Arguments
    /// * `reference_frame` - The reference frame that the returned direction is in.
    ///
    /// # Return
    /// The direction as a unit vector.
    fn direction(&self, reference_frame: &ReferenceFrame) -> Vector3 {
        SpaceCenter.Vessel_Direction(self, reference_frame)
            .ok_or(KrpcError::NullResponseValue)
    });

    rpc_method!(
    /// Returns the angular velocity of the vessel, in the given reference frame.
    ///
    /// **Game Scenes**: Flight
    ///
    /// # Arguments
    /// * `reference_frame` - The reference frame that the returned angular velocity is in.
    ///
    /// # Return
    /// The angular velocity as a vector. The magnitude of the vector is the rotational speed of
    /// the vessel, in radians per second. The direction of the vector indicates the axis of
    /// rotation, using the right-hand rule.
    fn angular_velocity(&self, reference_frame: &ReferenceFrame) -> Vector3 {
        SpaceCenter.Vessel_AngularVelocity(self, reference_frame)
            .ok_or(KrpcError::NullResponseValue)
    });
}

remote_type!(
/// Represents crew in a vessel. Can be obtained using `Vessel::crew()`.
object CrewMember {}
);

impl CrewMember {
    rpc_property!(
        Name: String {
            service: SpaceCenter,
            class: CrewMember,
            /// Returns the crew member's name.
            ///
            /// **Game Scenes**: All
            name,
            /// Sets the crew member's name.
            ///
            /// **Game Scenes**: All
            set_name(name)
        }
    );

    rpc_property!(
        Type: CrewMemberType {
            service: SpaceCenter,
            class: CrewMember,
            /// Returns the type of the crew member.
            ///
            /// **Game Scenes**: All
            crew_type
        }
    );

    rpc_property!(
        OnMission: bool {
            service: SpaceCenter,
            class: CrewMember,
            /// Returns whether the crew member is on a mission.
            ///
            /// **Game Scenes**: All
            is_on_mission
        }
    );

    rpc_property!(
        Courage: f32 {
            service: SpaceCenter,
            class: CrewMember,
            /// Returns the crew member's courage.
            ///
            /// **Game Scenes**: All
            courage,
            /// Sets the crew member's courage.
            ///
            /// **Game Scenes**: All
            set_courage(courage)
        }
    );


    rpc_property!(
        Stupidity: f32 {
            service: SpaceCenter,
            class: CrewMember,
            /// Returns the crew member's stupidity.
            ///
            /// **Game Scenes**: All
            stupidity,
            /// Sets the crew member's stupidity.
            ///
            /// **Game Scenes**: All
            set_stupidity(stupidity)
        }
    );

    rpc_property!(
        Experience: f32 {
            service: SpaceCenter,
            class: CrewMember,
            /// Returns the crew member's experience.
            ///
            /// **Game Scenes**: All
            experience,
            /// Sets the crew member's experience.
            ///
            /// **Game Scenes**: All
            set_experience(experience)
        }
    );

    rpc_property!(
        Badass: bool {
            service: SpaceCenter,
            class: CrewMember,
            /// Returns whether the crew member is a badass.
            ///
            /// **Game Scenes**: All
            is_badass,
            /// Sets whether the crew member is a badass.
            ///
            /// **Game Scenes**: All
            set_badass(is_badass)
        }
    );

    rpc_property!(
        veteran: bool {
            service: SpaceCenter,
            class: CrewMember,
            /// Returns whether the crew member is a veteran.
            ///
            /// **Game Scenes**: All
            is_veteran,
            /// Sets whether the crew member is a veteran.
            ///
            /// **Game Scenes**: All
            set_veteran(is_veteran)
        }
    );
}

remote_type!(
/// The type of a vessel.
enum VesselType {
    Base => 0,
    Debris => 1,
    Lander => 2,
    Plane => 3,
    Probe => 4,
    Relay => 5,
    Rover => 6,
    Ship => 7,
    Station => 8
});

remote_type!(
/// The type of a vessel.
enum VesselSituation {
    /// Vessel is awaiting launch.
    PreLaunch => 0,
    /// Vessel is orbiting a body,
    Orbiting => 1,
    /// Vessel is on a sub-orbital trajectory.
    SubOrbital => 2,
    /// Vessel is escaping it's orbiting body.
    Escaping => 3,
    /// Vessel is flying through an atmosphere.
    Flying => 4,
    /// Vessel is landed on the surface of a body.
    Landed => 5,
    /// Vessel has splashed down in an ocean.
    Splashed => 6,
    /// The vessel is docked to another.
    Docked => 7
});

remote_type!(
/// The type of a crew member.
enum CrewMemberType {
    /// An applicant for crew.
    Applicant => 0,
    /// Rocket crew.
    Crew => 1,
    /// A tourist.
    Tourist => 2,
    /// An unowned crew member.
    Unowned => 3
});