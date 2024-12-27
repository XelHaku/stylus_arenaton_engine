// Allow `cargo stylus export-abi` to generate a main function.
#![cfg_attr(not(any(test, feature = "export-abi")), no_main)]

extern crate alloc;
mod test;

// Imports from the Stylus SDK
use stylus_sdk::prelude::*;
use stylus_sdk::{
    alloy_primitives::{Address, U256},
    alloy_sol_types::sol,
    call::transfer_eth,
    call::Call,
    contract, evm, msg,
};

// Storage definition for the ERC20Aton contract.
sol_storage! {
    /// ERC20Aton storage structure containing core elements for an ERC20 token.
    ///
    /// # Fields
    /// - `owner`: The owner of the contract with special privileges (e.g., updating stake engines).
    /// - `balances`: Maps each address to its token balance.
    /// - `allowances`: Maps each owner to a mapping of spender addresses and their approved spending amounts.
    /// - `total_supply`: The total supply of tokens in existence.
    /// - `stake_engine`: Mapping that indicates which addresses are allowed to mint tokens.
    /// - `vault_address`: Address of the vault contract for commission handling.
    #[entrypoint]
    pub struct Erc20Aton {
        /// The owner of the contract with privileged access.
        address owner;
        /// Mapping of addresses to token balances.
        #[allow(clippy::used_underscore_binding)]
        mapping(address => uint256) balances;
        /// Mapping of addresses to allowances for spenders.
        mapping(address => mapping(address => uint256)) allowances;
        /// Total supply of the token.
        uint256 total_supply;
        /// Mapping to track staking engines (privileged accounts for minting).
        mapping(address => bool) stake_engine;
        /// Address of the vault contract for commission handling.
        address vault_address;
    }
}

// Interface definition for the vault contract.
sol_interface! {
    /// Vault interface for managing player commissions.
    ///
    /// # Methods
    /// - `playerCommission(address)`: Returns the accrued commission for a player.
    /// - `clearCommission(address)`: Clears the commission for a player.
    interface IVault {
        /// Retrieves the commission for a specific player.
        function playerCommission(address player) external view returns (uint256);

        /// Clears the commission for a specific player.
        function clearCommission(address player) external;
    }
}

// Definition of events, errors, and associated data structures for the contract.
sol! {
    // Events related to commissions.
    event CommissionAccumulate(uint256 indexed amount, uint256 indexed newAccPerToken, uint256 indexed totalCommission);
    event EngineUpdated(address indexed account, bool status);
    error Zero(address account);

    // Access control events.
    event EngineRoleGranted(address indexed account, address indexed sender);
    event EngineRoleRevoked(address indexed account, address indexed sender);

    // Ownership-related events.
    event OwnershipTransferred(address indexed previous_owner, address indexed new_owner);
    error UnauthorizedAccount(address account);

    // ERC20 events.
    event Transfer(address indexed from, address indexed to, uint256 value);
    event Approval(address indexed owner, address indexed spender, uint256 value);

    // ERC20-related errors.
    error ERC20InsufficientBalance(address sender, uint256 balance, uint256 needed);
    error ERC20InvalidSender(address sender);
    error ERC20InvalidReceiver(address receiver);
    error ERC20InsufficientAllowance(address spender, uint256 allowance, uint256 needed);
    error ERC20InvalidSpender(address spender);
    error ERC20InvalidApprover(address approver);
}

/// Enum representing the various errors that can occur in the contract.
#[derive(SolidityError)]
pub enum Error {
    /// Thrown when an operation involves an address or value that is zero.
    Zero(Zero),
    /// Thrown when a caller without authorization attempts an owner-only function.
    UnauthorizedAccount(UnauthorizedAccount),
    /// Thrown when the sender's balance is insufficient to complete a transfer.
    InsufficientBalance(ERC20InsufficientBalance),
    /// Thrown when the `from` address in a transfer is invalid.
    InvalidSender(ERC20InvalidSender),
    /// Thrown when the `to` address in a transfer is invalid.
    InvalidReceiver(ERC20InvalidReceiver),
    /// Thrown when the spender does not have sufficient allowance.
    InsufficientAllowance(ERC20InsufficientAllowance),
    /// Thrown when the spender address is invalid.
    InvalidSpender(ERC20InvalidSpender),
    /// Thrown when the approver address is invalid.
    InvalidApprover(ERC20InvalidApprover),
}
#[public]
impl Erc20Aton {
    /// Retrieves the current number from storage.
    pub fn number(&self) -> U256 {
        U256::from(0)
    }
    /// Immutable token name
    pub fn name(&self) -> String {
        "ATON Stylus".into()
    }

