#![cfg_attr(not(any(test, feature = "export-abi")), no_main)]
extern crate alloc;

// -- Internal Modules ---------------------------------------------------------
mod test;
mod tools;

use crate::tools::{bytes8_to_string, string_to_bytes8};

use std::string::String;

// -- Stylus Imports ----------------------------------------------------------
use stylus_sdk::prelude::*;
use stylus_sdk::storage::{StorageFixedBytes, StorageVec};
use stylus_sdk::{
    alloy_primitives::{Address, FixedBytes, Uint, U256},
    alloy_sol_types::sol,
    block, evm, msg,
    stylus_proc::{public, sol_storage, SolidityError},
};

// -- External Interface ------------------------------------------------------
sol_interface! {
    /// Interface for interacting with the ATON contract.
    interface IATON {

        /// Transfers `value` amount of tokens from `from` to `to`.
        function transferFrom(address from, address to, uint256 value) external returns (bool);

        /// Transfers `amount` tokens from `msg.sender` to `to`.
        function transfer(address to, uint256 amount) external returns (bool);

        /// Approves `spender` to spend `value` tokens on behalf of `msg.sender`.
        function approve(address spender, uint256 value) external returns (bool);

        /// Returns the remaining number of tokens that `spender` can spend on behalf of `owner`.
        function allowance(address owner, address spender) external view returns (uint256);

        /// Returns the address of the Vault contract if tied to this token.
        function vault() external view returns (address);
    }
}

// -- Events & Errors ---------------------------------------------------------
sol! {
    /// Emitted when a new event is added with the given string ID and start date.
    event AddEvent(
        string event_id,
        uint64 start_date
    );

    /// Emitted when an event is closed with the given string ID and the winning team/side.
    event CloseEvent(
        string event_id,
        uint8 winner
    );

    /// Emitted when ownership of this contract is transferred from `previous_owner` to `new_owner`.
    event OwnershipTransferred(
        address indexed previous_owner,
        address indexed new_owner
    );

    // -- Errors --------------------------------------------------------------
    error AlreadyInitialized();   // Thrown when the contract is initialized more than once.
    error AlreadyAdded();         // Thrown when attempting to add an event that already exists.
    error AlreadyStarted();       // Thrown when trying to start an event that is already started.
    error NotStartedYet();        // Thrown when an action is performed on an event that hasn’t started.
    error WrongStatus();          // Thrown when an action requires a particular status which isn’t met.
    error WrongWinner();          // Thrown when the winner value is invalid or does not match requirements.
    error NotAuthorized();        // Thrown when the caller does not have the required authorization.

    error InvalidTeam();          // Thrown when the provided team identifier is invalid.
    error OwnableUnauthorizedAccount(address account); 
    error UnauthorizedOracle(address account);
    error OwnableInvalidOwner(address owner);
}

/// Represents the ways methods in this contract may fail.
#[derive(SolidityError)]
pub enum Error {
    AlreadyInitialized(AlreadyInitialized),
    AlreadyAdded(AlreadyAdded),
    AlreadyStarted(AlreadyStarted),
    NotStartedYet(NotStartedYet),
    NotAuthorized(NotAuthorized),
    WrongStatus(WrongStatus),
    WrongWinner(WrongWinner),
    InvalidTeam(InvalidTeam),
    UnauthorizedAccount(OwnableUnauthorizedAccount),
    UnauthorizedOracle(UnauthorizedOracle),
    InvalidOwner(OwnableInvalidOwner),
}

// -- Storage Layout ----------------------------------------------------------
// `CoreEvents` will be the entrypoint (the contract).
sol_storage! {
    /// Main storage structure that holds events, player data, and contract settings.
    #[entrypoint]
    pub struct CoreEvents {
        /// Mapping from event ID (in bytes8) to an `EventCore` struct.
        mapping(bytes8 => EventCore) events;

        /// Mapping from a player's address to a list of event IDs (bytes8) the player is involved in.
        mapping(address => bytes8[]) player_events;

        /// An array tracking the IDs of currently "opened" events (status = 1).
        bytes8[] opened_events;

        /// An array tracking the IDs of closed events (status = 2).
        bytes8[] closed_events;

        /// The address of the Oracle, authorized to perform specific actions.
        address oracle_address;

        /// The owner address of this contract for administrative or emergency functions.
        address _owner;
    }

    /// Structure representing a single event with necessary details for betting or any game logic.
    pub struct EventCore {
        /// Unique identifier for the event in bytes8 format.
        bytes8 event_id_bytes;

        /// The start date/time of the event (UNIX timestamp).
        uint64 start_date;

        /// The status of the event:
        /// 0 -> Not opened,
        /// 1 -> Opened,
        /// 2 -> Closed.
        uint8 status;

        /// The winner of the event:
        /// 1 for Team A,
        /// 2 for Team B,
        /// 255 (or another sentinel) for a tie/canceled, etc.
        /// (Exact usage can be adjusted as needed.)
        uint8 winner;
    }
}
#[public] // Exposes these functions publicly (Stylus-specific).
impl CoreEvents {
    /// Initializes the contract by setting the owner to `msg::sender()` 
    /// if no owner has been set previously (i.e., `_owner` is zero).
    ///
    /// # Returns
    /// * `Ok(true)` if the contract is successfully initialized.
    /// * `Err(Error::AlreadyInitialized)` if `_owner` is already set.
    pub fn initialize(&mut self) -> Result<bool, Error> {
        if self._owner.get() != Address::ZERO {
            return Err(Error::AlreadyInitialized(AlreadyInitialized {}));
        }
        self._transfer_ownership(msg::sender());

        Ok(true)
    }

