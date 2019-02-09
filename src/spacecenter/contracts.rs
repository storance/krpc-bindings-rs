use crate::codec::{Decode, Encode};
use crate::{remote_type, RemoteEnum, RemoteObject};

use std::collections::HashSet;

remote_type!(
/// Contracts manager. Obtained by calling `SpaceCenter::contract_manager()`.
object SpaceCenter.ContractManager {
    properties: {
        {
            Types {
                /// Returns a list of all contract types.
                ///
                /// **Game Scenes**: All
                get: types ->  HashSet<String>
            }
        }
        {
            AllContracts {
                /// Returns a list of all contracts.
                ///
                /// **Game Scenes**: All
                get: all_contracts -> Vec<Contract>
            }
        }
        {
            ActiveContracts {
                /// Returns a list of all active contracts.
                ///
                /// **Game Scenes**: All
                get: active_contracts -> Vec<Contract>
            }
        }
        {
            OfferedContracts {
                /// Returns a list of all offered, but unaccepted, contracts.
                ///
                /// **Game Scenes**: All
                get: offered_contracts -> Vec<Contract>
            }
        }
        {
            CompletedContracts {
                /// Returns a list of all completed contracts.
                ///
                /// **Game Scenes**: All
                get: completed_contracts -> Vec<Contract>
            }
        }
        {
            FailedContracts {
                /// Returns a list of all failed contracts.
                ///
                /// **Game Scenes**: All
                get: failed_contracts -> Vec<Contract>
            }
        }
    }
});

remote_type!(
/// A contract.
object SpaceCenter.Contract {
    properties: {
        {
            Type {
                /// Returns the type of the contract.
                ///
                /// **Game Scenes**: All
                get: contract_type -> String
            }
        }
        {
            Title {
                /// Returns the title of the contract.
                ///
                /// **Game Scenes**: All
                get: title -> String
            }
        }
        {
            Description {
                /// Returns the description of the contract.
                ///
                /// **Game Scenes**: All
                get: description -> String
            }
        }
        {
            Notes {
                /// Returns the notes for the contract.
                ///
                /// **Game Scenes**: All
                get: notes -> String
            }
        }
        {
            Synopsis {
                /// Returns the synopsis for the contract.
                ///
                /// **Game Scenes**: All
                get: synopsis -> String
            }
        }
        {
            Keywords {
                /// Returns the keywords for the contract.
                ///
                /// **Game Scenes**: All
                get: keywords -> Vec<String>
            }
        }
        {
            Seen {
                /// Returns whether the contract has been seen.
                ///
                /// **Game Scenes**: All
                get: is_seen -> bool
            }
        }
        {
            Read {
                /// Returns whether the contract has been read.
                ///
                /// **Game Scenes**: All
                get: is_read -> bool
            }
        }
        {
            Active {
                /// Returns whether the contract is active.
                ///
                /// **Game Scenes**: All
                get: is_active -> bool
            }
        }
        {
            Failed {
                /// Returns whether the contract has been failed.
                ///
                /// **Game Scenes**: All
                get: is_failed -> bool
            }
        }
        {
            CanBeCanceled {
                /// Returns whether the contract can be canceled.
                ///
                /// **Game Scenes**: All
                get: is_cancelable -> bool
            }
        }
        {
            CanBeDeclined {
                /// Returns whether the contract can be declined.
                ///
                /// **Game Scenes**: All
                get: is_declinable -> bool
            }
        }
        {
            CanBeFailed {
                /// Returns whether the contract can be failed.
                ///
                /// **Game Scenes**: All
                get: is_failable -> bool
            }
        }
        {
            FundsAdvance {
                /// Returns the funds received when accepting the contract.
                ///
                /// **Game Scenes**: All
                get: funds_advance -> f64
            }
        }
        {
            FundsCompletion {
                /// Returns the funds received on completion of the contract.
                ///
                /// **Game Scenes**: All
                get: funds_completion -> f64
            }
        }
        {
            FundsFailed {
                /// Returns the funds lost if the contract is failed.
                ///
                /// **Game Scenes**: All
                get: funds_failed -> f64
            }
        }
        {
            ReputationCompletion {
                /// Returns the reputation received on completion of the contract.
                ///
                /// **Game Scenes**: All
                get: reputation_completion -> f64
            }
        }
        {
            ReputationFailed {
                /// Returns the reputation lost if the contract is failed.
                ///
                /// **Game Scenes**: All
                get: reputation_failed -> f64
            }
        }
        {
            ScienceCompletion {
                /// Returns the science received on completion of the contract.
                ///
                /// **Game Scenes**: All
                get: science_completion -> f64
            }
        }
        {
            Parameters {
                /// Returns the parameters for the contract.
                ///
                /// **Game Scenes**: All
                get: parameters -> Vec<ContractParameter>
            }
        }
    }
    methods: {
        {
            /// Accept an offered contract.
            ///
            /// **Game Scenes**: All
            fn accept() {
                Accept()
            }
        }
        {
            /// Cancel an active contract.
            ///
            /// **Game Scenes**: All
            fn cancel() {
                Cancel()
            }
        }
        {
            /// Decline an offered contract.
            ///
            /// **Game Scenes**: All
            fn decline() {
                Decline()
            }
        }
    }
});