    pub fn vault_address(&self) -> Address {
        self.vault_address.get()
    }

    /// Immutable token symbol
    pub fn symbol(&self) -> String {
        "ATON".into()
    }

    /// Immutable token decimals
    pub fn decimals(&self) -> u8 {
        18u8
    }

    /// Total supply of tokens
    pub fn total_supply(&self) -> U256 {
        self.total_supply.get()
    }

    /// Balance of `address`
    pub fn balance_of(&self, owner: Address) -> U256 {
        self.balances.get(owner)
    }

    /// Transfers `value` tokens from `from` to `to`
    /// (msg::sender() must be able to spend at least `value` tokens from `from`)
    pub fn transfer_from(
        &mut self,
        from: Address,
        to: Address,
        value: U256,
    ) -> Result<bool, Error> {
        // Check msg::sender() allowance
        let mut sender_allowances = self.allowances.setter(from);
        let mut allowance = sender_allowances.setter(msg::sender());
        let old_allowance = allowance.get();
        if old_allowance < value {
            return Err(Error::InsufficientAllowance(
                ERC20InsufficientAllowance {
                    spender: msg::sender(),
                    allowance: old_allowance,
                    needed: value,
                },
            ));
        }

        // Decreases allowance
        allowance.set(old_allowance - value);

        self._pay_commissions(to, from).map_err(|_| {
            Error::InsufficientBalance(ERC20InsufficientBalance {
                sender: from,
                balance: self.balances.get(from),
                needed: value,
            })
        })?;
        // Calls the internal transfer function
        self._transfer(from, to, value)?;

        Ok(true)
    }
    fn approve(&mut self, spender: Address, value: U256) -> Result<bool, Error> {
        let owner = msg::sender();
        self._approve(owner, spender, value, true)
    }

    /// Returns the allowance of `spender` on `owner`'s tokens
    pub fn allowance(&self, owner: Address, spender: Address) -> U256 {
        self.allowances.getter(owner).get(spender)
    }

    fn owner(&self) -> Address {
        self.owner.get()
    }

    fn transfer_ownership(&mut self, new_owner: Address) -> Result<(), Error> {
        self._only_owner()?;

        if new_owner.is_zero() {
            return Err(Error::UnauthorizedAccount(UnauthorizedAccount {
                account: Address::ZERO,
            }));
        }

        self._transfer_ownership(new_owner);

        Ok(())
    }
    pub fn initialize(&mut self) -> bool {
        if self.owner.get() != Address::ZERO {
            // Access the value using .get()
            return false;
        }
        self.owner.set(msg::sender());
        true
    }

    pub fn set_vault(&mut self, vault_address: Address) -> bool {
        if self.vault_address.get() != Address::ZERO {
            // Access the value using .get()
            return false;
        }
        self.vault_address.set(vault_address);
        true
    }


    pub fn vault(&self) -> Address {
        self.vault_address.get()
    }

    pub fn transfer(&mut self, to: Address, amount: U256) -> Result<bool, Error> {
        let caller = msg::sender();

        self._pay_commissions(to, caller).map_err(|_| {
            Error::InsufficientBalance(ERC20InsufficientBalance {
                sender: msg::sender(),
                balance: self.balances.get(caller),
                needed: amount,
            })
        })?;

        // Perform the transfer
        self._transfer(caller, to, amount) // 100
            .map(|_| true)
            .map_err(|_| {
                Error::InsufficientBalance(ERC20InsufficientBalance {
                    sender: msg::sender(),
                    needed: amount,
                    balance: self.balances.get(msg::sender()),
                })
            })
    }

