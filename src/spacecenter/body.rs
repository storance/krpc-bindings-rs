use crate::*;
use crate::codec::*;
use super::{Orbit};

use std::rc::{Rc};
use std::cell::{RefCell};

remote_type!(object CelestialBody {});

impl CelestialBody {
    rpc_method!(
    /// Returns the name of the body.
    fn get_name(&self) -> String {
        if let Some(value) = SpaceCenter.CelestialBody_get_Name(self) as String {
            value
        } else {
            return Err(KrpcError::NullResponseValue)
        }
    });

    rpc_method!(
    /// Returns a list of celestial bodies that are in orbit around this celestial body.
    fn get_satellites(&self) -> Vec<CelestialBody> {
        if let Some(value) = SpaceCenter.CelestialBody_get_Satellites(self) as Vec<CelestialBody> {
            value
        } else {
            return Err(KrpcError::NullResponseValue)
        }
    });

    rpc_method!(
    /// Returns the orbit of the body.
    fn get_orbit(&self) -> Orbit {
        if let Some(value) = SpaceCenter.CelestialBody_get_Orbit(self) as Orbit {
            value
        } else {
            return Err(KrpcError::NullResponseValue)
        }
    });
}