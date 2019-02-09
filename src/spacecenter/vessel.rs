use super::{AutoPilot, Comms, Control, Flight, Orbit, Parts, ReferenceFrame, Resources};
use crate::codec::{Decode, Encode};
use crate::{remote_type, Quaternion, RemoteEnum, RemoteObject, Vector3};

remote_type!(
/// These objects are used to interact with vessels in KSP. This includes getting orbital and
/// flight data, manipulating control inputs and managing resources.
object SpaceCenter.Vessel {
    properties: {
        {
            Name {
                /// Returns the name of the vessel.
                ///
                /// **Game Scenes**: All
                get: name -> String,
                /// Sets the name of the vessel to given `name`.
                ///
                /// **Game Scenes**: All
                set: set_name(&str)
            }
        }
        {
            Type {
                /// Returns the type of the vessel.
                ///
                /// **Game Scenes**: All
                get: vessel_type -> VesselType,
                /// Sets the type of the vessel to given `vessel_type`.
                ///
                /// **Game Scenes**: All
                set: set_vessel_type(VesselType)
            }
        }
        {
            Situation {
                /// Returns the situation the vessel is in.
                ///
                /// **Game Scenes**: All
                get: situation -> VesselSituation
            }
        }
        {
            Recoverable {
                /// Returns whether the vessel is recoverable.
                ///
                /// **Game Scenes**: All
                get: is_recoverable -> bool
            }
        }
        {
            MET {
                /// Returns the mission elapsed time in seconds.
                ///
                /// **Game Scenes**: All
                get: met -> f64
            }
        }
        {
            Biome {
                /// Returns the name of the biome the vessel is currently in.
                ///
                /// **Game Scenes**: All
                get: biome -> String
            }
        }
        {
            Orbit {
                /// Returns the current orbit of the vessel.
                ///
                /// **Game Scenes**: All
                get: orbit -> Orbit
            }
        }
        {
            Control {
                /// Returns a `Control` object that can be used to manipulate the vessel’s
                /// control inputs.  For example, its pitch/yaw/roll controls, RCS and thrust.
                ///
                /// **Game Scenes**: Flight
                get: control -> Control
            }
        }
        {
            Comms {
                /// Returns a `Comms` object that can be used to interact with CommNet for this vessel.
                ///
                /// **Game Scenes**: Flight
                get: comms -> Comms
            }
        }
        {
            AutoPilot {
                /// Returns an `AutoPilot` object, that can be used to perform simple auto-piloting of
                /// the vessel.
                ///
                /// **Game Scenes**: Flight
                get: auto_pilot -> AutoPilot
            }
        }
        {
            CrewCapacity {
                /// Returns the number of crew that can occupy the vessel.
                ///
                /// **Game Scenes**: All
                get: crew_capacity -> i32
            }
        }
        {
            CrewCount {
                /// Returns the number of crew that are occupying the vessel.
                ///
                /// **Game Scenes**: All
                get: crew_count -> i32
            }
        }
        {
            Crew {
                /// Returns the crew in the vessel.
                ///
                /// **Game Scenes**: All
                get: crew -> Vec<CrewMember>
            }
        }
        {
            Resources {
                /// Returns a `Resources` object, that can used to get information about resources
                /// stored in the vessel.
                ///
                /// **Game Scenes**: Flight
                get: resources -> Resources
            }
        }
        {
            Parts {
                /// Returns a `Parts` object, that can used to interact with the parts that make up this vessel.
                ///
                /// **Game Scenes**: Flight
                get: parts -> Parts
            }
        }
        {
            Mass {
                /// Returns the total mass of the vessel, including resources, in kg.
                ///
                /// **Game Scenes**: Flight
                get: mass -> f32
            }
        }
        {
            DryMass {
                /// Returns the dry mass of the vessel, excluding resources, in kg.
                ///
                /// **Game Scenes**: Flight
                get: dry_mass -> f32
            }
        }
        {
            Thrust {
                /// Returns the total thrust currently being produced by the vessel’s engines, in
                /// Newtons. This is computed by summing `Engine::thrust()` for every engine
                /// in the vessel.
                ///
                /// **Game Scenes**: Flight
                get: thrust -> f32
            }
        }
        {
            AvailableThrust {
                /// Gets the total available thrust that can be produced by the vessel’s active
                /// engines, in Newtons. This is computed by summing `Engine::available_thrust()`
                /// for every active engine in the vessel.
                ///
                /// **Game Scenes**: Flight
                get: available_thrust -> f32
            }
        }
        {
            MaxThrust {
                /// Returns the total maximum thrust that can be produced by the vessel’s active engines,
                /// in Newtons. This is computed by summing `Engine::max_thrust()` for every
                /// active engine.
                ///
                /// **Game Scenes**: Flight
                get: max_thrust -> f32
            }
        }
        {
            MaxVacuumThrust {
                /// Returns the total maximum thrust that can be produced by the vessel’s active
                /// engines when the vessel is in a vacuum, in Newtons. This is computed by summing
                /// `Engine::max_vacuum_thrust()` for every active engine.
                ///
                /// **Game Scenes**: Flight
                get: max_vacuum_thrust -> f32
            }
        }
        {
            SpecificImpulse {
                /// The combined specific impulse of all active engines, in seconds. This is
                /// computed using the formula [described here](https://wiki.kerbalspaceprogram.com/wiki/Specific_impulse#Multiple_engines).
                ///
                /// **Game Scenes**: Flight
                get: isp -> f32
            }
        }
        {
            VacuumSpecificImpulse {
                /// The combined vacuum specific impulse of all active engines, in seconds. This is
                /// computed using the formula [described here](https://wiki.kerbalspaceprogram.com/wiki/Specific_impulse#Multiple_engines).
                ///
                /// **Game Scenes**: Flight
                get: vacuum_isp -> f32
            }
        }
        {
            KerbinSeaLevelSpecificImpulse {
                /// The combined specific impulse of all active engines at sea level on Kerbin,
                /// in seconds. This is computed using the formula
                /// [described here](https://wiki.kerbalspaceprogram.com/wiki/Specific_impulse#Multiple_engines).
                ///
                /// **Game Scenes**: Flight
                get: kerbin_sea_level_isp -> f32
            }
        }
        {
            MomentOfIntertia {
                /// The moment of inertia of the vessel around its center of mass in kg·m<sup>2</sup>.
                /// The inertia values in the returned 3-tuple are around the pitch, roll and
                /// yaw directions respectively. This corresponds to the vessels reference
                /// frame (`ReferenceFrame`).
                ///
                /// **Game Scenes**: Flight
                get: moment_of_intertia -> Vector3
            }
        }
        {
            InertiaTensor {
                /// The inertia tensor of the vessel around its center of mass, in the vessels
                /// reference frame (`ReferenceFrame`). Returns the 3x3 matrix as a list of elements,
                /// in row-major order.
                ///
                /// **Game Scenes**: All
                get: intertia_tensor -> Vec<f64>
            }
        }
        {
            AvailableTorque {
                /// The maximum torque that the vessel generates. Includes contributions from
                /// reaction wheels, RCS, gimballed engines and aerodynamic control surfaces. Returns
                /// the torques in N·m around each of the coordinate axes of the vessels reference
                /// frame (`ReferenceFrame`). These axes are equivalent to the pitch, roll and yaw
                /// axes of the vessel.
                ///
                /// **Game Scenes**: Flight
                get: available_torque -> (Vector3, Vector3)
            }
        }
        {
            AvailableReactionWheelTorque {
                /// The maximum torque that the currently active and powered reaction wheels can generate.
                /// Returns the torques in N·m around each of the coordinate axes of the vessels reference
                /// frame (`ReferenceFrame`). These axes are equivalent to the pitch, roll and yaw axes of
                /// the vessel.
                ///
                /// **Game Scenes**: Flight
                get: available_reaction_wheel_torque -> (Vector3, Vector3)
            }
        }
        {
            AvailableRCSTorque {
                /// The maximum torque that the currently active RCS thrusters can generate.
                /// Returns the torques in N·m around each of the coordinate axes of the vessels reference
                /// frame (`ReferenceFrame`). These axes are equivalent to the pitch, roll and yaw axes of
                /// the vessel.
                ///
                /// **Game Scenes**: Flight
                get: available_rcs_torque -> (Vector3, Vector3)
            }
        }
        {
            AvailableEngineTorque {
                /// The maximum torque that the currently active and gimballed engines can generate.
                /// Returns the torques in N·m around each of the coordinate axes of the vessels reference
                /// frame (`ReferenceFrame`). These axes are equivalent to the pitch, roll and yaw axes of
                /// the vessel.
                ///
                /// **Game Scenes**: Flight
                get: available_engine_torque -> (Vector3, Vector3)
            }
        }
        {
            AvailableControlSurfaceTorque {
                /// The maximum torque that the aerodynamic control surfaces can generate.
                /// Returns the torques in N·m around each of the coordinate axes of the vessels reference
                /// frame (`ReferenceFrame`).  These axes are equivalent to the pitch, roll and yaw axes of
                /// the vessel.
                ///
                /// **Game Scenes**: Flight
                get: available_control_surface_torque -> (Vector3, Vector3)
            }
        }
        {
            AvailableOtherTorque {
                /// The maximum torque that parts (excluding reaction wheels, gimballed engines, RCS and
                /// control surfaces) can generate.  Returns the torques in N·m around each of the coordinate
                /// axes of the vessels reference frame (`ReferenceFrame`).  These axes are equivalent to the
                /// pitch, roll and yaw axes of the vessel.
                ///
                /// **Game Scenes**: Flight
                get: available_other_torque -> (Vector3, Vector3)
            }
        }
        {
            ReferenceFrame {
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
                get: reference_frame -> ReferenceFrame
            }
        }
        {
            OrbitalReferenceFrame {
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
                get: orbital_reference_frame -> ReferenceFrame
            }
        }
        {
            SurfaceReferenceFrame {
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
                get: surface_reference_frame -> ReferenceFrame
            }
        }
        {
            SurfaceVelocityReferenceFrame {
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
                get: surface_velocity_reference_frame -> ReferenceFrame
            }
        }
    }
    methods: {
        {
            /// Recover the vessel.
            ///
            /// **Game Scenes**: All
            fn recover() {
                Recover()
            }
        }
        {
            /// Returns a Flight object that can be used to get flight telemetry for the vessel, in
            /// the specified reference frame.
            ///
            /// **Game Scenes**: Flight
            ///
            /// # Arguments
            /// * `reference_frame` - Reference frame. If `None`, uses the vessel’s surface reference
            /// frame (`Vessel::surface_reference_frame()`).
            ///
            /// # Note
            /// When this is called with no arguments, the vessel’s surface reference frame is
            /// used. This reference frame moves with the vessel, therefore velocities and
            /// speeds returned by the flight object will be zero. See the
            /// [reference frames tutorial](https://krpc.github.io/krpc/tutorials/reference-frames.html#tutorial-reference-frames)
            /// for examples of getting [the orbital and surface speeds of a vessel](https://krpc.github.io/krpc/tutorials/reference-frames.html#tutorial-reference-frames-vessel-speed).
            fn flight(reference_frame: Option<&ReferenceFrame>) -> Flight{
                Flight(reference_frame)
            }
        }
        {
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
            fn resources_in_decouple_stage(stage: i32, cumulative: bool) -> Resources {
                ResourcesInDecoupleStage(stage, cumulative)
            }
        }
        {
            /// Returns the position of the center of mass of the vessel, in the given reference frame.
            ///
            /// **Game Scenes**: Flight
            ///
            /// # Arguments
            /// * `reference_frame` - The reference frame that the returned position vector is in.
            ///
            /// # Return
            /// The positions of the minimum and maximum vertices of the box, as position vectors.
            fn position(reference_frame: &ReferenceFrame) -> Vector3 {
                Position(reference_frame)
            }
        }
        {
            /// Returns the axis-aligned bounding box of the vessel in the given reference frame.
            ///
            /// **Game Scenes**: Flight
            ///
            /// # Arguments
            /// * `reference_frame` - The reference frame that the returned position vectors are in.
            ///
            /// # Return
            /// The positions of the minimum and maximum vertices of the box, as position vectors.
            fn bounding_box(reference_frame: &ReferenceFrame) -> (Vector3, Vector3) {
                BoundingBox(reference_frame)
            }
        }
        {
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
            fn velocity(reference_frame: &ReferenceFrame) -> Vector3 {
                Velocity(reference_frame)
            }
        }
        {
            /// Returns the rotation of the vessel, in the given reference frame.
            ///
            /// **Game Scenes**: Flight
            ///
            /// # Arguments
            /// * `reference_frame` - The reference frame that the returned rotation is in.
            ///
            /// # Return
            /// The rotation as a quaternion of the form (*x*,*y*,*z*,*w*).
            fn rotation(reference_frame: &ReferenceFrame) -> Quaternion {
                Rotation(reference_frame)
            }
        }
        {
            /// Returns the direction in which the vessel is pointing, in the given reference frame..
            ///
            /// **Game Scenes**: Flight
            ///
            /// # Arguments
            /// * `reference_frame` - The reference frame that the returned direction is in.
            ///
            /// # Return
            /// The direction as a unit vector.
            fn direction(reference_frame: &ReferenceFrame) -> Vector3 {
                Direction(reference_frame)
            }
        }
        {
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
            fn angular_velocity(reference_frame: &ReferenceFrame) -> Vector3 {
                AngularVelocity(reference_frame)
            }
        }
    }
});

