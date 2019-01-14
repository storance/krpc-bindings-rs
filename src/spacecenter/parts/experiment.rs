use super::Part;
use crate::codec::*;
use crate::*;

use std::rc::Rc;

remote_type!(
/// An experiment. Obtained by calling `Part::experiment().`
object SpaceCenter.Experiment {
    properties: {
        {
            Part: Part,
            /// The part object for this experiment
            get: part
        }
    }
});

remote_type!(
/// Obtained by calling `Experiment::data()`.
object SpaceCenter.ScienceData {});

remote_type!(
/// Obtained by calling `Experiment::science_subject()`.
object SpaceCenter.ScienceSubject {});
