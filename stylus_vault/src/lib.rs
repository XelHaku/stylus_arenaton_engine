// This allows `cargo stylus export-abi` to generate a main function.
// By default (outside of test or the "export-abi" feature), we do not define a main.
#![cfg_attr(not(any(test, feature = "export-abi")), no_main)]

extern crate alloc;
mod test;

// -- Imports ------------------------------------------------------------------
use stylus_sdk::prelude::*;
use stylus_sdk::{
    alloy_primitives::{Address, U256},
    alloy_sol_types::sol,
    call::Call,
    contract, evm, msg,
};

// -- Storage Layout -----------------------------------------------------------
sol_storage! {
    /// `Vault` is the main storage struct for the contract.
    #[entrypoint]
    pub struct Vault {
        /// Accumulated commission per token.
        uint256 accumulated_commission_per_token;
        
        /// Total commission in ATON tokens.
        uint256 total_commission_in_aton;
        
        /// Mapping of account to the last commission-per-token value.
        mapping(address => uint256) last_commission_per_token;
        
        /// Mapping of account to the total claimed commissions.
        mapping(address => uint256) claimed_commissions;
        
        /// The address of the ATON token contract.
        address aton_address;
    }
}

// -- External Interface -------------------------------------------------------
sol_interface! {
    interface IATON {
        /// Transfer tokens from `from` to `to`.
        function transferFrom(address from, address to, uint256 value) external returns (bool);
        
        /// Return the balance of the given account.
        function balanceOf(address owner) external view returns (uint256);
        
        /// Transfer tokens from `msg.sender` to `to`.
        function transfer(address to, uint256 amount) external returns (bool);
        
        /// Returns the total supply of the token.
        function totalSupply() external view returns (uint256);
    }
}

// -- Events & Errors ----------------------------------------------------------
sol! {
    /// Emitted when the commission is accumulated.
    event CommissionAccumulate(uint256 indexed amount, uint256 indexed newAccPerToken, uint256 indexed totalCommission);
    
    error Zero(address account);
    error UnauthorizedAccount(address account);
}

/// Represents the ways `Vault` methods may fail.
#[derive(SolidityError)]
pub enum VaultError {
    Zero(Zero),
    UnauthorizedAccount(UnauthorizedAccount),
}

#[public] // Expose these functions publicly.
impl Vault {
    /// Returns the address of the ATON token contract 
    /// that was stored in `Vault::aton_address` during initialization.
    pub fn aton_address(&self) -> Address {
        self.aton_address.get()
    }

    /// Initializes the vault by setting the address of the ATON token contract.
    /// This can only be done once. If `aton_address` is already set, the function fails and returns `false`.
    ///
    /// # Arguments
    /// * `_aton_address` - The address of the ATON token contract to store.
    ///
    /// # Returns
    /// `true` if initialization is successful, otherwise `false`.
    pub fn initialize(&mut self, _aton_address: Address) -> bool {
        // Ensure we have not already initialized the contract.
        if self.aton_address.get() != Address::ZERO {
            return false;
        }
        // Store the provided ATON contract address in state.
        self.aton_address.set(_aton_address);
        true
    }

    /// Allows the contract to receive ATON tokens from a user and accumulates a global commission.
    /// Logs the `CommissionAccumulate` event on success.
    ///
    /// # Arguments
    /// * `amount` - The amount of ATON tokens the user is sending to accumulate as commission.
    ///
    /// # Returns
    /// * `Ok(true)` on success, or an error variant of `VaultError`.
    pub fn accumulate_aton(&mut self, amount: U256) -> Result<bool, VaultError> {
        // Reject zero-amount accumulation to avoid unnecessary transaction overhead or confusion.
        if amount == U256::ZERO {
            return Err(VaultError::Zero(Zero {
                account: msg::sender(),
            }));
        }

        // Instantiate an interface to the ATON contract,
        // and configure an internal call context for `transferFrom`.
        let aton_contract = IATON::new(self.aton_address.get());
        let config = Call::new_in(self);

        // Transfer `amount` ATON tokens from the sender to this contract (the Vault).
        aton_contract
            .transfer_from(config, msg::sender(), contract::address(), amount)
            .map_err(|_| {
                VaultError::Zero(Zero {
                    account: msg::sender(),
                })
            })?;

        // Internally update the Vault’s commission tracking with the deposited ATON.
        self._add_commission(amount)?;

        // Log the accumulation for off-chain tracking.
        evm::log(CommissionAccumulate {
            amount,
            newAccPerToken: self.accumulated_commission_per_token.get(),
            totalCommission: self.total_commission_in_aton.get(),
        });

        Ok(true)
    }

    /// Provides a summary of a player's relevant balances and commission data.
    ///
    /// # Returns
    /// A tuple of:
    /// 1) The player's native token (ETH) balance,
    /// 2) The player's ATON balance,
    /// 3) The player's unclaimed commission,
    /// 4) The player's claimed commissions (from storage),
    /// 5) The total commission in ATON currently tracked by the vault.
    pub fn summary(&self, player: Address) -> Result<(U256, U256, U256, U256, U256), VaultError> {
        let aton_balance = self._balance_of(player);
        Ok((
            player.balance(),
            aton_balance,
            self.player_commission(player),
            self.claimed_commissions.get(player),
            *self.total_commission_in_aton,
        ))
    }

