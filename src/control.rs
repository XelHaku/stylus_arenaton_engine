//! Implementation of the ERC-20 standard
//!
//! The eponymous [`AccessControl`] type provides all the standard methods,
//! and is intended to be inherited by other contract types.

//! Note that this code is unaudited and not fit for production use.

// Imported packages
use crate::constants;
use alloy_primitives::FixedBytes;
use alloy_primitives::{Address, B256, U256};
use alloy_sol_types::sol;
use stylus_sdk::{evm, msg, prelude::*};

sol_storage! {
    /// AccessControl implements all ERC-20 methods.
    pub struct AccessControl {
        /// Role identifier -> Role information.
        mapping(bytes32 => RoleData) _roles;
    }
    pub struct RoleData {
        /// Whether an account is member of a certain role.
        mapping(address => bool) has_role;
        /// The admin role for this role.
        bytes32 admin_role;
    }
}

// Declare events and Solidity error types
sol! {

    // Access Control
    event RoleAdminChanged(bytes32 indexed role, bytes32 indexed previous_admin_role, bytes32 indexed new_admin_role);
    event RoleGranted(bytes32 indexed role, address indexed account, address indexed sender);
    event RoleRevoked(bytes32 indexed role, address indexed account, address indexed sender);
    error AccessControlUnauthorizedAccount(address account, bytes32 needed_role);
    error AccessControlBadConfirmation();

}

/// Represents the ways methods may fail.
#[derive(SolidityError)]
pub enum ATONError {
    AccessUnauthorizedAccount(AccessControlUnauthorizedAccount),
    BadConfirmation(AccessControlBadConfirmation),
}

// These methods aren't exposed to other contracts
// Methods marked as "pub" here are usable outside of the AccessControl module (i.e. they're callable from lib.rs)
// Note: modifying storage will become much prettier soon
impl AccessControl {
    /// Sets `admin_role` as `role`'s admin role.
    ///
    /// # Arguments
    ///
    /// * `&mut self` - Write access to the contract's state.
    /// * `role` - The identifier of the role we are changing the admin to.
    /// * `new_admin_role` - The new admin role.
    ///
    /// # Events
    ///
    /// Emits a [`RoleAdminChanged`] event.
    pub fn _set_role_admin(&mut self, role: B256, new_admin_role: B256) {
        let previous_admin_role = self.get_role_admin(role);
        self._roles.setter(role).admin_role.set(new_admin_role);
        evm::log(RoleAdminChanged {
            role,
            previous_admin_role,
            new_admin_role,
        });
    }

    /// Checks if `account` has been granted `role`.
    ///
    /// # Arguments
    ///
    /// * `&self` - Read access to the contract's state.
    /// * `role` - The role identifier.
    /// * `account` - The account to check for membership.
    ///
    /// # Errors
    ///
    /// If [`msg::sender`] has not been granted `role`, then the error
    /// [`Error::AccessUnauthorizedAccount`] is returned.
    pub fn _check_role(&self, role: B256, account: Address) -> Result<(), ATONError> {
        if !self.has_role(role, account) {
            return Err(ATONError::AccessUnauthorizedAccount(
                AccessControlUnauthorizedAccount {
                    account,
                    needed_role: role,
                },
            ));
        }

        Ok(())
    }

    /// Attempts to grant `role` to `account` and returns a boolean indicating
    /// if `role` was granted.
    ///
    /// Internal function without access restriction.
    ///
    /// # Arguments
    ///
    /// * `&mut self` - Write access to the contract's state.
    /// * `role` - The role identifier.
    /// * `account` - The account which will be granted the role.
    ///
    /// # Events
    ///
    /// May emit a [`RoleGranted`] event.
    pub fn _grant_role(&mut self, role: B256, account: Address) -> bool {
        if self.has_role(role, account) {
            false
        } else {
            self._roles.setter(role).has_role.insert(account, true);
            evm::log(RoleGranted {
                role,
                account,
                sender: msg::sender(),
            });
            true
        }
    }

    /// Attempts to revoke `role` from `account` and returns a boolean
    /// indicating if `role` was revoked.
    ///
    /// Internal function without access restriction.
    ///
    /// # Arguments
    ///
    /// * `&mut self` - Write access to the contract's state.
    /// * `role` - The role identifier.
    /// * `account` - The account which will be granted the role.
    ///
    /// # Events
    ///
    /// May emit a [`RoleRevoked`] event.
    pub fn _revoke_role(&mut self, role: B256, account: Address) -> bool {
        if self.has_role(role, account) {
            self._roles.setter(role).has_role.insert(account, false);
            evm::log(RoleRevoked {
                role,
                account,
                sender: msg::sender(),
            });
            true
        } else {
            false
        }
    }
}

// These methods are public to other contracts
// Note: modifying storage will become much prettier soon
#[public]
impl AccessControl {
    #[must_use]
    pub fn has_role(&self, role: B256, account: Address) -> bool {
        self._roles.getter(role).has_role.get(account)
    }

    pub fn only_role(&self, role: B256) -> Result<(), ATONError> {
        self._check_role(role, msg::sender())
    }

    #[must_use]
    pub fn get_role_admin(&self, role: B256) -> B256 {
        *self._roles.getter(role).admin_role
    }

    // pub fn grant_role(&mut self, role: B256, account: Address) -> Result<(), ATONError> {
    //     let admin_role = self.get_role_admin(role);
    //     self.only_role(admin_role)?;
    //     self._grant_role(role, account);
    //     Ok(())
    // }

    pub fn grant_oracle_role(&mut self, account: Address) -> Result<(), ATONError> {
        let admin_role = self.get_role_admin(FixedBytes::from(constants::ARENATON_ENGINE_ROLE));
        self.only_role(admin_role)?;
        self._grant_role(FixedBytes::from(constants::ARENATON_ENGINE_ROLE), account); // Add missing closing parenthesis
        Ok(())
    }

    pub fn revoke_oracle_role(&mut self, account: Address) -> Result<(), ATONError> {
        let admin_role = self.get_role_admin(FixedBytes::from(constants::ARENATON_ENGINE_ROLE));
        self.only_role(admin_role)?;

        self._revoke_role(FixedBytes::from(constants::ARENATON_ENGINE_ROLE), account);
        Ok(())
    }
}
