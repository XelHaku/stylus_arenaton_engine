// Enable conditional compilation for testing and other features
#![cfg_attr(not(any(test, feature = "export-abi")), no_main)]
extern crate alloc;

// Import modules and dependencies
mod test;
mod tools;
use crate::tools::{bytes8_to_string, string_to_bytes8};

use std::string::String;
use stylus_sdk::prelude::*;
use stylus_sdk::{
    alloy_primitives::FixedBytes,
    alloy_primitives::Uint,
    alloy_primitives::{Address, U256},
    alloy_sol_types::sol,
    block,
    call::Call,
    contract, evm, msg,
    stylus_proc::{public, sol_storage, SolidityError},
};

// Define Solidity-compatible interfaces
sol_interface! {
    interface IATON {
        function mintAton() external payable returns (bool);
        function transferFrom(address from, address to, uint256 value) external returns (bool);
        function transfer(address to, uint256 amount) external returns (bool);
        function vault() external view returns (address);
    }

    interface ICoreEvents {
        function getEvent(string calldata _event_id_string) external view returns (uint64, uint8, uint8);
    }
}

// Define Solidity-compatible error and events
sol! {
    error ErrorCode();
   
    event NewStake(
        bytes8  event_id,  // Changed from string to bytes8
        address  player,
        uint256 amount,
        uint8 team
    );
}

/// Represents the ways methods may fail.
#[derive(SolidityError)]
pub enum Error {
    ErrorCode(ErrorCode),
}

// Define the main storage structure and its components
sol_storage! {
    #[entrypoint]
    pub struct StakeEngine {
        mapping(bytes8 => Event) events; // Mapping for storing events
        mapping(bytes8 => Stake[]) stakes; // Mapping for storing stakes per event
        mapping(address => bytes8[]) player_events; // Mapping of player addresses to their participated events

        address aton_address; // Address of the ATON contract
        address vault_address; // Address of the vault contract
        address core_address; // Address of the core contract
    }

    /// Structure representing a player's stake in an event
    pub struct Stake {
        uint256 amount; // The total amount of tokens staked by the player.
        uint8 team; // The team the player is betting on.
        uint64 timestamp; // Timestamp of the stake.
    }

    /// Structure representing a player in an event
    pub struct Player {
        uint256 stake; // The total amount of tokens staked by the player.
        uint8 team; // The team the player is betting on.
        bool paid; // Whether the player has been paid their reward.
    }

    /// Structure representing an event for betting
    pub struct Event {
        uint64 start_date; // The start date and time of the event.
        mapping(address => Player) player; // Mapping of players in the event.
        uint256[2] total; // Total stakes for each team.
        uint8 winner; // The winner of the event.
        uint8 status; // Status of the event (e.g., unopen, open, closed, paid).
    }
}

// Implementation of the StakeEngine functionality
#[public]
impl StakeEngine {
    /// Initialize the StakeEngine with the required addresses
    pub fn initialize(
        &mut self,
        _aton_address: Address,
        _core_address: Address,
    ) -> Result<bool, Error> {
        if self.aton_address.get() != Address::ZERO {
            return Err(Error::ErrorCode(ErrorCode {}));
        }

        self.aton_address.set(_aton_address);

        let aton_contract = self._get_aton_contract();
        let config = Call::new_in(self);

        let _vault_address = aton_contract
            .vault(config)
            .map_err(|_| Error::ErrorCode(ErrorCode {}))?;

        self.vault_address.set(_vault_address);
        self.core_address.set(_core_address);

        Ok(true)
    }

    /// Retrieve event details from the core contract
    pub fn read_event_core(&mut self, _event_id: String) -> Result<(u64, u8, u8), Error> {
        self._read_event_core(&_event_id)
    }

    /// Stake tokens for an event
    #[payable]
    pub fn stake(&mut self, _event_id: String, _amount: U256, _team: u8) -> Result<bool, Error> {
        let _value = msg::value();
        let event_id_bytes = string_to_bytes8(&_event_id);

        self._can_stake_event(event_id_bytes.clone())?;

        let aton_contract = self._get_aton_contract();
        let mut _final_amount = U256::from(0);

        if _value > U256::from(0) {
            let config = Call::new_in(self).value(_value);
            aton_contract.mint_aton(config).map_err(|_| Error::ErrorCode(ErrorCode {}))?;
            _final_amount = _value;
        } else {
            let config = Call::new_in(self);
            aton_contract.transfer_from(config, msg::sender(), contract::address(), _amount)
                .map_err(|_| Error::ErrorCode(ErrorCode {}))?;
            _final_amount = _amount;
        }

        self._add_stake(event_id_bytes, _final_amount, _team)?;
        Ok(true)
    }

