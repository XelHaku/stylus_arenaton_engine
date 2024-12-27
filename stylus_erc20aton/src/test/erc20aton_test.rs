// tests/erc20aton_test.rs

#[cfg(test)]
mod tests {
    use crate::Erc20Aton;
    use stylus_sdk::{
        alloy_primitives::{address, Address, U256}, msg,
        prelude::*,
    };

    // Vault address constant used for testing
    const VAULT_ADDRESS: &str = "0x7e32B54800705876D3B5CfBC7d9C226A211F7C1A";
    // ArenaTon engine address constant used for testing
    const ARENATON_ENGINE: &str = "0x7e32B54800705876D3B5CfBC7d9C226A211F7C1A";

    /// Test the ERC20 contract parameters such as name, symbol, decimals, and vault address.
    #[motsu::test]
    fn erc20params(contract: Erc20Aton) {
        let name = contract.name();
        let symbol = contract.symbol();
        let decimals = contract.decimals();
        let vault_address = contract.vault_address();

        println!(
            "\n\nName: {}, Symbol: {}, Decimals: {}, Vault Address: {}",
            name, symbol, decimals, vault_address
        );

        // Assert the expected values for ERC20 token parameters
        assert_eq!(decimals, 18u8);
        assert_eq!(name, "ATON Stylus");
        assert_eq!(symbol, "ATON");
        assert!(vault_address.is_zero()); // Vault address should initially be zero
    }

    /// Test the initialization of the ERC20 contract.
    #[motsu::test]
    fn initialize(contract: Erc20Aton) {
        // Ensure the contract initializes successfully
        assert!(contract.initialize());
        
        // Check and print the owner of the contract after initialization
        let owner = contract.owner();
        println!("\n\nOwner: {}", owner);
        assert!(!owner.is_zero()); // Owner should not be zero after initialization
    }

    /// Test the functionality for setting the vault address.
    #[motsu::test]
    fn set_vault(contract: Erc20Aton) {
        // Parse the vault address from the constant
        let parsed: Address = VAULT_ADDRESS
            .parse()
            .expect("Should parse valid hex address");

        // Expected vault address in lowercase for comparison
        let expected = address!("7e32b54800705876d3b5cfbc7d9c226a211f7c1a");

        // Assert that the parsed address matches the expected address
        assert_eq!(parsed, expected);

        // Set the vault address in the contract
        contract.set_vault(parsed);

        // Verify that the vault address was correctly set
        assert_eq!(contract.vault_address(), parsed);
    }


    /// Test updating and verifying the ArenaTon engine functionality.
    #[motsu::test]
    fn update_new_arenaton_engine(contract: Erc20Aton) {
        let sender = msg::sender(); // Get the address of the sender

        // Verify that the sender is not initially set as a stake engine
        assert!(!contract.is_stake_engine(sender));

        // Initialize the contract
        assert!(contract.initialize());

        // Verify that the sender is the owner after initialization
        assert!(sender == contract.owner());

        // Update the sender to be recognized as a stake engine
        assert!(contract.update_stake_engine(sender, true).is_ok());

        // Verify that the sender is now recognized as a stake engine
        assert!(contract.is_stake_engine(sender));
    }
}
