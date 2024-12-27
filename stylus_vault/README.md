

# README for Vault

## Overview

The `Vault` contract is responsible for managing and distributing commissions earned from the ATON token transactions within the Arenaton platform. It accumulates commissions and allows players to claim their share based on their token holdings.

## Key Functions

### 1. `initialize(_aton_address: Address)`

- **Description:** Initializes the Vault with the address of the ATON token contract.
- **Access:** Public
- **Returns:** `bool`

### 2. `accumulate_aton(amount: U256)`

- **Description:** Accumulates commissions from ATON tokens. Transfers ATON tokens from the sender to the contract and updates commission records.
- **Access:** Public
- **Returns:** `Result<bool, VaultError>`

### 3. `summary(player: Address)`

- **Description:** Provides a summary of the player's commission, claimed commissions, and total commission in the vault.
- **Access:** Public
- **Returns:** `(U256, U256, U256)`

### 4. `clear_commission(player: Address)`

- **Description:** Clears the commission for a player. Only callable by the ATON contract.
- **Access:** Public
- **Returns:** `Result<(), VaultError>`

## Usage Example

```rust
// Assuming you have a deployed instance of Vault at `vault_address`
let vault = Vault::new(vault_address);

// Initialize the Vault with the ATON contract address
let initialize_tx = vault.initialize(aton_contract_address).call();
assert!(initialize_tx.is_ok());

// Accumulate commissions from ATON tokens
let accumulate_tx = vault.accumulate_aton(U256::from(100)).call();
assert!(accumulate_tx.is_ok());

// Get player commission summary
let summary = vault.summary(player_address).call();
assert!(summary.is_ok());
let (commission, claimed, total) = summary.unwrap();
```

---