    #[payable]
    pub fn mint_aton(&mut self) ->  Result<bool, Error> {
        if self.stake_engine.get(msg::sender()) == false {
            return Err(Error::UnauthorizedAccount(UnauthorizedAccount {
                account: msg::sender(),
                
            }));
        }

        let _ = self._mint(msg::sender(), msg::value());

        // Emitting the transfer event
        evm::log(Transfer {
            from: Address::ZERO,
            to: msg::sender(),
            value: msg::value(),
        });

        Ok(true)
    }



    pub fn swap(&mut self, amount: U256) -> Result<bool, Error> {
        let sender = msg::sender();

        let contract_balance = contract::balance();

        if amount == U256::from(0)
            || self.balances.get(sender) < amount
            || contract_balance < amount
        {
            return Err(Error::Zero(Zero { account: sender })); // Add the error struct
        }
        let _ = self._burn(sender, amount);
        let _ = transfer_eth(sender, amount);

        Ok(true)
    }

    /// Allows the owner to update the status of `stake_engine` for a specific address.
    pub fn update_stake_engine(&mut self, account: Address, status: bool) -> Result<(), Error> {
        // Ensure only the owner can call this function
        self._only_owner()?;

        // Update the `stake_engine` mapping
        let mut engine = self.stake_engine.setter(account);
        engine.set(status);

        // Emit an event (optional, but recommended for transparency)
        evm::log(EngineUpdated { account, status });

        Ok(())
    }

    pub fn is_stake_engine(&self, account: Address) -> bool {
        self.stake_engine.get(account)
    }
}

// Private Functions
impl Erc20Aton {
    /// Sets a `value` number of tokens as the allowance of `spender` over the
    /// caller's tokens.
    ///
    /// Returns a boolean value indicating whether the operation succeeded.
    /// # Arguments
    ///
    /// * `&mut self` - Write access to the contract's state.
    /// * `owner` - Account that owns the tokens.
    /// * `spender` - Account that will spend the tokens.
    /// * `emit_event` - Emit an [`Approval`] event flag.
    ///
    /// # Errors
    ///
    /// If the `spender` address is `Address::ZERO`, then the error
    /// [`Error::InvalidSpender`] is returned.
    ///
    /// # Events
    ///
    /// Emits an [`Approval`] event.
    fn _approve(
        &mut self,
        owner: Address,
        spender: Address,
        value: U256,
        emit_event: bool,
    ) -> Result<bool, Error> {
        if owner.is_zero() {
            return Err(Error::InvalidApprover(ERC20InvalidApprover {
                approver: Address::ZERO,
            }));
        }

        if spender.is_zero() {
            return Err(Error::InvalidSpender(ERC20InvalidSpender {
                spender: Address::ZERO,
            }));
        }

        self.allowances.setter(owner).insert(spender, value);
        if emit_event {
            evm::log(Approval {
                owner,
                spender,
                value,
            });
        }
        Ok(true)
    }
    /// Internal implementation of transferring tokens between two accounts.
    ///
    /// # Arguments
    ///
    /// * `&mut self` - Write access to the contract's state.
    /// * `from` - Account to transfer tokens from.
    /// * `to` - Account to transfer tokens to.
    /// * `value` - The number of tokens to transfer.
    ///
    /// # Errors
    ///
    /// * If the `from` address is `Address::ZERO`, then the error
    ///   [`Error::InvalidSender`] is returned.
    /// * If the `to` address is `Address::ZERO`, then the error
    ///   [`Error::InvalidReceiver`] is returned.
    /// * If the `from` address doesn't have enough tokens, then the error
    ///   [`Error::InsufficientBalance`] is returned.
    ///
    /// # Events
    ///
    /// Emits a [`Transfer`] event.
    fn _transfer(&mut self, from: Address, to: Address, value: U256) -> Result<(), Error> {
        if from.is_zero() {
            return Err(Error::InvalidSender(ERC20InvalidSender {
                sender: Address::ZERO,
            }));
        }
        if to.is_zero() {
            return Err(Error::InvalidReceiver(ERC20InvalidReceiver {
                receiver: Address::ZERO,
            }));
        }

        self._update(from, to, value)?;

        Ok(())
    }

