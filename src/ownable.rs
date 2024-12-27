//! Implementation of the ERC-20 standard
//!
//! The eponymous [`Ownable`] type provides all the standard methods,
//! and is intended to be inherited by other contract types.

//! Note that this code is unaudited and not fit for production use.

// Imported packages
use alloy_primitives::{Address, U256};
use alloy_sol_types::sol;
use stylus_sdk::{evm, msg, prelude::*};

sol_storage! {
    /// Ownable implements all ERC-20 methods.
    pub struct Ownable {

        address _owner;

    }
}

// Declare events and Solidity error types
sol! {


    // Ownership
    event OwnershipTransferred(address indexed previous_owner, address indexed new_owner);
    error OwnableUnauthorizedAccount(address account);
    error OwnableInvalidOwner(address owner);
}

#[derive(SolidityError)]
pub enum OwnableError {
    UnauthorizedAccount(OwnableUnauthorizedAccount),
    InvalidOwner(OwnableInvalidOwner),
}

// These methods aren't exposed to other contracts
// Methods marked as "pub" here are usable outside of the Ownable module (i.e. they're callable from lib.rs)
// Note: modifying storage will become much prettier soon
impl Ownable {
    pub fn only_owner(&self) -> Result<(), OwnableError> {
        let account = msg::sender();
        if self._owner.get() != account {
            return Err(OwnableError::UnauthorizedAccount(
                OwnableUnauthorizedAccount { account },
            ));
        }

        Ok(())
    }

    pub fn _transfer_ownership(&mut self, new_owner: Address) {
        let previous_owner = self._owner.get();
        self._owner.set(new_owner);
        evm::log(OwnershipTransferred {
            previous_owner,
            new_owner,
        });
    }
}

// These methods are public to other contracts
// Note: modifying storage will become much prettier soon
#[public]
impl Ownable {
    fn owner(&self) -> Address {
        self._owner.get()
    }

    fn transfer_ownership(&mut self, new_owner: Address) -> Result<(), OwnableError> {
        self.only_owner()?;

        if new_owner.is_zero() {
            return Err(OwnableError::InvalidOwner(OwnableInvalidOwner {
                owner: Address::ZERO,
            }));
        }

        self._transfer_ownership(new_owner);

        Ok(())
    }
}