    /// Sets or updates the `oracle_address`. Only callable by the current owner.
    ///
    /// # Arguments
    /// * `_oracle_address` - The new address that will act as the oracle.
    ///
    /// # Returns
    /// * `Ok(true)` on success.
    /// * `Err(...)` if the caller is not the owner (via `self.only_owner()`).
    pub fn set_oracle(&mut self, _oracle_address: Address) -> Result<bool, Error> {
        self.only_owner()?;

        self.oracle_address.set(_oracle_address);
        Ok(true)
    }

    // -------------------------------------------------------------------------
    // Event Management
    // -------------------------------------------------------------------------

    /// Adds a new event if it does not already exist and the current time is before its start date.
    /// This function requires `msg::sender()` to be the oracle. Upon success, 
    /// the event is set to status = 1 (opened), and it is logged via the `AddEvent` event.
    ///
    /// # Arguments
    /// * `event_id` - A string ID uniquely identifying the event.
    /// * `start_date` - The timestamp (in seconds) at which the event will start.
    ///
    /// # Returns
    /// * `Ok(true)` on successful addition.
    /// * `Err(Error::AlreadyAdded)` if the event is already registered.
    /// * `Err(Error::AlreadyStarted)` if the current block time exceeds `start_date`.
    /// * `Err(...)` if the caller is not the oracle (via `self.only_oracle()`).
    pub fn add_event(&mut self, event_id: String, start_date: u64) -> Result<bool, Error> {
        self.only_oracle()?;

        let start_date = start_date.try_into().unwrap_or_default();
        let event_id_bytes = string_to_bytes8(&event_id);
        let mut e = self.events.setter(event_id_bytes);

        // Check if event is already initialized (status != 0).
        if e.status.get() != Uint::<8, 1>::from(0u8) {
            return Err(Error::AlreadyAdded(AlreadyAdded {}));
        }

        // Ensure the event has not already started.
        if block::timestamp() > start_date {
            return Err(Error::AlreadyStarted(AlreadyStarted {}));
        }

        // Populate the event fields.
        e.event_id_bytes.set(event_id_bytes);
        e.start_date.set(Uint::<64, 1>::from(start_date));
        e.winner.set(Uint::<8, 1>::from(99u8)); // 99 indicates "not decided" or "in progress" in your logic.
        e.status.set(Uint::<8, 1>::from(1u8));  // 1 => opened.

        // Track this opened event.
        self.opened_events.push(event_id_bytes);

        // Emit the `AddEvent` log.
        evm::log(AddEvent {
            event_id,
            start_date,
        });

        Ok(true)
    }

    /// Closes an event by setting its `winner` and changing its status to 2 (closed).
    /// This function requires `msg::sender()` to be the oracle. It ensures the event is 
    /// in the correct status and that its start date has passed.
    ///
    /// # Arguments
    /// * `event_id` - The string identifier of the event.
    /// * `winner` - A `u8` representing the winning outcome. (0 => tie, 1 => Team A, 2 => Team B, 3 => canceled)
    ///
    /// # Returns
    /// * `Ok(true)` on success.
    /// * `Err(Error::WrongWinner)` if the `winner` is out of accepted range (0..=3).
    /// * `Err(Error::WrongStatus)` if the event is not currently opened (status=1).
    /// * `Err(Error::NotStartedYet)` if the block timestamp is still before the event’s start date.
    /// * `Err(...)` if the caller is not the oracle.
    pub fn close_event(&mut self, event_id: String, winner: u8) -> Result<bool, Error> {
        self.only_oracle()?;

        // Winner must be between 0 and 3.
        if winner > 3 {
            return Err(Error::WrongWinner(WrongWinner {}));
        }

        let event_id_bytes = string_to_bytes8(&event_id);
        let mut e = self.events.setter(event_id_bytes);

        // Ensure event is "opened" (status=1).
        if e.status.get() != Uint::<8, 1>::from(1u8) {
            return Err(Error::WrongStatus(WrongStatus {}));
        }

        // The event must have started (block time >= event's start_date).
        if e.start_date.get() < Uint::<64, 1>::from(block::timestamp()) {
            return Err(Error::NotStartedYet(NotStartedYet {}));
        }

        // Mark the winner and close the event.
        e.winner.set(Uint::<8, 1>::from(winner));
        e.status.set(Uint::<8, 1>::from(2u8)); // 2 => closed.
        self.closed_events.push(event_id_bytes);

        // Remove it from the opened events array.
        CoreEvents::_remove_event(event_id_bytes, &mut self.opened_events)?;

        // Emit the `CloseEvent` log.
        evm::log(CloseEvent { event_id, winner });

        Ok(true)
    }

