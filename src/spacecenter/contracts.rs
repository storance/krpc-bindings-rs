use crate::codec::*;
use crate::*;

use std::collections::HashSet;
use std::rc::Rc;

remote_type!(
/// Contracts manager. Obtained by calling `SpaceCenter::contract_manager()`.
object SpaceCenter.ContractManager {
    properties: {
        {
            Types: HashSet<String>,
            /// Returns a list of all contract types.
            ///
            /// **Game Scenes**: All
            get: types
        }
        {
            AllContracts: Vec<Contract>,
            /// Returns a list of all contracts.
            ///
            /// **Game Scenes**: All
            get: all_contracts
        }
        {
            ActiveContracts: Vec<Contract>,
            /// Returns a list of all active contracts.
            ///
            /// **Game Scenes**: All
            get: active_contracts
        }
        {
            OfferedContracts: Vec<Contract>,
            /// Returns a list of all offered, but unaccepted, contracts.
            ///
            /// **Game Scenes**: All
            get: offered_contracts
        }
        {
            CompletedContracts: Vec<Contract>,
            /// Returns a list of all completed contracts.
            ///
            /// **Game Scenes**: All
            get: completed_contracts
        }
        {
            FailedContracts: Vec<Contract>,
            /// Returns a list of all failed contracts.
            ///
            /// **Game Scenes**: All
            get: failed_contracts
        }
    }
});

remote_type!(
/// A contract.
object SpaceCenter.Contract {
    properties: {
        {
            Type: String,
            /// Returns the type of the contract.
            ///
            /// **Game Scenes**: All
            get: contract_type
        }
        {
            Title: String,
            /// Returns the title of the contract.
            ///
            /// **Game Scenes**: All
            get: title
        }
        {
            Description: String,
            /// Returns the description of the contract.
            ///
            /// **Game Scenes**: All
            get: description
        }
        {
            Notes: String,
            /// Returns the notes for the contract.
            ///
            /// **Game Scenes**: All
            get: notes
        }
        {
            Synopsis: String,
            /// Returns the synopsis for the contract.
            ///
            /// **Game Scenes**: All
            get: synopsis
        }
        {
            Keywords: Vec<String>,
            /// Returns the keywords for the contract.
            ///
            /// **Game Scenes**: All
            get: keywords
        }
        {
            Seen: bool,
            /// Returns whether the contract has been seen.
            ///
            /// **Game Scenes**: All
            get: is_seen
        }
        {
            Read: bool,
            /// Returns whether the contract has been read.
            ///
            /// **Game Scenes**: All
            get: is_read
        }
        {
            Active: bool,
            /// Returns whether the contract is active.
            ///
            /// **Game Scenes**: All
            get: is_active
        }
        {
            Failed: bool,
            /// Returns whether the contract has been failed.
            ///
            /// **Game Scenes**: All
            get: is_failed
        }
        {
            CanBeCanceled: bool,
            /// Returns whether the contract can be canceled.
            ///
            /// **Game Scenes**: All
            get: is_cancelable
        }
        {
            CanBeDeclined: bool,
            /// Returns whether the contract can be declined.
            ///
            /// **Game Scenes**: All
            get: is_declinable
        }
        {
            CanBeFailed: bool,
            /// Returns whether the contract can be failed.
            ///
            /// **Game Scenes**: All
            get: is_failable
        }
        {
            FundsAdvance: f64,
            /// Returns the funds received when accepting the contract.
            ///
            /// **Game Scenes**: All
            get: funds_advance
        }
        {
            FundsCompletion: f64,
            /// Returns the funds received on completion of the contract.
            ///
            /// **Game Scenes**: All
            get: funds_completion
        }
        {
            FundsFailed: f64,
            /// Returns the funds lost if the contract is failed.
            ///
            /// **Game Scenes**: All
            get: funds_failed
        }
        {
            ReputationCompletion: f64,
            /// Returns the reputation received on completion of the contract.
            ///
            /// **Game Scenes**: All
            get: reputation_completion
        }
        {
            ReputationFailed: f64,
            /// Returns the reputation lost if the contract is failed.
            ///
            /// **Game Scenes**: All
            get: reputation_failed
        }
        {
            ScienceCompletion: f64,
            /// Returns the science received on completion of the contract.
            ///
            /// **Game Scenes**: All
            get: science_completion
        }
        {
            Parameters: Vec<ContractParameter>,
            /// Returns the parameters for the contract.
            ///
            /// **Game Scenes**: All
            get: parameters
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
            Title: String,
            /// Returns the title of the parameter.
            ///
            /// **Game Scenes**: All
            get: title
        }
        {
            Notes: String,
            /// Returns the notes of the parameter.
            ///
            /// **Game Scenes**: All
            get: notes
        }
        {
            Children: Vec<ContractParameter>,
            /// Returns the child contract parameters.
            ///
            /// **Game Scenes**: All
            get: children
        }
        {
            Completed: bool,
            /// Returns whether the parameter has been completed.
            ///
            /// **Game Scenes**: All
            get: is_completed
        }
        {
            Failed: bool,
            /// Returns whether the parameter has been failed.
            ///
            /// **Game Scenes**: All
            get: is_failed
        }
        {
            Optional: bool,
            /// Returns whether the parameter is optional.
            ///
            /// **Game Scenes**: All
            get: is_optional
        }
        {
            FundsCompletion: f64,
            /// Returns the funds received on completion of the contract parameter.
            ///
            /// **Game Scenes**: All
            get: funds_completion
        }
        {
            FundsFailed: f64,
            /// Returns the funds lost if the contract  parameter is failed.
            ///
            /// **Game Scenes**: All
            get: funds_failed
        }
        {
            ReputationCompletion: f64,
            /// Returns the reputation received on completion of the contract  parameter.
            ///
            /// **Game Scenes**: All
            get: reputation_completion
        }
        {
            ReputationFailed: f64,
            /// Returns the reputation lost if the contract  parameter is failed.
            ///
            /// **Game Scenes**: All
            get: reputation_failed
        }
        {
            ScienceCompletion: f64,
            /// Returns the science received on completion of the contract parameter.
            ///
            /// **Game Scenes**: All
            get: science_completion
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