    /// Calculates the unclaimed commission for a given player.
    /// 
    /// The formula: 
    /// (accumulated_commission_per_token - last_commission_per_token_for_player) * player_aton_balance
    /// 
    /// Then it scales down the result to factor in `10^18` decimals, returning the net unclaimed commission.
    ///
    /// # Arguments
    /// * `player` - The address of the player whose commission is being computed.
    ///
    /// # Returns
    /// The player's unclaimed commission (in ATON) as a `U256`.
    pub fn player_commission(&self, player: Address) -> U256 {
        // 1) Calculate the difference in commission-per-token since the player's last update.
        let owed_per_token = self
            .accumulated_commission_per_token
            .saturating_sub(self.last_commission_per_token.get(player));

        // 2) Retrieve player's ATON balance.
        let player_aton_balance = self._balance_of(player);

        // Setup scales: decimals = 10^18, pct_denom = 10^7 for further subdivision if needed.
        let decimals = U256::from(10).pow(U256::from(18));
        let pct_denom = U256::from(10000000u64);

        // 3) Multiply and scale.
        let scaled = player_aton_balance
            .checked_mul(owed_per_token)
            .unwrap_or(U256::ZERO)
            .checked_mul(pct_denom)
            .unwrap_or(U256::ZERO)
            / decimals;

        // 4) Return the final scaled value or zero if scaled is zero.
        if scaled > U256::ZERO {
            scaled / pct_denom
        } else {
            U256::ZERO
        }
    }

    /// Resets a player's unclaimed commission to zero by aligning their `last_commission_per_token`
    /// with the current global `accumulated_commission_per_token`. This can only be invoked by
    /// the ATON contract (verified by `msg::sender()`).
    ///
    /// Typically called internally when a player's ATON tokens are transferred via the overridden
    /// transfer logic in the ATON contract.
    pub fn clear_commission(&mut self, player: Address) -> Result<(), VaultError> {
        // Only the ATON contract can call this function.
        if msg::sender() != self.aton_address.get() {
            return Err(VaultError::UnauthorizedAccount(UnauthorizedAccount {
                account: msg::sender(),
            }));
        }

        // Align this player's `last_commission_per_token` with the global accumulated_commission_per_token.
        self.last_commission_per_token
            .setter(player)
            .set(self.accumulated_commission_per_token.get());

        Ok(())
    }
}

// -- Private Functions --------------------------------------------------------
impl Vault {
    /// Updates the global commission counters when new ATON is deposited.
    /// 
    /// It calculates how much commission is added per token by dividing 
    /// `new_commission_aton` by the total supply of ATON tokens, scaled by `10^18`.
    /// 
    /// Then, it updates the stored `accumulated_commission_per_token` and
    /// `total_commission_in_aton`.
    ///
    /// # Arguments
    /// * `new_commission_aton` - The amount of newly deposited ATON.
    ///
    /// # Returns
    /// `Ok(())` if commission added successfully, or an error variant if the call fails.
    pub fn _add_commission(&mut self, new_commission_aton: U256) -> Result<(), VaultError> {
        // Get a handle to the ATON contract interface.
        let aton_contract = IATON::new(self.aton_address.get());
        let config = Call::new_in(self);

        // Retrieve the total supply of ATON to compute the new commission rate.
        let total_supply_tokens = aton_contract.total_supply(config).map_err(|_| {
            VaultError::Zero(Zero {
                account: msg::sender(),
            })
        })?;

        // Avoid division by zero if the supply is somehow zero.
        if total_supply_tokens > U256::ZERO {
            // Commission per token scaled by 10^18 to maintain precision.
            let additional_commission =
                (new_commission_aton * U256::from(10).pow(U256::from(18u8))) / total_supply_tokens;

            // Update the global `accumulated_commission_per_token`.
            self.accumulated_commission_per_token
                .set(self.accumulated_commission_per_token.get() + additional_commission);

            // Update the total ATON commission stored.
            self.total_commission_in_aton
                .set(self.total_commission_in_aton.get() + new_commission_aton);
        }

        Ok(())
    }

    /// Returns the ATON balance for a given `player` by calling `balanceOf` on the ATON contract.
    /// If the call fails, this function returns `U256::ZERO`.
    ///
    /// # Arguments
    /// * `player` - The address whose ATON balance is to be retrieved.
    ///
    /// # Returns
    /// The ATON balance of the player (`U256`), or zero if an error occurs.
    pub fn _balance_of(&self, player: Address) -> U256 {
        // Instantiate the ATON contract interface.
        let aton_contract = IATON::new(self.aton_address.get());

        // Attempt to fetch the player's ATON balance, defaulting to zero on error.
        match aton_contract.balance_of(self, player) {
            Ok(balance) => balance,
            Err(_) => U256::ZERO,
        }
    }
}
