// tests/Vault_test.rs

#[cfg(test)]
mod tests {
    use crate::Vault;
    use stylus_sdk::{
        alloy_primitives::{address, Address, U256},
        msg,
        prelude::*,
    };
    // If you are not actually using these two, comment them out:
    // use crate::test::constants::env_vars::{get_env_vars, EnvVars};
    const VAULT_ADDRESS: &str = "0x7e32B54800705876D3B5CfBC7d9C226A211F7C1A";
    const ARENATON_ENGINE: &str = "0x7e32B54800705876D3B5CfBC7d9C226A211F7C1A";
    const ATON_ADDRESS: &str = "0x7e32B54800705876D3B5CfBC7d9C226A211F7C1A";
    #[motsu::test]
    fn initialize(contract: Vault) {
        assert!(contract.aton_address() == Address::ZERO);

                    let aton_address = ATON_ADDRESS;
            let parsed: Address = aton_address
                .parse()
                .expect("Should parse valid hex address");
        assert!(contract.initialize(parsed));


        assert!(contract.aton_address() == parsed);

        assert!(true);
    }
    #[motsu::test]
    fn summary(contract: Vault) {

        assert!(true);
    }
  
}
