# README for Arenaton Engine Stylus Smart Contract

## Overview

The **ArenatonEngine** is a decentralized sports betting contract designed to manage events, stakes, payouts, and commission collection. It serves as the core of the Arenaton platform, providing functionality for event creation, user staking, event finalization, and reward distribution—all in a secure and decentralized manner.

---

## Key Features

1. **Decentralized Sports Betting**: Allows players to stake on various sports events with ATON tokens.
2. **Oracle-Based Management**: Ensures event creation and finalization are managed by a trusted oracle.
3. **Dynamic Payouts**: Handles winnings distribution in batches for scalability.
4. **Integrated Commission Collection**: Transfers commissions to the Vault for platform sustainability.

---

## Functionality Breakdown

### **Initialization and Configuration**

#### 1. `initialize(_aton_address: Address)`  
Initializes the engine with the ATON token and Vault contract addresses.

- **Access**: Public  
- **Returns**: `Result<bool, EngineError>`  
- **Usage**: Called once to configure the engine. Ensures the contract is linked to ATON and the Vault.

---

#### 2. `set_oracle(_oracle_address: Address)`  
Sets the oracle address responsible for managing events.

- **Access**: Only callable by the contract owner.  
- **Returns**: `bool`  
- **Usage**: Updates the oracle address to manage events securely.

---

#### 3. `is_oracle()`  
Verifies if the caller is the currently authorized oracle.

- **Access**: Public  
- **Returns**: `bool`  
- **Usage**: Used to ensure only the oracle can perform restricted operations like adding or closing events.

---

### **Event Management**

#### 4. `add_event(event_id: String, start_date: u64, sport: u8)`  
Creates a new betting event.

- **Access**: Only callable by the oracle.  
- **Returns**: `Result<bool, EngineError>`  
- **Usage**:  
    - `event_id`: Unique identifier for the event (e.g., "Match123").  
    - `start_date`: UNIX timestamp for the event start time.  
    - `sport`: A numeric identifier for the sport (e.g., 1 for soccer, 2 for basketball).  

---

#### 5. `close_event(event_id: String, winner: u8)`  
Closes an event and sets the winner.

- **Access**: Only callable by the oracle.  
- **Returns**: `Result<bool, EngineError>`  
- **Usage**:  
    - `event_id`: The ID of the event to close.  
    - `winner`: Specifies the winning team (1 for Team A, 2 for Team B, -2 for a tie).  

---

#### 6. `get_event_list(page_size: u64, page: u64)`  
Retrieves a paginated list of active events.

- **Access**: Public  
- **Returns**: `Result<Vec<(String, u8, u64, u8, U256, U256, u8)>, EngineError>`  
- **Usage**: Useful for displaying active events to users. Returns details like event ID, sport type, start date, total stakes, and winner status.

---

### **Staking and Payouts**

#### 7. `stake(event_id: String, amount: U256, team: u8)`  
Allows players to stake on a specific team for an event. Players can stake ATON tokens directly or send ETH to mint ATON.

- **Access**: Public, Payable  
- **Returns**: `Result<bool, EngineError>`  
- **Usage**:  
    - `event_id`: The ID of the event to stake on.  
    - `amount`: The amount of ATON tokens to stake.  
    - `team`: Specifies the team (1 for Team A, 2 for Team B).  

---

#### 8. `pay_event(event_id: String, batch_size: U256)`  
Pays out winnings to players for a closed event in batches.

- **Access**: Public  
- **Returns**: `Result<bool, EngineError>`  
- **Usage**:  
    - `event_id`: The ID of the event.  
    - `batch_size`: The number of players to process in the current batch.  
- **Details**:  
    - Calculates rewards for winning players, deducts commissions, and transfers payouts.  
    - Updates the event status to "paid" once all players are processed.

---

#### 9. `get_stakes(event_id: String, page_size: u64, page: u64)`  
Retrieves a paginated list of stakes for a specific event.

- **Access**: Public  
- **Returns**: `Result<Vec<(U256, u8, u64)>, EngineError>`  
- **Usage**:  
    - `event_id`: The ID of the event.  
    - Provides details of individual stakes, including the amount, team, and timestamp.

---

### **Internal Utilities**

#### 10. `_calculate_event_commission(event_id: FixedBytes<8>)`  
Calculates the commission for an event and determines if it should be waived (e.g., if only one player staked).

- **Returns**: `(bool, Uint, U256, U256, usize)`  
    - `waive_commission`: Indicates if commission should be waived.  
    - `event_winner`: The winning team.  
    - `total_staked`: Total staked amount.  
    - `commission`: Calculated commission.  
    - `players_len`: Total number of players.  

---

#### 11. `_transfer_player(to: Address, amount: U256)`  
Transfers ATON tokens to a player.

- **Returns**: `Result<(), EngineError>`  

---

#### 12. `_add_stake(event_id: String, amount: U256, team: u8)`  
Internal method for handling the staking logic.

- **Returns**: `Result<bool, EngineError>`  

---

## Error Management

The contract uses custom error types to provide clear feedback when operations fail. Common errors include:

- **`NotAuthorized`**: Caller is not authorized to perform the action.
- **`AlreadyStarted`**: Attempted to modify an event that has already started.
- **`AleadyAdded`**: Event with the given ID already exists.
- **`WrongStatus`**: Operation is not allowed in the current status of the event.
- **`InvalidTeam`**: Specified team is invalid (not 1 or 2).

---

## Event Logs

The following events are emitted for better observability:

- **`AddEvent(event_id, start_date, sport)`**: Emitted when a new event is added.
- **`DonateATON(sender, amount)`**: Emitted when a donation is made in ATON tokens.
- **`Accumulate(new_commission, accumulated, total)`**: Emitted when commissions are accumulated.

---

## Usage Example

Here’s how to interact with the **ArenatonEngine** contract:

```rust
// Assuming a deployed instance of ArenatonEngine at `engine_address`
let engine = ArenatonEngine::new(engine_address);

// 1. Initialize the engine with the ATON contract address
let initialize_tx = engine.initialize(aton_address).call();
assert!(initialize_tx.is_ok());

// 2. Set the oracle address
let set_oracle_tx = engine.set_oracle(oracle_address).call();
assert!(set_oracle_tx.is_ok());

// 3. Add a new event
let add_event_tx = engine.add_event("Match123".to_string(), start_date, 1).call();
assert!(add_event_tx.is_ok());

// 4. Stake on the event
let stake_tx = engine.stake("Match123".to_string(), U256::from(100), 1).call();
assert!(stake_tx.is_ok());

// 5. Close the event and set the winner
let close_event_tx = engine.close_event("Match123".to_string(), 1).call();
assert!(close_event_tx.is_ok());

// 6. Pay out winnings in batches
let pay_event_tx = engine.pay_event("Match123".to_string(), U256::from(10)).call();
assert!(pay_event_tx.is_ok());
```

---

## License

This contract is licensed under the **MIT License**. For more information, see the LICENSE file.