    /// Retrieve stakes for an event with pagination
    pub fn get_stakes(
        &self,
        _event_id: String,
        page_size: u64,
    ) -> Result<Vec<(U256, u8, u64)>, Error> {
        let id8 = string_to_bytes8(&_event_id);
        let stakes = self.stakes.getter(id8);
        let event = self.events.getter(id8);

        if event.status.get() == Uint::<8, 1>::from(0u8) {
            return Ok(Vec::new());
        }

        let end_index = std::cmp::min(page_size as usize, stakes.len());

        let mut result: Vec<(U256, u8, u64)> = Vec::with_capacity(end_index);

        for i in 0..end_index {
            if let Some(stake_guard) = stakes.get(i) {
                result.push((
                    stake_guard.amount.get(),
                    stake_guard.team.get().try_into().unwrap(),
                    stake_guard.timestamp.get().try_into().unwrap(),
                ));
            }
        }

        Ok(result)
    }

    /// Pay rewards for a completed event
    pub fn pay_event(&mut self, event_id: String, player_address: Address) -> Result<bool, Error> {
        self._pay_event(event_id, player_address)
    }

    /// Retrieve events a player has participated in
    pub fn get_player_events(
        &self,
        page_size: u64,
    ) -> Result<Vec<(String, u64, u8, U256, U256, u8)>, Error> {
        let events = self.player_events.getter(msg::sender());
        let end_index = std::cmp::min(page_size, events.len() as u64);

        let mut event_list = Vec::new();

        for i in 0..end_index {
            if let Some(event_id_bytes) = events.get(i as usize) {
                let event = self.events.get(event_id_bytes);
                event_list.push((
                    bytes8_to_string(event_id_bytes),
                    event.start_date.get().try_into().unwrap_or_default(),
                    event.status.get().try_into().unwrap_or_default(),
                    event.total.get(0).unwrap_or_default(),
                    event.total.get(1).unwrap_or_default(),
                    event.winner.get().try_into().unwrap_or_default(),
                ));
            }
        }

        Ok(event_list)
    }
}

impl StakeEngine {
    fn _read_event_core(&mut self, _event_id: &str) -> Result<(u64, u8, u8), Error> {
        // Create an instance of the ATON contract interface
        let core_contract = ICoreEvents::new(self.core_address.get());
        let config = Call::new_in(self);

        // Retrieve the event details from the ATON contract
        let (start_date, status, winner) = core_contract
            .get_event(config, _event_id.to_string())
            .map_err(|_| Error::ErrorCode(ErrorCode {}))?;
        let event_id_bytes = string_to_bytes8(_event_id);
        let mut event = self.events.setter(event_id_bytes);

        if event.status.get() == Uint::<8, 1>::from(0u8) {
            return Err(Error::ErrorCode(ErrorCode {}));
        }

       
        event.status.set(Uint::<8, 1>::from(status)); // Set the event status
        event.winner.set(Uint::<8, 1>::from(winner));
        event.start_date.set(Uint::<64, 1>::from(start_date));

        Ok((
            start_date, 
            status
                .try_into()
                .unwrap_or_default(),
            winner
                .try_into()
                .unwrap_or_default(),
        ))
    }
    fn _calculate_event_commission(
        &mut self,
        event_id_bytes: FixedBytes<8>,
    ) -> Result<(bool, U256, U256), Error> {
        let premium = U256::from(200000);
        let pct_denom = U256::from(10000000);

        let e = self.events.setter(event_id_bytes);

        let total_staked = e.total.get(0).unwrap() + e.total.get(1).unwrap();
        let commission = total_staked * premium / pct_denom;
        let mut waive_commission = true;

        if e.total.get(0).unwrap() == U256::ZERO || e.total.get(1).unwrap() == U256::ZERO {
            waive_commission = false;
        }


        return Ok((waive_commission, total_staked, commission));
    }

