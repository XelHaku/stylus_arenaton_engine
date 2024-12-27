# README for ERC20ATON

## Overview
The `ERC20ATON` contract is a custom ERC20 token implementation named "ATON" built on the Stylus SDK. This token is designed for use within the Arenaton decentralized sports betting platform. It includes features such as commission handling, minting from ETH, and swapping ATON tokens back to ETH.

## Key Features
- **Custom ERC20 Implementation:** Implements the ERC20 standard with additional custom logic tailored for the Arenaton platform.
- **Commission System:** Manages player commissions and integrates seamlessly with an external vault contract.
- **Minting from ETH:** Enables users to mint ATON tokens by sending ETH to the contract.
- **Swapping to ETH:** Allows users to swap their ATON tokens back to ETH efficiently.
- **Engine Role Management:** Provides secure access control by granting specific addresses Arenaton engine roles.

---

## Key Functions

### 1. `initialize()`
- **Description:** Initializes the contract and designates the deployer as the owner.
- **Access:** Public
- **Returns:** `bool`
- **Purpose:** Ensures that initialization can only occur once.

### 2. `mint_aton_from_eth()`
- **Description:** Mints ATON tokens in exchange for ETH. Restricted to addresses marked as Arenaton engines.
- **Access:** Public, Payable
- **Returns:** `bool`

### 3. `swap(amount: U256)`
- **Description:** Swaps ATON tokens back to ETH, ensuring sufficient balance and liquidity.
- **Access:** Public
- **Returns:** `Result<bool, ATONError>`

### 4. `transfer(to: Address, amount: U256)`
- **Description:** Transfers ATON tokens while handling commission distribution.
- **Access:** Public
- **Returns:** `Result<bool, ATONError>`

### 5. `approve(spender: Address, value: U256)`
- **Description:** Approves a spender to transfer tokens on behalf of the sender.
- **Access:** Public
- **Returns:** `bool`

### 6. `transfer_from(from: Address, to: Address, value: U256)`
- **Description:** Transfers tokens on behalf of another address using the allowance mechanism.
- **Access:** Public
- **Returns:** `Result<bool, ATONError>`

### 7. `update_stake_engine(account: Address, status: bool)`
- **Description:** Grants or revokes Arenaton engine status for a specified address. Only callable by the owner.
- **Access:** Public
- **Returns:** `Result<(), ATONError>`

---

## Events
- **`Transfer`:** Emitted during token transfers.
- **`Approval`:** Emitted when allowances are updated.
- **`EngineUpdated`:** Emitted when engine roles are updated.
- **`OwnershipTransferred`:** Emitted when ownership changes.
- **`CommissionAccumulate`:** Emitted when player commissions are updated.

---

## Errors
- **`Zero`:** Indicates an operation involved a zero address.
- **`UnauthorizedAccount`:** Indicates an unauthorized attempt to perform an action.
- **`ERC20InsufficientBalance`:** Insufficient balance for a token transfer.
- **`ERC20InvalidSender`:** Invalid sender address.
- **`ERC20InvalidReceiver`:** Invalid receiver address.
- **`ERC20InsufficientAllowance`:** Insufficient allowance for a token transfer.
- **`ERC20InvalidSpender`:** Invalid spender address.
- **`ERC20InvalidApprover`:** Invalid approver address.

---

## Example Usage

```rust
// Assuming you have a deployed instance of ERC20ATON at `contract_address`
let contract = ERC20ATON::new(contract_address);

// Mint ATON tokens from ETH
let mint_tx = contract.mint_aton_from_eth().call();
assert!(mint_tx.is_ok());

// Approve spending allowance
let approve_tx = contract.approve(spender_address, U256::from(1000)).call();
assert!(approve_tx.is_ok());

// Transfer tokens from one address to another
let transfer_tx = contract.transfer_from(from_address, to_address, U256::from(500)).call();
assert!(transfer_tx.is_ok());
```

---

## Testing

### Sample Tests

#### 1. `erc20params`
- Verifies the token name, symbol, decimals, and initial vault address.

#### 2. `initialize`
- Ensures the contract initializes correctly and sets the owner.

#### 3. `set_vault`
- Tests the ability to set a vault address.

#### 4. `mint_aton_debug_test`
- Verifies minting logic for debugging purposes.

#### 5. `update_new_arenaton_engine`
- Confirms proper management of Arenaton engine roles.

---

## Deployment and Integration

### Deployment
1. Compile the contract using the Stylus SDK.
2. Deploy the contract to the desired blockchain network.
3. Initialize the contract with the `initialize()` function.

### Integration
- Use the provided interface to interact with the contract.
- Ensure proper management of Arenaton engine roles and vault configurations.
- Implement necessary security practices when interacting with sensitive functions like `update_stake_engine` or `mint_aton_from_eth()`.

---

## Contributing
We welcome contributions to improve the `ERC20ATON` contract. Please submit issues or pull requests on the project repository.

---

## License
This project is licensed under the MIT License. See the LICENSE file for details.

