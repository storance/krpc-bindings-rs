use super::Part;
use crate::codec::{Decode, Encode};
use crate::{remote_type, RemoteObject};

remote_type!(
/// An experiment. Obtained by calling `Part::experiment().`
object SpaceCenter.Experiment {
    properties: {
        {
            Part {
                /// Returns the part object for this experiment.
                ///
                /// **Game Scenes**: All
                get: part -> Part
            }
        }
        {
            Deployed {
                /// Returns whether the experiment has been deployed.
                ///
                /// **Game Scenes**: All
                get: is_deployed -> bool
            }
        }
        {
            Rerunnable {
                /// Returns whether the experiment can be re-run.
                ///
                /// **Game Scenes**: All
                get: is_rerunnable -> bool
            }
        }
        {
            Inoperable {
                /// Returns whether the experiment is inoperable.
                ///
                /// **Game Scenes**: All
                get: is_inoperable -> bool
            }
        }
        {
            HasData {
                /// Returns whether the experiment contains data.
                ///
                /// **Game Scenes**: All
                get: has_data -> bool
            }
        }
        {
            Data {
                /// Returns the data contained in this experiment.
                ///
                /// **Game Scenes**: All
                get: data -> Vec<ScienceData>
            }
        }
        {
            Biome {
                /// Returns the name of the biome the experiment is currently in.
                ///
                /// **Game Scenes**: All
                get: biome -> String
            }
        }
        {
            Available {
                /// Determines if the experiment is available given the current conditions.
                ///
                /// **Game Scenes**: All
                get: is_available -> bool
            }
        }
        {
            ScienceSubject {
                /// Containing information on the corresponding specific science result for the
                /// current conditions. Returns `None` if the experiment is unavailable.
                ///
                /// **Game Scenes**: All
                get: science_subject -> Option<ScienceSubject>
            }
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
            DataAmount {
                /// Returns the data mount.
                ///
                /// **Game Scenes**: All
                get: data_amount -> f32
            }
        }
        {
            ScienceValue {
                /// Returns the science value.
                ///
                /// **Game Scenes**: All
                get: science_value -> f32
            }
        }
        {
            TransmitValue {
                /// Returns the transmit value.
                ///
                /// **Game Scenes**: All
                get: transmit_value -> f32
            }
        }
    }
});

remote_type!(
/// Obtained by calling `Experiment::science_subject()`.
object SpaceCenter.ScienceSubject {
    properties: {
        {
            Title {
                /// Returns the title of science subject, displayed in science archives.
                ///
                /// **Game Scenes**: All
                get: title -> String
            }
        }
        {
            IsComplete {
                /// Returns whether the experiment has been completed.
                ///
                /// **Game Scenes**: All
                get: is_complete -> bool
            }
        }
        {
            Science {
                /// Returns the amount of science already earned from this subject, not updated
                /// until after transmission/recovery.
                ///
                /// **Game Scenes**: All
                get: science -> f32
            }
        }
        {
            ScienceCap {
                /// Returns the total science allowable for this subject.
                ///
                /// **Game Scenes**: All
                get: science_cap -> f32
            }
        }
        {
            DataScale {
                /// Returns the multiply science value by this to determine data amount in mits.
                ///
                /// **Game Scenes**: All
                get: data_scale -> f32
            }
        }
        {
            SubjectValue {
                /// Returns the multiplier for specific Celestial Body/Experiment
                /// Situation combination.
                ///
                /// **Game Scenes**: All
                get: subject_value -> f32
            }
        }
        {
            ScientificValue {
                /// Returns the diminishing value multiplier for decreasing the science value
                /// returned from repeated experiments.
                ///
                /// **Game Scenes**: All
                get: scientific_value -> f32
            }
        }
    }
});
