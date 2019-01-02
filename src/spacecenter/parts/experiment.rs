use crate::*;
use crate::codec::*;
use super::{Part};

use std::rc::{Rc};
use std::cell::{RefCell};

remote_type!(
/// An experiment. Obtained by calling `Part::experiment().`
object Experiment {});

impl Experiment {
    rpc_property!(
        Part: Part {
            service: SpaceCenter,
            class: Experiment,
            /// The part object for this experiment.
            part
        }
    );
}

remote_type!(
/// Obtained by calling `Experiment::data()`.
object ScienceData {});

remote_type!(
/// Obtained by calling `Experiment::science_subject()`.
object ScienceSubject {});