    /// Destroys a `value` amount of tokens from `account`,
    /// lowering the total supply.
    ///
    /// Relies on the `update` mechanism.
    ///
    /// # Arguments
    ///
    /// * `account` - Owner's address.
    /// * `value` - Amount to be burnt.
    ///
    /// # Errors
    ///
    /// * If the `from` address is `Address::ZERO`, then the error
    ///   [`Error::InvalidSender`] is returned.
    /// * If the `from` address doesn't have enough tokens, then the error
    ///   [`Error::InsufficientBalance`] is returned.
    ///
    /// # Events
    ///
    /// Emits a [`Transfer`] event.
    pub fn _burn(&mut self, account: Address, value: U256) -> Result<(), Error> {
        if account == Address::ZERO {
            return Err(Error::InvalidSender(ERC20InvalidSender {
                sender: Address::ZERO,
            }));
        }
        self._update(account, Address::ZERO, value)
    }

    /// Updates `owner`'s allowance for `spender` based on spent `value`.
    ///
    /// Does not update the allowance value in the case of infinite allowance.
    ///
    /// # Arguments
    ///
    /// * `&mut self` - Write access to the contract's state.
    /// * `owner` - Account to transfer tokens from.
    /// * `to` - Account to transfer tokens to.
    /// * `value` - The number of tokens to transfer.
    ///
    /// # Errors
    ///
    /// If not enough allowance is available, then the error
    /// [`Error::InsufficientAllowance`] is returned.
    ///
    /// # Events
    ///
    /// Emits an [`Approval`] event.
    pub fn _spend_allowance(
        &mut self,
        owner: Address,
        spender: Address,
        value: U256,
    ) -> Result<(), Error> {
        let current_allowance = self.allowance(owner, spender);
        if current_allowance != U256::MAX {
            if current_allowance < value {
                return Err(Error::InsufficientAllowance(
                    ERC20InsufficientAllowance {
                        spender,
                        allowance: current_allowance,
                        needed: value,
                    },
                ));
            }

            self._approve(owner, spender, current_allowance - value, false)?;
        }

        Ok(())
    }

    /// Creates a `value` amount of tokens and assigns them to `account`,
    /// by transferring it from `Address::ZERO`.
    ///
    /// Relies on the `_update` mechanism.
    ///
    /// # Panics
    ///
    /// If `total_supply` exceeds `U256::MAX`.
    ///
    /// # Errors
    ///
    /// If the `account` address is `Address::ZERO`, then the error
    /// [`Error::InvalidReceiver`] is returned.
    ///
    /// # Events
    ///
    /// Emits a [`Transfer`] event.
    pub fn _mint(&mut self, account: Address, value: U256) -> Result<(), Error> {
        if account.is_zero() {
            return Err(Error::InvalidReceiver(ERC20InvalidReceiver {
                receiver: Address::ZERO,
            }));
        }
        self._update(Address::ZERO, account, value)
    }
    /// Transfers a `value` amount of tokens from `from` to `to`, or
    /// alternatively mints (or burns) if `from` (or `to`) is the zero address.
    ///
    /// All customizations to transfers, mints, and burns should be done by
    /// using this function.
    ///
    /// # Arguments
    ///
    /// * `from` - Owner's address.
    /// * `to` - Recipient's address.
    /// * `value` - Amount to be transferred.
    ///
    /// # Panics
    ///
    /// If `total_supply` exceeds `U256::MAX`. It may happen during `mint`
    /// operation.
    ///
    /// # Errors
    ///
    /// If the `from` address doesn't have enough tokens, then the error
    /// [`Error::InsufficientBalance`] is returned.
    ///
    /// # Events
    ///
    /// Emits a [`Transfer`] event.
    pub fn _update(&mut self, from: Address, to: Address, value: U256) -> Result<(), Error> {
        if from.is_zero() {
            // Mint operation. Overflow check required: the rest of the code
            // assumes that `total_supply` never overflows.
            let current_supply = self.total_supply.get();
            let new_supply = current_supply.checked_add(value).ok_or_else(|| {
                Error::InsufficientBalance(ERC20InsufficientBalance {
                    sender: from,
                    balance: current_supply,
                    needed: value,
                })
            })?;
            self.total_supply.set(new_supply);
        } else {
            // Check the `from` balance before deduction
            let from_balance = self.balances.get(from);
            if from_balance < value {
                return Err(Error::InsufficientBalance(ERC20InsufficientBalance {
                    sender: from,
                    balance: from_balance,
                    needed: value,
                }));
            }
            // Safely decrease the `from` balance
            self.balances.setter(from).set(from_balance - value);
        }

        if to.is_zero() {
            // Burn operation: decrease total supply
            let current_supply = self.total_supply.get();
            let new_supply = current_supply.checked_sub(value).ok_or_else(|| {
                Error::InsufficientBalance(ERC20InsufficientBalance {
                    sender: from,
                    balance: current_supply,
                    needed: value,
                })
            })?;
            self.total_supply.set(new_supply);
        } else {
            // Safely increase the `to` balance
            let to_balance = self.balances.get(to);
            let new_balance = to_balance.checked_add(value).ok_or_else(|| {
                Error::InsufficientBalance(ERC20InsufficientBalance {
                    sender: to,
                    balance: to_balance,
                    needed: value,
                })
            })?;
            self.balances.setter(to).set(new_balance);
        }

        // Emit a Transfer event
        evm::log(Transfer { from, to, value });

        Ok(())
    }

