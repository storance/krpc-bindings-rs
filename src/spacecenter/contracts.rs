use crate::*;
use crate::codec::*;

use std::rc::{Rc};
use std::cell::{RefCell};

remote_type!(
/// Contracts manager. Obtained by calling `SpaceCenter::contract_manager()`.
object ContractManager {}
);

remote_type!(
/// A contract.
object Contract {}
);

remote_type!(
/// A contract parameter. See `Contract::parameters()`.
object ContractParameter {}
);

remote_type!(
/// The state of a contract. See <see cref=>"M:SpaceCenter.Contract.State" />.
enum ContractState  {
    /// The contract is active.
    Active => 0,
    /// The contract has been canceled.
    Canceled => 1,
    /// The contract has been completed.
    Completed => 2,
    /// The deadline for the contract has expired.
    DeadlineExpired => 3,
    /// The contract has been declined.
    Declined => 4,
    /// The contract has been failed.
    Failed => 5,
    /// The contract has been generated.
    Generated => 6,
    /// The contract has been offered to the player.
    Offered => 7,
    /// The contract was offered to the player, but the offer expired.
    OfferExpired => 8,
    /// The contract has been withdrawn.
    Withdrawn => 9
});