    fn _add_stake(&mut self, event_id_key: FixedBytes<8>, amount: U256, team: u8) -> Result<bool, Error> {
        // Ensure the team is valid
        if team != 1 && team != 2 {
            return Err(Error::ErrorCode(ErrorCode {}));
        }


        // Check if the event exists
        let event = self.events.get(event_id_key);
        if event.status.get() != Uint::<8, 1>::from(1u8) {
            return Err(Error::ErrorCode(ErrorCode {}));
        }

        // Ensure the event has started
        if Uint::<64, 1>::from(block::timestamp()) < event.start_date.get() {
            return Err(Error::ErrorCode(ErrorCode {}));
        }

        let mut event_data = self.events.setter(event_id_key);

    

        let player = msg::sender();
        let previous_stake = event_data.player.get(player).stake.get();
        let previous_team = event_data.player.get(player).team.get();

        // Validate team change and staking logic
        if previous_team != Uint::<8, 1>::from(team) && previous_stake != U256::ZERO {
            return Err(Error::ErrorCode(ErrorCode {}));
        }

     
        let updated_stake = previous_stake + amount;

        let mut _player =event_data.player.setter(player);

        _player.stake.set(updated_stake);
        _player.team.set(Uint::<8, 1>::from(team));


       
        let mut _stakes = self.stakes.setter(event_id_key);

        // Add stake to event's stake records
        let mut new_stake = _stakes.grow();
        new_stake.amount.set(amount);
        new_stake.team.set(Uint::<8, 1>::from(team));
        new_stake
            .timestamp
            .set(Uint::<64, 1>::from(block::timestamp()));

        //  Use the hashed bytes32 for the indexed parameter
        evm::log(NewStake {
            event_id: event_id_key, // Now FixedBytes<32>
            player: player,
            amount: amount,
            team: team,
        });

        Ok(true)
    }

    fn _can_stake_event(&self, event_id_key: FixedBytes<8>) -> Result<(), Error> {
        let event = self.events.get(event_id_key);
        if event.status.get() != Uint::<8, 1>::from(1u8)
            || Uint::<64, 1>::from(block::timestamp()) >= event.start_date.get()
        {
            return Err(Error::ErrorCode(ErrorCode {}));
        }
        Ok(())
    }

    /// Generic function to remove an event from a `StorageVec`.
    fn _remove_event(&mut self,
        event_id_bytes: FixedBytes<8>,
       player_address: Address
    ) -> Result<(), Error> {
    let mut events = self.player_events.setter(player_address);

        // Get the length of the events vector
        let length = events.len();

        // Find the index of the event to remove
        let mut index_to_remove: Option<usize> = None;

        for i in 0..length {
            if let Some(event) = events.get(i) {
                if event == event_id_bytes {
                    index_to_remove = Some(i);
                    break;
                }
            }
        }

        // If the event is not found, return an error
        if index_to_remove.is_none() {
            return Err(Error::ErrorCode(ErrorCode {}));
        }

        // Swap the event to remove with the last element and pop it off
        let index = index_to_remove.unwrap();

        if index < length - 1 {
            // Replace the element at `index` with the last element
            if let Some(last_event) = events.get(length - 1) {
                events.setter(index).unwrap().set(last_event);
            }
        }

        // Remove the last element
        events.pop();

        Ok(())
    }

    fn _transfer_aton(&mut self, to: Address, amount: U256) -> Result<(), Error> {
        // Create an instance of the ATON contract interface

        // Retrieve the vault address from the ATON contract
        let _ = self._get_aton_contract()
            .transfer(Call::new_in(self), to, amount)
            .map_err(|_| Error::ErrorCode(ErrorCode {}))?;

        return Ok(());
    }


// Adjust how you're getting the mutable reference for `player_event`
fn _pay_event(&mut self, event_id: String, player_address: Address) -> Result<bool, Error> {
    let event_id_bytes = string_to_bytes8(&event_id);

    let (waive_commission, total_staked, commission) =
        self._calculate_event_commission(event_id_bytes.clone())?;

    let mut e = self.events.setter(event_id_bytes);
    let event_winner = e.winner.get();

    if e.status.get() != Uint::<8, 1>::from(2u8) {
        return Err(Error::ErrorCode(ErrorCode {}));
    }

    let player_stake = e.player.get(player_address);
    let _stake = player_stake.stake.get();
    let player_reward = if _stake > U256::ZERO && !e.player.get(player_address).paid.get() {
        if event_winner == player_stake.team.get() {
            let reward = if waive_commission {
                _stake
            } else {
                (_stake * (total_staked - commission)) / total_staked
            };
           
            reward
        } else {
            U256::ZERO
        }
    } else {
        U256::ZERO
    };

        let mut _player =e.player.setter(player_address);
        _player.paid.set(true);



  
         if player_reward > U256::ZERO {
                self._transfer_aton(player_address, player_reward)?;
            }

        self._remove_event(event_id_bytes, player_address)?;




    Ok(true)
}

    fn _get_aton_contract(&self) -> IATON {
        IATON::new(self.aton_address.get())
    }
}