remote_type!(
/// Represents crew in a vessel. Can be obtained using `Vessel::crew()`.
object SpaceCenter.CrewMember {
    properties: {
        {
            Name {
                /// Returns the crew member's name.
                ///
                /// **Game Scenes**: All
                get: name -> String,
                /// Sets the crew member's name.
                ///
                /// **Game Scenes**: All
                set: set_name(&str)
            }
        }
        {
            Type {
                /// Returns the type of the crew member.
                ///
                /// **Game Scenes**: All
                get: crew_type -> CrewMemberType
            }
        }
        {
            OnMission {
                /// Returns whether the crew member is on a mission.
                ///
                /// **Game Scenes**: All
                get: is_on_mission -> bool
            }
        }
        {
            Courage {
                /// Returns the crew member's courage.
                ///
                /// **Game Scenes**: All
                get: courage -> f32,
                /// Sets the crew member's courage.
                ///
                /// **Game Scenes**: All
                set: set_courage(f32)
            }
        }
        {
            Stupidity {
                /// Returns the crew member's stupidity.
                ///
                /// **Game Scenes**: All
                get: stupidity -> f32,
                /// Sets the crew member's stupidity.
                ///
                /// **Game Scenes**: All
                set: set_stupidity(f32)
            }
        }
        {
            Experience {
                /// Returns the crew member's experience.
                ///
                /// **Game Scenes**: All
                get: experience -> f32,
                /// Sets the crew member's experience.
                ///
                /// **Game Scenes**: All
                set: set_experience(f32)
            }
        }
        {
            Badass {
                /// Returns whether the crew member is a badass.
                ///
                /// **Game Scenes**: All
                get: is_badass -> bool,
                /// Sets whether the crew member is a badass.
                ///
                /// **Game Scenes**: All
                set: set_badass(bool)
            }
        }
        {
            Veteran {
                /// Returns whether the crew member is a veteran.
                ///
                /// **Game Scenes**: All
                get: is_veteran -> bool,
                /// Sets whether the crew member is a veteran.
                ///
                /// **Game Scenes**: All
                set: set_veteran(bool)
            }
        }
    }
});

remote_type!(
    /// The type of a vessel.
    enum VesselType {
        Base = 0,
        Debris = 1,
        Lander = 2,
        Plane = 3,
        Probe = 4,
        Relay = 5,
        Rover = 6,
        Ship = 7,
        Station = 8,
    }
);

remote_type!(
    /// The type of a vessel.
    enum VesselSituation {
        /// Vessel is awaiting launch.
        PreLaunch = 0,
        /// Vessel is orbiting a body,
        Orbiting = 1,
        /// Vessel is on a sub-orbital trajectory.
        SubOrbital = 2,
        /// Vessel is escaping it's orbiting body.
        Escaping = 3,
        /// Vessel is flying through an atmosphere.
        Flying = 4,
        /// Vessel is landed on the surface of a body.
        Landed = 5,
        /// Vessel has splashed down in an ocean.
        Splashed = 6,
        /// The vessel is docked to another.
        Docked = 7,
    }
);

remote_type!(
    /// The type of a crew member.
    enum CrewMemberType {
        /// An applicant for crew.
        Applicant = 0,
        /// Rocket crew.
        Crew = 1,
        /// A tourist.
        Tourist = 2,
        /// An unowned crew member.
        Unowned = 3,
    }
);