remote_type!(
/// A contract parameter. See `Contract::parameters()`.
object SpaceCenter.ContractParameter {
    properties: {
        {
            Title {
                /// Returns the title of the parameter.
                ///
                /// **Game Scenes**: All
                get: title -> String
            }
        }
        {
            Notes {
                /// Returns the notes of the parameter.
                ///
                /// **Game Scenes**: All
                get: notes -> String
            }
        }
        {
            Children {
                /// Returns the child contract parameters.
                ///
                /// **Game Scenes**: All
                get: children -> Vec<ContractParameter>
            }
        }
        {
            Completed {
                /// Returns whether the parameter has been completed.
                ///
                /// **Game Scenes**: All
                get: is_completed -> bool
            }
        }
        {
            Failed {
                /// Returns whether the parameter has been failed.
                ///
                /// **Game Scenes**: All
                get: is_failed -> bool
            }
        }
        {
            Optional {
                /// Returns whether the parameter is optional.
                ///
                /// **Game Scenes**: All
                get: is_optional -> bool
            }
        }
        {
            FundsCompletion {
                /// Returns the funds received on completion of the contract parameter.
                ///
                /// **Game Scenes**: All
                get: funds_completion -> f64
            }
        }
        {
            FundsFailed {
                /// Returns the funds lost if the contract  parameter is failed.
                ///
                /// **Game Scenes**: All
                get: funds_failed -> f64
            }
        }
        {
            ReputationCompletion {
                /// Returns the reputation received on completion of the contract  parameter.
                ///
                /// **Game Scenes**: All
                get: reputation_completion -> f64
            }
        }
        {
            ReputationFailed {
                /// Returns the reputation lost if the contract  parameter is failed.
                ///
                /// **Game Scenes**: All
                get: reputation_failed -> f64
            }
        }
        {
            ScienceCompletion {
                /// Returns the science received on completion of the contract parameter.
                ///
                /// **Game Scenes**: All
                get: science_completion -> f64
            }
        }
    }
});

remote_type!(
    /// The state of a contract. See <see cref=>"M:SpaceCenter.Contract.State" />.
    enum ContractState {
        /// The contract is active.
        Active = 0,
        /// The contract has been canceled.
        Canceled = 1,
        /// The contract has been completed.
        Completed = 2,
        /// The deadline for the contract has expired.
        DeadlineExpired = 3,
        /// The contract has been declined.
        Declined = 4,
        /// The contract has been failed.
        Failed = 5,
        /// The contract has been generated.
        Generated = 6,
        /// The contract has been offered to the player.
        Offered = 7,
        /// The contract was offered to the player, but the offer expired.
        OfferExpired = 8,
        /// The contract has been withdrawn.
        Withdrawn = 9,
    }
);