    // -------------------------------------------------------------------------
    // Pagination & Retrieval
    // -------------------------------------------------------------------------

    /// Returns a paginated list of currently opened events.
    ///
    /// # Arguments
    /// * `page_size` - The maximum number of events to retrieve in one batch.
    /// * `page` - The page index, used to compute the slice of events to return.
    ///
    /// # Returns
    /// * A `Vec` of tuples, each containing:
    ///   1. The event ID as a `String`.
    ///   2. The start date (`u64`).
    ///   3. The status (`u8`).
    ///   4. The winner (`u8`).
    ///
    /// If `page_size` or total events is 0, or if the computed range is invalid, returns an empty list.
    pub fn get_opened_event_list(
        &self,
        page_size: u64,
        page: u64,
    ) -> Result<Vec<(String, u64, u8, u8)>, Error> {
        let length = self.opened_events.len() as u64;
        if page_size == 0 || length == 0 {
            return Ok(Vec::new());
        }

        let end_index = length.saturating_sub(page * page_size);
        let start_index = end_index.saturating_sub(page_size);

        if end_index == 0 {
            return Ok(Vec::new());
        }

        let mut event_list = Vec::new();

        for i in (start_index..end_index).rev() {
            if let Some(event_id_bytes) = self.opened_events.get(i as usize) {
                let event = self.events.get(event_id_bytes);
                event_list.push((
                    bytes8_to_string(event_id_bytes),
                    event.start_date.get().try_into().unwrap_or_default(),
                    event.status.get().try_into().unwrap_or_default(),
                    event.winner.get().try_into().unwrap_or_default(),
                ));
            }
        }

        Ok(event_list)
    }

    /// Returns a paginated list of closed events.
    ///
    /// # Arguments
    /// * `page_size` - The maximum number of events to retrieve in one batch.
    /// * `page` - The page index, used to compute the slice of events to return.
    ///
    /// # Returns
    /// * A `Vec` of tuples, each containing:
    ///   1. The event ID as a `String`.
    ///   2. The start date (`u64`).
    ///   3. The status (`u8`).
    ///   4. The winner (`u8`).
    ///
    /// If `page_size` or total events is 0, or if the computed range is invalid, returns an empty list.
    pub fn get_closed_event_list(
        &self,
        page_size: u64,
        page: u64,
    ) -> Result<Vec<(String, u64, u8, u8)>, Error> {
        let length = self.closed_events.len() as u64;
        if page_size == 0 || length == 0 {
            return Ok(Vec::new());
        }

        let end_index = length.saturating_sub(page * page_size);
        let start_index = end_index.saturating_sub(page_size);

        if end_index == 0 {
            return Ok(Vec::new());
        }

        let mut event_list = Vec::new();

        for i in (start_index..end_index).rev() {
            if let Some(event_id_bytes) = self.closed_events.get(i as usize) {
                let event = self.events.get(event_id_bytes);
                event_list.push((
                    bytes8_to_string(event_id_bytes),
                    event.start_date.get().try_into().unwrap_or_default(),
                    event.status.get().try_into().unwrap_or_default(),
                    event.winner.get().try_into().unwrap_or_default(),
                ));
            }
        }

        Ok(event_list)
    }

    /// Retrieves the details of a single event by its string ID.
    ///
    /// # Arguments
    /// * `_event_id_string` - The string identifier of the event to retrieve.
    ///
    /// # Returns
    /// * A tuple containing:
    ///   1. The start date (`u64`),
    ///   2. The status (`u8`),
    ///   3. The winner (`u8`).
    pub fn get_event(&self, _event_id_string: String) -> Result<(u64, u8, u8), Error> {
        let event_id_bytes = string_to_bytes8(&_event_id_string);
        let event = self.events.get(event_id_bytes);

        Ok((
            event.start_date.get().try_into().unwrap_or_default(),
            event.status.get().try_into().unwrap_or_default(),
            event.winner.get().try_into().unwrap_or_default(),
        ))
    }

    // -------------------------------------------------------------------------
    // Ownership
    // -------------------------------------------------------------------------