    // Ownable
    pub fn _only_owner(&self) -> Result<(), Error> {
        let account = msg::sender();
        if self.owner.get() != account {
            return Err(Error::UnauthorizedAccount(UnauthorizedAccount {
                account,
            }));
        }

        Ok(())
    }

    pub fn _transfer_ownership(&mut self, new_owner: Address) {
        let previous_owner = self.owner.get();
        self.owner.set(new_owner);
        evm::log(OwnershipTransferred {
            previous_owner,
            new_owner,
        });
    }

    // Helper function to calculate commission
    fn _player_commission(&mut self, vault: &IVault, account: Address) -> U256 {
        vault
            .player_commission(Call::new_in(self), account)
            .map_err(|_| Error::Zero(Zero { account }))
            .unwrap_or_default()
    }

    // Helper function to perform a transfer
    pub fn _perform_transfer(
        &mut self,
        from: Address,
        to: Address,
        amount: U256,
    ) -> Result<(), Error> {
        self._transfer(from, to, amount).map_err(|_| {
            Error::InsufficientBalance(ERC20InsufficientBalance {
                sender: from,
                needed: amount,
                balance: self.balances.get(from),
            })
        })
    }

    // Helper function to clear commission in the vault
    fn _clear_commission(&mut self, vault: &IVault, account: Address) -> Result<(), Error> {
        vault
            .clear_commission(Call::new_in(self), account)
            .map_err(|_| Error::Zero(Zero { account }))
    }

    pub fn _pay_commissions(&mut self, to: Address, from: Address) -> Result<(), Error> {
        let vault_contract = IVault::new(self.vault_address.get());

        let from_commission = self._player_commission(&vault_contract, from);
        let to_commission = self._player_commission(&vault_contract, to);

        let _to = to;
        let _from = from;
        let _owner = self.owner.get();
        let mut owner_commission = U256::from(0);
        if from == contract::address() || to == contract::address() {
            owner_commission = self._player_commission(&vault_contract, to);
        }
        let vault_address = self.vault_address.get();

        let _ = self._perform_transfer(vault_address, _to, to_commission);
        let _ = self._perform_transfer(vault_address, _from, from_commission);

        let _ = self._clear_commission(&vault_contract, to);
        let _ = self._clear_commission(&vault_contract, from);

        if from == contract::address() || to == contract::address() {
            let _ = self._perform_transfer(vault_address, _owner, owner_commission);

            let _ = self._clear_commission(&vault_contract, _owner);
        }
        return Ok(());
    }
}
