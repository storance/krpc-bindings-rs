use uom::si::{ISQ, SI, Quantity};
use uom::si::quantities::{Energy};
use uom::typenum::*;

pub mod angle;
pub mod angular_velocity;

pub use crate::units::angle::*;
pub use crate::units::angular_velocity::*;


/// Unit of measure for gravitational parameter (m<sup>3</sup>/s<sup>2</sup>).
pub type GravitationalParameter<V> = Quantity<ISQ<P3, Z0, N2, Z0, Z0, Z0, Z0>, SI<V>, V>;

/// Unit of measure for the gravitational constant (m<sup>3</sup>/kg·s<sup>2</sup>).
pub type GravitationalConstant<V> = Quantity<ISQ<P3, N1, N2, Z0, Z0, Z0, Z0>, SI<V>, V>;

/// Unit of measure for moments of inertia (m<sup>2</sup>·kg).
pub type MomentOfInertia<V> = Quantity<ISQ<P2, P1, Z0, Z0, Z0, Z0, Z0>, SI<V>, V>;

/// Torque tuple representing the available torque around both the positive and negative pitch, yaw,
/// and roll axes.
pub type TorqueTuple<V> = ((Energy<V>, Energy<V>, Energy<V>), (Energy<V>, Energy<V>, Energy<V>));