    /// Returns the current owner of this contract.
    fn owner(&self) -> Address {
        self._owner.get()
    }

    /// Transfers ownership from the current owner to `new_owner`. 
    /// The new owner must not be the zero address.
    ///
    /// # Arguments
    /// * `new_owner` - The address of the new owner.
    ///
    /// # Returns
    /// * `Ok(())` if successful.
    /// * `Err(Error::InvalidOwner)` if `new_owner` is zero.
    /// * `Err(...)` if `msg::sender()` is not the current owner.
    fn transfer_ownership(&mut self, new_owner: Address) -> Result<(), Error> {
        self.only_owner()?;

        if new_owner.is_zero() {
            return Err(Error::InvalidOwner(OwnableInvalidOwner {
                owner: Address::ZERO,
            }));
        }

        self._transfer_ownership(new_owner);

        Ok(())
    }

    /// Renounces ownership by setting the owner to the zero address. 
    /// This effectively leaves the contract without an owner, disabling 
    /// any functionality that is restricted to the owner only.
    ///
    /// # Returns
    /// * `Ok(())` if successful.
    /// * `Err(...)` if `msg::sender()` is not the current owner.
    fn renounce_ownership(&mut self) -> Result<(), Error> {
        self.only_owner()?;
        self._transfer_ownership(Address::ZERO);
        Ok(())
    }
}


impl CoreEvents {
    /// Removes a specific event, identified by `event_id_bytes`, from a `StorageVec`.
    /// 
    /// - Searches the provided `events` vector for a matching event ID.
    /// - If found, swaps it with the last element and then pops the last element to maintain a compact array.
    /// - If not found, returns an `Error::NotAuthorized`. (You may want to replace this with a more specific error type, such as "EventNotFound.")
    ///
    /// # Arguments
    /// * `event_id_bytes` - The bytes8 identifier of the event to remove.
    /// * `events` - A mutable reference to the `StorageVec` containing the events.
    ///
    /// # Returns
    /// * `Ok(())` if the event was successfully removed.
    /// * `Err(Error::NotAuthorized(...))` if the event was not found in `events`.
    fn _remove_event(
        event_id_bytes: FixedBytes<8>,
        events: &mut StorageVec<StorageFixedBytes<8>>,
    ) -> Result<(), Error> {
        // Get the length of the events vector.
        let length = events.len();

        // Find the index of the event to remove.
        let mut index_to_remove: Option<usize> = None;
        for i in 0..length {
            if let Some(event) = events.get(i) {
                if event == event_id_bytes {
                    index_to_remove = Some(i);
                    break;
                }
            }
        }

        // If the event is not found, return an error.
        if index_to_remove.is_none() {
            return Err(Error::NotAuthorized(NotAuthorized {}));
        }

        // Swap the event to remove with the last element and pop it off.
        let index = index_to_remove.unwrap();
        if index < length - 1 {
            // Replace the element at `index` with the last element.
            if let Some(last_event) = events.get(length - 1) {
                events.setter(index).unwrap().set(last_event);
            }
        }
        // Remove the last element.
        events.pop();

        Ok(())
    }

    /// Checks if the caller (`msg::sender()`) is the oracle.
    /// 
    /// # Returns
    /// * `Ok(())` if `msg::sender()` matches `oracle_address`.
    /// * `Err(Error::UnauthorizedOracle(...))` otherwise.
    pub fn only_oracle(&self) -> Result<(), Error> {
        let account = msg::sender();
        if self.oracle_address.get() != account {
            return Err(Error::UnauthorizedOracle(UnauthorizedOracle { account }));
        }
        Ok(())
    }

    /// Checks if the caller (`msg::sender()`) is the owner.
    /// 
    /// # Returns
    /// * `Ok(())` if `msg::sender()` matches the contract owner (`self.owner()`).
    /// * `Err(Error::UnauthorizedAccount(...))` otherwise.
    pub fn only_owner(&self) -> Result<(), Error> {
        let account = msg::sender();
        if self.owner() != account {
            return Err(Error::UnauthorizedAccount(OwnableUnauthorizedAccount {
                account,
            }));
        }
        Ok(())
    }

    /// Internal helper to set the new owner address, emitting the `OwnershipTransferred` event.
    ///
    /// This function does not perform any checks on the old or new address;
    /// callers should perform any necessary validation before calling.
    ///
    /// # Arguments
    /// * `new_owner` - The address to become the new owner of the contract.
    pub fn _transfer_ownership(&mut self, new_owner: Address) {
        let previous_owner = self._owner.get();
        self._owner.set(new_owner);

        // Log the ownership transfer, including the old and new owner addresses.
        evm::log(OwnershipTransferred {
            previous_owner,
            new_owner,
        });
    }
}
