use super::Part;
use crate::codec::{Decode, Encode};
use crate::{remote_type, RemoteObject};

remote_type!(
/// An experiment. Obtained by calling `Part::experiment().`
object SpaceCenter.Experiment {
    properties: {
        {
            Part: Part,
            /// Returns the part object for this experiment.
            ///
            /// **Game Scenes**: All
            get: part
        }
        {
            Deployed: bool,
            /// Returns whether the experiment has been deployed.
            ///
            /// **Game Scenes**: All
            get: is_deployed
        }
        {
            Rerunnable: bool,
            /// Returns whether the experiment can be re-run.
            ///
            /// **Game Scenes**: All
            get: is_rerunnable
        }
        {
            Inoperable: bool,
            /// Returns whether the experiment is inoperable.
            ///
            /// **Game Scenes**: All
            get: is_inoperable
        }
        {
            HasData: bool,
            /// Returns whether the experiment contains data.
            ///
            /// **Game Scenes**: All
            get: has_data
        }
        {
            Data: Vec<ScienceData>,
            /// Returns the data contained in this experiment.
            ///
            /// **Game Scenes**: All
            get: data
        }
        {
            Biome: String,
            /// Returns the name of the biome the experiment is currently in.
            ///
            /// **Game Scenes**: All
            get: biome
        }
        {
            Available: bool,
            /// Determines if the experiment is available given the current conditions.
            ///
            /// **Game Scenes**: All
            get: is_available
        }
        {
            ScienceSubject: Option<ScienceSubject>,
            /// Containing information on the corresponding specific science result for the
            /// current conditions. Returns `None` if the experiment is unavailable.
            ///
            /// **Game Scenes**: All
            get: science_subject
        }
    }
    methods: {
        {
            /// Run the experiment.
            ///
            /// **Game Scenes**: All
            fn run() {
                Run()
            }
        }
        {
            /// Transmit all experimental data contained by this part.
            ///
            /// **Game Scenes**: All
            fn transmit() {
                Transmit()
            }
        }
        {
            /// Dump the experimental data contained by the experiment.
            ///
            /// **Game Scenes**: All
            fn dump() {
                Dump()
            }
        }
        {
            /// Reset the experiment.
            ///
            /// **Game Scenes**: All
            fn reset() {
                Reset()
            }
        }
    }
});

remote_type!(
/// Obtained by calling `Experiment::data()`.
object SpaceCenter.ScienceData {
    properties: {
        {
            DataAmount: f32,
            /// Returns the data mount.
            ///
            /// **Game Scenes**: All
            get: data_amount
        }
        {
            ScienceValue: f32,
            /// Returns the science value.
            ///
            /// **Game Scenes**: All
            get: science_value
        }
        {
            TransmitValue: f32,
            /// Returns the transmit value.
            ///
            /// **Game Scenes**: All
            get: transmit_value
        }
    }
});

remote_type!(
/// Obtained by calling `Experiment::science_subject()`.
object SpaceCenter.ScienceSubject {
    properties: {
        {
            Title: String,
            /// Returns the title of science subject, displayed in science archives.
            ///
            /// **Game Scenes**: All
            get: title
        }
        {
            IsComplete: bool,
            /// Returns whether the experiment has been completed.
            ///
            /// **Game Scenes**: All
            get: is_complete
        }
        {
            Science: f32,
            /// Returns the amount of science already earned from this subject, not updated
            /// until after transmission/recovery.
            ///
            /// **Game Scenes**: All
            get: science
        }
        {
            ScienceCap: f32,
            /// Returns the total science allowable for this subject.
            ///
            /// **Game Scenes**: All
            get: science_cap
        }
        {
            DataScale: f32,
            /// Returns the multiply science value by this to determine data amount in mits.
            ///
            /// **Game Scenes**: All
            get: data_scale
        }
        {
            SubjectValue: f32,
            /// Returns the multiplier for specific Celestial Body/Experiment
            /// Situation combination.
            ///
            /// **Game Scenes**: All
            get: subject_value
        }
        {
            ScientificValue: f32,
            /// Returns the diminishing value multiplier for decreasing the science value
            /// returned from repeated experiments.
            ///
            /// **Game Scenes**: All
            get: scientific_value
        }
    }
});
