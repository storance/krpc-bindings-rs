use crate::*;
use crate::codec::*;
use super::{Part};

use std::rc::{Rc};
use std::cell::{RefCell};

remote_type!(
/// This can be used to interact with a specific part module. This includes part modules in
/// stock KSP, and those added by mods.
///
/// In KSP, each part has zero or more PartModules associated with it. Each one contains some of
/// the functionality of the part. For example, an engine has a “ModuleEngines” part module
/// that contains all the functionality of an engine.
object Module {});

impl Module {
    rpc_property!(
        Part: Part {
            service: SpaceCenter,
            class: Module,
            /// The part that contains this module.
            part
        }
    );
}