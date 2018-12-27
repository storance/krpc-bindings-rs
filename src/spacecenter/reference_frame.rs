use crate::*;
use crate::codec::*;

use std::rc::{Rc};
use std::cell::{RefCell};

remote_type!(object ReferenceFrame {});

impl ReferenceFrame {
    rpc_method!(fn create_relative(client: &KrpcClient,
                           reference_frame: &ReferenceFrame,
                           position: &Option<Vector3>,
                           rotation: &Option<Quaternion>,
                           velocity: &Option<Vector3>,
                           angular_velocity: &Option<Vector3>) -> ReferenceFrame {
        if let Some(value) = SpaceCenter.ReferenceFrame_static_CreateRelative(
            reference_frame,
            position.unwrap_or((0.0, 0.0, 0.0)),
            rotation.unwrap_or((0.0, 0.0, 0.0, 1.0)),
            velocity.unwrap_or((0.0, 0.0, 0.0)),
            angular_velocity.unwrap_or((0.0, 0.0, 0.0))) as ReferenceFrame {
            value
        } else {
            return Err(KrpcError::NullResponseValue)
        }
    });

    rpc_method!(fn create_hybrid(client: &KrpcClient,
            position: &ReferenceFrame,
            rotation: &Option<ReferenceFrame>,
            velocity: &Option<ReferenceFrame>,
            angular_velocity: &Option<ReferenceFrame>) -> ReferenceFrame {
        if let Some(value) = SpaceCenter.ReferenceFrame_static_CreateHybrid(position,
            rotation,
            velocity,
            angular_velocity) as ReferenceFrame {
            value
        } else {
            return Err(KrpcError::NullResponseValue)
        }
   });
}
